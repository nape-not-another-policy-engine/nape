use crate::error::{Error, Kind};

/// The [`Name`] struct represents a name that is used to identify an NAPE entity.  It is a human-readable name that only allows alphanumeric characters and dashes. The [`Name`] struct contains the following fields:
///
/// * `value` - The name value
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Name {
    pub value: String
}

impl Name {

    /// Create a new [`Name`] instance
    ///
    /// # Arguments
    ///
    /// * `name` - A string slice containing the name value
    ///
    /// # Returns
    ///
    /// * `Result<Name, Error>` - A result containing either an [`Error`] or a new [`Name`] instance.
    ///
    /// ## Errors
    ///
    /// An [`Error`] of [`Kind::InvalidInput`] for [`Audience::User`] is returned if:
    ///
    /// * the input is empty,
    /// * contains invalid characters (i.e., characters that are not alphanumeric or dashes), or
    /// * starts or ends with a dash.
    ///
    pub fn try_from(name: &str) -> Result<Name, Error> {

        trim_and_check_if_empty(name)?;
        check_for_non_alphanumeric_and_dashes_only(name)?;
        check_for_leading_or_trailing_dashes(name)?;
        let cleaned_name = name.to_lowercase();

        Ok(Name { value: cleaned_name })
    }

}

fn trim_and_check_if_empty(name: &str) -> Result<(), Error> {
    if name.trim().is_empty() {
        return Err(Error::for_user(Kind::InvalidInput, "You provided an empty name. A name must contain at least one character.".to_string()));
    }
    Ok(())
}
fn check_for_non_alphanumeric_and_dashes_only(name: &str) -> Result<(), Error> {
    if name.chars().all(|c| c.is_alphanumeric() || c == '-') {
         return Ok(())
    }
     Err(Error::for_user(Kind::InvalidInput,  format!("You provided the name '{}' which contains invalid characters. A name must contain only alphanumeric characters and dashes.", name)))

}
fn check_for_leading_or_trailing_dashes(name: &str) -> Result<(), Error> {
    if name.starts_with('-') || name.ends_with('-') {
        return Err(Error::for_user(Kind::InvalidInput, format!("The name cannot start or end with a dash, and you provided '{}'.", name)));
    }
    Ok(())
}
