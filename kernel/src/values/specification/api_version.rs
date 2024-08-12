use crate::error::{Error, Kind};

/// [`APIVersion`] represents the version of the NAPE API versioning primarily for the NAPE Specifications and follows the [Semantic Versioning 2.0.0 specification](https://github.com/semver/semver)
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct APIVersion {
    pub major: u8,
    pub minor: u8,
    pub patch: u8,
}

impl APIVersion {

    /// Create a new [`APIVersion`] instance
    ///
    /// # Arguments
    ///
    /// * `major` - The major version number
    /// * `minor` - The minor version number
    /// * `patch` - The patch version number
    ///
    /// # Returns
    ///
    /// A new [`APIVersion`] instance
    ///
    pub fn new(major: u8, minor: u8, patch: u8) -> Self {
        APIVersion {  major,  minor,  patch, }
    }

    /// Create a new [`APIVersion`] instance from a string
    ///
    /// # Arguments
    ///
    /// * `version` - A string representing the version in the format 'major.minor.patch'
    ///
    /// # Returns
    ///
    /// A Result containing the [`APIVersion`] if the version string is valid, otherwise an ['Error']
    ///
    /// ## Errors
    ///
    ///  An ['Error'] of ['Kind::Audience'] for ['Audience::User'] will be returned if any of the following are true:
    ///
    /// * The version string is empty,
    /// * The version string is not in the format 'major.minor.patch',
    /// * The major, minor, or patch version is not a valid number between 0 and 255.
    pub fn from_str(version: &str) -> Result<Self, Error> {

        if version.trim().is_empty() {
            return Err(invalid_input_error("You provided an empty version. Please provide a version value which conforms to the Semver specification of 'major.minor.patch'.".to_string()));
        }

        let parts: Vec<&str> = version.split('.').collect();
        if parts.len() != 3 {
            return Err(invalid_input_error(format!("Version string '{}' is not in the format 'major.minor.patch'.", version)));
        }

        let major = parts[0].parse::<u8>().map_err(|_| invalid_input_error(format!("The major version '{}' from the supplied api version of '{}' is not a valid number. It must be a number between 0 and 255.", parts[0], version)) )?;
        let minor = parts[1].parse::<u8>().map_err(|_| invalid_input_error(format!("The minor version '{}' from the supplied api version of '{}' is not a valid number.  It must be a number between 0 and 255", parts[1], version)) )?;
        let patch = parts[2].parse::<u8>().map_err(|_| invalid_input_error(format!("The patch version '{}' from the supplied api version of '{}' is not a valid number. It must be a number between 0 and 255", parts[2], version)) )?;

        Ok(APIVersion { major, minor, patch })

    }

    /// Get the [`APIVersion`] as a string
    pub fn as_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }

}

fn invalid_input_error(message: String) ->Error {
    Error::for_user(Kind::InvalidInput, message)
}