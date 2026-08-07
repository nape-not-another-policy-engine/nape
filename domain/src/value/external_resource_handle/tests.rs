//! Verifies bounded external resource handles.
//!
//! Requirement validation points:
//! - External paths are handles rather than semantic identities.

use kernel_oss::error::Kind;
use test_framework_oss::is_ok;

use super::ExternalResourceHandle;

/// Requirement validation: a nonempty opaque handle is accepted.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn external_resource_handle_success() {
    let handle = is_ok!(ExternalResourceHandle::try_new("./evidence.json"));
    assert_eq!(handle.value(), "./evidence.json");
}

/// Requirement validation: an empty opaque handle is rejected.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn empty_external_resource_handle_error() {
    let error = ExternalResourceHandle::try_new(" ").expect_err("empty handle must fail");
    assert_eq!(error.kind(), Kind::InvalidInput);
}
