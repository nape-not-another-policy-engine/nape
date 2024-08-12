use crate::error::{Error, Kind};
use crate::values::specification::description::Description;
use crate::values::specification::name::Name;
use crate::values::strings::exceeds_max_length;

/// The maximum length of a key in the [`MetaData`] struct or 64 characters
pub const KEY_MAX: usize = 64;

/// The maximum length of a value in the [`MetaData`] struct or 256 characters
pub const VALUE_MAX: usize = 256;

/// The [`MetaData`] struct is a simple key-value store for NAPE Metadata for any type of NAPE Specification.
#[derive(Clone, Debug, Default, Eq, Hash, PartialEq)]
pub struct MetaData {
    pub data: Vec<(Name, Description)>,
}

impl MetaData {

    /// Add a key-value pair to the [`MetaData`]
    pub fn add(&mut self, key: &str, value: &str) -> Result<(), Error> {
        let cleaned_key = clean_key(key)?;
        let cleaned_value = clean_value(value)?;
        self.data.push((cleaned_key, cleaned_value));
        Ok(())
    }

    /// Remove a key-value pair from the [`MetaData`] based on the key
    pub fn remove(&mut self, key: &str) {
        self.data.retain(|(k, _)| k.value != key);
    }

    /// Get a value from the [`MetaData`] based on the key
    pub fn get(&self, key: &str) -> Option<String> {
        self.data.iter()
            .find(|(k, _)| k.value == key)
            .map(|(_, v)| v.value.clone())
    }

    /// Update a key-value pair in the [`MetaData`] based on the key if the key exists, otherwise add the key-value pair.
    pub fn upsert(&mut self, key: &str, value: &str) -> Result<(), Error> {
        let cleaned_key = clean_key(key)?;
        let cleaned_value = clean_value(value)?;

        match self.data.iter_mut().find(|(k, _)| k == &cleaned_key) {
            Some((_, v)) => *v = cleaned_value,
            None => self.data.push((cleaned_key, cleaned_value)),
        }
        Ok(())
    }

}

fn clean_key(key: &str) -> Result<Name, Error> {
    if exceeds_max_length(key, KEY_MAX) {
        return Err(Error::for_user(Kind::InvalidInput, format!("The metadata key '{}' exceeds the maximum allowed length of '{}' characters.  Please provide a metadata key that is less than '{}' characters.", key, KEY_MAX, KEY_MAX ) ));
    }
    let key_as_name = Name::try_from(key).map_err(|error|
        Error::for_user(Kind::InvalidInput, format!("The metadata key '{}' is invalid: {}", key, error)))?;
    Ok(key_as_name)
}

fn clean_value(value: &str) -> Result<Description, Error> {
    if exceeds_max_length(value, VALUE_MAX) {
        return Err(Error::for_user(Kind::InvalidInput,format!("The metadata value '{}' exceeds the maximum allowed length of '{}' characters.  Please provide a metadata value that is less than '{}' characters.", value, VALUE_MAX, VALUE_MAX)));
    }
    let value_as_desc = Description::try_from(value).map_err(|error|
        Error::for_user(Kind::InvalidInput, format!("The metadata value '{}' is invalid: {}", value, error)))?;
    Ok(value_as_desc)
}

