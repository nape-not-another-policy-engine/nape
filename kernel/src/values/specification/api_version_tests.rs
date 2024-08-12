
use crate::error::{Audience, Kind};
use crate::values::specification::api_version::APIVersion;

#[test]
fn new_api_version_success() {
    let api_version = APIVersion::new(1, 2, 3);
    assert_eq!(api_version.major, 1);
    assert_eq!(api_version.minor, 2);
    assert_eq!(api_version.patch, 3);
    assert_eq!(api_version.as_string(), "1.2.3");
}

#[test]
fn from_str_api_version_success() {
    let api_version = APIVersion::from_str("1.2.3").unwrap();
    assert_eq!(api_version.major, 1);
    assert_eq!(api_version.minor, 2);
    assert_eq!(api_version.patch, 3);
    assert_eq!(api_version.as_string(), "1.2.3");
}

#[test]
fn new_api_version_error_empty_input() {
    let error = APIVersion::from_str("  ").unwrap_err();
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.message, "You provided an empty version. Please provide a version value which conforms to the Semver specification of 'major.minor.patch'.");
}

#[test]
fn new_api_version_error_invalid_input() {
    let error = APIVersion::from_str("soemthing-invalid").unwrap_err();
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.message, "Version string 'soemthing-invalid' is not in the format 'major.minor.patch'.");
}

#[test]
fn new_api_version_error_invalid_major() {
    let error = APIVersion::from_str("a.2.3").unwrap_err();
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.message, "The major version 'a' from the supplied api version of 'a.2.3' is not a valid number. It must be a number between 0 and 255.");
}

#[test]
fn new_api_version_error_invalid_minor() {
    let error = APIVersion::from_str("1.b.3").unwrap_err();
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.message, "The minor version 'b' from the supplied api version of '1.b.3' is not a valid number.  It must be a number between 0 and 255");
}

#[test]
fn new_api_version_error_invalid_patch() {
    let error = APIVersion::from_str("1.2.c").unwrap_err();
    assert_eq!(error.kind, Kind::InvalidInput);
    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.message, "The patch version 'c' from the supplied api version of '1.2.c' is not a valid number. It must be a number between 0 and 255");
}