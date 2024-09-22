

use std::fs;
use std::path::Path;
use crate::gateway_adapter::std_fs::file_move_gateway::move_file_on_filesystem;


#[test]
fn move_file_success() {

    // Arrange
    // 1st - Create the source document to move
    let source = "move_file_success/activity/test_file.txt";
    let source_path = Path::new(source);
    if let Some(source_parent) = source_path.parent() {
        fs::create_dir_all(source_parent).expect("Failed to create directory");
    }
    fs::write(&source_path, "test").expect("Failed to write file");
    assert!(source_path.exists(), "The source file was not created, check the test setup to ensure the file-to-move was created.");

    //2nd - Create the target directory to move the file to
    let target = "move_file_success_somewhere_else/activity/target_dir";
    let target_path = Path::new(target);
    fs::create_dir_all(&target_path).expect("Failed to create directory");
    assert!(target_path.exists(), "The target directory was not created, check the test setup to ensure the target directory was created.");

    // Act
    let result = move_file_on_filesystem(source, target);

    // Assert
    assert!(result.is_ok(), "An error occurred and none was expected: {:?}", result.err().unwrap());
    let new_target_path = target_path.join("test_file.txt");
    assert!(new_target_path.exists(),"The expected file was not fond in the directory is was supposed to be move to.");
    assert!(source_path.exists(), "The source file was removed, when it was not supposed to be.");

    // Clean up the test activity test directory after the test is run
    fs::remove_dir_all("move_file_success")
        .expect("Failed to remove activity test directory");
    fs::remove_dir_all("move_file_success_somewhere_else")
        .expect("Failed to remove activity test directory");
}
#[test]
fn move_directory_success() {

    // --- ARRANGE ---

    // 1st - Create the source directory to move files from, and verify it was created.
    let source_1 = "move_directory_success/the_subject_id/temp/activity/peer_review";
    let source_path_1 = Path::new(source_1);
    fs::create_dir_all(&source_path_1).expect("Failed to create directory");

    let source_2 = "move_directory_success/the_subject_id/temp/activity/static_code_analysis";
    let source_path_2 = Path::new(source_2);
    fs::create_dir_all(&source_path_2).expect("Failed to create directory");

    // Verify the directory was created
    assert!(source_path_1.exists(), "{}", format!("The source directory '{}' was not created, check the test setup to ensure the target directory was created.", source_path_1.display()));
    assert!(source_path_2.exists(), "{}", format!("The source directory '{}' was not created, check the test setup to ensure the target directory was created.", source_path_2.display()));

    // Create "test_file1.txt" and "test_file2.txt" in the directory and verify they are created.
    let source_path_1_file_1 = source_path_1.join("test_file1.txt");
    let source_path_1_file_2 = source_path_1.join("test_file2.txt");
    let source_path2_file_1 = source_path_2.join("code_analysis_results.json");

    // Write files to the "move_me" directory
    fs::write(&source_path_1_file_1, "Content for test_file1").expect("Failed to write file");
    fs::write(&source_path_1_file_2, "Content for test_file2").expect("Failed to write file");
    fs::write(&source_path2_file_1, "Content for code_analysis_results").expect("Failed to write file");

    // Verify the directory was created
    assert!(source_path_1_file_1.exists(), "The expected file 'test_file1.txt' was not found in the directory is was supposed to be move to.");
    assert!(source_path_1_file_2.exists(), "The expected file 'test_file2.txt' was not fond in the directory is was supposed to be move to.");
    assert!(source_path2_file_1.exists(), "The expected file 'code_analysis_results.json' was not fond in the directory is was supposed to be move to.");

    // 2nd - Create the target directory where to move the files, and verify it was created.
    let target =  "move_directory_success/the_subject_id/activity";
    let target_path = Path::new(target);
    fs::create_dir_all(&target_path).expect("Failed to create directory");

    // Verify the directory was created
    assert!(target_path.exists(), "The target directory was not created, check the test setup to ensure the target directory was created.");

    // --- ACT ---

    let result = move_file_on_filesystem("move_directory_success/the_subject_id/temp/activity", "move_directory_success/the_subject_id/activity");

    // --- ASSERT ---

    // Check that no errors were returned
    assert!(result.is_ok(), "An error occurred and none was expected: {:?}", result.err().unwrap());

    // Verify that the files were moved to the target directory
    let new_target_file1 = Path::new("move_directory_success/the_subject_id/activity/peer_review/test_file1.txt");
    let new_target_file2 = Path::new("move_directory_success/the_subject_id/activity/peer_review/test_file2.txt");
    let new_target_file3 = Path::new("move_directory_success/the_subject_id/activity/static_code_analysis/code_analysis_results.json");
    assert!(new_target_file1.exists(), "The new file 'test_file1.txt' was not found in the new directory.");
    assert!(new_target_file2.exists(), "The new file 'test_file2.txt' was not found in the new directory.");
    assert!(new_target_file3.exists(), "The new file 'code_analysis_results.json' was not found in the new directory.");

    // Clean up the activity test directory after the test is run
    fs::remove_dir_all("move_directory_success")
        .expect("Failed to remove activity test directory");
}

// TODO - Test - source not found
// TODO - Test - source is neither a file nor directory "/dev/null"
// TODO - Test - source is not readable

// target negative
// TODO - Test - target directory is a file
// TODO - Test - target directory does not exist
// TODO - Test - target directory is a null device ( "/dev/null" )
// TODO - Test - target directory is not accessible

// copy file negative
// TODO - test - could not create directory for file

// copy directory negative
// todo - figure out how to test these errors.  These errors should not be reachable because they should have been caught by the validate functions.
