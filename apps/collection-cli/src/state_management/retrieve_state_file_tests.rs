use nape_kernel::error::Error;
use nape_kernel::values::directory::directory_list::DirectoryList;
use nape_kernel::values::specification::metadata::MetaData;
use nape_kernel::values::specification::procedure::Procedure;
use nape_kernel::values::specification::subject::Subject;
use nape_testing_assertions::is_ok;
use nape_testing_filesystem::{create_file, remove};
use crate::state_management::cli_app_state::{CLIAppState, CLIAppStateBuilder};
use crate::state_management::retrieve_state_file::retrieve_from_filesystem;

#[test]
fn retrieve_state_success() {
    /* Assemble */
    let app_state_file = create_file!("retrieve_state_success/app_state_file.yaml", "Random doesn't matter text");

    /* Act  */
    let result = retrieve_from_filesystem(&app_state_file, mock_deserializer);

    /* Assert */
    is_ok!(&result);
    let  actual_state = result.unwrap();

    let expected_state = testing_state();
    assert_eq!(actual_state, expected_state);

    // Clean up
    remove!("retrieve_state_success");
}

// TODO - CREATE TEST | Negative - Deserializer Failure

fn testing_state() -> CLIAppState {
    let subject = Subject::try_new("nrn:sourcecode:nape/collection-cli", "1719326666").unwrap();
    let procedure = Procedure::try_new("https://example.com", "some/dir/location").unwrap();

    let mut metadata = MetaData::default();
    metadata.add("key-1", "value 1").unwrap();
    metadata.add("key-2", "value 2").unwrap();

    let directory_list = DirectoryList::default();
    let directory_list = directory_list.try_add("path-1", "some/path/one").unwrap();
    let directory_list = directory_list.try_add("path-2", "some/path/two").unwrap();
    let directory_list = directory_list.try_add("path-3", "some/path/three/file.txt").unwrap();

    CLIAppStateBuilder::default()
        .for_subject(&subject)
        .with_procedure(&procedure)
        .with_metadata(&metadata)
        .with_directory_list(&directory_list)
        .try_build().unwrap()
}

fn mock_deserializer(_file_contents: &str) -> Result<CLIAppState, Error> {
    Ok(testing_state())
}
