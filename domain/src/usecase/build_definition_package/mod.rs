//! Build one deterministic local Attestify definition package.

#[cfg(test)]
mod tests;

use std::{collections::BTreeSet, sync::Arc};

use kernel_oss::{
    error::{Error, Kind},
    gateway::Gateway,
    usecase::UseCase,
};

use crate::{
    diagnostic::{NapeDiagnostic, NapeOutcome},
    gateway::{
        authored_definition_source::{
            AuthoredDefinitionSourceGW, AuthoredDefinitionSourceHandle,
            AuthoredDefinitionSourceRequest, NewPackageOutputHandle,
        },
        controlled_document_projection::{
            ControlledDocumentProjectionGW, ControlledDocumentProjectionRequest,
            ControlledMediaProfile,
        },
        definition_package_build::{
            BuiltDefinitionPackage, DefinitionPackageBuildGW, DefinitionPackageBuildRequest,
        },
        local_definition_package_acquisition::LocalDefinitionPackageAcquisitionGW,
    },
    service::definition_admission::admit_build_definition,
    value::package::{LocalPackageHandle, PackageReleasePurl},
};

/// Complete package-build request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuildDefinitionPackageRequest {
    source: AuthoredDefinitionSourceHandle,
    package: PackageReleasePurl,
    dependency_package: Vec<LocalPackageHandle>,
    output: NewPackageOutputHandle,
}

impl BuildDefinitionPackageRequest {
    /// Starts a builder.
    pub fn builder() -> BuildDefinitionPackageRequestBuilder {
        BuildDefinitionPackageRequestBuilder::default()
    }

    /// Returns the authored-source handle.
    pub fn source(&self) -> &AuthoredDefinitionSourceHandle {
        &self.source
    }
    /// Returns the intended package PURL.
    pub fn package(&self) -> &PackageReleasePurl {
        &self.package
    }
    /// Returns ordered dependency package handles.
    pub fn dependency_packages(&self) -> &[LocalPackageHandle] {
        &self.dependency_package
    }
    /// Returns the required-new output handle.
    pub fn output(&self) -> &NewPackageOutputHandle {
        &self.output
    }
}

/// Builder for [`BuildDefinitionPackageRequest`].
#[derive(Default)]
pub struct BuildDefinitionPackageRequestBuilder {
    source: Option<AuthoredDefinitionSourceHandle>,
    package: Option<PackageReleasePurl>,
    dependency_package: Vec<LocalPackageHandle>,
    output: Option<NewPackageOutputHandle>,
}

impl BuildDefinitionPackageRequestBuilder {
    /// Sets the authored source.
    pub fn source(mut self, value: AuthoredDefinitionSourceHandle) -> Self {
        self.source = Some(value);
        self
    }
    /// Sets the exact versioned PURL.
    pub fn package(mut self, value: PackageReleasePurl) -> Self {
        self.package = Some(value);
        self
    }
    /// Sets the complete ordered dependency set.
    pub fn dependency_packages(mut self, value: Vec<LocalPackageHandle>) -> Self {
        self.dependency_package = value;
        self
    }
    /// Sets the required-new output.
    pub fn output(mut self, value: NewPackageOutputHandle) -> Self {
        self.output = Some(value);
        self
    }
    /// Validates and builds the request.
    pub fn try_build(self) -> Result<BuildDefinitionPackageRequest, Error> {
        Ok(BuildDefinitionPackageRequest {
            source: self
                .source
                .ok_or_else(|| Error::for_user(Kind::InvalidInput, "package source is required"))?,
            package: self
                .package
                .ok_or_else(|| Error::for_user(Kind::InvalidInput, "package PURL is required"))?,
            dependency_package: self.dependency_package,
            output: self
                .output
                .ok_or_else(|| Error::for_user(Kind::InvalidInput, "package output is required"))?,
        })
    }
}

/// Bounded package-build outcome.
pub type BuildDefinitionPackageOutcome = NapeOutcome<BuiltDefinitionPackage>;

/// Public marker seam for deterministic package build.
pub trait BuildDefinitionPackageUC:
    UseCase<Request = BuildDefinitionPackageRequest, Response = BuildDefinitionPackageOutcome>
{
}

/// Core package-build orchestration.
pub struct BuildDefinitionPackage {
    source: Arc<dyn AuthoredDefinitionSourceGW>,
    projection: Arc<dyn ControlledDocumentProjectionGW>,
    local_acquisition: Arc<dyn LocalDefinitionPackageAcquisitionGW>,
    build: Arc<dyn DefinitionPackageBuildGW>,
}

