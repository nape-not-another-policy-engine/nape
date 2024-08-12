
use crate::error::{Audience, Kind};
use crate::values::specification::file_path::FilePath;

/* Happy Path */

#[test]
fn default_success() {
    let file_path = FilePath::default();
    assert_eq!(file_path.as_str(), "");
}

#[test]
fn from_success() {
    let file_path = FilePath::from("some/file/path.file.txt");
    assert_eq!(file_path.as_str(), "some/file/path.file.txt");
}

#[test]
fn try_from_success() {
    let result = FilePath::try_from("C:\\Users\\johndoe\\Documents\\example.txt");

    assert!(result.is_ok(), "{}", format!("An error was returned when one was not expected: {:?}", result.err()));

    let file_path = result.unwrap();
    assert_eq!(file_path.as_str(), "C:\\Users\\johndoe\\Documents\\example.txt");
}

/* Error Path */


#[test]
fn try_from_missing_value_error() {
    let result = FilePath::try_from("");

    assert!(result.is_err(), "{}", format!("An was expected by one was not returned: {:?}", result.unwrap() ) );

    let err = result.unwrap_err();
    assert_eq!(err.kind, Kind::InvalidInput);
    assert_eq!(err.audience, Audience::User);
    assert!(err.message.starts_with("The file path is required, although a value was not provided."));
}