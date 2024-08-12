use crate::error::Error;

/// # Overview
///
/// A [`FileDataGateway`] gateway that retrieves the data from a file.
///
/// # Arguments
///
/// * `file_path` - The path to the file to retrieve the data from.
///
/// # Returns
///
/// A `Result<Vec<u8>, Error>` containing the data from the file or an [`Error`] if the data could not be retrieved.
///
pub type FileDataGateway = fn(file_path: &str) -> Result<Vec<u8>, Error>;