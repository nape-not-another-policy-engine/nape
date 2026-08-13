//! Verifies Verification summary invariants.
//!
//! Requirement validation points:
//! - VerificationReport summary counts exactly equal Action conclusions.

use super::VerificationSummary;

/// Requirement validation: complete conclusion counts cover every Action.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn complete_verification_summary_success() {
    let summary = VerificationSummary {
        action_count: 3,
        conclusion_true: 1,
        conclusion_false: 1,
        conclusion_inconclusive: 1,
        ..VerificationSummary::default()
    };

    assert!(summary.is_complete());
}
