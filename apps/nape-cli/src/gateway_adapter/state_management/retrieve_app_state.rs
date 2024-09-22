use nape_kernel::error::{Error, Kind};
use crate::filesystem_state_configuration::nape_cli_config_file_path;
use crate::state_management::cli_app_state::CLIAppState;
use crate::state_management::retrieve_state_file::retrieve_from_filesystem;
use crate::state_management::yaml_serializer::deserialize_from_yaml;

/// Retrieve the application state from the NAPE configuration file.
pub fn app_state_from_nape_config() -> Result<CLIAppState, Error> {
    let local_state_file = nape_cli_config_file_path()
        .map_err(|error| Error::for_system(Kind::NotFound,
                                           format!("Failed to retrieve the local NAPE state configuration file. {}", error.message)))?;
    let app_state = retrieve_from_filesystem(&local_state_file, deserialize_from_yaml)
        .map_err(|error| Error::for_system(Kind::ProcessingFailure,
                                           format!("Failed to deserialize the local NAPE state configuration file. {}", error.message)))?;
    Ok(app_state)
}