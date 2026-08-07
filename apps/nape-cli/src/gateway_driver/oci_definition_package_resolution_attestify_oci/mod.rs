//! Exact OCI root-plus-Lock closure resolution driver.

#[cfg(test)]
mod tests;

use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use kernel_oss::{gateway::AsyncGateway, response::ResponseFuture};
use nape_domain::{
    diagnostic::NapeOutcome,
    gateway::oci_definition_package_resolution::OciDefinitionPackageResolutionGW,
    value::package::{PackageReleaseIdentity, VerifiedPackageClosure},
};

use super::{
    definition_package_profile_attestify_oci::definition_package_profile,
    package_support::{project_closure, translate_oci_error, VerifiedPackageStore},
};

static NEXT_RESOLUTION: AtomicU64 = AtomicU64::new(1);

/// Resolves one exact root and every exact package named by its canonical Lock.
pub struct AttestifyOciDefinitionPackageResolutionDriver {
    store: VerifiedPackageStore,
    registry_map: attestify_oci_oss::registry::RegistryMap,
    staging_root: PathBuf,
}

impl AttestifyOciDefinitionPackageResolutionDriver {
    /// Creates a resolver with explicit mapping and private staging.
    pub fn new(
        store: VerifiedPackageStore,
        registry_map: attestify_oci_oss::registry::RegistryMap,
        staging_root: impl Into<PathBuf>,
    ) -> Self {
        Self {
            store,
            registry_map,
            staging_root: staging_root.into(),
        }
    }
}

impl AsyncGateway for AttestifyOciDefinitionPackageResolutionDriver {
    type Request = PackageReleaseIdentity;
    type Response = NapeOutcome<VerifiedPackageClosure>;

    fn execute<'a>(&'a self, request: Self::Request) -> ResponseFuture<'a, Self::Response> {
        Box::pin(async move {
            let operation = self.staging_root.join(format!(
                "resolution-{}-{}",
                std::process::id(),
                NEXT_RESOLUTION.fetch_add(1, Ordering::Relaxed)
            ));
            let resolved = match attestify_oci_oss::registry::resolve_verified_package_closure(
                definition_package_profile()?,
                &self.registry_map,
                request.purl().value(),
                request.manifest_digest().value(),
                &operation,
            )
            .await
            {
                Ok(value) => value,
                Err(error) => {
                    return Ok(NapeOutcome::Rejected(translate_oci_error(error)?));
                }
            };
            let projected =
                project_closure(&resolved.closure.root, &resolved.closure.dependencies)?;
            self.store.insert(resolved.closure.root)?;
            for dependency in resolved.closure.dependencies {
                self.store.insert(dependency)?;
            }
            Ok(NapeOutcome::Completed(projected))
        })
    }
}

impl OciDefinitionPackageResolutionGW for AttestifyOciDefinitionPackageResolutionDriver {}
