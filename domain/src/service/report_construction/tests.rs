//! Verifies Report metadata ownership.
//!
//! Requirement validation points:
//! - Report V2 preserves caller metadata and adds the three Engine-owned fields.

use std::collections::BTreeMap;

use test_framework_oss::is_ok;

use super::construct_report_envelope;
use crate::value::{
    current_verification::{
        CurrentVerification, VerificationEvidenceState, VerificationInvocationId,
        VerificationStartState,
    },
    definition::DefinitionKind,
    invocation_metadata::{InvocationMetadata, InvocationMetadataEntry},
    package::{
        ManifestDigest, PackageReleaseIdentity, PackageReleasePurl, VerifiedPackage,
        VerifiedPackageClosure,
    },
    subject::VerificationSubject,
    verification_outcome::VerificationSummary,
};

/// Requirement validation: current-run metadata is preserved into the Report envelope.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn report_invocation_metadata_success() {
    let package = is_ok!(VerifiedPackage::try_new(
        PackageReleaseIdentity::new(
            is_ok!(PackageReleasePurl::try_new(
                "pkg:attestify/acme.example/verification-procedure/release/readiness@1.0.0"
            )),
            is_ok!(ManifestDigest::try_new(format!(
                "sha256:{}",
                "a".repeat(64)
            ))),
        ),
        DefinitionKind::VerificationProcedure,
        "release-readiness",
        "application/yaml",
        Vec::new(),
        BTreeMap::new(),
        Vec::new(),
    ));
    let current = CurrentVerification::new(
        VerificationStartState::new(
            is_ok!(VerificationInvocationId::try_new(
                "01KWJK2J5W0VYF6V72JYF6DTRQ"
            )),
            is_ok!(VerificationSubject::try_new(
                "risk-source:01KWJK2M4W6AX3XJ7C0Q8N5R2T",
                None,
                None
            )),
            InvocationMetadata::from_entries(vec![is_ok!(InvocationMetadataEntry::try_new(
                "build-id", "42"
            ))]),
            VerifiedPackageClosure::new(package, Vec::new()),
            1_785_169_199_000,
        ),
        VerificationEvidenceState::new(Vec::new(), Vec::new(), BTreeMap::new()),
    );
    let report = is_ok!(construct_report_envelope(
        &current,
        "01KWJK2J5W0VYF6V72JYF6DTRA",
        "2026-08-03T00:00:00Z",
        VerificationSummary::default(),
    ));

    assert_eq!(
        report
            .get("metadata")
            .and_then(|value| value.get("build-id")),
        Some(&crate::value::controlled_value::ControlledValue::String(
            "42".to_string()
        ))
    );
}
