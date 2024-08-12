

use std::{fs};
use std::os::unix::fs::PermissionsExt;
use nape_kernel::error::{Audience, Kind};
use nape_kernel::values::directory::directory_list::DirectoryList;
use crate::gateway_adapter::std_fs::directory_creation_gateway::create_directories_on_filesystem;

#[test]
fn create_directories_on_filesystem_success() {
    let directory_list = DirectoryList {
        paths: vec![
            ("test1".to_string(), "test_create_directories_on_filesystem/test1".to_string()),
            ("test2".to_string(), "test_create_directories_on_filesystem/test2".to_string()),
        ].into_iter().collect()
    };

    let result = create_directories_on_filesystem(&directory_list);

    assert!(result.is_ok());
    assert!(fs::metadata("test_create_directories_on_filesystem/test1").is_ok(),
            "Expected 'test_create_directories_on_filesystem/test1' to exist, but it does not.");
    assert!(fs::metadata("test_create_directories_on_filesystem/test2").is_ok(),
            "Expected 'test_create_directories_on_filesystem/test2' to exist, but it does not." );

    // Clean up the directories
    fs::remove_dir_all("test_create_directories_on_filesystem").unwrap();

}

#[test]
fn create_directories_on_filesystem_error_no_permissions_to_create_dir() {

    // Create a temporary directory
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let temp_dir = &current_dir.join("create_directories_on_filesystem_error_no_permissions_to_create_dir");
    fs::create_dir_all(temp_dir).expect("Failed to create temporary directory");

    // Change the permissions of the temporary directory to remove read and write access
    let metadata = fs::metadata(temp_dir).expect("Failed to get metadata");
    let mut permissions = metadata.permissions().clone();
    permissions.set_mode(0);
    fs::set_permissions(temp_dir, permissions.clone()).expect("Failed to set permissions");

    let directory_list = DirectoryList {
        paths: vec![
            ("test1".to_string(), "create_directories_on_filesystem_error_no_permissions_to_create_dir/test1".to_string()),
            ("test2".to_string(), "create_directories_on_filesystem_error_no_permissions_to_create_dir/test2".to_string()),
        ].into_iter().collect()
    };

    let result = create_directories_on_filesystem(&directory_list);

    assert!(result.is_err());
    let error = result.err().unwrap();
    assert_eq!(error.audience, Audience::System);
    assert_eq!(error.kind, Kind::GatewayError);
    assert!(error.message.starts_with("Failed to create directory 'create_directories_on_filesystem_error_no_permissions_to_create_dir/test1'. "));

    // Reset the permissions of the temporary directory to allow read and write access
    permissions.set_mode(0o755); // Read, write, execute for owner, read and execute for group and others
    fs::set_permissions(temp_dir, permissions).expect("Failed to reset permissions");

    // Clean up the temporary directory
    fs::remove_dir_all(temp_dir).expect("Failed to remove temporary directory");

}
