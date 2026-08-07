//! Exact local build-result acquisition through `attestify-oci`.

#[cfg(test)]
mod tests;

use std::{
    path::Path,
    sync::atomic::{AtomicU64, Ordering},
};

use kernel_oss::{error::Error, gateway::Gateway};
use nape_domain::{
    diagnostic::NapeOutcome,
    gateway::local_definition_package_acquisition::LocalDefinitionPackageAcquisitionGW,
    value::package::{LocalPackageHandle, VerifiedPackageClosure},
};

use super::{
    definition_package_profile_attestify_oci::definition_package_profile,
    package_support::{project_closure, translate_package_error, VerifiedPackageStore},
};

static NEXT_STAGE: AtomicU64 = AtomicU64::new(1);

/// Verifies one local package result and retains its exact bytes outside Domain.
pub struct AttestifyOciLocalPackageAcquisitionDriver {
    store: VerifiedPackageStore,
    staging_root: std::path::PathBuf,
}

impl AttestifyOciLocalPackageAcquisitionDriver {
    /// Creates a driver with explicit private staging and shared package custody.
    pub fn new(store: VerifiedPackageStore, staging_root: impl Into<std::path::PathBuf>) -> Self {
        Self {
            store,
            staging_root: staging_root.into(),
        }
    }
}

impl Gateway for AttestifyOciLocalPackageAcquisitionDriver {
    type Request = LocalPackageHandle;
    type Response = NapeOutcome<VerifiedPackageClosure>;

    fn execute(&self, request: Self::Request) -> Result<Self::Response, Error> {
        let staging = self.staging_root.join(format!(
            "local-package-{}-{}",
            std::process::id(),
            NEXT_STAGE.fetch_add(1, Ordering::Relaxed)
        ));
        let result = attestify_oci::admit_local_package(
            definition_package_profile()?,
            Path::new(request.value()),
            &staging,
        );
        let outcome = match result {
            Ok(package) => {
                let projected = project_closure(&package, &[])?;
                self.store.insert(package)?;
                NapeOutcome::Completed(projected)
            }
            Err(error) => NapeOutcome::Rejected(translate_package_error(error)?),
        };
        Ok(outcome)
    }
}

impl LocalDefinitionPackageAcquisitionGW for AttestifyOciLocalPackageAcquisitionDriver {}
