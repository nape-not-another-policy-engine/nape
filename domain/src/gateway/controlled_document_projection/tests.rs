//! Verifies controlled-document gateway request bounds.
//!
//! Requirement validation points:
//! - Q12.3 payload ceiling is configurable only within the 256 MiB profile ceiling.

use kernel_oss::error::Kind;
use test_framework_oss::is_ok;

use super::{ControlledDocumentProjectionRequest, ControlledMediaProfile};

/// Requirement validation: bytes within an effective ceiling are accepted.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn controlled_document_request_success() {
    let request = is_ok!(ControlledDocumentProjectionRequest::try_new(
        ControlledMediaProfile::Json,
        br#"{"value":1}"#.to_vec(),
        1_024,
    ));
    assert_eq!(request.profile(), ControlledMediaProfile::Json);
}

/// Requirement validation: an effective ceiling cannot exceed 256 MiB.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn controlled_document_profile_ceiling_error() {
    let error = ControlledDocumentProjectionRequest::try_new(
        ControlledMediaProfile::Json,
        Vec::new(),
        268_435_457,
    )
    .expect_err("widened profile ceiling must fail");
    assert_eq!(error.kind(), Kind::InvalidInput);
}
