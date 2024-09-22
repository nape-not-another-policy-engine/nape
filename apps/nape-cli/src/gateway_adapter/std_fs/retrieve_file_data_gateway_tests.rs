use std::fs;
use std::path::Path;
use crate::gateway_adapter::std_fs::retrieve_file_data_gateway::retrieve_file_data_from_filesystem;

// Medium Test
#[test]
fn retrieve_file_data_from_filesystem_success() {
    let file_name = "evidence_file.txt";
    let file_data = b"test data";
    let target_directory = "retrieve_file_data_from_filesystem_success";
    let file_path = Path::new(target_directory).join(file_name);

    // create the file
    fs::create_dir_all(target_directory)
        .expect("Failed to create target directory 'retrieve_file_data_from_filesystem_success'.");
    fs::write(&file_path, file_data)
        .expect("Failed to write file data to file 'evidence_file.txt'.");

    let file_path_str = "retrieve_file_data_from_filesystem_success/evidence_file.txt";
    // read the file
    let retrieved_file = retrieve_file_data_from_filesystem(file_path_str).unwrap();

    let retrieved_file_name = retrieved_file.0;
    let retrieved_file_data = retrieved_file.1;

    // assert that the file data is the same as the data that was written to the file
    assert_eq!(b"test data".to_vec(), retrieved_file_data, "The file data was not the same as the data that was written to the file.");
    assert_eq!("evidence_file.txt", retrieved_file_name, "The file name was not the same as the file name that was retrieved.");

    // delete the file and directory
    fs::remove_dir_all(target_directory)
        .expect("Failed to remove target directory 'test_retrieve_file_data_from_filesystem'.")
}

// todo - create a negative test to ensure an error is thrown when the file does not exist
// todo - create a negative test to throw and error when tyring to read the file, make the file unreadable by the user so there is a permissions error.