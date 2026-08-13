//! Verifies Test preflight defaults and limits.
//!
//! Requirement validation points:
//! - Q11 Test limits are always effective even when omitted by authors.

use test_framework_oss::is_ok;

use super::{preflight_test, EffectiveTestResources};

/// Requirement validation: a nonempty Test and positive resources are admitted.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn test_preflight_success() {
    let resources = is_ok!(preflight_test(
        b"def evaluate(evidence, evaluations, metadata): return {}",
        EffectiveTestResources {
            maximum_execution_work_units: 25_000_000,
            maximum_result_bytes: 131_072
        },
    ));
    assert_eq!(resources.maximum_result_bytes, 131_072);
}
