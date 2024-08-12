use crate::error::Audience;
use crate::error::Kind;
use crate::values::time::start_time::StartTime;

#[test]
fn now_success() {
    let start_time = StartTime::now();
    assert_ne!(start_time.time, 0);
}

#[test]
fn from_success() {
    let start_time = StartTime::from(1000);
    assert_eq!(start_time.time, 1000);
}

#[test]
fn try_from_success() {
    let start_time = StartTime::try_from(1000);
    assert!(start_time.is_ok());
}

#[test]
fn to_string_success() {
    let start_time = StartTime::from(1000);
    assert_eq!(start_time.to_string(), "1000");
}

#[test]
fn try_from_zero_error() {
    let result = StartTime::try_from(0);
    assert!(result.is_err());
    let error = result.unwrap_err();
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.message, "The start time cannot be 0.  Please provide a valid utc time.");
}