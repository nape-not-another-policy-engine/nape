

use std::{env, fs};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use nape_kernel::error::{Audience, Kind};
use crate::gateway_adapter::std_fs::file_delete_gateway::delete_file_on_filesystem;

#[test]
fn delete_source_file_success() {

    // Arrange
    let source = "delete_source_file_success/activity/test_file.txt";
    let source_path = Path::new(source);
    if let Some(parent) = source_path.parent() {
        fs::create_dir_all(parent).expect("Failed to create directory");
    }

    fs::write(&source_path, "test").expect("Failed to write file");
    assert!(source_path.exists(), "The files was not created, check the test setup to ensure the file-to-delete was created.");

    // Act
    let result = delete_file_on_filesystem(source);

    // Assert
    assert!(result.is_ok());
    assert!(!source_path.exists());

    // Clean up the test activity test directory after the test is run
    fs::remove_dir_all("delete_source_file_success")
        .expect("Failed to remove activity test directory");

}
#[test]
fn delete_source_directory_success() {

    // Arrange
    let source = "delete_source_directory_success/activity/test_dir";
    let source_path = Path::new(source);

    fs::create_dir_all(&source_path).expect("Failed to create directory");
    assert!(source_path.exists(), "The directory was not created, check the test setup to ensure the directory-to-delete was created.");

    // Act
    let result = delete_file_on_filesystem(source);

    // Assert
    assert!(result.is_ok(), "{}", format!("An error was returned when one was not expected: {:?}", result.err()));
    assert!(!source_path.exists());

    // Clean up the activity test directory after the test is run
    fs::remove_dir_all("delete_source_directory_success")
        .expect("Failed to remove activity test directory");

}

#[test]
fn delete_source_error_dir_cannot_be_accessed_not_found() {

    let source = "delete_source_error_dir_cannot_be_accessed_not_found";

    let result = delete_file_on_filesystem(source);

    assert!(result.is_err(), "Expected an error but one was not provided.");

    let result_error = result.unwrap_err();
    assert_eq!(result_error.kind, Kind::GatewayError);
    assert_eq!(result_error.audience, Audience::System);
    assert!(result_error.message.starts_with("The path 'delete_source_error_dir_cannot_be_accessed_not_found' cannot be accessed. "));

}
#[test]
fn delete_source_error_file_cannot_be_accessed_not_found() {

    let source = "delete_source_error_file_cannot_be_accessed_not_found.txt";

    let result = delete_file_on_filesystem(source);

    assert!(result.is_err(), "Expected an error but one was not provided.");

    let result_error = result.unwrap_err();
    assert_eq!(result_error.kind, Kind::GatewayError);
    assert_eq!(result_error.audience, Audience::System);
    assert!(result_error.message.starts_with("The path 'delete_source_error_file_cannot_be_accessed_not_found.txt' cannot be accessed. "));

}
#[test]
fn delete_source_error_file_cannot_be_accessed_no_permissions() {

    // Arrange - Creat the file to delete
    let source = "delete_source_error_file_cannot_be_accessed_no_permissions/activity/test_file.txt";
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let absolute_source = Path::new(&current_dir).join(source);
    if let Some(parent) = absolute_source.parent() {
        fs::create_dir_all(parent).expect("Failed to create directory");
    }

    fs::write(source, "test-1").expect("Failed to write file");
    assert!(absolute_source.exists(), "The files was not created, check the test setup to ensure the file-to-delete was created.");

    // Change the permissions of the parent directory
    let parent_dir = absolute_source.parent().expect("Failed to get parent directory");
    let metadata = fs::metadata(&parent_dir).expect("Failed to get metadata");
    let mut permissions = metadata.permissions();
    permissions.set_mode(0); // Read and execute for owner, group and others
    fs::set_permissions(&parent_dir, permissions.clone()).expect("Failed to set permissions");

    // Call delete_source with the name of the file
    let result = delete_file_on_filesystem(source);

    // Assert that the file no longer exists
    assert!(result.is_err(), "Expected an error but one was not provided.");
    let result_error = result.unwrap_err();
    assert_eq!(result_error.kind, Kind::GatewayError);
    assert_eq!(result_error.audience, Audience::System);
    assert!(result_error.message.starts_with("The path 'delete_source_error_file_cannot_be_accessed_no_permissions/activity/test_file.txt' cannot be accessed. "));

    // Reset the permissions of the parent directory to allow read and write access
    permissions.set_mode(0o755); // Read, write, execute for owner, read and execute for group and others
    fs::set_permissions(&parent_dir, permissions).expect("Failed to reset permissions");

    // Clean up the temporary directory
    fs::remove_dir_all("delete_source_error_file_cannot_be_accessed_no_permissions").expect("Failed to remove temporary directory");

}
#[test]
fn delete_source_error_failed_to_delete_dir() {

    // Create a temporary directory
    let temp_dir = "delete_source_error_failed_to_delete_dir";
    fs::create_dir_all(temp_dir).expect("Failed to create temporary directory");

    // Change the permissions of the temporary directory to remove read and write access
    let metadata = fs::metadata(temp_dir).expect("Failed to get metadata");
    let mut permissions = metadata.permissions().clone();
    permissions.set_mode(0);
    fs::set_permissions(temp_dir, permissions.clone()).expect("Failed to set permissions");

    // Call delete_source with the name of the file
    let result = delete_file_on_filesystem(temp_dir);

    // Assert that the file no longer exists
    assert!(result.is_err(), "Expected an error but one was not provided.");
    let result_error = result.unwrap_err();
    assert_eq!(result_error.kind, Kind::GatewayError);
    assert_eq!(result_error.audience, Audience::System);
    assert!(result_error.message.starts_with("Failed to delete directory 'delete_source_error_failed_to_delete_dir'. "));

    // Reset the permissions of the temporary directory to allow read and write access
    permissions.set_mode(0o755); // Read, write, execute for owner, read and execute for group and others
    fs::set_permissions(temp_dir, permissions).expect("Failed to reset permissions");

    // Clean up the temporary directory
    fs::remove_dir_all(temp_dir).expect("Failed to remove temporary directory");
}
#[test]
fn delete_source_error_not_file_or_dir() {

    let source = "/dev/null";

    let result = delete_file_on_filesystem(source);

    assert!(result.is_err(), "Expected an error but one was not provided.");

    let result_error = result.unwrap_err();
    assert_eq!(result_error.kind, Kind::GatewayError);
    assert_eq!(result_error.audience, Audience::System);
    assert!(result_error.message.starts_with("The path '/dev/null' is neither a file nor a directory."));
}


/***
    *NOTE*
    - was unable to test the filesystem::remove_file of the delete_source function beasue all the test casses such as, 1) file does not exist, or 2) file is not accessible, or 3) file is not a file, are already covered.
 */
