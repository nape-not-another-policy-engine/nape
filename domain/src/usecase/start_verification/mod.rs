//! Prepare and atomically select one new staged Verification run.

#[cfg(test)]
mod tests;

use std::{collections::BTreeMap, sync::Arc};

use kernel_oss::{
    error::{Error, Kind},
    gateway::{
        current_utc_timestamp::CurrentUTCTimestampGW, new_identity::NewIdentityGW, AsyncGateway,
        Gateway, VoidGateway,
    },
    response::ResponseFuture,
    usecase::AsyncUseCase,
};

use crate::{
    diagnostic::NapeOutcome,
    gateway::{
        local_definition_package_acquisition::LocalDefinitionPackageAcquisitionGW,
        oci_definition_package_resolution::OciDefinitionPackageResolutionGW,
        verification_start_commit::{
            VerificationStartCommitGW, VerificationStartCommitObservation,
            VerificationStartCommitRequest,
        },
        verification_subject_acquisition::VerificationSubjectAcquisitionGW,
    },
    service::effective_graph::resolve_effective_verification_graph,
    value::{
        current_verification::{
            CurrentVerification, VerificationAcquisition, VerificationEvidenceState,
            VerificationInvocationId, VerificationStartState,
        },
        invocation_metadata::{InvocationMetadata, InvocationMetadataEntry},
        package::{LocalPackageHandle, PackageReleaseIdentity, VerifiedPackageClosure},
        subject::VerificationSubjectHandle,
    },
};

/// The two approved Procedure acquisition modes.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum VerificationProcedureSource {
    /// One exact verified local build result.
    Local(LocalPackageHandle),
    /// One exact OCI package-release identity.
    Oci(PackageReleaseIdentity),
}

/// Complete staged Start request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StartVerificationRequest {
    source: VerificationProcedureSource,
    subject: VerificationSubjectHandle,
    metadata: Vec<InvocationMetadataEntry>,
}

impl StartVerificationRequest {
    /// Starts a builder.
    pub fn builder() -> StartVerificationRequestBuilder {
        StartVerificationRequestBuilder::default()
    }
    /// Returns the selected source mode.
    pub fn source(&self) -> &VerificationProcedureSource {
        &self.source
    }
    /// Returns the subject document handle.
    pub fn subject(&self) -> &VerificationSubjectHandle {
        &self.subject
    }
    /// Returns ordered caller metadata.
    pub fn metadata(&self) -> &[InvocationMetadataEntry] {
        &self.metadata
    }
}

/// Builder for [`StartVerificationRequest`].
#[derive(Default)]
pub struct StartVerificationRequestBuilder {
    source: Option<VerificationProcedureSource>,
    subject: Option<VerificationSubjectHandle>,
    metadata: Vec<InvocationMetadataEntry>,
}

impl StartVerificationRequestBuilder {
    /// Sets the exact Procedure source.
    pub fn source(mut self, value: VerificationProcedureSource) -> Self {
        self.source = Some(value);
        self
    }
    /// Sets the subject input handle.
    pub fn subject(mut self, value: VerificationSubjectHandle) -> Self {
        self.subject = Some(value);
        self
    }
    /// Sets ordered caller metadata.
    pub fn metadata(mut self, value: Vec<InvocationMetadataEntry>) -> Self {
        self.metadata = value;
        self
    }
    /// Validates and builds the request.
    pub fn try_build(self) -> Result<StartVerificationRequest, Error> {
        Ok(StartVerificationRequest {
            source: self.source.ok_or_else(|| {
                Error::for_user(
                    Kind::InvalidInput,
                    "Verification Procedure source is required",
                )
            })?,
            subject: self.subject.ok_or_else(|| {
                Error::for_user(Kind::InvalidInput, "Verification subject is required")
            })?,
            metadata: self.metadata,
        })
    }
}

/// Completed Start meaning; no Report identity exists yet.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct StartedVerification {
    /// Complete frozen current state.
    pub current: CurrentVerification,
    /// Atomic current-run commitment observation.
    pub commitment: VerificationStartCommitObservation,
}

/// Bounded Start outcome.
pub type StartVerificationOutcome = NapeOutcome<StartedVerification>;

/// Public marker seam for staged Verification Start.
pub trait StartVerificationUC:
    AsyncUseCase<Request = StartVerificationRequest, Response = StartVerificationOutcome>
{
}

/// Core Start orchestration.
pub struct StartVerification {
    local: Arc<dyn LocalDefinitionPackageAcquisitionGW>,
    oci: Arc<dyn OciDefinitionPackageResolutionGW>,
    subject: Arc<dyn VerificationSubjectAcquisitionGW>,
    commit: Arc<dyn VerificationStartCommitGW>,
    new_identity: Arc<dyn NewIdentityGW>,
    current_time: Arc<dyn CurrentUTCTimestampGW>,
}

