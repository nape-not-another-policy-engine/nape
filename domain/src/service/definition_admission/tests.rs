//! Verifies common definition-envelope admission.
//!
//! Requirement validation points:
//! - Verification Procedure, Activity, and Action roots use apiVersion 2.0.0.

use std::collections::BTreeMap;

use test_framework_oss::is_ok;

use super::{admit_build_definition, admit_definition};
use crate::value::{controlled_value::ControlledValue, definition::DefinitionKind};

/// Requirement validation: a controlled Procedure envelope is admitted.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn verification_procedure_envelope_success() {
    let document = ControlledValue::Object(BTreeMap::from([
        (
            "apiVersion".to_string(),
            ControlledValue::String("2.0.0".to_string()),
        ),
        (
            "kind".to_string(),
            ControlledValue::String("VerificationProcedure".to_string()),
        ),
        (
            "spec".to_string(),
            ControlledValue::Object(BTreeMap::from([(
                "name".to_string(),
                ControlledValue::String("release-readiness".to_string()),
            )])),
        ),
    ]));
    let admitted = is_ok!(admit_definition(document));

    assert_eq!(admitted.kind(), DefinitionKind::VerificationProcedure);
}

fn text(value: &str) -> ControlledValue {
    ControlledValue::String(value.to_string())
}

fn object(values: &[(&str, ControlledValue)]) -> ControlledValue {
    ControlledValue::Object(
        values
            .iter()
            .map(|(name, value)| ((*name).to_string(), value.clone()))
            .collect(),
    )
}

fn action_root(extra: Option<(&str, ControlledValue)>) -> ControlledValue {
    let mut action = BTreeMap::from([
        ("claim".to_string(), text("The endpoint is approved.")),
        (
            "evidence".to_string(),
            object(&[
                ("file", text("application-configuration.json")),
                ("name", text("application-configuration")),
            ]),
        ),
        ("label".to_string(), text("Database Connection")),
        ("name".to_string(), text("database-connection")),
        (
            "test".to_string(),
            object(&[
                ("file", text("database-connection.py")),
                ("name", text("database-connection")),
            ]),
        ),
    ]);
    if let Some((name, value)) = extra {
        action.insert(name.to_string(), value);
    }
    object(&[
        ("apiVersion", text("2.0.0")),
        ("kind", text("VerificationAction")),
        ("spec", ControlledValue::Object(action)),
    ])
}

/// Requirement validation: a controlled independent Action derives its exact static paths.
#[test]
fn functional_action_static_paths_success() {
    let admitted = is_ok!(admit_build_definition(
        &action_root(None),
        "pkg:attestify/acme.example/verification-action/database/database-connection@1.0.0",
    ));

    assert_eq!(admitted.build.name, "database-connection");
    assert_eq!(
        admitted.build.authored_paths,
        vec![
            "action/database-connection/database-connection.py",
            "verification-action.yaml",
        ]
    );
}

/// Requirement validation: an unknown controlled Action field is rejected by Domain semantics.
#[test]
fn functional_action_unknown_field_error() {
    let result = admit_build_definition(
        &action_root(Some(("shadow", text("not admitted")))),
        "pkg:attestify/acme.example/verification-action/database/database-connection@1.0.0",
    );

    assert!(result.is_err());
}
