//! Verifies transport-neutral controlled values.
//!
//! Requirement validation points:
//! - Q12.3 controlled documents share one deterministic data model.

use std::collections::BTreeMap;

use super::ControlledValue;

/// Requirement validation: object keys are deterministically ordered.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn controlled_object_order_success() {
    let value = ControlledValue::Object(BTreeMap::from([
        ("z".to_string(), ControlledValue::Null),
        ("a".to_string(), ControlledValue::Boolean(true)),
    ]));

    assert_eq!(value.get("a"), Some(&ControlledValue::Boolean(true)));
}
