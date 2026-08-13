//! Resolve one exact OCI package closure without execution.

#[cfg(test)]
mod tests;

use std::sync::Arc;

use kernel_oss::{
    error::{Error, Kind},
    gateway::{AsyncGateway, Gateway},
    response::ResponseFuture,
    usecase::AsyncUseCase,
};

use crate::{
    diagnostic::NapeOutcome,
    gateway::{
        controlled_document_projection::{
            ControlledDocumentProjectionGW, ControlledDocumentProjectionRequest,
            ControlledMediaProfile,
        },
        oci_definition_package_resolution::OciDefinitionPackageResolutionGW,
    },
    value::package::{PackageReleaseIdentity, VerifiedPackageClosure},
};

/// Complete plan-resolution request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ResolveDefinitionPackagePlanRequest {
    root: PackageReleaseIdentity,
}

impl ResolveDefinitionPackagePlanRequest {
    /// Creates an exact-root request.
    pub fn new(root: PackageReleaseIdentity) -> Self {
        Self { root }
    }
    /// Returns the exact root identity.
    pub fn root(&self) -> &PackageReleaseIdentity {
        &self.root
    }
}

/// Resolved transport-neutral package plan.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefinitionPackagePlan {
    /// Complete verified closure.
    pub closure: VerifiedPackageClosure,
    /// Canonical dependency node count.
    pub dependency_count: u64,
    /// Lock/profile validation passed.
    pub lock_validated: bool,
}

/// Bounded resolve-plan outcome.
pub type ResolveDefinitionPackagePlanOutcome = NapeOutcome<DefinitionPackagePlan>;

/// Public marker seam for exact package-plan resolution.
pub trait ResolveDefinitionPackagePlanUC:
    AsyncUseCase<
    Request = ResolveDefinitionPackagePlanRequest,
    Response = ResolveDefinitionPackagePlanOutcome,
>
{
}

/// Core exact resolution orchestration.
pub struct ResolveDefinitionPackagePlan {
    resolution: Arc<dyn OciDefinitionPackageResolutionGW>,
    projection: Arc<dyn ControlledDocumentProjectionGW>,
}

impl ResolveDefinitionPackagePlan {
    /// Creates the use case with explicit dependencies.
    pub fn new(
        resolution: Arc<dyn OciDefinitionPackageResolutionGW>,
        projection: Arc<dyn ControlledDocumentProjectionGW>,
    ) -> Self {
        Self {
            resolution,
            projection,
        }
    }
}

impl AsyncUseCase for ResolveDefinitionPackagePlan {
    type Request = ResolveDefinitionPackagePlanRequest;
    type Response = ResolveDefinitionPackagePlanOutcome;

    fn execute<'a>(&'a self, request: Self::Request) -> ResponseFuture<'a, Self::Response> {
        Box::pin(async move {
            let closure = match AsyncGateway::execute(
                self.resolution.as_ref() as &dyn OciDefinitionPackageResolutionGW,
                request.root,
            )
            .await?
            {
                NapeOutcome::Completed(value) => value,
                NapeOutcome::Rejected(value) => return Ok(NapeOutcome::Rejected(value)),
            };
            let profile = match closure.root().semantic_root_media_type() {
                "application/json" => ControlledMediaProfile::Json,
                "application/yaml" => ControlledMediaProfile::Yaml,
                "application/toml" => ControlledMediaProfile::Toml,
                _ => {
                    return Err(Error::for_system(
                        Kind::ProcessingFailure,
                        "verified package has unsupported root media type",
                    ))
                }
            };
            match Gateway::execute(
                self.projection.as_ref() as &dyn ControlledDocumentProjectionGW,
                ControlledDocumentProjectionRequest::try_new(
                    profile,
                    closure.root().semantic_root_bytes().to_vec(),
                    4 * 1_024 * 1_024,
                )?,
            )? {
                NapeOutcome::Completed(_) => {}
                NapeOutcome::Rejected(value) => return Ok(NapeOutcome::Rejected(value)),
            }
            let dependency_count = closure.dependencies().len() as u64;
            Ok(NapeOutcome::Completed(DefinitionPackagePlan {
                closure,
                dependency_count,
                lock_validated: true,
            }))
        })
    }
}

impl ResolveDefinitionPackagePlanUC for ResolveDefinitionPackagePlan {}
