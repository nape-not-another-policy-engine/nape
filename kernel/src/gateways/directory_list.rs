use crate::error::Error;

/// Retrieve the directory path based on a given directory key.
pub type RetrieveDirectoryPath = fn(directory_key: &str) -> Result<String, Error>;