
use crate::error::{Kind};
use crate::values::specification::subject_id::SubjectId;

#[test]
fn new_subject_id() {
    let subject_id = SubjectId::new("subjectid");
    assert_eq!(subject_id.is_ok(), true);
    assert_eq!(subject_id.unwrap().value, "subjectid".to_string());
}
#[test]
fn new_subject_id_error_empty() {
    let subject_id = SubjectId::new("");
    assert_eq!(subject_id.is_err(), true);
    let error = subject_id.err().unwrap();
    assert_eq!(error.message, "The Subject Id cannot be empty.");
    assert_eq!(error.kind, Kind::InvalidInput);
}
#[test]
fn new_subject_id_error_max_length() {
    let subject_id = SubjectId::new("a".repeat(257).as_str());
    assert_eq!(subject_id.is_err(), true);
    let error = subject_id.err().unwrap();
    assert_eq!(error.message, "The SubjectId exceeds the maximum length of 256 characters.");
    assert_eq!(error.kind, Kind::InvalidInput);
}
#[test]
fn new_subject_id_error_not_alphanumeric() {
    let subject_id = SubjectId::new("subject_id!");
    assert_eq!(subject_id.is_err(), true);
    let error = subject_id.err().unwrap();
    assert_eq!(error.message, "The SubjectId can only contain alphanumeric characters.");
    assert_eq!(error.kind, Kind::InvalidInput);
}