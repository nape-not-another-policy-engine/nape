
use crate::error::{Audience, Kind};
use crate::values::specification::short_description::ShortDescription;

#[test]
fn new_short_description() {
    let result = ShortDescription::try_from("This is a short description.");
    assert!(result.is_ok(), "{}", format!("An error was returned when one was not expected: {:?}", result.err()));
    let result = result.unwrap();
    assert_eq!(result.value, "This is a short description.");
}

#[test]
fn new_short_description_error_empty_value() {
    let result = ShortDescription::try_from("");

    assert!(result.is_err(), "{}", format!("Expected an error but non was returned: {:?}", result.unwrap() ) );
    let err = result.unwrap_err();
    assert_eq!(err.kind, Kind::InvalidInput);
    assert_eq!(err.audience, Audience::User);
    assert!(err.message.starts_with("There is an issue with your short description: "));
}

#[test]
fn test_short_description_error_too_long() {
    let max_length: usize  = 255;
    let short_description = "a".repeat(max_length + 1);
    let result = ShortDescription::try_from(short_description.as_str());

    assert!(result.is_err(), "{}", format!("An error was returned when one was not expected: {:?}", result.err()));

    let error = result.err().unwrap();
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert!(error.message.starts_with("There is an issue with your short description: The short description provided is too long. A description must have a value with at most 255 characters."));
}
