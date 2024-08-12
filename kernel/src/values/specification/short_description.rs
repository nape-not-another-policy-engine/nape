use crate::error::{Error, Kind};
use crate::values::specification::description::Description;
use crate::values::strings::exceeds_max_length;

/// the [`MAX_LENGTH`] is the maximum amount of characters the [`ShortDescription`] can contain
const MAX_LENGTH: usize = 255;

/// The [`ShortDescription`] struct represents a short description of something.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ShortDescription {
    pub value: String
}

impl ShortDescription {

    /// Creates a new [`ShortDescription`] instance.
    ///
    /// # Arguments
    ///
    /// * `value` - A string slice that holds the value of the short description.
    ///
    /// # Returns
    ///
    /// A Result containing either a [`ShortDescription`] instance or an [`Error`].
    ///
    /// ## Errors
    ///
    /// An [`Error`] of [`Kind::InvalidInput`] for [`Audience::User`] will be returned if:
    /// * the value is empty, or
    /// * if the value exceeds the maximum length of 255 characters
    ///
    pub fn try_from(value: &str) -> Result<Self, Error> {
        let desc = Description::try_from(value)
            .map_err(|e| customize_error(e.message.as_str()))?;
        if exceeds_max_length(desc.value.as_str(), MAX_LENGTH) {
            return Err(customize_error(
                format!("The short description provided is too long. A description must have a value with at most {} characters.", MAX_LENGTH).as_str()));
        }
        Ok( Self { value: desc.value } )
    }

}

fn customize_error(message: &str) -> Error {
    Error::for_user(Kind::InvalidInput, format!("There is an issue with your short description: {}", message))
}
