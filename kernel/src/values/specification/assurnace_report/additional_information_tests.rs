use nape_testing_assertions::kernel_error_contains;
use nape_testing_assertions::is_ok;
use crate::values::specification::assurnace_report::additional_information::AdditionalInformation;
use crate::error::{Audience, Kind};

#[test]
fn add_success() {
    let result = AdditionalInformation::builder().add("This is a test").try_build();

    is_ok!(&result);

    let additional_info = result.unwrap();
    assert_eq!(additional_info.count(), 1);
    assert_eq!(additional_info.list()[0].value, "This is a test");
}

#[test]
fn add_duplicate_success() {
    let result = AdditionalInformation::builder()
        .add("This is a test")
        .add("This is a test")
        .try_build();

    is_ok!(&result);

    let additional_info = result.unwrap();
    assert_eq!(additional_info.count(), 1);
    assert_eq!(additional_info.list()[0].value, "This is a test");

}

#[test]
fn add_error() {
    let  result = AdditionalInformation::builder().add(" ").try_build();

    kernel_error_contains!(&result, Kind::InvalidInput, Audience::User, "We could not add the additional information ' '. ");

}