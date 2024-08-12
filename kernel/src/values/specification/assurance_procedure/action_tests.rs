
use nape_testing_assertions::is_ok;
use nape_testing_assertions::kernel_error_starts_with;
use crate::error::{Audience, Kind};
use crate::values::specification::description::Description;
use crate::values::specification::file_path::FilePath;
use crate::values::specification::name::Name;
use crate::values::specification::assurance_procedure::action::Action;
use crate::values::specification::short_description::ShortDescription;

#[test]
fn new_success() {

    let result = Action::builder()
        .name("name")
        .short_description("short")
        .long_description("long")
        .test_file_path("test")
        .evidence_file_path("evidence")
        .try_build();

    is_ok!(&result);

    let result = result.unwrap();
    assert_eq!(result.name, Name::try_from("name").unwrap());
    assert_eq!(result.short, ShortDescription::try_from("short").unwrap());
    assert_eq!(result.description, Description::try_from("long").unwrap());
    assert_eq!(result.test, FilePath::try_from("test").unwrap());
    assert_eq!(result.evidence, FilePath::try_from("evidence").unwrap());
}

#[test]
fn no_name_error() {
    let result = Action::builder()
        .short_description("short")
        .long_description("long")
        .test_file_path("test")
        .evidence_file_path("evidence")
        .try_build();

    kernel_error_starts_with!(result, Kind::InvalidInput, Audience::User, "The Action for an Assurnace Procedure could not be created. The name is required, but was not provided.");
}
#[test]
fn bad_name_error() {
    let result = Action::builder()
        .name("")
        .short_description("short")
        .long_description("long")
        .test_file_path("test")
        .evidence_file_path("evidence")
        .try_build();

    kernel_error_starts_with!(result, Kind::InvalidInput, Audience::User, "The Action for an Assurnace Procedure could not be created. There is an issue with the name ''. ");
}

#[test]
fn no_short_description_error() {

    let result = Action::builder().name("name")
        .long_description("long")
        .test_file_path("test")
        .evidence_file_path("evidence")
        .try_build();

    kernel_error_starts_with!(result, Kind::InvalidInput, Audience::User, "The Action for an Assurnace Procedure could not be created. The short description is required, but was not provided.");

}
#[test]
fn bad_short_description_error() {

        let result = Action::builder().name("name")
            .short_description("")
            .long_description("long")
            .test_file_path("test")
            .evidence_file_path("evidence")
            .try_build();

        kernel_error_starts_with!(result, Kind::InvalidInput, Audience::User, "The Action for an Assurnace Procedure could not be created. There is an issue with the short description ''. ");
}

#[test]
fn no_long_description_error() {

    let result = Action::builder().name("action-name")
        .short_description("short")
        .test_file_path("test")
        .evidence_file_path("evidence")
        .try_build();

    kernel_error_starts_with!(result, Kind::InvalidInput, Audience::User, "The Action for an Assurnace Procedure could not be created. The long description is required, but was not provided.");

}
#[test]
fn bad_long_description_error() {

    let result = Action::builder().name("action-name")
        .short_description("short")
        .long_description("")
        .test_file_path("test")
        .evidence_file_path("evidence")
        .try_build();

    kernel_error_starts_with!(result, Kind::InvalidInput, Audience::User, "The Action for an Assurnace Procedure could not be created. There is an issue with the long description ''. ");
}

#[test]
fn no_test_file_path_error() {

    let result = Action::builder().name("action-name")
        .short_description("short")
        .long_description("long")
        .evidence_file_path("evidence")
        .try_build();

    kernel_error_starts_with!(result, Kind::InvalidInput, Audience::User, "The Action for an Assurnace Procedure could not be created. The test file path is required, but was not provided.");

}
#[test]
fn bad_test_file_path_error() {

    let result = Action::builder().name("action-name")
        .short_description("short")
        .long_description("long")
        .test_file_path("")
        .evidence_file_path("evidence")
        .try_build();

    kernel_error_starts_with!(result, Kind::InvalidInput, Audience::User, "The Action for an Assurnace Procedure could not be created. There is an issue with the test file path ''. ");
}

#[test]
fn no_evidence_file_path_error() {

    let result = Action::builder().name("action-name")
        .short_description("short")
        .long_description("long")
        .test_file_path("test")
        .try_build();

    kernel_error_starts_with!(result, Kind::InvalidInput, Audience::User, "The Action for an Assurnace Procedure could not be created. The evidence file path is required, but was not provided.");

}
#[test]
fn bad_evidence_file_path_error() {

    let result = Action::builder().name("action-name")
        .short_description("short")
        .long_description("long")
        .test_file_path("test")
        .evidence_file_path("")
        .try_build();

    kernel_error_starts_with!(result, Kind::InvalidInput, Audience::User, "The Action for an Assurnace Procedure could not be created. There is an issue with the evidence file path ''. ");
}