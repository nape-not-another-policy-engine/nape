use std::fs;
use std::path::{Path, PathBuf};
use git2::build::RepoBuilder;
use git2::{Commit, ObjectType, Reference, Repository, Tree, TreeEntry};

use nape_kernel::error::{Error, Kind};
use nape_kernel::values::directory::directory_list::DirectoryList;
use nape_kernel::values::specification::repository_link::RepositoryLink;

/// # Overview
/// The [`git2`] implementation of the Gateway [`domain::evidence_collection::usecase::for_process::start_collection_process::ProcessRetrievalGateway`] which retrieves the NAPE procedure data from a git repository and writes the procedure files to disk.
///
/// # Arguments
///
/// * `repo_link` - A [`RepositoryLink`] which contains the link to the git repository.
/// * `procedure_directory` - A string representing the directory in the git repository where the procedure files and subdirectories are located.
/// * `download_directory` - A string representing the directory where the procedure files are written to.
///
/// # Returns
///
/// A [`DirectoryList`] which contains the paths to the procedure files and subdirectories, or an [`Error`].
///
/// They [`DirectoryList`] contains only two entries:
///  - *assurance-procedure-file* - a link to the Assurnace Procedure file that defines the assurnace procedure
/// - *activity-dir* - a link to the activity test directory which contains all the test files for the activity actions in the procedure document
///
/// # Errors
///
/// - All [`git2::Error`] are mapped to [`Error`]
///  - All [`Error`] returned are for the [`Audience::System`] with the [`Kind::GatewayError`] and a message indicating the issues that is either bubbled up from the git2 library or a custom message describing the issue.
///
pub fn retrieve_procedure_from_git(repo_link: &RepositoryLink, procedure_directory: &str, download_directory: &str) -> Result<DirectoryList, Error> {

    let dir_to_clone_to = format!("{}/{}", &download_directory, "clone");
    let git_repo =  clone_repo(repo_link, &dir_to_clone_to)?;
    let repo_head = get_head_from_repo(&git_repo, &repo_link.value)?;
    let head_commit = get_commit_from_head(&repo_head, &repo_link.value)?;
    let commit_tree = get_tree_from_commit(&head_commit, &repo_link.value)?;
    let process_directory_tree = get_tree_for_process_directory_only(&commit_tree, &git_repo, procedure_directory, &repo_link.value)?;
    write_process_directory_tree_files_to_disk(&process_directory_tree, &git_repo, &download_directory)?;
    remove_clone_directory(&dir_to_clone_to)?;
    let directory_list = build_directory_list(&download_directory)?;
    Ok(directory_list)
}


fn clone_repo(repo_link: &RepositoryLink, clone_directory: &str) -> Result<Repository, Error> {
    let mut fetch_options = git2::FetchOptions::new();
    fetch_options.depth(1);

    let mut builder = RepoBuilder::new();
    builder.bare(true);
    builder.fetch_options(fetch_options);
    let repo_link_value = repo_link.value.as_str();
     match builder.clone(repo_link_value, &Path::new(&clone_directory)) {
        Ok(repo) =>  Ok(repo),
        Err(error) => Err(Error::for_system(Kind::GatewayError,
                                                       format!("Could not clone the git repository '{}'. {}", repo_link_value, error)))

    }
}

fn get_head_from_repo<'a>(repo: &'a Repository, repo_link: &'a str) -> Result<Reference<'a>, Error> {
    match repo.head() {
        Ok(head) => Ok(head),
        Err(error) =>  Err(Error::for_system(Kind::GatewayError,
    format!("Could not get the head for the git repository '{}'. {}", repo_link, error)))
    }
}

fn get_commit_from_head<'a>(head: &'a Reference<'a>, repo_link: &'a str) -> Result<Commit<'a>, Error> {
    match head.peel_to_commit() {
        Ok(commit) => Ok(commit),
        Err(error) => Err(Error::for_system(Kind::GatewayError,
    format!("Could not get the commit for the git repository '{}'. {}", repo_link, error)))
    }
}

fn get_tree_from_commit<'a>(commit: &'a Commit<'a>, repo_link: &'a str) -> Result<Tree<'a>, Error> {
    match commit.tree() {
        Ok(tree) => Ok(tree),
        Err(error) => Err(Error::for_system(Kind::GatewayError,
    format!("Could not get the tree for the git repository '{}'. {}", repo_link, error)))
    }
}

