//! Verifies effective graph occurrence uniqueness.
//!
//! Requirement validation points:
//! - Contextual aliases identify unique Action occurrences.

use test_framework_oss::is_ok;

use super::admit_effective_graph;
use crate::value::{
    definition::DefinitionKind,
    effective_graph::{CanonicalActionSelector, EffectiveActionOccurrence},
    package::{ManifestDigest, PackageReleaseIdentity, PackageReleasePurl},
};

fn occurrence() -> EffectiveActionOccurrence {
    EffectiveActionOccurrence::new(
        is_ok!(CanonicalActionSelector::try_new(
            "release-readiness.database-connection"
        )),
        PackageReleaseIdentity::new(
            is_ok!(PackageReleasePurl::try_new(
                "pkg:attestify/acme.example/verification-procedure/release/readiness@1.0.0"
            )),
            is_ok!(ManifestDigest::try_new(format!(
                "sha256:{}",
                "a".repeat(64)
            ))),
        ),
    )
}

/// Requirement validation: duplicate contextual occurrences are rejected.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn duplicate_contextual_occurrence_error() {
    let value = occurrence();
    let error = admit_effective_graph(vec![value.clone(), value])
        .expect_err("duplicate selector must fail");

    assert_eq!(error.kind(), kernel_oss::error::Kind::InvalidInput);
    let _ = DefinitionKind::VerificationProcedure;
}
