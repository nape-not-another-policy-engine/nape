use crate::state_management::cli_app_state::CLIAppState;
use kernel_oss::error::{Error, Kind};
use std::path::PathBuf;

pub fn retrieve_from_filesystem(
    file_location: &PathBuf,
    deserialize: fn(&str) -> Result<CLIAppState, Error>,
) -> Result<CLIAppState, Error> {
    let file_contents = std::fs::read_to_string(file_location).map_err(|e| {
        Error::for_system(
            Kind::GatewayError,
            format!(
                "Could not read the CLI App State file '{}'. {}",
                file_location.display(),
                e
            ),
        )
    })?;

    let app_state = deserialize(&file_contents)?;

    Ok(app_state)
}
