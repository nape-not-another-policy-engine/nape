//! Application-owned Registry Profile acquisition and exact location projection.

use std::path::Path;

use attestify_oci_oss::registry::{RegistryLocation, RegistryMap};
use kernel_oss::error::{Error, Kind};
use nape_domain::{diagnostic::NapeDiagnostic, value::package::PackageReleaseIdentity};

use super::{
    definition_package_profile_attestify_oci::definition_package_profile,
    package_support::translate_oci_error,
};

/// One admitted command-line Registry Profile source.
pub enum RegistryConfigurationSource<'a> {
    /// Controlled Registry Map document.
    Profile(&'a str),
    /// Explicit development endpoint shorthand.
    Endpoint(&'a str),
}

/// Acquires one exact Registry Map without exposing its bytes to Domain.
pub enum RegistryConfigurationFailure {
    /// Controlled command-input rejection.
    Rejected(NapeDiagnostic),
    /// Unexpected diagnostic-construction or host failure.
    Unexpected(Error),
}

/// Acquires one controlled Registry Map while preserving governed diagnostics.
pub fn acquire_registry_map(
    source: RegistryConfigurationSource<'_>,
) -> Result<RegistryMap, RegistryConfigurationFailure> {
    match source {
        RegistryConfigurationSource::Profile(path) => {
            let bytes = super::local_file::read_regular_stable(Path::new(path), 1_048_576)
                .map_err(|_| registry_map_rejection("Registry Profile file could not be read"))?;
            attestify_oci_oss::registry::parse_registry_map(&bytes).map_err(registry_rejection)
        }
        RegistryConfigurationSource::Endpoint(endpoint) => {
            attestify_oci_oss::registry::registry_map_from_endpoint(endpoint, "attestify")
                .map_err(registry_rejection)
        }
    }
}

/// Maps one exact identity using the same reusable Attestify OCI mapping
/// operation used by acquisition and publication.
pub fn project_registry_location(
    registry_map: &RegistryMap,
    identity: &PackageReleaseIdentity,
) -> Result<RegistryLocation, Error> {
    attestify_oci_oss::registry::map_package(
        registry_map,
        &definition_package_profile()?,
        identity.purl().value(),
        identity.manifest_digest().value(),
    )
    .map_err(oci_error)
}

fn oci_error(error: attestify_oci_oss::registry::OciError) -> Error {
    match translate_oci_error(error) {
        Ok(diagnostic) => Error::for_user(
            Kind::InvalidInput,
            format!("{}: {}", diagnostic.code(), diagnostic.detail()),
        ),
        Err(error) => error,
    }
}

fn registry_rejection(
    error: attestify_oci_oss::registry::OciError,
) -> RegistryConfigurationFailure {
    match translate_oci_error(error) {
        Ok(value) => RegistryConfigurationFailure::Rejected(value),
        Err(error) => RegistryConfigurationFailure::Unexpected(error),
    }
}

fn registry_map_rejection(detail: &str) -> RegistryConfigurationFailure {
    match NapeDiagnostic::try_new(
        "registry_map_invalid",
        "registry-map-invalid-v1",
        "registry-map-admission",
        detail,
    ) {
        Ok(value) => RegistryConfigurationFailure::Rejected(value),
        Err(error) => RegistryConfigurationFailure::Unexpected(error),
    }
}
