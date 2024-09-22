use nape_kernel::values::specification::repository_link::RepositoryLink;
use nape_testing_assertions::is_ok;
use crate::gateway_adapter::std_fs::procedure_retrieval_gateway::retrieve_procedure_from_local_fs;

#[test]
/// This is a LARGE Test
fn retrieve_procedure_from_git_success() {

    // Arrange

    // todo - create the directory "local/procedure-repo/some/procedure"
    // todo - create the assurnace_procedure file "local/procedure-repo/some/procedure/assurance_procedure.yaml"
    // todo - create the activity directory "local/procedure-repo/some/procedure/activity"
    // todo - create the action-1 dir file "local/procedure-repo/some/procedure/activity/action-1/the-test-of-detail.py"
    let repo_link = RepositoryLink::try_new("local/procedure-repo").unwrap();
    let process_directory = "some/procedure";
    let download_directory = "retrieve_process_from_local_fs_success";


    // Act
    let result = retrieve_procedure_from_local_fs(&repo_link, process_directory, download_directory);


    // Assert
    is_ok!(&result);

}
