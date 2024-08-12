
use crate::error::{Audience, Kind};
use crate::values::directory::directory_list::DirectoryList;


/*** Happy Path Tests ***/
#[test]
fn directory_list_default_success() {
    let directory_list = DirectoryList::default();
    assert_eq!(directory_list.paths.len(), 0);
}
#[test]
fn directory_list_from_vec_success() {
    let paths = vec![("path-key".to_string(), "/path/to/file".to_string())];
    let result = DirectoryList::try_from_vec(paths);

    assert!(result.is_ok(), "{}", format!("An error was returned when one was not expected: {:?}", result.err()));

    let list = result.unwrap();
    assert_eq!(list.paths.len(), 1);

}
#[test]
fn directory_list_add_success() {
    let directory_list = DirectoryList::default();
    let result = directory_list.try_add("path-key", "/path/to/directory");

    assert!(result.is_ok(), "{}", format!("An error was returned when one was not expected: {:?}", result.err()));
    let directory_list = result.unwrap();
    assert_eq!(directory_list.paths.len(), 1);
}
#[test]
fn directory_list_add_multiple_success() {
    let directory_list = DirectoryList::default();
    let result = directory_list.try_add("path-key", "/path/to/directory");

    assert!(result.is_ok(), "{}", format!("An error was returned when one was not expected: {:?}", result.err()));

    let directory_list = result.unwrap();
    assert_eq!(directory_list.paths.len(), 1);

    let result = directory_list.try_add("path-key-2", "/path/to/directory-2");

    assert!(result.is_ok(), "{}", format!("An error was returned when one was not expected: {:?}", result.err()));
    let directory_list = result.unwrap();
    assert_eq!(directory_list.paths.len(), 2);
}
#[test]
fn directory_list_get_success() {
    let directory_list = DirectoryList::default();
    let result = directory_list.try_add("path-key", "/path/to/directory");

    assert!(result.is_ok(), "{}", format!("An error was returned when one was not expected: {:?}", result.err()));

    let directory_list = result.unwrap();

    let result = directory_list.try_get("path-key");
    assert_eq!(result, Some("/path/to/directory".to_string()));

}

#[test]
fn try_merge_success() {
    let directory_list = DirectoryList::default();
    let result = directory_list.try_add("path-key", "/path/to/directory");

    assert!(result.is_ok(), "{}", format!("An error was returned when one was not expected: {:?}", result.err()));

    let directory_list = result.unwrap();

    let other_directory_list = DirectoryList::default();
    let result = other_directory_list.try_add("path-key-2", "/path/to/directory-2");

    assert!(result.is_ok(), "{}", format!("An error was returned when one was not expected: {:?}", result.err()));

    let other_directory_list = result.unwrap();

    let result = directory_list.try_merge(&other_directory_list);

    assert!(result.is_ok(), "{}", format!("An error was returned when one was not expected: {:?}", result.err()));

    let merged_directory_list = result.unwrap();
    assert_eq!(merged_directory_list.paths.len(), 2);
}

/*** Sad Path Tests ***/
#[test]
fn directory_list_error_from_empty_vec() {
    let paths = vec![];
    let result = DirectoryList::try_from_vec(paths);

    assert!(result.is_err(), "{}", format!("An error was expected but one was not returned: {:?}", result.unwrap()));

    let error = result.err().unwrap();

    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::System);
    assert_eq!(error.message, "No directory or file paths have been provided.");

}
#[test]
fn directory_list_error_add_empty_path_name() {
    let directory_list = DirectoryList::default();
    let result = directory_list.try_add("", "/path/to/directory");

    assert!(result.is_err(), "{}", format!("An error was expected but one was not returned: {:?}", result.unwrap()));

    let error = result.err().unwrap();

    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::System);
    assert_eq!(error.message, "You provided an empty path name to add to the Directory List.  Please provide a non-empty path name.");

}
#[test]
fn directory_list_error_add_duplicate_path_name() {
    let directory_list = DirectoryList::default();
    let result = directory_list.try_add("path-key", "/path/to/directory");

    assert!(result.is_ok(), "{}", format!("An error was returned when one was not expected: {:?}", result.err()));

    let directory_list = result.unwrap();

    let result = directory_list.try_add("path-key", "/path/to/directory-2");

    assert!(result.is_err(), "{}", format!("An error was expected but one was not returned: {:?}", result.unwrap()));

    let error = result.err().unwrap();

    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::System);
    assert_eq!(error.message, "The path name 'path-key' already exists in the Directory List.  Please provide a unique path name.");

}
#[test]
fn directory_list_error_add_empty_path() {
    let directory_list = DirectoryList::default();
    let result = directory_list.try_add("path-key", "");

    assert!(result.is_err(), "{}", format!("An error was expected but one was not returned: {:?}", result.unwrap()));

    let error = result.err().unwrap();

    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::System);
    assert_eq!(error.message, "You provided an empty path to add to the Directory List.  Please provide a non-empty path.");

}
#[test]
fn directory_list_error_get_not_found() {
    let directory_list = DirectoryList::default();
    let result = directory_list.try_add("path-key", "/path/to/directory");

    assert!(result.is_ok(), "{}", format!("An error was returned when one was not expected: {:?}", result.err()));

    let directory_list = result.unwrap();

    let result = directory_list.try_get("path-key-2");
    assert_eq!(result, None);

}