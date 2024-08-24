use nape_kernel::error::{Audience, Error, Kind};
use nape_kernel::values::directory::directory_list::DirectoryList;
use nape_kernel::values::specification::file_path::FilePath;
use nape_kernel::values::specification::repository_link::RepositoryLink;
use nape_testing_assertions::kernel_error_eq;
use crate::evidence_collection::usecases::start_collection::usecase::{start_collection};
use crate::evidence_collection::usecases::start_collection::usecase_boundary::request::{StartProcedureBuilder, StartProcedure};

#[test]
fn start_collection_success() {
    let request = generate_valid_request();
    let directory_list = generate_valid_directory_list();
    let result = start_collection(
        &request,
        &directory_list,
        directory_creation_gateway_success,
        procedure_retrieval_gateway_success,
        file_move_gateway_success,
        file_delete_gateway_success
    );

    assert!(result.is_ok(), "{}", format!("An error was returned when one was not expected: {:?}", result.err()));
}

/* Directory List Test - Sad Path tests */

#[test]
fn start_collection_error_missing_home_dir() {
    let request = generate_valid_request();
    let directories = vec![
        ("evidence".to_string(), "nrn_sourcecode_example/1714646108364/evidence".to_string()),
        ("activity-test".to_string(), "nrn_sourcecode_example/1714646108364/activity".to_string()),
        ("temp".to_string(), "nrn_sourcecode_example/1714646108364/temp".to_string())
    ];
    let directory_list = DirectoryList::try_from_vec(directories).unwrap();

    let result = start_collection(
        &request,
        &directory_list,
        directory_creation_gateway_success,
        procedure_retrieval_gateway_success,
        file_move_gateway_success,
        file_delete_gateway_success
    );

    kernel_error_eq!(&result,
        Kind::NotFound,
        Audience::System,
        "We could not start the collection procedure. Could not locate the 'home' directory in the provided directory list."
    );
}
#[test]
fn start_collection_error_missing_activity_dir() {
    let request = generate_valid_request();
    let directories = vec![
        ("home".to_string(), "nrn_sourcecode_example/1714646108364".to_string()),
        ("evidence".to_string(), "nrn_sourcecode_example/1714646108364/evidence".to_string()),
        ("temp".to_string(), "nrn_sourcecode_example/1714646108364/temp".to_string())
    ];
    let directory_list = DirectoryList::try_from_vec(directories).unwrap();

    let result = start_collection(
        &request,
        &directory_list,
        directory_creation_gateway_success,
        procedure_retrieval_gateway_success,
        file_move_gateway_success,
        file_delete_gateway_success
    );

    kernel_error_eq!(&result,
        Kind::NotFound,
        Audience::System,
        "We could not start the collection procedure. Could not locate the 'activity-test' directory in the provided directory list."
    );

}
#[test]
fn start_collection_error_missing_temp_dir() {
    let request = generate_valid_request();
    let directories = vec![
        ("home".to_string(), "nrn_sourcecode_example/1714646108364".to_string()),
        ("activity-test".to_string(), "nrn_sourcecode_example/1714646108364/activity".to_string()),
        ("evidence".to_string(), "nrn_sourcecode_example/1714646108364/evidence".to_string())
    ];
    let directory_list = DirectoryList::try_from_vec(directories).unwrap();

    let result = start_collection(
        &request,
        &directory_list,
        directory_creation_gateway_success,
        procedure_retrieval_gateway_success,
        file_move_gateway_success,
        file_delete_gateway_success
    );

    kernel_error_eq!(&result,
        Kind::NotFound,
        Audience::System,
        "We could not start the collection procedure. Could not locate the 'temp' directory in the provided directory list."
    );

}

/*** User Audience Error Tests ***/

