use nape_kernel::error::Error;
use nape_kernel::values::directory::directory_list::DirectoryList;
use nape_kernel::values::specification::metadata::MetaData;
use nape_kernel::values::specification::procedure::Procedure;
use nape_kernel::values::specification::subject::Subject;
use nape_testing_assertions::is_ok;
use nape_testing_filesystem::{path_for, file_contents_eq, remove};
use crate::state_management::cli_app_state::{CLIAppState, CLIAppStateBuilder};
use crate::state_management::write_state_file::write_to_filesystem;


#[test]
fn write_state_success() {

    /* Assemble */
    let app_state = generate_valid_cli_app_state();
    let app_state_file = path_for!("write_state_success/app_state_file.yaml");

    /* Act  */
    let result = write_to_filesystem(&app_state, mock_app_state_serializers, &app_state_file );

    /* Assert */
    is_ok!(&result);
    file_contents_eq!("There is some yaml here", app_state_file);

    // Clean up
    remove!("write_state_success");

}

// TODO - CREATE TEST | Negative - Serializer Failure

fn generate_valid_cli_app_state() -> CLIAppState {
    let subject = Subject::try_new("nrn:sourcecode:nape/nape-cli", "1719326666").unwrap();
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

fn mock_app_state_serializers(_app_state: &CLIAppState) -> Result<String, Error> {
    Ok("There is some yaml here".to_string())
}