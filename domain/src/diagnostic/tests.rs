//! Verifies governed diagnostic and outcome construction.
//!
//! Requirement validation points:
//! - Packet-4 diagnostic identifiers and safe detail remain bounded.

use kernel_oss::error::Kind;
use test_framework_oss::is_ok;

use super::{NapeDiagnostic, NapeOutcome};

/// Requirement validation: Packet-4 diagnostics preserve structured fields.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn governed_diagnostic_success() {
    let diagnostic = is_ok!(NapeDiagnostic::try_new(
        "verification_procedure_invalid",
        "verification-procedure-invalid-v1",
        "invocation-admission",
        "procedure is invalid",
    ));
    let outcome = NapeOutcome::<()>::Rejected(diagnostic.clone());

    assert_eq!(diagnostic.code(), "verification_procedure_invalid");
    assert_eq!(
        diagnostic.reason_template(),
        "verification-procedure-invalid-v1"
    );
    assert_eq!(diagnostic.phase(), "invocation-admission");
    assert_eq!(diagnostic.detail(), "procedure is invalid");
    assert!(matches!(outcome, NapeOutcome::Rejected(_)));
}

/// Requirement validation: ungoverned diagnostic identifiers fail at construction.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn ungoverned_diagnostic_identifier_error() {
    let error = NapeDiagnostic::try_new("Bad Code", "reason-v1", "phase", "safe")
        .expect_err("invalid identifier must fail");

    assert_eq!(error.kind(), Kind::ProcessingFailure);
}
