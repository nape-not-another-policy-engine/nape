use std::fs;
use std::path::Path;
use crate::gateway_adapter::std_fs::copy_file_gateway::copy_file_to_filesystem;

/// Medium Test
#[test]
fn copy_file_to_filesystem_success() {

    // Create a temporary directory to copy the file to
    let target_directory = "copy_file_to_filesystem_success/some-action";
    fs::create_dir_all(target_directory)
        .expect("Failed to create target directory 'copy_file_to_filesystem_success/some-action'.");

    // Generate some sample file data and a sample file name
    let file_data = vec![1, 2, 3, 4, 5];
    let file_name = "the_evidence.txt";

    // Call copy_file_to_filesystem to copy the file to the directory
    let result = copy_file_to_filesystem(file_name, &file_data, target_directory);

    assert!(result.is_ok(), "An error occurred when one was not expected: {:?}", result.err());

    // Read the copied file
    let file_path = Path::new(target_directory).join(file_name);
    let copied_data = fs::read(&file_path).unwrap();

    // Check that the copied file's data is the same as the original data
    assert_eq!(file_data, copied_data, "Copied file data does not match original data");

    // Clean up
    fs::remove_dir_all("copy_file_to_filesystem_success")
        .expect("Failed to remove testing target directory 'copy_file_to_filesystem_success'.")
}

// todo - add a test to ensure an error is thrown when the target path does not exist
// todo - add a test to ensure an error is thrown when the file cannot be written to the target path becasue the user does not have permission to write to the target directory