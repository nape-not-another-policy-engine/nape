use std::fmt::{Debug};
use nape_kernel::error::{Error, Kind};
use nape_kernel::gateways::directory_list::RetrieveDirectoryPath;
use nape_kernel::values::directory::name::DirectoryName;
use nape_kernel::values::specification::name::Name;

/// `ActionEvidenceFile` is a trait that represents an evidence file associated with an action.
///
/// This trait is used as a boundary interface to establish of a file that serves as evidence for an action.
/// It provides methods to get the action name, file path, and file name.
///
/// # Type Parameters
///
/// * `'a`: The lifetime parameter. This is used to ensure that the references returned by the methods
///   are valid for as long as the object implementing this trait.
///
/// # Requirements
///
/// Types that implement `ActionEvidenceFile` must also implement the `Clone`, `Display`, and `Debug` traits.
///
/// # Methods
///
/// * `action_name`: Returns a reference to a string that represents the name of the action.
/// * `file_path`: Returns a reference to a string that represents the path of the file.
/// * `file_name`: Returns an `Option` that contains a reference to a string that represents the name of the file. If the file name is not available, this method should return `None`
#[derive(Clone, Debug)]
pub struct CollectEvidenceRequest<'a> {
    pub action_name: &'a str,
    pub file_path: &'a str,
    pub file_name:  Option<&'a str>,
}

#[derive(Clone, Debug)]
pub struct CollectedEvidence {
    pub file_location: String
}


/// `UCCollectEvidenceFile` is a function pointer type that represents a use case for collecting evidence files.
///
/// This function pointer is used as a boundary interface which delineates the details of collecting an evidence file associated with an [`Action`].
///
/// # Type Parameters
///
/// This function pointer does not have any type parameters.
///
/// # Parameters
///
/// * `request`: A dynamic reference to any type that implements the [`CollectEvidenceRequest`] trait. This represents the evidence file to be collected.
///
/// # Returns
///
/// This function returns a [`Result`] that contains an empty tuple `()` if the evidence file was successfully collected, or an [`Error`] if the evidence file could not be collected.
pub type UCCollectEvidenceFile = fn(request: &CollectEvidenceRequest) -> Result<CollectedEvidence, Error>;

pub type CopyFileGateway = fn(file_name: &str, file_data: &Vec<u8>, target_directory: &str) -> Result<String, Error>;

pub type RetrieveFileDataGateway = fn(file_path: &str) -> Result<(String, Vec<u8>), Error>;


pub fn collect_action_evidence(
    request: &CollectEvidenceRequest,
    retrieve_directory: RetrieveDirectoryPath,
    retrieve_file_data: RetrieveFileDataGateway,
    copy_file: CopyFileGateway) -> Result<CollectedEvidence, Error> {

    let valid_action_name = Name::try_from(request.action_name)
        .map_err(|error| Error::for_user(Kind::InvalidInput,
                                         format!("There is an issue with the action name '{}'. {}", request.action_name, error.message)))?;

    let (current_file_name, file_data) = retrieve_file_data(request.file_path)
        .map_err(|error| Error::for_system(Kind::GatewayError,
                                           format!("There was an issue retrieving the file '{}'. {}", request.file_path, error.message)))?;

    let target_file_name = match request.file_name {
        Some(file_name) => file_name,
        None => &current_file_name
    };

    let evidence_root_directory = retrieve_directory("evidence")
        .map_err(|error| Error::for_system(Kind::GatewayError,
                                           format!("There was an issue retrieving the evidence directory path. {}", error.message)))?;

    let action_dir_name = DirectoryName::from(&valid_action_name);
    let target_directory = format!("{}/{}",evidence_root_directory, action_dir_name.value);

    let copied_file_location = copy_file(&target_file_name, &file_data, &target_directory)
        .map_err(|error| Error::for_system(Kind::GatewayError,
                                           format!("There was an issue copying the file '{}' to '{}'. {}", request.file_path, target_directory, error.message)))?;

    Ok(CollectedEvidence { file_location: copied_file_location })

}