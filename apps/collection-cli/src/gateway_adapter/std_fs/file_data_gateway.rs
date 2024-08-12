use std::fs;
use std::io::Read;
use nape_kernel::error::{Error, Kind};


/// # Overview
///
/// An implementation for the [`FileDataGateway`] gateway that retrieves the data from a file using the [`fs`] (file system) implementation of the ['std'] rust library.
///
/// # Returns
///
/// A [`FileDataGateway`] function which will attempt to retrieve the data from a file on the filesystem.
///
/// # Example
///
/// ```no_run
/// use nape_kernel::gateways::file_data_gateway::FileDataGateway;
///
/// let result = read_file_data("path/to/file.txt");
/// // Now do something with the result
///
/// ```
///
pub fn read_file_data( file_path: &str) -> Result <Vec<u8>, Error>  {

        let mut file = fs::File::open(file_path)
            .map_err(|e| Error::for_system(Kind::ProcessingFailure,
                                           format!("Could not open the file '{}' to read its data. {}", &file_path, e.to_string())))?;

        let mut data = Vec::new();

        file.read_to_end(&mut data)
            .map_err(|e| Error::for_system(Kind::ProcessingFailure,
            format!("Could not read data from the file '{}'. {}", & file_path, e.to_string())))?;

        Ok(data)

}