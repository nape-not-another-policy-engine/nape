use std::fs;
use std::path::Path;
use nape_kernel::values::specification::repository_link::RepositoryLink;
use crate::gateway_adapter::git2::process_retrieval_gateway::retrieve_procedure_from_git;


#[test]
/// This is a LARGE Test
fn retrieve_procedure_from_git_success() {
    let repo_link = RepositoryLink::new("https://github.com/nape-dev/catalog.git").unwrap();
    let process_directory = "rust_ci/sourcecode_integration";
    let download_directory = "retrieve_process_from_git_success";

    let result = retrieve_procedure_from_git(&repo_link, process_directory, download_directory);

    assert!(result.is_ok(), "Expected success, got {:?}", result);

    let directory = result.unwrap();
    let nape_procedure_yaml_path = format!("{}/assurance_procedure.yaml", download_directory);
    let activity_directory_path = format!("{}/activity", download_directory);
    let peer_review_py = format!("{}/peer_review/at_least_two_reviewers.py", activity_directory_path);
    let requester_py = format!("{}/peer_review/requester_not_a_reviewer.py", activity_directory_path);


    assert!(directory.paths.contains(&("assurance-procedure-file".to_string(), nape_procedure_yaml_path.clone())), "Expected directory to contain '{}'", nape_procedure_yaml_path);
    assert!(directory.paths.contains(&("activity-dir".to_string(), activity_directory_path.clone())), "Expected directory to contain '{}'", activity_directory_path);

    assert!(Path::new(&nape_procedure_yaml_path).exists(), "Expected file '{}' to exist.", nape_procedure_yaml_path);
    assert!(Path::new(&activity_directory_path).exists(), "Expected directory '{}' to exist.", activity_directory_path);
    assert!(Path::new(&peer_review_py).exists(), "Expected file '{}' to exist.", peer_review_py);
    assert!(Path::new(&requester_py).exists(), "Expected file '{}' to exist.", requester_py);

    fs::remove_dir_all("retrieve_process_from_git_success")
        .expect("Filed to remove testing directory 'retrieve_process_from_git_success'.")
}
