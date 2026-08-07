//! NAPE CLI composition root and explicit dependency injection.

mod runtime_configuration;
#[cfg(test)]
mod tests;

use std::{path::PathBuf, sync::Arc};

use kernel_oss::{
    error::{Error, Kind},
    gateway::AsyncGateway,
    response::ResponseFuture,
    usecase::{AsyncUseCase, UseCase},
};
use nape_domain::{
    diagnostic::NapeOutcome,
    gateway::oci_definition_package_resolution::OciDefinitionPackageResolutionGW,
    usecase::{
        build_definition_package::{
            BuildDefinitionPackage, BuildDefinitionPackageOutcome, BuildDefinitionPackageRequest,
        },
        collect_verification_evidence::{
            CollectVerificationEvidence, CollectVerificationEvidenceOutcome,
            CollectVerificationEvidenceRequest,
        },
        publish_definition_package::{
            PublishDefinitionPackage, PublishDefinitionPackageOutcome,
            PublishDefinitionPackageRequest,
        },
        resolve_definition_package_plan::{
            ResolveDefinitionPackagePlan, ResolveDefinitionPackagePlanOutcome,
            ResolveDefinitionPackagePlanRequest,
        },
        start_verification::{
            StartVerification, StartVerificationOutcome, StartVerificationRequest,
        },
        verify_procedure::{VerifyProcedure, VerifyProcedureOutcome, VerifyProcedureRequest},
    },
    value::package::{PackageReleaseIdentity, VerifiedPackageClosure},
};

use crate::gateway_driver::{
    action_evaluation_nape_evaluator::NapeEvaluatorActionEvaluationDriver,
    application_identity_local,
    authored_definition_source_local::LocalAuthoredDefinitionSourceDriver,
    controlled_document_projection::ControlledDocumentProjectionDriver,
    current_run_state::CurrentRunStateStore,
    current_utc_timestamp_system::SystemUtcTimestampDriver,
    current_verification_acquisition_local::LocalCurrentVerificationAcquisitionDriver,
    definition_package_build_attestify_oci::AttestifyOciDefinitionPackageBuildDriver,
    definition_package_publication_attestify_oci::AttestifyOciDefinitionPackagePublicationDriver,
    evidence_acquisition_local::LocalEvidenceAcquisitionDriver,
    local_definition_package_acquisition_attestify_oci::AttestifyOciLocalPackageAcquisitionDriver,
    new_identity_ulid::UlidIdentityDriver,
    oci_definition_package_resolution_attestify_oci::AttestifyOciDefinitionPackageResolutionDriver,
    package_support::VerifiedPackageStore,
    restricted_schema_validation::RestrictedSchemaValidationDriver,
    verification_evidence_commit_local::LocalVerificationEvidenceCommitDriver,
    verification_outcome_commit_local::LocalVerificationOutcomeCommitDriver,
    verification_start_commit_local::LocalVerificationStartCommitDriver,
    verification_subject_acquisition_local::LocalVerificationSubjectAcquisitionDriver,
};

use self::runtime_configuration::select_evaluators;

/// One process-local composition root. Durable package and run custody remain
/// in their dedicated drivers across commands.
pub struct NapeApplication {
    home: PathBuf,
    working_directory: PathBuf,
    staging_root: PathBuf,
    packages: VerifiedPackageStore,
    build_digest: String,
}

impl NapeApplication {
    /// Reads the bounded process configuration owned by the composition root.
    pub fn initialize() -> Result<Self, Error> {
        let home = std::env::var_os("HOME")
            .map(PathBuf::from)
            .ok_or_else(|| Error::for_system(Kind::GatewayError, "HOME is unavailable"))?;
        let working_directory = std::env::current_dir().map_err(|_| {
            Error::for_system(
                Kind::GatewayError,
                "command working directory is unavailable",
            )
        })?;
        let executable = std::env::current_exe().map_err(|_| {
            Error::for_system(Kind::GatewayError, "running NAPE executable is unavailable")
        })?;
        let build_digest = application_identity_local::executable_digest(&executable)?;
        Ok(Self {
            home,
            working_directory,
            staging_root: std::env::temp_dir().join("nape-cli-v2"),
            packages: VerifiedPackageStore::default(),
            build_digest,
        })
    }

    /// Returns the exact running NAPE build digest used by Reports and Receipts.
    pub fn build_digest(&self) -> &str {
        &self.build_digest
    }

    /// Returns the current invocation context required only by a failed
    /// governed Verify Receipt.
    pub fn current_verify_receipt_context(&self) -> Result<(String, String), Error> {
        let current = self.state().load()?;
        let source = match current.acquisition() {
            nape_domain::value::current_verification::VerificationAcquisition::LocalBuild => {
                "local-package"
            }
            nape_domain::value::current_verification::VerificationAcquisition::OciPull => {
                "oci-registry"
            }
        };
        Ok((
            current.invocation_id().value().to_string(),
            source.to_string(),
        ))
    }

