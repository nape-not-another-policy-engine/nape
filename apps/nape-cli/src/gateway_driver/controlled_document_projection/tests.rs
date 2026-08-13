//! Verifies controlled JSON, YAML, and TOML projection.
//!
//! Requirement validation points:
//! - Q12.3 controlled formats project to one equal immutable value.

use kernel_oss::gateway::Gateway;
use nape_domain::{
    diagnostic::NapeOutcome,
    gateway::controlled_document_projection::{
        ControlledDocumentProjectionGW, ControlledDocumentProjectionRequest, ControlledMediaProfile,
    },
};
use test_framework_oss::is_ok;

use super::ControlledDocumentProjectionDriver;

fn project(
    profile: ControlledMediaProfile,
    bytes: &[u8],
) -> nape_domain::value::controlled_value::ControlledValue {
    let request = is_ok!(ControlledDocumentProjectionRequest::try_new(
        profile,
        bytes.to_vec(),
        1_024,
    ));
    match is_ok!(Gateway::execute(
        &ControlledDocumentProjectionDriver as &dyn ControlledDocumentProjectionGW,
        request,
    )) {
        NapeOutcome::Completed(value) => value,
        NapeOutcome::Rejected(diagnostic) => panic!("unexpected rejection: {}", diagnostic.code()),
    }
}

/// Requirement validation: equivalent JSON, YAML, and TOML project equally.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn controlled_formats_project_equally_success() {
    let json = project(
        ControlledMediaProfile::Json,
        br#"{"database":{"endpoint":"primary"}}"#,
    );
    let yaml = project(
        ControlledMediaProfile::Yaml,
        b"database:\n  endpoint: primary\n",
    );
    let toml = project(
        ControlledMediaProfile::Toml,
        b"[database]\nendpoint = \"primary\"\n",
    );

    assert_eq!(json, yaml);
    assert_eq!(yaml, toml);
}

/// Requirement validation: duplicate JSON members are rejected.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn duplicate_json_member_error() {
    let request = is_ok!(ControlledDocumentProjectionRequest::try_new(
        ControlledMediaProfile::Json,
        br#"{"value":1,"value":2}"#.to_vec(),
        1_024,
    ));
    let outcome = is_ok!(Gateway::execute(
        &ControlledDocumentProjectionDriver as &dyn ControlledDocumentProjectionGW,
        request,
    ));

    assert!(matches!(outcome, NapeOutcome::Rejected(_)));
}
