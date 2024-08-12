use nape_kernel::values::directory::directory_list::DirectoryList;
use nape_kernel::values::specification::metadata::MetaData;
use nape_kernel::values::specification::procedure::Procedure;
use nape_kernel::values::specification::subject::Subject;
use nape_testing_assertions::is_ok;
use crate::state_management::cli_app_state::{CLIAppStateBuilder};
use crate::state_management::yaml_serializer::{deserialize_from_yaml, serialize_to_yaml};

#[test]
fn serialize_to_yaml_success() {

    /* ASSEMBLE */

    let subject = Subject::try_new("nrn:sourcecode:nape/collection-cli", "1719326666").unwrap();
    let procedure = Procedure::try_new("https://example.com", "some/dir/location").unwrap();

    let mut metadata = MetaData::default();
    metadata.add("key-1", "value 1").unwrap();
    metadata.add("key-2", "value 2").unwrap();

    let directory_list = DirectoryList::default();
    let directory_list = directory_list.try_add("path-1", "some/path/one").unwrap();
    let directory_list = directory_list.try_add("path-2", "some/path/two").unwrap();
    let directory_list = directory_list.try_add("path-3", "some/path/three/file.txt").unwrap();

    let state = CLIAppStateBuilder::default()
        .for_subject(&subject)
        .with_procedure(&procedure)
        .with_metadata(&metadata)
        .with_directory_list(&directory_list)
        .try_build().unwrap();

    /* ACT */

    let result = serialize_to_yaml(&state);

    /* ASSERT */

    is_ok!(&result);

}

#[test]
fn deserialize_from_yaml_success() {
    let yaml_contents = "---\nmetadata:\n  key-1: value 1\n  key-2: value 2\nsubject_id: \"1719326666\"\nsubject_nrn: nrn:sourcecode:nape/collection-cli\nprocedure_directory: some/dir/location\nprocedure_repository: https://example.com\ndirectories:\n  path-1: some/path/one\n  path-2: some/path/two\n  path-3: some/path/three/file.txt\n";

    let result = deserialize_from_yaml(&yaml_contents);

    is_ok!(&result);

    let deserialized_app_state = &result.unwrap();

    let subject = Subject::try_new("nrn:sourcecode:nape/collection-cli", "1719326666").unwrap();
    let procedure = Procedure::try_new("https://example.com", "some/dir/location").unwrap();

    let mut metadata = MetaData::default();
    metadata.add("key-1", "value 1").unwrap();
    metadata.add("key-2", "value 2").unwrap();

    let directory_list = DirectoryList::default();
    let directory_list = directory_list.try_add("path-1", "some/path/one").unwrap();
    let directory_list = directory_list.try_add("path-2", "some/path/two").unwrap();
    let directory_list = directory_list.try_add("path-3", "some/path/three/file.txt").unwrap();

    let expected_state = CLIAppStateBuilder::default()
        .for_subject(&subject)
        .with_procedure(&procedure)
        .with_metadata(&metadata)
        .with_directory_list(&directory_list)
        .try_build().unwrap();

    assert_eq!(expected_state.clone(), deserialized_app_state.clone());

}
