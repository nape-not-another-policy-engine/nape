use kernel_oss::gateway::AsyncGateway;
use nape_domain::value::{
    definition::DefinitionKind,
    package::{
        ManifestDigest, PackageReleaseIdentity, PackageReleasePurl, VerifiedPackage,
        VerifiedPackageClosure,
    },
};

use attestify_oci_oss::registry::RegistryLocation;

use super::{
    publication_location, AttestifyOciDefinitionPackagePublicationDriver, VerifiedPackageStore,
};

fn location(reference_class: &str) -> RegistryLocation {
    RegistryLocation {
        profile_version: "attestify-oci-registry-map/1".to_string(),
        publisher: "acme.example".to_string(),
        scheme: "http".to_string(),
        registry: "localhost:5001".to_string(),
        repository: "attestify/acme.example/verification-procedure/release/readiness".to_string(),
        publication_tag: "1.0.0".to_string(),
        reference_class: reference_class.to_string(),
        reference: format!("sha256:{}", "a".repeat(64)),
        complete_reference: format!(
            "http://localhost:5001/attestify/acme.example/verification-procedure/release/readiness@sha256:{}",
            "a".repeat(64)
        ),
    }
}

/// Requirement validation: publication exposes the exact pushed tag to NAPE.
#[test]
fn publication_location_projects_existing_tag_success() {
    let value =
        publication_location(location("manifest-digest")).expect("test setup should succeed");
    assert_eq!(value.reference_class, "tag");
    assert_eq!(value.reference, "1.0.0");
}

/// Requirement validation: an unknown client-library class cannot reach a Receipt.
#[test]
fn publication_location_rejects_unknown_reference_class_error() {
    assert!(publication_location(location("unknown")).is_err());
}

/// Requirement validation: exercises one bounded logical path.

#[tokio::test]
async fn unavailable_exact_package_bytes_are_an_unexpected_custody_failure_error_async() {
    let root = std::env::temp_dir().join(format!("nape-publication-driver-{}", std::process::id()));
    std::fs::create_dir_all(&root).expect("root");
    let identity = PackageReleaseIdentity::new(
        PackageReleasePurl::try_new(
            "pkg:attestify/acme.example/verification-procedure/release/readiness@1.0.0",
        )
        .expect("PURL"),
        ManifestDigest::try_new(format!("sha256:{}", "a".repeat(64))).expect("digest"),
    );
    let package = VerifiedPackage::try_new(
        identity,
        DefinitionKind::VerificationProcedure,
        "readiness",
        "application/yaml",
        Vec::new(),
        Default::default(),
        Vec::new(),
    )
    .expect("package");
    let driver = AttestifyOciDefinitionPackagePublicationDriver::new(
        VerifiedPackageStore::default(),
        attestify_oci_oss::registry::registry_map_from_endpoint(
            "http://localhost:5001",
            "attestify",
        )
        .expect("map"),
        &root,
    );
    let result =
        AsyncGateway::execute(&driver, VerifiedPackageClosure::new(package, Vec::new())).await;
    assert!(result.is_err());
    std::fs::remove_dir_all(root).expect("cleanup");
}
