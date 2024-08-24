
use crate::error;
use crate::error::Error;
use crate::values::specification::kind::Kind;

#[test]
fn new_kind_enum_success() {
    let result = Kind::new("AssuranceReport");
    assert_eq!(result, Ok(Kind::AssuranceReport));
}

#[test]
fn new_kind_enum_error_empty_value() {
    let result = Kind::new("");
    assert_eq!(result, Err(Error::for_user(error::Kind::InvalidInput, "You have provided an empty Kind value. Please provide a Kind value.".to_string())));
}

#[test]
fn new_kind_enum_error_invalid_input_value() {
    let kind = Kind::new("invalid");
    assert_eq!(kind, Err(Error::for_user(error::Kind::InvalidInput, "'invalid' is not a valid Kind. Must be one of: [AssuranceReport, AssuranceProcedure].".to_string())));
}

#[test]
fn to_string_nape_assurance_report() {
    let kind = Kind::AssuranceReport;
    assert_eq!(kind.to_string(), "AssuranceReport");
}

#[test]
fn to_string_nape_procedure_definition() {
    let kind = Kind::AssuranceProcedure;
    assert_eq!(kind.to_string(), "AssuranceProcedure");
}