use kernel_oss::gateway::AsyncGateway;
use nape_domain::{
    diagnostic::NapeOutcome,
    value::package::{ManifestDigest, PackageReleaseIdentity, PackageReleasePurl},
};

use super::{AttestifyOciDefinitionPackageResolutionDriver, VerifiedPackageStore};

/// Requirement validation: exercises one bounded logical path.

#[tokio::test]
async fn non_local_transport_is_rejected_before_network_use_error_async() {
    let map = attestify_oci_oss::registry::parse_registry_map(b"profileVersion: attestify-oci-registry-map/1\npublishers:\n  acme.example:\n    scheme: https\n    registry: registry.example\n    repositoryPrefix: attestify\n").expect("map");
    let driver = AttestifyOciDefinitionPackageResolutionDriver::new(
        VerifiedPackageStore::default(),
        map,
        std::env::temp_dir(),
    );
    let request = PackageReleaseIdentity::new(
        PackageReleasePurl::try_new(
            "pkg:attestify/acme.example/verification-procedure/release/readiness@1.0.0",
        )
        .expect("PURL"),
        ManifestDigest::try_new(format!("sha256:{}", "a".repeat(64))).expect("digest"),
    );
    let outcome = AsyncGateway::execute(&driver, request)
        .await
        .expect("gateway");
    let NapeOutcome::Rejected(diagnostic) = outcome else {
        panic!("transport accepted");
    };
    assert_eq!(diagnostic.code(), "oci_transport_policy_violation");
}