impl BuildDefinitionPackage {
    /// Creates the use case with explicit capability dependencies.
    pub fn new(
        source: Arc<dyn AuthoredDefinitionSourceGW>,
        projection: Arc<dyn ControlledDocumentProjectionGW>,
        local_acquisition: Arc<dyn LocalDefinitionPackageAcquisitionGW>,
        build: Arc<dyn DefinitionPackageBuildGW>,
    ) -> Self {
        Self {
            source,
            projection,
            local_acquisition,
            build,
        }
    }
}

impl UseCase for BuildDefinitionPackage {
    type Request = BuildDefinitionPackageRequest;
    type Response = BuildDefinitionPackageOutcome;

    fn execute(&self, request: Self::Request) -> Result<Self::Response, Error> {
        let source = match Gateway::execute(
            self.source.as_ref() as &dyn AuthoredDefinitionSourceGW,
            AuthoredDefinitionSourceRequest::new(request.source.clone(), request.package.clone()),
        )? {
            NapeOutcome::Completed(value) => value,
            NapeOutcome::Rejected(value) => return Ok(NapeOutcome::Rejected(value)),
        };
        let root_bytes = source
            .files()
            .get(source.semantic_root())
            .cloned()
            .ok_or_else(|| {
                Error::for_system(
                    Kind::ProcessingFailure,
                    "sealed source omitted its semantic root",
                )
            })?;
        let profile = match source.semantic_root_media_type() {
            "application/json" => ControlledMediaProfile::Json,
            "application/yaml" => ControlledMediaProfile::Yaml,
            "application/toml" => ControlledMediaProfile::Toml,
            _ => {
                return Err(Error::for_system(
                    Kind::ProcessingFailure,
                    "sealed source returned an unsupported semantic-root media type",
                ))
            }
        };
        let projected = match Gateway::execute(
            self.projection.as_ref() as &dyn ControlledDocumentProjectionGW,
            ControlledDocumentProjectionRequest::try_new(
                profile,
                root_bytes.clone(),
                4 * 1_024 * 1_024,
            )?,
        )? {
            NapeOutcome::Completed(value) => value,
            NapeOutcome::Rejected(value) => return Ok(NapeOutcome::Rejected(value)),
        };
        let admitted_definition = match admit_build_definition(&projected, request.package.value())
        {
            Ok(value) => value.build,
            Err(_) => {
                return Ok(NapeOutcome::Rejected(NapeDiagnostic::try_new(
                    match source.semantic_root() {
                        "verification-action.yaml" => "verification_action_invalid",
                        "verification-activity.yaml" => "verification_activity_invalid",
                        _ => "verification_procedure_invalid",
                    },
                    match source.semantic_root() {
                        "verification-action.yaml" => "verification-action-invalid-v1",
                        "verification-activity.yaml" => "verification-activity-invalid-v1",
                        _ => "verification-procedure-invalid-v1",
                    },
                    "invocation-admission",
                    "package source failed Product definition admission",
                )?));
            }
        };
        let sealed_paths = source.files().keys().cloned().collect::<BTreeSet<_>>();
        let admitted_paths = admitted_definition
            .authored_paths
            .iter()
            .cloned()
            .collect::<BTreeSet<_>>();
        if sealed_paths != admitted_paths {
            return Ok(NapeOutcome::Rejected(NapeDiagnostic::try_new(
                "package_archive_invalid",
                "package-archive-invalid-v1",
                "archive-file-table-validation",
                "authored source contains a missing or unclassified package-owned path",
            )?));
        }
        let mut dependencies = Vec::with_capacity(request.dependency_package.len());
        for handle in request.dependency_package {
            match Gateway::execute(
                self.local_acquisition.as_ref() as &dyn LocalDefinitionPackageAcquisitionGW,
                handle,
            )? {
                NapeOutcome::Completed(value) => dependencies.push(value),
                NapeOutcome::Rejected(value) => return Ok(NapeOutcome::Rejected(value)),
            }
        }
        Gateway::execute(
            self.build.as_ref() as &dyn DefinitionPackageBuildGW,
            DefinitionPackageBuildRequest::new(
                source,
                admitted_definition,
                request.package,
                dependencies,
                request.output,
            ),
        )
    }
}

impl BuildDefinitionPackageUC for BuildDefinitionPackage {}
