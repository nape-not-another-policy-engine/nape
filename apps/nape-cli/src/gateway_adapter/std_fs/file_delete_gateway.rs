use std::{fs};
use std::path::Path;
use nape_kernel::error::{Error, Kind};

pub fn delete_file_on_filesystem(source: &str) -> Result<(), Error> {

    let source_path = Path::new(source);
    let metadata = match fs::metadata(&source_path) {
        Ok(metadata) => metadata,
        Err(err) => return Err(Error::for_system(Kind::GatewayError,
            format!("The path '{}' cannot be accessed. {}", source, err)
        )),
    };

    if metadata.is_file() {
        match fs::remove_file(source_path) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::for_system( Kind::GatewayError,
                format!("Failed to delete file '{}'. {}", source, err)
            )) }
    } else if metadata.is_dir() {
        match fs::remove_dir_all(source_path) {
            Ok(_) => Ok(()),
            Err(err) => Err(Error::for_system(Kind::GatewayError,
                format!("Failed to delete directory '{}'. {}", source, err)
            )) }
    } else {
        Err(Error::for_system(Kind::GatewayError,
            format!("The path '{}' is neither a file nor a directory.", source)))
    }

}