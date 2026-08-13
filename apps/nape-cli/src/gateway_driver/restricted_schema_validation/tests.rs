//! Verifies deterministic restricted JSON Schema behavior.
//!
//! Requirement validation points:
//! - Q12.3 excludes unknown keywords, remote references, and mutation.

use std::collections::BTreeMap;

use kernel_oss::gateway::Gateway;
use nape_domain::{
    diagnostic::NapeOutcome,
    gateway::restricted_schema_validation::{
        RestrictedSchemaProfile, RestrictedSchemaValidationGW, RestrictedSchemaValidationRequest,
    },
    value::controlled_value::ControlledValue,
};

use super::RestrictedSchemaValidationDriver;

/// Requirement validation: an exact same-document schema validates without mutation.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn restricted_schema_validation_success() {
    let schema = ControlledValue::Object(BTreeMap::from([
        (
            "$schema".to_string(),
            ControlledValue::String("https://json-schema.org/draft/2020-12/schema".to_string()),
        ),
        (
            "type".to_string(),
            ControlledValue::String("string".to_string()),
        ),
    ]));
    let request = RestrictedSchemaValidationRequest::new(
        RestrictedSchemaProfile::AttestifyJsonSchema202012Profile1,
        schema,
        ControlledValue::String("candidate".to_string()),
    );
    let outcome = Gateway::execute(
        &RestrictedSchemaValidationDriver as &dyn RestrictedSchemaValidationGW,
        request,
    )
    .expect("driver seam must succeed");

    assert!(matches!(outcome, NapeOutcome::Completed(_)));
}

/// Requirement validation: excluded `pattern` fails before general validation.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn excluded_pattern_keyword_error() {
    let schema = ControlledValue::Object(BTreeMap::from([(
        "pattern".to_string(),
        ControlledValue::String("^a".to_string()),
    )]));
    let request = RestrictedSchemaValidationRequest::new(
        RestrictedSchemaProfile::AttestifyJsonSchema202012Profile1,
        schema,
        ControlledValue::String("a".to_string()),
    );
    let outcome = Gateway::execute(
        &RestrictedSchemaValidationDriver as &dyn RestrictedSchemaValidationGW,
        request,
    )
    .expect("driver seam must succeed");

    assert!(matches!(outcome, NapeOutcome::Rejected(_)));
}