/// This test verifies the expected error behavior if the "assurance-procedure-file" was not returned by the procedure retrieval gateway_adapter.
/// This error is generated at the time when the procedure definition document is moved from the download directory to its target location.
///
#[test]
fn start_collection_error_move_file_missing_procedure_doc() {
    let request = generate_valid_request();

    let directory_list = generate_valid_directory_list();
    let result = start_collection(
        &request,
        &directory_list,
        directory_creation_gateway_success,
        procedure_retrieval_gateway_missing_procedure_doc,
        file_move_gateway_success,
        file_delete_gateway_success
    );

    assert!(result.is_err(), "{}", format!("An error was expected although none was returned: {:?}", result.err()));

    let err = result.unwrap_err();
    assert_eq!(err.kind, Kind::NotFound);
    assert_eq!(err.audience, Audience::User);
    assert_eq!(err.message, "We could not start the collection procedure. We could not find the procedure definition document in the repository link you provided. Please check the repository and make sure the appropriate procedure definition document exists.");
}

/// This test verifies the expected error behavior if the "activity-dir" was not returned by the procedure retrieval gateway_adapter.
/// This error is generated at the time when the activity tests are moved from the download directory to their target location.
#[test]
fn start_collection_error_move_file_missing_activity_dir() {
    let request = generate_valid_request();
    let directory_list = generate_valid_directory_list();
    let result = start_collection(
        &request,
        &directory_list,
        directory_creation_gateway_success,
        procedure_retrieval_gateway_missing_activity_dir,
        file_move_gateway_success,
        file_delete_gateway_success
    );

    assert!(result.is_err(), "{}", format!("An error was expected although none was returned: {:?}", result.err()));

    let err = result.unwrap_err();
    assert_eq!(err.kind, Kind::NotFound);
    assert_eq!(err.audience, Audience::User);
    assert_eq!(err.message, "We could not start the collection procedure. We could not find the activity test directory in the repository link you provided.  Please check the repository and make sure the appropriate activity test directory exists.");
}


/*** Gateway Error Tests ***/

#[test]
fn start_collection_error_procedure_retrieval_gateway_error() {
    let request = generate_valid_request();
    let directory_list = generate_valid_directory_list();
    let result = start_collection(
        &request,
        &directory_list,
        directory_creation_gateway_success,
        procedure_retrieval_gateway_error,
        file_move_gateway_success,
        file_delete_gateway_success,
    );

    kernel_error_eq!(result,
        Kind::GatewayError,
        Audience::System,
        "We could not start the collection procedure. Could not download the procedure files from the repository: Procedure Retrieval Gateway Failure"
    );
}

#[test]
fn start_collection_error_directory_creation_gateway_error() {
    let request = generate_valid_request();
    let directory_list = generate_valid_directory_list();
    let result = start_collection(
        &request,
        &directory_list,
        directory_creation_gateway_error,
        procedure_retrieval_gateway_success,
        file_move_gateway_success,
        file_delete_gateway_success,
    );

    kernel_error_eq!(result,
        Kind::GatewayError,
        Audience::System,
        "We could not start the collection procedure. Could not create the directory structure for the evidence collection procedure: Directory Creation Gateway Failure"
    );
}

#[test]
fn start_collection_error_file_delete_gateway_error() {
    let request = generate_valid_request();
    let directory_list = generate_valid_directory_list();
    let result = start_collection(
        &request,
        &directory_list,
        directory_creation_gateway_success,
        procedure_retrieval_gateway_success,
        file_move_gateway_success,
        file_delete_gateway_error,
    );

    assert!(result.is_err(), "{}", format!("An error was expected although none was returned: {:?}", result.err()));

    let err = result.unwrap_err();
    assert_eq!(err.kind, Kind::GatewayError);
    assert_eq!(err.audience, Audience::System);
    assert!(err.message.contains("Could not delete the 'temp' directory: File Delete Gateway Failure" ));
}


/***  Medium Tests
    These test verify that the state the dependencies recieve when invoked is the expected state, and if it's not the expected state, that the proper error is returned.
*/

