//! Verifies restricted-schema request ownership.
//!
//! Requirement validation points:
//! - Q12.3 restricted validation never mutates controlled values.

use super::{RestrictedSchemaProfile, RestrictedSchemaValidationRequest};
use crate::value::controlled_value::ControlledValue;

/// Requirement validation: request preserves exact schema and instance values.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn restricted_schema_request_success() {
    let request = RestrictedSchemaValidationRequest::new(
        RestrictedSchemaProfile::AttestifyJsonSchema202012Profile1,
        ControlledValue::Boolean(true),
        ControlledValue::String("candidate".to_string()),
    );

    assert_eq!(request.schema(), &ControlledValue::Boolean(true));
    assert_eq!(
        request.instance(),
        &ControlledValue::String("candidate".to_string())
    );
}