fn get_tree_for_process_directory_only<'a>(commit_tree: &'a Tree<'a>, git_repo: &'a Repository, process_directory: &'a str, repo_link: &'a str) -> Result<Tree<'a>, Error> {
    commit_tree
        .get_path(Path::new(&process_directory))
        .map_err(|_| Error::for_system(Kind::GatewayError,
                                       format!("Failed to get directory for the procedure directory '{}' from the git repo '{}'.", process_directory, repo_link)))?
        .to_object(&git_repo)
        .map_err(|_| Error::for_system(Kind::GatewayError,
                                       format!("Failed to convert to object for the procedure directory '{}' from the git repo '{}'.", process_directory, repo_link)))?
        .into_tree()
        .map_err(|_| Error::for_system(Kind::GatewayError,
                                       format!("Failed to convert to tree for the procedure directory '{}' from the git repo '{}'.", process_directory, repo_link)))
}

fn write_process_directory_tree_files_to_disk(process_directory_tree: &Tree, git_repo: &Repository, download_directory: &str) -> Result<(), Error> {
     for process_directory_tree_entry in process_directory_tree.iter() {
        let write_result = write_file_to_disk(&git_repo, &process_directory_tree_entry, &PathBuf::from(download_directory));
         match write_result {
             Ok(()) => (),
             Err(error) => return Err(Error::for_system(Kind::GatewayError,
                                                 format!("Could not write the procedure directory to disk. {}", error)))
         }
    }
    Ok(())
}

fn write_file_to_disk(repo: &Repository, tree_entry: &TreeEntry, base_path: &Path) -> Result<(), git2::Error> {
    match tree_entry.kind() {
        Some(ObjectType::Blob) => {
            let blob = repo.find_blob(tree_entry.id()).expect("Failed to find blob");
            let content = blob.content().to_vec();
            let output_path = base_path.join(tree_entry.name().unwrap());
            fs::create_dir_all(output_path.parent().unwrap()).expect("Failed to create directories");
            fs::write(output_path, content).expect("Failed to write file in blob section");
        },
        Some(ObjectType::Tree) => {
            let tree = tree_entry.to_object(repo)?.into_tree().expect("Failed to convert to tree in write_file");
            for entry in tree.iter() {
                write_file_to_disk(repo, &entry, &base_path.join(tree_entry.name().unwrap())).expect("Failed to write file in tree section");
            }
        },
        _ => {}
    }
    Ok(())
}

fn remove_clone_directory(dir_to_clone_to: &str) -> Result<(), Error> {
    let remove_result = fs::remove_dir_all(&dir_to_clone_to);
    match remove_result {
        Ok(()) => Ok(()),
        Err(error) => Err(Error::for_system(Kind::GatewayError,
                                            format!("Could not remove the directory '{}' which holds all the git clone files. {}", dir_to_clone_to, error)))
    }
}

// Design Decision - I did not make this dynamic by passing the directory names as arguments because the directory names are fixed and will not change.  Why?  This is per the protocol of the NAPE procedure repository, and instead I check to ensure the procedure doc and directory are present before returning the directory list.  If not, I return an error.
fn build_directory_list(download_directory: &str) -> Result<DirectoryList, Error> {
    let process_def_doc_yaml_path = format!("{}/assurance_procedure.yaml", download_directory);
    if !Path::new(&process_def_doc_yaml_path).exists() {
        return Err(Error::for_system(Kind::GatewayError,
                                     format!("The procedure definition document '{}' does not exist in the download directory '{}'.  Check that the procedure definition document exists in the repository.", process_def_doc_yaml_path, download_directory)))
    }

    let activity_directory_path = format!("{}/activity", download_directory);
    if !Path::new(&activity_directory_path).exists() {
        return Err(Error::for_system(Kind::GatewayError,
                                     format!("The activity test directory '{}' does not exist in the download directory '{}'.  Check that the activity test directory exists in the repository.", activity_directory_path, download_directory)))
    }

    let mut directory_list = DirectoryList::default()
        .try_add("assurance-procedure-file", &process_def_doc_yaml_path)
        .map_err(|error| Error::for_system(Kind::GatewayError,
                                           format!("Failed to add the procedure definition document '{}' to the directory list. {}", process_def_doc_yaml_path, error)))?;

    directory_list = directory_list.try_add("activity-dir", &activity_directory_path)
         .map_err(|error| Error::for_system(Kind::GatewayError,
                                            format!("Failed to add the activity-test directory '{}' to the directory list. {}", activity_directory_path, error)))?;

    Ok(directory_list)
}