use nape_kernel::error::{Audience, Error, Kind};
use nape_testing_assertions::kernel_error_eq;
use crate::evidence_collection::usecases::collect_evidence::usecase::{collect_action_evidence, CollectEvidenceRequest};

/*** Happy Path Tests ***/

#[test]
fn collect_file_success() {

    let file_name_assertion = move |file_name: &str, _file_data: &Vec<u8>, _target_directory: &str| copy_evidence_file_gateway_file_name_assertion(file_name, _file_data, _target_directory, "file.txt");

    let request = request_no_name_override();
    let result= collect_action_evidence(&request,
                                        retrieve_directory_success,
                                        retrieve_file_data_success,
                                        file_name_assertion);

    assert!(result.is_ok(), "{}", format!("An error was not expected but one was not returned: {:?}", result.err()) );

}
#[test]
fn collect_file_success_with_name_override() {

    let file_name_assertion = move |file_name: &str, _file_data: &Vec<u8>, _target_directory: &str| copy_evidence_file_gateway_file_name_assertion(file_name, _file_data, _target_directory, "new_file_name.txt");

    let request = request_with_name_override();
    let result= collect_action_evidence(&request,
                                        retrieve_directory_success,
                                        retrieve_file_data_success,
                                        file_name_assertion);

    assert!(result.is_ok(), "{}", format!("An error was returned when one was not expected: {:?}", result.err()) );

}

/*** Sad Path Tests ***/

#[test]
fn collect_file_error_action_name_error() {
    let request = CollectEvidenceRequest {
        action_name: "a band action name",
        file_path: "./some/relative/link/to/file.txt",
        file_name: None
    };

    let result= collect_action_evidence(&request,
                                        retrieve_directory_success,
                                        retrieve_file_data_success,
                                        copy_evidence_file_gateway_success);

    assert!(result.is_err(), "{}", format!("An error was expected but one was not returned: {:?}", result.unwrap()) );

    let error = result.unwrap_err();

    assert_eq!(error.audience, Audience::User);
    assert_eq!(error.kind, Kind::InvalidInput);

    let expected_message = "There is an issue with the action name 'a band action name'.".to_string();
    assert!(error.message.starts_with(&expected_message),
            "{}", format!("The error message '{}' did not start with the expected message '{}'", error.message, expected_message));
}

#[test]
fn collect_file_error_retrieve_directory_gateway_error() {

    let request = request_no_name_override();
    let result= collect_action_evidence(&request,
                                        retrieve_directory_error,
                                        retrieve_file_data_success,
                                        copy_evidence_file_gateway_success);

    kernel_error_eq!(result,
        Kind::GatewayError,
        Audience::System,
        "There was an issue retrieving the evidence directory path. Some RetrieveDirectoryPath Gateway Error");

}

#[test]
fn collect_file_error_retrieve_file_data_gateway_error() {

    let request = request_no_name_override();
    let result= collect_action_evidence(&request,
                                        retrieve_directory_success,
                                        retrieve_file_data_error,
                                        copy_evidence_file_gateway_success);

    assert!(result.is_err(), "{}", format!("An error was expected but one was not returned: {:?}", result.unwrap()) );

    let error = result.unwrap_err();

    assert_eq!(error.audience, Audience::System);
    assert_eq!(error.kind, Kind::GatewayError);

    let expected_message = "There was an issue retrieving the file './some/relative/link/to/file.txt'. Some RetrieveFileData Gateway Error".to_string();
    assert_eq!(error.message, expected_message);
}
#[test]
fn collect_file_error_copy_evidence_file_gateway_error() {

    let request = request_no_name_override();
    let result= collect_action_evidence(&request,
                                        retrieve_directory_success,
                                        retrieve_file_data_success,
                                        copy_evidence_file_gateway_error);

    kernel_error_eq!(result,
        Kind::GatewayError,
        Audience::System,
        "There was an issue copying the file './some/relative/link/to/file.txt' to 'evidence/peer-review'. Some CopyEvidenceFile Gateway Error");


}

/*** Testing Utilities, Mocks, and Assertions ***/

fn request_no_name_override() -> CollectEvidenceRequest<'static> {
    CollectEvidenceRequest {
        action_name: "peer-review",
        file_path: "./some/relative/link/to/file.txt",
        file_name: None
    }
}

fn request_with_name_override() -> CollectEvidenceRequest<'static> {
    CollectEvidenceRequest {
        action_name: "peer-review",
        file_path: "./some/relative/link/to/file.txt",
        file_name: Some("new_file_name.txt")
    }
}

/*** RetrieveDirectoryPath Gateway Mocks***/

fn retrieve_directory_success(_directory_name: &str) -> Result<String, Error> {
    Ok("evidence".to_string())
}

fn retrieve_directory_error(_directory_name: &str) -> Result<String, Error> {
    Err(Error::for_user(Kind::GatewayError, "Some RetrieveDirectoryPath Gateway Error".to_string()))
}

/*** RetrieveFileData Gateway Mocks***/

/// This assumes that the file_path which is being provided has a file name of 'file.txt'
/// This assumption is aligned with the file_path provided in the [`request_no_name_override()`] and [`request_with_name_override()`] functions.
fn retrieve_file_data_success(_file_path: &str) -> Result<(String, Vec<u8>), Error> {
    Ok( ("file.txt".to_string(), vec![1,2,3,4,5]) )
}
fn retrieve_file_data_error(_file_path: &str) -> Result<(String, Vec<u8>), Error> {
    Err(Error::for_user(Kind::GatewayError, "Some RetrieveFileData Gateway Error".to_string()))
}

/*** CopyEvidenceFile Gateway Mocks & Assertions***/

fn copy_evidence_file_gateway_success(_file_name: &str, _file_data: &Vec<u8>, _target_directory: &str) -> Result<String, Error> {
    Ok("".to_string())
}
fn copy_evidence_file_gateway_error(_file_name: &str, _file_data: &Vec<u8>, _target_directory: &str) -> Result<String, Error> {
    Err(Error::for_user(Kind::GatewayError, "Some CopyEvidenceFile Gateway Error".to_string()))
}
fn copy_evidence_file_gateway_file_name_assertion(file_name: &str, _file_data: &Vec<u8>, _target_directory: &str, expected_file_name: &str) -> Result<String, Error> {
    assert_eq!(file_name, expected_file_name.to_string(),
               "{}", format!("The file name '{}' was not the expected file name of '{}'.", file_name, expected_file_name));
    Ok("".to_string())
}