    /// Executes deterministic package build.
    pub fn build(
        &self,
        request: BuildDefinitionPackageRequest,
    ) -> Result<BuildDefinitionPackageOutcome, Error> {
        let projection = Arc::new(ControlledDocumentProjectionDriver);
        let local = Arc::new(AttestifyOciLocalPackageAcquisitionDriver::new(
            self.packages.clone(),
            self.staging_root.join("package-build-dependency"),
        ));
        UseCase::execute(
            &BuildDefinitionPackage::new(
                Arc::new(LocalAuthoredDefinitionSourceDriver),
                projection,
                local,
                Arc::new(AttestifyOciDefinitionPackageBuildDriver::new(
                    self.packages.clone(),
                )),
            ),
            request,
        )
    }

    /// Publishes one exact package build result.
    pub async fn publish(
        &self,
        request: PublishDefinitionPackageRequest,
        registry_map: attestify_oci::registry::RegistryMap,
    ) -> Result<PublishDefinitionPackageOutcome, Error> {
        AsyncUseCase::execute(
            &PublishDefinitionPackage::new(
                Arc::new(AttestifyOciLocalPackageAcquisitionDriver::new(
                    self.packages.clone(),
                    self.staging_root.join("package-publish-admission"),
                )),
                Arc::new(ControlledDocumentProjectionDriver),
                Arc::new(AttestifyOciDefinitionPackagePublicationDriver::new(
                    self.packages.clone(),
                    registry_map,
                    self.staging_root.join("package-publication"),
                )),
            ),
            request,
        )
        .await
    }

    /// Resolves one exact OCI package plan.
    pub async fn resolve(
        &self,
        request: ResolveDefinitionPackagePlanRequest,
        registry_map: attestify_oci::registry::RegistryMap,
    ) -> Result<ResolveDefinitionPackagePlanOutcome, Error> {
        AsyncUseCase::execute(
            &ResolveDefinitionPackagePlan::new(
                Arc::new(AttestifyOciDefinitionPackageResolutionDriver::new(
                    self.packages.clone(),
                    registry_map,
                    self.staging_root.join("package-resolution"),
                )),
                Arc::new(ControlledDocumentProjectionDriver),
            ),
            request,
        )
        .await
    }

    /// Starts one staged local or OCI Verification run.
    pub async fn start(
        &self,
        request: StartVerificationRequest,
        registry_map: Option<attestify_oci::registry::RegistryMap>,
    ) -> Result<StartVerificationOutcome, Error> {
        let state = self.state();
        let oci: Arc<dyn OciDefinitionPackageResolutionGW> = match registry_map {
            Some(value) => Arc::new(AttestifyOciDefinitionPackageResolutionDriver::new(
                self.packages.clone(),
                value,
                self.staging_root.join("verification-resolution"),
            )),
            None => Arc::new(UnavailableOciResolution),
        };
        AsyncUseCase::execute(
            &StartVerification::new(
                Arc::new(AttestifyOciLocalPackageAcquisitionDriver::new(
                    self.packages.clone(),
                    self.staging_root.join("verification-local-admission"),
                )),
                oci,
                Arc::new(LocalVerificationSubjectAcquisitionDriver),
                Arc::new(LocalVerificationStartCommitDriver::new(state)),
                Arc::new(UlidIdentityDriver),
                Arc::new(SystemUtcTimestampDriver),
            ),
            request,
        )
        .await
    }

    /// Adds or replaces one Evidence payload in the current run.
    pub fn evidence(
        &self,
        request: CollectVerificationEvidenceRequest,
    ) -> Result<CollectVerificationEvidenceOutcome, Error> {
        let state = self.state();
        UseCase::execute(
            &CollectVerificationEvidence::new(
                Arc::new(LocalCurrentVerificationAcquisitionDriver::new(
                    state.clone(),
                )),
                Arc::new(LocalEvidenceAcquisitionDriver),
                Arc::new(LocalVerificationEvidenceCommitDriver::new(state)),
            ),
            request,
        )
    }

    /// Executes and atomically commits the current run.
    pub async fn verify(&self) -> Result<VerifyProcedureOutcome, Error> {
        let state = self.state();
        let evaluator = select_evaluators()?;
        let use_case = VerifyProcedure::new(
            Arc::new(LocalCurrentVerificationAcquisitionDriver::new(state)),
            Arc::new(NapeEvaluatorActionEvaluationDriver::new(
                evaluator,
                self.staging_root.join("evaluator"),
            )),
            Arc::new(LocalVerificationOutcomeCommitDriver),
            Arc::new(UlidIdentityDriver),
            Arc::new(SystemUtcTimestampDriver),
        )
        .with_controlled_evidence(
            Arc::new(ControlledDocumentProjectionDriver),
            Arc::new(RestrictedSchemaValidationDriver),
            self.build_digest.clone(),
        );
        AsyncUseCase::execute(&use_case, VerifyProcedureRequest).await
    }

    fn state(&self) -> CurrentRunStateStore {
        CurrentRunStateStore::new(
            self.home.clone(),
            self.working_directory.clone(),
            self.packages.clone(),
        )
    }
}

struct UnavailableOciResolution;

impl AsyncGateway for UnavailableOciResolution {
    type Request = PackageReleaseIdentity;
    type Response = NapeOutcome<VerifiedPackageClosure>;

    fn execute<'a>(&'a self, _request: Self::Request) -> ResponseFuture<'a, Self::Response> {
        Box::pin(async {
            Err(Error::for_system(
                Kind::GatewayError,
                "OCI resolution was not configured for this local Start",
            ))
        })
    }
}

impl OciDefinitionPackageResolutionGW for UnavailableOciResolution {}
