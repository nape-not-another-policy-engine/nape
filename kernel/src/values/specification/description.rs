use crate::error::{Error, Kind};

/// The [`Description`] struct represents a long form text describing something.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Description {
    pub value: String
}

impl Description {

    /// Creates a new [`Description`] instance.
    ///
    /// # Arguments
    ///
    /// * `value` - A string slice that holds the value of the description.
    ///
    /// # Returns
    ///
    /// A Result containing either a [`Description`] instance or an [`Error`].
    ///
    /// ## Errors
    ///
    /// An [`Error`] of [`Kind::InvalidInput`] for [`Audience::User`] will be returned if the value is empty.
    ///
    /// # Mutations
    /// * all leading whitespace, tabs, new lines, and carriage returns are removed
    ///
    pub fn try_from(value: &str) -> Result<Self, Error> {
        let cleaned_value = value.trim();
        check_for_empty_value(cleaned_value)?;
        Ok( Self { value: String::from(cleaned_value) } )
    }

}

fn check_for_empty_value(value: &str) -> Result<(), Error> {
    if value.is_empty() {
        return Err(
            Error::for_user(
                Kind::InvalidInput,
                "You provided an empty description. A description must have a value with at least one character.".to_string() ) )
    }
    Ok(())
}