impl StartVerification {
    /// Creates the use case with exact acquisition, identity, time, and commit capabilities.
    pub fn new(
        local: Arc<dyn LocalDefinitionPackageAcquisitionGW>,
        oci: Arc<dyn OciDefinitionPackageResolutionGW>,
        subject: Arc<dyn VerificationSubjectAcquisitionGW>,
        commit: Arc<dyn VerificationStartCommitGW>,
        new_identity: Arc<dyn NewIdentityGW>,
        current_time: Arc<dyn CurrentUTCTimestampGW>,
    ) -> Self {
        Self {
            local,
            oci,
            subject,
            commit,
            new_identity,
            current_time,
        }
    }
}

impl AsyncUseCase for StartVerification {
    type Request = StartVerificationRequest;
    type Response = StartVerificationOutcome;

    fn execute<'a>(&'a self, request: Self::Request) -> ResponseFuture<'a, Self::Response> {
        Box::pin(async move {
            let (closure, acquisition): (VerifiedPackageClosure, VerificationAcquisition) =
                match request.source {
                    VerificationProcedureSource::Local(handle) => (
                        match Gateway::execute(
                            self.local.as_ref() as &dyn LocalDefinitionPackageAcquisitionGW,
                            handle,
                        )? {
                            NapeOutcome::Completed(value) => value,
                            NapeOutcome::Rejected(value) => {
                                return Ok(NapeOutcome::Rejected(value))
                            }
                        },
                        VerificationAcquisition::LocalBuild,
                    ),
                    VerificationProcedureSource::Oci(identity) => (
                        match AsyncGateway::execute(
                            self.oci.as_ref() as &dyn OciDefinitionPackageResolutionGW,
                            identity,
                        )
                        .await?
                        {
                            NapeOutcome::Completed(value) => value,
                            NapeOutcome::Rejected(value) => {
                                return Ok(NapeOutcome::Rejected(value))
                            }
                        },
                        VerificationAcquisition::OciPull,
                    ),
                };
            if acquisition == VerificationAcquisition::LocalBuild
                && !closure.root().lock().is_empty()
            {
                return Err(Error::for_user(
                    Kind::InvalidInput,
                    "local Start accepts only one exact embedded Procedure build with an empty Lock",
                ));
            }
            let graph = resolve_effective_verification_graph(&closure)?;
            let subject = match Gateway::execute(
                self.subject.as_ref() as &dyn VerificationSubjectAcquisitionGW,
                request.subject,
            )? {
                NapeOutcome::Completed(value) => value,
                NapeOutcome::Rejected(value) => return Ok(NapeOutcome::Rejected(value)),
            };
            let identity = VoidGateway::execute(self.new_identity.as_ref() as &dyn NewIdentityGW)?;
            let timestamp =
                VoidGateway::execute(self.current_time.as_ref() as &dyn CurrentUTCTimestampGW)?;
            let requirements = graph
                .occurrences
                .iter()
                .map(|occurrence| {
                    let detail = occurrence.detail().ok_or_else(|| {
                        Error::for_system(
                            Kind::ProcessingFailure,
                            "effective occurrence omitted execution detail",
                        )
                    })?;
                    crate::value::evidence::EvidenceRequirement::try_new(
                        occurrence.selector().clone(),
                        detail.evidence_name.clone(),
                        crate::value::evidence::EvidenceFileName::try_new(
                            detail
                                .evidence_file
                                .rsplit('/')
                                .next()
                                .unwrap_or(&detail.evidence_file),
                        )?,
                        detail.maximum_evidence_bytes,
                    )
                })
                .collect::<Result<Vec<_>, Error>>()?;
            let current = CurrentVerification::new(
                VerificationStartState::new(
                    VerificationInvocationId::try_new(identity.to_string())?,
                    subject,
                    InvocationMetadata::from_entries(request.metadata),
                    closure,
                    timestamp.as_milli(),
                ),
                VerificationEvidenceState::new(requirements, Vec::new(), BTreeMap::new()),
            )
            .with_resolution(graph, acquisition);
            let commitment = match Gateway::execute(
                self.commit.as_ref() as &dyn VerificationStartCommitGW,
                VerificationStartCommitRequest {
                    verification: current.clone(),
                },
            )? {
                NapeOutcome::Completed(value) => value,
                NapeOutcome::Rejected(value) => return Ok(NapeOutcome::Rejected(value)),
            };
            let current = current.with_managed_locations(
                commitment.current_run.clone(),
                commitment.result_output.clone(),
            );
            Ok(NapeOutcome::Completed(StartedVerification {
                current,
                commitment,
            }))
        })
    }
}

impl StartVerificationUC for StartVerification {}
