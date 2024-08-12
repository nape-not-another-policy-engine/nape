
use crate::error;
use crate::values::nrn::nrn::NRN;
use crate::values::specification::description::Description;
use crate::values::specification::assurance_procedure::procedure::Procedure;
use crate::values::specification::short_description::ShortDescription;

#[test]
fn new_process() {
    let result = Procedure::new("nrn:sourcecode:example", "Short Description", "Long Description");

    assert!(result.is_ok(), "{}", format!("An error was returned when one was not expected: {:?}", result.err()));

    let procedure = result.unwrap();
    assert_eq!(procedure.nrn, NRN::new("nrn:sourcecode:example").unwrap());
    assert_eq!(procedure.short, ShortDescription::try_from("Short Description").unwrap());
    assert_eq!(procedure.description, Description::try_from("Long Description").unwrap());
}

#[test]
fn new_process_error_invalid_nrn() {
    let result = Procedure::new("invalid-nrn", "Short Description", "Long Description");

    assert!(result.is_err(), "{}", format!("An error was expected: {:?}", result.unwrap()));

    let result = result.err().unwrap();
    assert_eq!(result.kind, error::Kind::InvalidInput);
    assert_eq!(result.audience, error::Audience::User);
    assert!(result.message.starts_with("There is an issue with your procedure information: "));

}

#[test]
fn new_process_error_invalid_short_description() {
    let result = Procedure::new("nrn:sourcecode:example", " ", "Long Description");

    assert!(result.is_err(), "{}", format!("An error was expected: {:?}", result.unwrap()));

    let result = result.err().unwrap();
    assert_eq!(result.kind, error::Kind::InvalidInput);
    assert_eq!(result.audience, error::Audience::User);
    assert!(result.message.starts_with("There is an issue with your procedure information: The short description has an issue: "));
}

#[test]
fn new_process_error_invalid_long_description() {
    let result = Procedure::new("nrn:sourcecode:example", "Short Description ", " ");

    assert!(result.is_err(), "{}", format!("An error was expected: {:?}", result.unwrap()));

    let result = result.err().unwrap();
    assert_eq!(result.kind, error::Kind::InvalidInput);
    assert_eq!(result.audience, error::Audience::User);
    assert!(result.message.starts_with("There is an issue with your procedure information: The description has an issue: "));
}