use crate::error;
use crate::error::Error;
use crate::values::specification::outcome::Outcome;

#[test]
fn new_outcome_enum_success() {
    let result = Outcome::try_from("fail");
    assert_eq!(result, Ok(Outcome::FAIL));
}
#[test]
fn new_outcome_enum_error_empty_value() {
    let result = Outcome::try_from("");
    assert_eq!(result, Err(Error::for_user(error::Kind::InvalidInput, "You have provided an empty Outcome value. Please provide an Outcome value.".to_string())));
}

#[test]
fn new_outcome_enum_error_invalid_input_value() {
    let outcome = Outcome::try_from("invalid");
    assert_eq!(outcome, Err(Error::for_user(error::Kind::InvalidInput, "'invalid' is not a valid Outcome. Must be one of: [fail, inconclusive, pass, error].".to_string())));
}

#[test]
fn to_string_fail() {
    let outcome = Outcome::FAIL;
    assert_eq!(outcome.to_string(), "fail");
}

#[test]
fn to_string_inconclusive() {
    let outcome = Outcome::INCONCLUSIVE;
    assert_eq!(outcome.to_string(), "inconclusive");
}

#[test]
fn to_string_pass() {
    let outcome = Outcome::PASS;
    assert_eq!(outcome.to_string(), "pass");
}

#[test]
fn to_string_error() {
    let outcome = Outcome::ERROR;
    assert_eq!(outcome.to_string(), "error");
}

#[test]
fn default_outcome() {
    let outcome = Outcome::default();
    assert_eq!(outcome, Outcome::INCONCLUSIVE);
}

#[test]
fn handle_uppercase_input_success() {
    let pass_outcome =  Outcome::try_from("PASS").unwrap();
    let fail_outcome =  Outcome::try_from("Fail").unwrap();
    let inconclusive_outcome =  Outcome::try_from("InCoNcLuSiVe").unwrap();
    let error_outcome =  Outcome::try_from("ERROR").unwrap();
    assert_eq!(pass_outcome, Outcome::PASS);
    assert_eq!(fail_outcome, Outcome::FAIL);
    assert_eq!(inconclusive_outcome, Outcome::INCONCLUSIVE);
    assert_eq!(error_outcome, Outcome::ERROR);
}
