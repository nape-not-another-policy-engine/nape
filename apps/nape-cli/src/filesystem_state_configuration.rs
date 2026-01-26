use kernel_oss::algorithms::os_home_directory;
use kernel_oss::error::Error;
use std::path::PathBuf;

const CONFIG_DIRECTORY: &str = "nape";
const CONFIG_FILE_NAME: &str = ".nape_cli_config";

pub fn nape_cli_config_file_path() -> Result<PathBuf, Error> {
    let path = os_home_directory::retrieve()?;
    Ok(path.join(CONFIG_DIRECTORY).join(CONFIG_FILE_NAME))
}
