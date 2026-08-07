//! Verifies authorized evaluation criteria overrides.
//!
//! Requirement validation points:
//! - Q9 permits only package-authorized criteria assignments.

use std::collections::BTreeMap;

use test_framework_oss::is_ok;

use super::resolve_evaluation;
use crate::value::{controlled_value::ControlledValue, evaluation::EffectiveEvaluation};

/// Requirement validation: an explicitly authorized value replaces the package default.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn authorized_criteria_override_success() {
    let package = is_ok!(EffectiveEvaluation::try_new(
        "minimum-tls-version",
        BTreeMap::from([(
            "equals".to_string(),
            ControlledValue::String("1.3".to_string())
        )]),
    ));
    let requested = BTreeMap::from([(
        "equals".to_string(),
        ControlledValue::String("1.2".to_string()),
    )]);
    let authorized = BTreeMap::from([(
        "equals".to_string(),
        vec![ControlledValue::String("1.2".to_string())],
    )]);

    let effective = is_ok!(resolve_evaluation(&package, Some(&requested), &authorized));
    assert_eq!(effective.criteria().get("equals"), requested.get("equals"));
}
