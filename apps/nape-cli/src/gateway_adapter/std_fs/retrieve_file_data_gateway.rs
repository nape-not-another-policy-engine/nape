use std::fs;
use std::path::Path;
use nape_kernel::error::{Error, Kind};

pub fn retrieve_file_data_from_filesystem(file: &str) -> Result<(String, Vec<u8>), Error> {

    let file_path = Path::new(file);

    if !file_path.exists() {
        return Err(Error::for_system(Kind::GatewayError,
                                     format!("The file '{}' does not exist.", file)));
    }

    let file_name = file_path.file_name()
        .ok_or_else(|| Error::for_system(Kind::GatewayError,
                                         format!("The file path '{}' does not contain a recognizable file name.", file)))?
        .to_os_string().into_string()
        .map_err(|os_string| Error::for_system(Kind::GatewayError,
                                                             format!("A file name for the file '{}' cannot be generated because '{:?}' does not contain not a valid Unicode data as a file name.", file, os_string)))?;

    let file_data = fs::read(file)
        .map_err(|error| Error::for_system(Kind::GatewayError,
                                           format!("There was an issue reading the file '{}'. {}", file, error)))?;

    Ok( (file_name.to_string(), file_data) )
}