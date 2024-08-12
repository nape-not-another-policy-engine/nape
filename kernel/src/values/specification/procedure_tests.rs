use nape_testing_assertions::kernel_error_eq;
use crate::error::{Audience, Kind};
use crate::values::specification::procedure::Procedure;

#[test]
fn new_procedure_success() {
    let result = Procedure::try_new("https://example.com", "some/directory");
    let procedure = result.unwrap();
    assert_eq!(procedure.repository, "https://example.com");
    assert_eq!(procedure.directory, "some/directory");
}
#[test]
fn new_procedure_success_empty_directory() {
    let result = Procedure::try_new("https://example.com", "");
    let procedure = result.unwrap();
    assert_eq!(procedure.repository, "https://example.com");
    assert_eq!(procedure.directory, "/");
}
#[test]
fn new_procedure_success_root_directory() {
    let result = Procedure::try_new("https://example.com", "/");
    let procedure = result.unwrap();
    assert_eq!(procedure.repository, "https://example.com");
    assert_eq!(procedure.directory, "/");
}

#[test]
fn new_procedure_error_invalid_procedure_repository() {
    let result = Procedure::try_new("", "some/directory");
    let error = result.unwrap_err();
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert!(error.message.contains("'' is not a valid procedure location: "));
}
#[test]
fn new_procedure_error_contiguous_slashes() {
    let result = Procedure::try_new("https://example.com", "a/bad/directory//path");
    let error = result.unwrap_err();
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.message, "The directory path cannot contain contiguous forward slashes at position 16. Please remove the extra forward slash.");
}
#[test]
fn new_procedure_error_directory_path_starts_with_slash() {
    let result = Procedure::try_new("https://example.com", "/starts/with/slash");
    let error = result.unwrap_err();
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.message, "The directory path cannot start or end with a forward slash at position.");
}
#[test]
fn new_procedure_error_directory_path_ends_with_slash() {
    let result = Procedure::try_new("https://example.com", "ends/with/slash/");
    let error = result.unwrap_err();
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.message, "The directory path cannot start or end with a forward slash at position.");
}
#[test]
fn new_procedure_error_directory_path_invalid_character() {
    let result = Procedure::try_new("https://example.com", "has@invalid/character");
    let error = result.unwrap_err();
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.message, "Invalid character at position 3: '@'. The directory path can only contain alphanumeric characters, underscores, and non-contiguous forward slashes.");
}

#[test]
fn error_directory_path_starts_with_dash() {
    let result = Procedure::try_new("https://example.com", "-starts/with/dash");

    kernel_error_eq!(result,  Kind::InvalidInput, Audience::User, "The directory path cannot start or end with a dash.");

}

#[test]
fn error_directory_path_ends_with_dash() {
    let result = Procedure::try_new("https://example.com", "ends/with/dash-");
    kernel_error_eq!(result,  Kind::InvalidInput, Audience::User, "The directory path cannot start or end with a dash.");
}
