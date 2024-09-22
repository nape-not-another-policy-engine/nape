use std::path::Path;
use nape_domain::evidence_collection::usecases::start_collection::gateway::FileMoveGateway;
use nape_kernel::error::{Error, Kind};
use nape_kernel::values::directory::directory_list::DirectoryList;
use nape_kernel::values::specification::repository_link::{RepositoryLink, RepositoryLinkScheme};

// TODO - LEFT OFF HERE - CREATE the impl for local ProcedureRetrievalGateway
pub fn retrieve_procedure_from_local_fs(
    repo_link: &RepositoryLink,
    procedure_directory: &str,
    download_directory: &str,
    file_move_gateway: FileMoveGateway) -> Result<DirectoryList, Error> {

    verify_local_schema(repo_link)?;
    // WHEN BACK, work out the preocess for copy the proceudre directory to the download directory

    let process_directory = Path::new(&procedure_directory);
    let download_dir = Path::new(&download_directory);
    let process_directory_tree = get_tree_for_process_directory_only(process_directory, download_dir)?;
    write_process_directory_tree_files_to_disk(process_directory_tree, download_dir)?;
    let directory_list = build_directory_list(download_dir)?;
    Ok(directory_list)
}


fn verify_local_schema(repo_link: &RepositoryLink) -> Result<(), Error> {
    if ! repo_link.is_scheme(&RepositoryLinkScheme::Local) {
        return Err(Error::for_user(Kind::UsecaseError,
            format!("The repository link '{}' is not a local file system link.  Cannot retrieve the procedure unless a local file system link is provided.", repo_link.value)));
    }
    Ok(())
}