use std::env;
use std::path::PathBuf;
use crate::error::{Error, Kind};


// TODO - Create test for this once you research how to mock out environment variables that will not effect the rest of the test suite.
/// Retrieve the home directory of the current OS user.
pub fn retrieve() -> Result<PathBuf, Error> {
    match env::var("HOME")
        .or_else(|_| env::var("USERPROFILE")) {
        Ok(dir_value) => Ok(PathBuf::from(dir_value)),
        Err(_) => Err(Error::for_system(Kind::NotFound, "Home directory not found".to_string()))
    }
}