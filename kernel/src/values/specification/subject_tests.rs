
use crate::error::{Audience, Kind};
use crate::values::specification::subject::Subject;

#[test]
fn new_subject_success() {
    let nrn = "nrn:procedure:example";
    let id = "example";
    let result = Subject::try_new(nrn, id);

    assert!(result.is_ok(), "{}", format!("An error was returned when one was not expected: {:?}", result.err()));
}

#[test]
fn new_subject_error_handle_nrn_error() {
    let nrn = "bad-nrn";
    let id = "example";
    let result = Subject::try_new(nrn, id);

    assert!(result.is_err(), "{}", format!("An error was expected bu non was returned {:?}", result.unwrap()));

    let error = result.err().unwrap();
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert!(error.message.contains("We were unable to create the Subject: "));
}

#[test]
fn new_subject_error_handle_subject_id_error() {
    let nrn = "nrn:procedure:example";
    let id = "a-bad-id!";
    let result = Subject::try_new(nrn, id);

    assert!(result.is_err(), "{}", format!("An error was expected, but non was returned: {:?}", result.unwrap()));

    let error = result.err().unwrap();
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert!(error.message.contains("We were unable to create the Subject: "));
}
