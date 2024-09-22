use std::fs;
use std::path::PathBuf;
use nape_kernel::error::{Error, Kind};
use crate::state_management::cli_app_state::CLIAppState;

/// Writes the CLIAppState to the filesystem.
///
/// # Arguments
///
/// * `app_state` - A Reference to the CLIAppState to write to the filesystem.
/// * `serialize` - The function to serialize the CLIAppState to a string.
/// * `file_location` - A Reference to the location of the file where the CLIAppState will be written.
///
/// # Returns
///
/// * `Result<(), Error>` - The result of the operation.
///
pub fn write_to_filesystem(
    app_state: &CLIAppState,
    serialize: fn(&CLIAppState) -> Result<String, Error>,
    file_location: &PathBuf) -> Result<(), Error> {

    // Create the parent directories if they do not exist
    if let Some(parent) = file_location.parent() {
        fs::create_dir_all(parent)
            .map_err(|e| Error::for_system(Kind::GatewayError,
                                           format!("Could not create the parent directories for the App State file '{}'. {}", file_location.display(), e)))?;
    }

    let file_contents = serialize(app_state)?;

    // Use fs::write to create or overwrite the existing file
    fs::write(&file_location, &file_contents)
        .map_err(|e| Error::for_system(Kind::GatewayError,
                                       format!("Could not write the CLI App State File to the filesystem. {}", e)))?;

    Ok(())

}