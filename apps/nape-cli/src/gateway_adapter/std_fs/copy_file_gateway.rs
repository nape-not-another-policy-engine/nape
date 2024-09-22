use std::fs;
use std::path::Path;
use nape_kernel::error::{Error, Kind};

// TODO - re-look at this for negative pathway tests

pub fn copy_file_to_filesystem(file_name: &str, file_data: &Vec<u8>, target_directory: &str) -> Result<String, Error> {

    let target_path = Path::new(target_directory);

    fs::create_dir_all(target_path)
        .map_err(|error| Error::for_system(Kind::GatewayError,
        format!("Was unable to create the target directory '{:?}' to copy the file  '{}'.  {}.", target_path.to_str(), file_name, error )))?;

    let file_path = target_path.join(file_name);

    fs::write(file_path.clone(), file_data)
        .map_err(|error| Error::for_system(Kind::GatewayError,
                                           format!("There was an issue copying the file '{}' to the target directory '{}'. {}.", file_name, target_directory, error)))?;

    Ok(file_path.to_str().unwrap_or("").to_string())
}