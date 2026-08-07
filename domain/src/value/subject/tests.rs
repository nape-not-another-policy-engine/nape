//! Verifies the closed Verification subject.
//!
//! Requirement validation points:
//! - I0-I4 subject input remains `{arn, label?, description?}`.

use test_framework_oss::is_ok;

use super::VerificationSubject;

/// Requirement validation: the external RiskSource ARN remains on the subject.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn verification_subject_success() {
    let subject = is_ok!(VerificationSubject::try_new(
        "risk-source:01KWJK2M4W6AX3XJ7C0Q8N5R2T",
        Some("Orders Service".to_string()),
        None,
    ));

    assert!(subject.arn().starts_with("risk-source:"));
}
