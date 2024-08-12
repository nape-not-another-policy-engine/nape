
use crate::error::Audience;
use crate::values::specification::description::Description;

#[test]
fn new_description_success() {
    let result = Description::try_from("This is a description.");
    assert!(result.is_ok());
}
#[test]
fn new_description_success_trim_whitespace() {
    let result = Description::try_from("    Some Description     ");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.value, "Some Description");
}
#[test]
fn new_description_success_trim_tabs() {
    let result = Description::try_from("\t\t\t\tSome Description\t\t\t\t");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.value, "Some Description");
}
#[test]
fn new_description_success_trim_newlines() {
    let result = Description::try_from("\n\n\n\nSome Description\n\n\n\n");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.value, "Some Description");
}
#[test]
fn new_description_success_trim_carriage_return() {
    let result = Description::try_from("\r\r\r\rSome Description\r\r\r\r");
    assert!(result.is_ok());
    let result = result.unwrap();
    assert_eq!(result.value, "Some Description");
}

#[test]
fn test_new_description_error_on_empty_value() {
    let result = Description::try_from("  ");
    assert!(result.is_err());

    let error = result.err().unwrap();
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, crate::error::Kind::InvalidInput);
    assert_eq!(error.message, "You provided an empty description. A description must have a value with at least one character.");
}
