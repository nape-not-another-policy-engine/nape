//! Verifies exact package-release identities.
//!
//! Requirement validation points:
//! - Attestify OCI profile Q4 requires PURL plus manifest digest.

use test_framework_oss::is_ok;

use super::{ManifestDigest, PackageReleaseIdentity, PackageReleasePurl};

/// Requirement validation: exact PURL and digest form one release identity.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn package_release_identity_success() {
    let purl = is_ok!(PackageReleasePurl::try_new(
        "pkg:attestify/acme.example/verification-action/security/tls-policy@2.1.0"
    ));
    let digest = is_ok!(ManifestDigest::try_new(format!(
        "sha256:{}",
        "a".repeat(64)
    )));
    let identity = PackageReleaseIdentity::new(purl, digest);

    assert!(identity.purl().value().contains("@2.1.0"));
    assert_eq!(identity.manifest_digest().value().len(), 71);
}
