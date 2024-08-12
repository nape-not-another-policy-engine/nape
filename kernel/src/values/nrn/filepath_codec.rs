use crate::error::{Error, Kind};
use crate::values::nrn::nrn::NRN;

pub fn encode_as_directory_name(nrn: &NRN) -> String {
    nrn.value
        .replace(":", "_")
        .replace("/", "_-_")
}

pub fn decode_from_directory_name(directory_name: &str) -> Result<NRN, Error> {
    let decoded_path = directory_name
        .replace("_-_", "/")
        .replace("_", ":");
    NRN::new(decoded_path.as_str()).map_err(|e|
        Error::for_user(Kind::InvalidInput, format!("Could not decode the NRN from the directory name: '{}': '{}'", directory_name, e.message.as_str())))
}