#[test]
fn start_collection_error_file_move_procedure_doc() {
    let request = generate_valid_request();
    let directory_list = generate_valid_directory_list();
    let result = start_collection(
        &request,
        &directory_list,
        directory_creation_gateway_success,
        procedure_retrieval_gateway_success,
        file_move_gateway_error_for_procedure_doc,
        file_delete_gateway_success
    );

    assert!(result.is_err(), "{}", format!("An error was expected although none was returned: {:?}", result.err()));

    let err = result.unwrap_err();
    assert_eq!(err.kind, Kind::GatewayError);
    assert_eq!(err.audience, Audience::System);
    assert_eq!(err.message, "We could not start the collection procedure. Could not move the downloaded procedure document '/some/path/to/assurance_procedure.yaml' to 'nrn_sourcecode_example/1714646108364': Move Procedure Doc Failure");
}

#[test]
fn start_collection_error_file_move_activity_dir() {
    let request = generate_valid_request();
    let directory_list = generate_valid_directory_list();
    let result = start_collection(
        &request,
        &directory_list,
        directory_creation_gateway_success,
        procedure_retrieval_gateway_success,
        file_move_gateway_error_for_activity_dir,
        file_delete_gateway_success
    );

    assert!(result.is_err(), "{}", format!("An error was expected although none was returned: {:?}", result.err()));

    let err = result.unwrap_err();
    assert_eq!(err.kind, Kind::GatewayError);
    assert_eq!(err.audience, Audience::System);
    assert_eq!(err.message, "We could not start the collection procedure. Could not move the downloaded activity test directory '/some/path/to/activity/' to 'nrn_sourcecode_example/1714646108364/activity': Move Activity Test Dir Failure");
}

/// Using the canned request, this test verifies that there is a file move request for the activity test directory, or procedure definition document, downloaded from the repository to the local 'activity-test' directory that will be used for execution of the assurance procedure.
///  It verifies that the source, generated from the request input, and target, give the standard protocol, are the expected values.
///
#[test]
fn success_activity_moved() {
    let request = generate_valid_request();
    let directory_list = generate_valid_directory_list();
    let result = start_collection(
        &request,
        &directory_list,
        directory_creation_gateway_success,
        procedure_retrieval_gateway_success,
        file_move_gateway_assert_correct_move_targets,
        file_delete_gateway_success
    );

    assert!(result.is_ok(), "{}", format!("An error was returned when one was not expected: {:?}", result.err()));

}

#[test]
fn temp_dir_deleted_success() {
    let request = generate_valid_request();
    let directory_list = generate_valid_directory_list();
    let result = start_collection(
        &request,
        &directory_list,
        directory_creation_gateway_success,
        procedure_retrieval_gateway_success,
        file_move_gateway_success,
        file_delete_gateway_assert_temp_dir_deleted
    );

    assert!(result.is_ok(), "{}", format!("An error was returned when one was not expected: {:?}", result.err()));

}

/***
    Testing mocks & other utilities
***/

/// Assumes canned start time of 1714646108364
fn generate_valid_request() -> StartProcedure {
    StartProcedureBuilder::default()
        .start_at(1714646108364)
        .api_version("1.0.0")
        .subject_nrn("nrn:sourcecode:example")
        .subject_id("123456789")
        .procedure_repository("https://example.com")
        .procedure_directory("some/location")
        .try_build()
        .unwrap()
}

/// Assumes canned start time of 1714646108364
fn generate_valid_directory_list() -> DirectoryList {
    let directories: Vec<(String, String)> = [
                ("home".to_string(), "nrn_sourcecode_example/1714646108364".to_string()),
                ("evidence".to_string(), "nrn_sourcecode_example/1714646108364/evidence".to_string()),
                ("activity-test".to_string(), "nrn_sourcecode_example/1714646108364/activity".to_string()),
                ("temp".to_string(), "nrn_sourcecode_example/1714646108364/temp".to_string())
    ].to_vec();
  DirectoryList::try_from_vec(directories).unwrap()

}

/* DirectoryCreationGateway Mocks */

