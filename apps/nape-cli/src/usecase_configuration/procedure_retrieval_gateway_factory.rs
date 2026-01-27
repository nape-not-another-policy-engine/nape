use crate::gateway_adapter::git2::process_retrieval_gateway::retrieve_procedure_from_git;
use crate::gateway_adapter::std_fs::procedure_retrieval_gateway::retrieve_procedure_from_fs;
use kernel_oss::error::{Error, Kind};
use kernel_oss::values::directory::directory_list::DirectoryList;
use kernel_oss::values::specification::repository_link::RepositoryLink;

pub fn retrieve_procedure_factory(
    repo_link: &RepositoryLink,
    procedure_directory: &str,
    download_directory: &str,
) -> Result<DirectoryList, Error> {
    match repo_link.url().scheme.as_str() {
		"file" => {
			retrieve_procedure_from_fs(
				repo_link,
				procedure_directory,
				download_directory,
			)
		},
		"git" => {
			retrieve_procedure_from_git(
				repo_link,
				procedure_directory,
				download_directory,
			)
		},
		"https" => { // TODO - BRITTLE IMPLEMENTATION - this is here because we are using https instead of the git scheme for github links.
			retrieve_procedure_from_git(
				repo_link,
				procedure_directory,
				download_directory,
			)
		}
		_ => Err(Error::for_system(
			Kind::NotFound,
			format!("The repository link scheme [{}] is not supported for assurnace procedure retrieval.", repo_link.url().scheme)
		))
	}
}
