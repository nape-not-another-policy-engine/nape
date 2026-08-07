use kernel_oss::gateway::VoidGateway;

use super::SystemUtcTimestampDriver;

/// Requirement validation: exercises one bounded logical path.

#[test]
fn current_timestamp_is_positive_and_millisecond_bounded_success() {
    let value = VoidGateway::execute(&SystemUtcTimestampDriver).expect("current time");
    assert!(value.as_milli() > 0);
    assert_eq!(value.as_nano() % 1_000_000, 0);
}