fn directory_creation_gateway_success(directory_list: &DirectoryList) -> Result<DirectoryList, Error> {
    Ok(directory_list.clone())
}
fn directory_creation_gateway_error(_directory_list: &DirectoryList) -> Result<DirectoryList, Error> {
    Err(Error::for_system(Kind::GatewayError, "Directory Creation Gateway Failure".to_string()))
}

const ACTIVITY_DIR: &str = "/some/path/to/activity/";
const PROCEDURE_DOC: &str = "/some/path/to/assurance_procedure.yaml";

/// Returns a successful response based upon the protocol of containing the following keys:
///
///     - assurance-procedure-file
///     - activity-dir
///
/// For a successful test, the value is arbitrary for each key
fn procedure_retrieval_gateway_success(_repo_link: &RepositoryLink, _procedure_dir: &str, _download_dir: &str) -> Result<DirectoryList, Error> {
    Ok(DirectoryList::default()
        .try_add("assurance-procedure-file", PROCEDURE_DOC)?
        .try_add("activity-dir", ACTIVITY_DIR)?)
}
fn procedure_retrieval_gateway_error(_repo_link:  &RepositoryLink, _procedure_dir: &str, _download_dir: &str) -> Result<DirectoryList, Error> {
    Err(Error::for_system(Kind::GatewayError, "Procedure Retrieval Gateway Failure".to_string()))
}
fn procedure_retrieval_gateway_missing_procedure_doc(_repo_link:  &RepositoryLink, _procedure_dir: &str, _download_dir: &str) -> Result<DirectoryList, Error> {
    Ok(DirectoryList::default()
        .try_add("activity-dir", ACTIVITY_DIR)?)
}
fn procedure_retrieval_gateway_missing_activity_dir(_repo_link:  &RepositoryLink, _procedure_dir: &str, _download_dir: &str) -> Result<DirectoryList, Error> {
    Ok(DirectoryList::default()
        .try_add("assurance-procedure-file", PROCEDURE_DOC)?)
}


fn file_move_gateway_success(_source: &str, _target: &str,) -> Result<FilePath, Error> {
    Ok(FilePath::from("some/path/to/moved_file/success.txt"))
}

/// This function will return an error if the source of the move is the procedure doc value.
fn file_move_gateway_error_for_procedure_doc(source: &str, _target: &str,) -> Result<FilePath, Error> {
    if source == PROCEDURE_DOC {
        return Err(Error::for_system(Kind::GatewayError, "Move Procedure Doc Failure".to_string()));
    }
    Ok(FilePath::from("some/path/to/moved_file/error_for_procedure_doc.txt"))
}

/// This function will return an error if the source of the move is the activity-test dir value.
fn file_move_gateway_error_for_activity_dir(source: &str, _target: &str,) -> Result<FilePath, Error> {
    if source == ACTIVITY_DIR {
        return Err(Error::for_system(Kind::GatewayError, "Move Activity Test Dir Failure".to_string()));
    }
    Ok(FilePath::from("some/path/to/moved_file/error_for_activity.txt"))
}

fn file_move_gateway_assert_correct_move_targets(source: &str, target: &str) -> Result<FilePath, Error> {
    if source == ACTIVITY_DIR {
        assert_eq!(target, "nrn_sourcecode_example/1714646108364/activity");
    };
    if source == PROCEDURE_DOC {
        assert_eq!(target, "nrn_sourcecode_example/1714646108364");
    };
    Ok(FilePath::from("some/path/to/moved_file/assert_correct_move_targets.txt"))
}

fn file_delete_gateway_success(_source: &str) -> Result<(), Error> {
    Ok(())
}

fn file_delete_gateway_error(_source: &str) -> Result<(), Error> {
    Err(Error::for_system(Kind::GatewayError, "File Delete Gateway Failure".to_string()))
}

fn file_delete_gateway_assert_temp_dir_deleted(source: &str) -> Result<(), Error> {
    assert_eq!(source, "nrn_sourcecode_example/1714646108364/temp");
    Ok(())
}

