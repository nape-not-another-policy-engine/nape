//! Verifies effective evaluation values.
//!
//! Requirement validation points:
//! - Q9 evaluation criteria are Domain-owned controlled values.

use std::collections::BTreeMap;

use test_framework_oss::is_ok;

use super::EffectiveEvaluation;
use crate::value::controlled_value::ControlledValue;

/// Requirement validation: effective criteria retain deterministic names.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn effective_evaluation_success() {
    let evaluation = is_ok!(EffectiveEvaluation::try_new(
        "minimum-tls-version",
        BTreeMap::from([(
            "equals".to_string(),
            ControlledValue::String("1.3".to_string()),
        )]),
    ));

    assert_eq!(evaluation.name(), "minimum-tls-version");
}
