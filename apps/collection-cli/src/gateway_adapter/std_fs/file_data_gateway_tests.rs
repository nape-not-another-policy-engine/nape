use nape_kernel::error::{Audience, Kind};
use nape_testing_assertions::{kernel_error_starts_with};
use nape_testing_filesystem::{canonical_path, create_file, create_write_only_file, remove};
use crate::gateway_adapter::std_fs::file_data_gateway::read_file_data;

#[test]
fn success() {

    // Clean up any artifacts left from previous tests
    remove!("std_fs_file_data_gateway_success");

    // Assemble
    let path= create_file!("std_fs_file_data_gateway_success/file.txt", "Hello, world!");
    let path_string = canonical_path!(path);

    // Assert
    let result = read_file_data(&path_string);

    // Assertions
    assert!(result.is_ok());
    let actual_data =result.unwrap();
    let expected_data = "Hello, world!".as_bytes().to_vec();
    assert_eq!(expected_data, actual_data);

    // Clean up
    remove!("std_fs_file_data_gateway_success");
}

#[test]
fn could_not_open_file_error() {

    // Clean up test artifacts from previous failed tests if they exist.
    remove!("std_fs_file_data_gateway_could_not_open_file_error");

    let path = create_write_only_file!("std_fs_file_data_gateway_could_not_open_file_error/file.txt", "Hello, world!");
    let path_string = canonical_path!(path);

    // Assemble & Act
    let result = read_file_data(&path_string);

    // Assertions
    let expected_message = format!("Could not open the file '{}' to read its data. ", path_string);
    kernel_error_starts_with!(result,
        Kind::ProcessingFailure,
        Audience::System,
       &expected_message);

    // Clean Up
    remove!("std_fs_file_data_gateway_could_not_open_file_error");

}