//! Verifies NAPE-managed staged invocation identity.
//!
//! Requirement validation points:
//! - Each successful Start owns one invocation ULID.

use test_framework_oss::is_ok;

use super::VerificationInvocationId;

/// Requirement validation: a canonical ULID is accepted.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn verification_invocation_id_success() {
    let identity = is_ok!(VerificationInvocationId::try_new(
        "01KWJK2J5W0VYF6V72JYF6DTRQ"
    ));
    assert_eq!(identity.value().len(), 26);
}
