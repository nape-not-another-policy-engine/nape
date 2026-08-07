//! Deterministic package build driver backed by `attestify-oci`.

#[cfg(test)]
mod tests;

use std::path::Path;

use attestify_oci::LockEdge;
use kernel_oss::{
    error::{Error, Kind},
    gateway::Gateway,
};
use nape_domain::{
    diagnostic::{NapeDiagnostic, NapeOutcome},
    gateway::definition_package_build::{
        BuiltDefinitionPackage, DefinitionPackageBuildGW, DefinitionPackageBuildRequest,
    },
    value::{
        definition::DefinitionKind,
        package::{LockSummary, ManifestDigest, PackageReleaseIdentity, PackageReleasePurl},
    },
};

use super::{
    definition_package_profile_attestify_oci::{
        definition_kind_for_package, definition_package_profile,
    },
    package_support::{project_closure, translate_package_error, VerifiedPackageStore},
};

/// Builds exact packages from already-sealed source bytes.
pub struct AttestifyOciDefinitionPackageBuildDriver {
    store: VerifiedPackageStore,
}

impl AttestifyOciDefinitionPackageBuildDriver {
    /// Creates the driver with shared package-byte custody.
    pub fn new(store: VerifiedPackageStore) -> Self {
        Self { store }
    }
}

impl Gateway for AttestifyOciDefinitionPackageBuildDriver {
    type Request = DefinitionPackageBuildRequest;
    type Response = NapeOutcome<BuiltDefinitionPackage>;

    fn execute(&self, request: Self::Request) -> Result<Self::Response, Error> {
        let output = Path::new(request.output().value());
        let result = self.build(&request, output);
        match result {
            Ok(value) => Ok(NapeOutcome::Completed(value)),
            Err(BuildFailure::Rejected(value)) => Ok(NapeOutcome::Rejected(value)),
            Err(BuildFailure::Unexpected(value)) => Err(value),
        }
    }
}

impl DefinitionPackageBuildGW for AttestifyOciDefinitionPackageBuildDriver {}

enum BuildFailure {
    Rejected(NapeDiagnostic),
    Unexpected(Error),
}

impl From<Error> for BuildFailure {
    fn from(value: Error) -> Self {
        Self::Unexpected(value)
    }
}

impl AttestifyOciDefinitionPackageBuildDriver {
    fn build(
        &self,
        request: &DefinitionPackageBuildRequest,
        output: &Path,
    ) -> Result<BuiltDefinitionPackage, BuildFailure> {
        let profile = definition_package_profile()?;
        let kind = definition_kind_for_package(request.package().value())?;
        let source = match attestify_oci::SealedPackageSource::try_from_files(
            profile,
            kind,
            request.package().value(),
            request.source().files().clone(),
        ) {
            Ok(value) => value,
            Err(error) => return Err(BuildFailure::Rejected(translate_package_error(error)?)),
        };
        if source.semantic_root() != request.source().semantic_root()
            || source.semantic_root_media_type() != request.source().semantic_root_media_type()
        {
            return Err(Error::for_system(
                Kind::ProcessingFailure,
                "sealed source projection changed across the package boundary",
            )
            .into());
        }
        let direct_edges = request
            .admitted_definition()
            .direct_references
            .iter()
            .map(|reference| LockEdge {
                from: request.package().value().to_string(),
                to: reference.package.clone(),
                use_selector: reference.use_selector.clone(),
            })
            .collect::<Vec<_>>();
        let mut raw_dependencies = Vec::new();
        for closure in request.dependencies() {
            for package in std::iter::once(closure.root()).chain(closure.dependencies().iter()) {
                let raw = self.store.get(package.identity())?;
                raw_dependencies.push(raw);
            }
        }
        let raw = match attestify_oci::build_verified_package_from_sealed_source_with_dependencies(
            &source,
            request.package().value(),
            &raw_dependencies,
            &direct_edges,
            output,
        ) {
            Ok(value) => value,
            Err(error) => return Err(BuildFailure::Rejected(translate_package_error(error)?)),
        };
        let projected = project_closure(&raw, &raw_dependencies)?;
        self.store.insert(raw.clone())?;
        let kind = DefinitionKind::try_from_product(&raw.kind)?;
        Ok(BuiltDefinitionPackage {
            kind,
            identity: PackageReleaseIdentity::new(
                PackageReleasePurl::try_new(&raw.identity.package)?,
                ManifestDigest::try_new(&raw.identity.manifest_digest)?,
            ),
            lock: LockSummary {
                node_count: raw.lock.nodes.len() as u64,
                edge_count: raw.lock.edges.len() as u64,
            },
            output: request.output().clone(),
            verified: projected,
        })
    }
}
