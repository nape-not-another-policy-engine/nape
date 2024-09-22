use std::collections::HashMap;
use nape_kernel::error::{Audience, Kind};
use nape_kernel::values::directory::directory_list::DirectoryList;
use nape_kernel::values::specification::metadata::MetaData;
use nape_kernel::values::specification::procedure::Procedure;
use nape_kernel::values::specification::subject::Subject;
use nape_testing_assertions::{is_ok, kernel_error_eq};
use crate::state_management::cli_app_state::CLIAppStateBuilder;

/* Happy Path */

#[test]
fn builder_success() {

    /* ASSEMBLE */

    let subject = Subject::try_new("nrn:sourcecode:nape/nape-cli", "1719326666").unwrap();
    let procedure = Procedure::try_new("https://example.com", "some/dir/location").unwrap();

    let mut metadata = MetaData::default();
    metadata.add("key-1", "value 1").unwrap();

    let directory_list = DirectoryList::default();
    let directory_list = directory_list.try_add("path1", "some/path/one").unwrap();
    let directory_list = directory_list.try_add("path2", "some/path/two").unwrap();
    let directory_list = directory_list.try_add("path3", "some/path/three/file.txt").unwrap();

    /* ACT */

    let builder = CLIAppStateBuilder::default()
        .for_subject(&subject)
        .with_procedure(&procedure)
        .with_metadata(&metadata)
        .with_directory_list(&directory_list)
        .try_build();

    /* ASSERT */

    let mut expeceted_metadata = HashMap::new();
    expeceted_metadata.insert("key-1".to_string(), "value 1".to_string());

    let mut expected_directories = HashMap::new();
    expected_directories.insert("path1".to_string(), "some/path/one".to_string());
    expected_directories.insert("path2".to_string(), "some/path/two".to_string());
    expected_directories.insert("path3".to_string(), "some/path/three/file.txt".to_string());

    is_ok!(&builder);
    let state = builder.unwrap();
    assert_eq!(state.metadata, expeceted_metadata);
    assert_eq!(state.directories, expected_directories);
    assert_eq!(state.subject_nrn, "nrn:sourcecode:nape/nape-cli".to_string());
    assert_eq!(state.subject_id, "1719326666".to_string());
    assert_eq!(state.procedure_repository, "https://example.com".to_string());
    assert_eq!(state.procedure_directory, "some/dir/location".to_string());
}

/* Sad Path */

#[test]
fn no_subject_error() {

    /* ASSEMBLE */

    let procedure = Procedure::try_new("https://example.com", "some/dir/location").unwrap();
    let metadata = MetaData::default();
    let directory_list = DirectoryList::default();

    /* ACT */

    let builder = CLIAppStateBuilder::default()
        .with_procedure(&procedure)
        .with_metadata(&metadata)
        .with_directory_list(&directory_list)
        .try_build();

    /* ASSERT */

    kernel_error_eq!(builder, Kind::InvalidInput, Audience::System, "There is an issue establishing the CLI Application State. A Subject is required, although one was not provided.");
}

#[test]
fn no_procedure_error() {

    /* ASSEMBLE */

    let subject = Subject::try_new("nrn:sourcecode:nape/nape-cli", "1719326666").unwrap();
    let metadata = MetaData::default();
    let directory_list = DirectoryList::default();

    /* ACT */

    let builder = CLIAppStateBuilder::default()
        .for_subject(&subject)
        .with_metadata(&metadata)
        .with_directory_list(&directory_list)
        .try_build();

    /* ASSERT */

    kernel_error_eq!(builder, Kind::InvalidInput, Audience::System, "There is an issue establishing the CLI Application State. A Procedure is required, although one was not provided.");
}

#[test]
fn no_metadata_error() {

    /* ASSEMBLE */

    let subject = Subject::try_new("nrn:sourcecode:nape/nape-cli", "1719326666").unwrap();
    let procedure = Procedure::try_new("https://example.com", "some/dir/location").unwrap();
    let directory_list = DirectoryList::default();

    /* ACT */

    let builder = CLIAppStateBuilder::default()
        .for_subject(&subject)
        .with_procedure(&procedure)
        .with_directory_list(&directory_list)
        .try_build();

    /* ASSERT */

    kernel_error_eq!(builder, Kind::InvalidInput, Audience::System, "There is an issue establishing the CLI Application State. Metadata is required, although none was provided.");
}

#[test]
fn no_directory_list_error() {

    /* ASSEMBLE */

    let subject = Subject::try_new("nrn:sourcecode:nape/nape-cli", "1719326666").unwrap();
    let procedure = Procedure::try_new("https://example.com", "some/dir/location").unwrap();
    let metadata = MetaData::default();

    /* ACT */

    let builder = CLIAppStateBuilder::default()
        .for_subject(&subject)
        .with_procedure(&procedure)
        .with_metadata(&metadata)
        .try_build();

    /* ASSERT */

    kernel_error_eq!(builder, Kind::InvalidInput, Audience::System, "There is an issue establishing the CLI Application State. A Directory List is required, although one was note provided.");
}
