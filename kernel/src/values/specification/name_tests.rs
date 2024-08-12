
use crate::error::{Audience, Kind};
use crate::values::specification::name::Name;

#[test]
fn new_name_success() {
    let result = Name::try_from("test-name");

    assert!(result.is_ok());
    let name = result.unwrap();
    assert_eq!(name.value, "test-name");
}
#[test]
fn new_name_success_lowercase_capital_letters() {
    let result = Name::try_from("Test-Name");

    assert!(result.is_ok());
    let name = result.unwrap();
    assert_eq!(name.value, "test-name");
}
#[test]
fn new_name_success_with_leading_dash() {
    let result = Name::try_from("-test-name");

    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.message, "The name cannot start or end with a dash, and you provided '-test-name'.".to_string());
}
#[test]
fn new_name_success_with_trailing_dash() {
    let result = Name::try_from("test-name-");

    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.message, "The name cannot start or end with a dash, and you provided 'test-name-'.".to_string());
}
#[test]
fn test_new_name_error_with_spaces() {
    let result = Name::try_from("some test name");

    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.message, "You provided the name 'some test name' which contains invalid characters. A name must contain only alphanumeric characters and dashes.".to_string());

}
#[test]
fn new_name_error_with_special_character() {
    let result = Name::try_from("some_test_name");

    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.message, "You provided the name 'some_test_name' which contains invalid characters. A name must contain only alphanumeric characters and dashes.".to_string());
}
#[test]
fn test_new_name_with_empty_name() {
    let result = Name::try_from("     ");

    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.message, "You provided an empty name. A name must contain at least one character.".to_string())
}
