//! Exact package publication driver backed by `attestify-oci`.

#[cfg(test)]
mod tests;

use std::{
    path::PathBuf,
    sync::atomic::{AtomicU64, Ordering},
};

use attestify_oci::registry::RegistryLocation;
use kernel_oss::{
    error::{Error, Kind},
    gateway::AsyncGateway,
    response::ResponseFuture,
};
use nape_domain::{
    diagnostic::NapeOutcome,
    gateway::definition_package_publication::{
        DefinitionPackagePublicationGW, PublishedDefinitionPackage, RegistryLocationObservation,
    },
    value::package::VerifiedPackageClosure,
};

use super::package_support::{translate_oci_error, VerifiedPackageStore};

static NEXT_PUBLICATION: AtomicU64 = AtomicU64::new(1);

/// Publishes exact verified bytes and proves a clean re-pull.
pub struct AttestifyOciDefinitionPackagePublicationDriver {
    store: VerifiedPackageStore,
    registry_map: attestify_oci::registry::RegistryMap,
    staging_root: PathBuf,
}

impl AttestifyOciDefinitionPackagePublicationDriver {
    /// Creates a publication driver with explicit mapping and staging.
    pub fn new(
        store: VerifiedPackageStore,
        registry_map: attestify_oci::registry::RegistryMap,
        staging_root: impl Into<PathBuf>,
    ) -> Self {
        Self {
            store,
            registry_map,
            staging_root: staging_root.into(),
        }
    }
}

impl AsyncGateway for AttestifyOciDefinitionPackagePublicationDriver {
    type Request = VerifiedPackageClosure;
    type Response = NapeOutcome<PublishedDefinitionPackage>;

    fn execute<'a>(&'a self, request: Self::Request) -> ResponseFuture<'a, Self::Response> {
        Box::pin(async move {
            let raw = self.store.get(request.root().identity())?;
            let operation = self.staging_root.join(format!(
                "publication-{}-{}",
                std::process::id(),
                NEXT_PUBLICATION.fetch_add(1, Ordering::Relaxed)
            ));
            let published = match attestify_oci::registry::publish_verified_package(
                &self.registry_map,
                &raw,
                &operation,
            )
            .await
            {
                Ok(value) => value,
                Err(error) => {
                    return Ok(NapeOutcome::Rejected(translate_oci_error(error)?));
                }
            };
            self.store.insert(published.clean_repull)?;
            let location = publication_location(published.location)?;
            Ok(NapeOutcome::Completed(PublishedDefinitionPackage {
                package: request,
                location,
                disposition: published.disposition.to_string(),
                clean_repull_equal: true,
            }))
        })
    }
}

impl DefinitionPackagePublicationGW for AttestifyOciDefinitionPackagePublicationDriver {}

fn publication_location(value: RegistryLocation) -> Result<RegistryLocationObservation, Error> {
    if value.reference_class != "manifest-digest" {
        return Err(Error::for_system(
            Kind::ProcessingFailure,
            "Attestify OCI publication returned an unsupported reference class",
        ));
    }
    Ok(RegistryLocationObservation {
        profile_version: value.profile_version,
        publisher: value.publisher,
        scheme: value.scheme,
        registry: value.registry,
        repository: value.repository,
        reference: value.publication_tag,
        reference_class: "tag".to_string(),
    })
}
