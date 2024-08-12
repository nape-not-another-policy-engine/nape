use crate::error::{Error, Kind};

#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct  FilePath {
    value: String
}

impl FilePath {

    /// Creates a new instance of a [`FilePath`] without any validations.
    ///
    /// # Arguments
    ///
    /// * `value` - A string slice containing the file path.
    ///
    /// # Returns
    ///
    /// A new instance of a [`FilePath`].
    pub fn from(value: &str) -> Self {
        Self { value: value.to_string() }
    }

    /// Creates a new instance of a [`FilePath`] and applies validations to the arguments.
    ///
    /// # Arguments
    ///
    /// * `value` - A string slice containing the file path.
    ///
    /// # Returns
    ///
    /// A new instance of a [`FilePath`], or an [`Error`] if the value is:
    /// * Empty
    pub fn try_from(value: &str) -> Result<Self, Error> {
        if value.trim().is_empty() {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "The file path is required, although a value was not provided.".to_string()));
        }
        Ok ( Self { value: value.to_string() })
    }

    pub fn to_string(&self) -> String {
        self.value.clone()
    }

    pub fn as_str(&self) -> &str {
        &self.value
    }

}
