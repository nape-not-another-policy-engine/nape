use crate::gateway_adapter::std_fs::procedure_retrieval_gateway::retrieve_procedure_from_fs;
use kernel_oss::values::specification::repository_link::RepositoryLink;
use std::fs;
use std::panic;
use std::path::Path;

/// Integration Test
#[test]
fn retrieve_procedure_from_fs_success() {
    let temp_base = "temp_test_base";
    let download_dir = "temp_download";

    // Create a temporary base directory
    fs::create_dir_all(temp_base).expect("Failed to create temp base dir");

    // Create test_repo directory
    let test_repo_dir = format!("{}/test_repo", temp_base);
    fs::create_dir_all(&test_repo_dir).expect("Failed to create test_repo dir");

    // Create procedure subdirectory with required files inside test_repo
    let procedure_dir = format!("{}/test_procedure", test_repo_dir);
    fs::create_dir_all(&procedure_dir).expect("Failed to create procedure dir");

    let yaml_path = format!("{}/assurance_procedure.yaml", procedure_dir);
    fs::write(&yaml_path, "sample yaml content").expect("Failed to write yaml");

    let activity_dir = format!("{}/activity", procedure_dir);

    fs::create_dir_all(&activity_dir).expect("Failed to create activity dir");
    fs::write(format!("{}/test_file.txt", activity_dir), "test content")
        .expect("Failed to write activity file");

    // Create download directory
    fs::create_dir_all(download_dir).expect("Failed to create download dir");

    // Create RepositoryLink with file:// URI pointing to test_repo
    let repo_link = RepositoryLink::builder()
        .default_scheme("git")
        .allowed_schema(["git".to_string(), "https".to_string(), "file".to_string()].to_vec())
        .repo_link(format!(
            "file://{}/",
            std::env::current_dir()
                .unwrap()
                .join(&test_repo_dir)
                .display()
        ))
        .build()
        .expect("Failed to create repo link");

    // Call the function with procedure_directory as "test_procedure"
    let result = retrieve_procedure_from_fs(&repo_link, "test_procedure", download_dir);

    // Assert success and check DirectoryList
    assert!(
        result.is_ok(),
        "Expected success, got error: {:?}",
        result.err()
    );

    fs::remove_dir_all(temp_base).expect("Failed to remove temp base");
    fs::remove_dir_all(download_dir).expect("Failed to remove download dir");
}
