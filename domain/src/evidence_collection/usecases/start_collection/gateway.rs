use nape_kernel::error::{Error};
use nape_kernel::values::directory::directory_list::DirectoryList;
use nape_kernel::values::specification::file_path::FilePath;
use nape_kernel::values::specification::repository_link::RepositoryLink;

/// The [`ProcedureRetrievalGateway`] is the function signature for an implementation which downloads all the procedure files.
///
/// # Arguments
///
/// * `repo_url` - The URL to the repository where the procedure definition and all the files associated with it are located.
/// * `procedure_directory` - The directory within the repository where the procedure definition and all the files associated is located/
/// * `download_dir` - The directory where the procedure definition and all the files associated with it will be downloaded to.
///   * NOTE: This directory is relative to where the application is running.
///
/// # Returns
///
/// A [`DirectoryList`] which contains the directory structure for all the files and directories downloaded, or an [`Error`] for the [`Audience::System`] of the [`Kind::GatewayError`], with a message describing the error.
///
/// For the [`DirectoryList`] the following keys will be supplied as part of the return:
///
/// * `assurance-procedure-file` - a link to the procedure definition document
/// * `activity-dir` - a link to the activity test directory which contains all the actions test outlined in the procedure definition document.
///
pub type ProcedureRetrievalGateway = fn(repo_link: &RepositoryLink, procedure_directory: &str, download_dir: &str) -> Result<DirectoryList, Error>;

///  The [`DirectoryCreationGateway`] i creates the structure on a file system to store evidence and any other files.
///
/// # Arguments
///
/// * `directory_list` - The [`DirectoryList`] that contains the directory structure that will be created.
///
/// #Errors
///
/// - If the directory structure cannot be created, an [`Error`] is returned for the [`Audience::System`] with a [`Kind::GatewayError`] and a message describing the error.
/// - If the directory structure already exists, an [`Error`] is returned for the [`Audience::User`] with a [`Kind::AlreadyExists`] and a message indicating that the directory structure already exists.
///
pub type DirectoryCreationGateway = fn(directory_list: &DirectoryList) -> Result<DirectoryList, Error>;

/// The [`FileMoveGateway`] is a function that moves a directory from one location to another.
///
/// # Arguments
///
/// * `source` - The source location of the file or directory to be moved.
/// * `target` - The target location where the file or directory will be moved to.
///
/// # Returns
///
/// A [`FilePath`] which contains the final path of the moved file, or an [`Error`]
pub type FileMoveGateway = fn(source: &str, target: &str) -> Result<FilePath, Error>;

/// The [`FileDeleteGateway`] is a function that deletes a file or directory form the provided source location
///
/// # Arguments
///
/// * `source` - The source location of the file or directory to be deleted.
pub type FileDeleteGateway = fn(source: &str) -> Result<(), Error>;

