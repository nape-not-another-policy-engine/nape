use nape_kernel::error::{Error, Kind};
use crate::state_management::cli_app_state::CLIAppState;

pub fn serialize_to_yaml(app_state: &CLIAppState) -> Result<String, Error> {
    let file_contents = serde_yaml::to_string(app_state)
        .map_err(|e| Error::for_system(Kind::GatewayError,
                                       format!("Could not serialize the CLI App State to YAML. {}", e)
        ))?;
    Ok(format!("---\n{}", file_contents))
}

pub fn deserialize_from_yaml(yaml: &str) -> Result<CLIAppState, Error> {
    let app_state: CLIAppState = serde_yaml::from_str(yaml)
        .map_err(|e| Error::for_system(Kind::GatewayError,
                                       format!("Could not deserialize the CLI App State from YAML. {}", e)
        ))?;
    Ok(app_state)
}
