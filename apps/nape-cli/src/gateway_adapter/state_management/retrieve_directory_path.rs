use nape_kernel::error::{Error, Kind};
use crate::gateway_adapter::state_management::retrieve_app_state::app_state_from_nape_config;

pub fn directory_path_from_app_state(path_name: &str) -> Result<String, Error> {
    let app_state = app_state_from_nape_config()?;

    app_state.try_directory_list()?
        .try_get(path_name)
        .ok_or(Error::for_system(Kind::InvalidInput,
                                 format!("The '{}' directory was not found in the NAPE state configuration file", path_name)))
}