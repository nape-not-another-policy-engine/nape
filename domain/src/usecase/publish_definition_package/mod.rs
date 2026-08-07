//! Publish one exact NAPE package build result.

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
        definition_package_publication::{
            DefinitionPackagePublicationGW, PublishedDefinitionPackage,
        },
        local_definition_package_acquisition::LocalDefinitionPackageAcquisitionGW,
    },
    value::package::LocalPackageHandle,
};

/// Complete publication request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublishDefinitionPackageRequest {
    local_package: LocalPackageHandle,
}

impl PublishDefinitionPackageRequest {
    /// Creates a request for one exact local package result.
    pub fn new(local_package: LocalPackageHandle) -> Self {
        Self { local_package }
    }
    /// Returns the exact local package handle.
    pub fn local_package(&self) -> &LocalPackageHandle {
        &self.local_package
    }
}

/// Bounded publication outcome.
pub type PublishDefinitionPackageOutcome = NapeOutcome<PublishedDefinitionPackage>;

/// Public marker seam for exact publication.
pub trait PublishDefinitionPackageUC:
    AsyncUseCase<Request = PublishDefinitionPackageRequest, Response = PublishDefinitionPackageOutcome>
{
}

/// Core publication orchestration.
pub struct PublishDefinitionPackage {
    local_acquisition: Arc<dyn LocalDefinitionPackageAcquisitionGW>,
    projection: Arc<dyn ControlledDocumentProjectionGW>,
    publication: Arc<dyn DefinitionPackagePublicationGW>,
}

impl PublishDefinitionPackage {
    /// Creates the use case with explicit dependencies.
    pub fn new(
        local_acquisition: Arc<dyn LocalDefinitionPackageAcquisitionGW>,
        projection: Arc<dyn ControlledDocumentProjectionGW>,
        publication: Arc<dyn DefinitionPackagePublicationGW>,
    ) -> Self {
        Self {
            local_acquisition,
            projection,
            publication,
        }
    }
}

impl AsyncUseCase for PublishDefinitionPackage {
    type Request = PublishDefinitionPackageRequest;
    type Response = PublishDefinitionPackageOutcome;

    fn execute<'a>(&'a self, request: Self::Request) -> ResponseFuture<'a, Self::Response> {
        Box::pin(async move {
            let package = match Gateway::execute(
                self.local_acquisition.as_ref() as &dyn LocalDefinitionPackageAcquisitionGW,
                request.local_package,
            )? {
                NapeOutcome::Completed(value) => value,
                NapeOutcome::Rejected(value) => return Ok(NapeOutcome::Rejected(value)),
            };
            let profile = match package.root().semantic_root_media_type() {
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
                    package.root().semantic_root_bytes().to_vec(),
                    4 * 1_024 * 1_024,
                )?,
            )? {
                NapeOutcome::Completed(_) => {}
                NapeOutcome::Rejected(value) => return Ok(NapeOutcome::Rejected(value)),
            }
            AsyncGateway::execute(
                self.publication.as_ref() as &dyn DefinitionPackagePublicationGW,
                package,
            )
            .await
        })
    }
}

impl PublishDefinitionPackageUC for PublishDefinitionPackage {}
