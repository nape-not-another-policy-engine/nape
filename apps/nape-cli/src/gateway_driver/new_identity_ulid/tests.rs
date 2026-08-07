use kernel_oss::gateway::VoidGateway;

use super::UlidIdentityDriver;

/// Requirement validation: exercises one bounded logical path.

#[test]
fn generated_identity_is_a_canonical_ulid_success() {
    let value = VoidGateway::execute(&UlidIdentityDriver).expect("system ULID");
    let text = value.to_string();
    assert_eq!(text.len(), 26);
    assert!(kernel_oss::ulid::ULID::from_string(&text).is_ok());
}
