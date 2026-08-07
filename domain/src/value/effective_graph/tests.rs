//! Verifies canonical occurrence selectors.
//!
//! Requirement validation points:
//! - Q1 contextual Action references use `<activity>.<action>`.

use kernel_oss::error::Kind;
use test_framework_oss::is_ok;

use super::CanonicalActionSelector;

/// Requirement validation: a canonical two-segment selector is accepted.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn canonical_action_selector_success() {
    let selector = is_ok!(CanonicalActionSelector::try_new(
        "release-readiness.database-connection"
    ));
    assert_eq!(selector.value(), "release-readiness.database-connection");
}

/// Requirement validation: a Procedure-prefixed three-segment selector is rejected.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn three_segment_action_selector_error() {
    let error = CanonicalActionSelector::try_new("procedure.activity.action")
        .expect_err("three segments must fail");
    assert_eq!(error.kind(), Kind::InvalidInput);
}
