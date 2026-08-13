//! Transport-neutral controlled JSON data-model value.

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;

/// One immutable value admitted from controlled JSON, YAML, or TOML.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum ControlledValue {
    /// JSON null.
    Null,
    /// JSON boolean.
    Boolean(bool),
    /// A canonical finite JSON number lexeme.
    Number(String),
    /// JSON string.
    String(String),
    /// JSON array.
    Array(Vec<ControlledValue>),
    /// JSON object with deterministic key ordering.
    Object(BTreeMap<String, ControlledValue>),
}

impl ControlledValue {
    /// Returns the object member when this value is an object.
    pub fn get(&self, name: &str) -> Option<&ControlledValue> {
        match self {
            Self::Object(values) => values.get(name),
            _ => None,
        }
    }

    /// Returns the value as an object.
    pub fn as_object(&self) -> Option<&BTreeMap<String, ControlledValue>> {
        match self {
            Self::Object(values) => Some(values),
            _ => None,
        }
    }

    /// Returns the value as an array.
    pub fn as_array(&self) -> Option<&[ControlledValue]> {
        match self {
            Self::Array(values) => Some(values),
            _ => None,
        }
    }

    /// Returns the value as text.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the canonical finite number lexeme.
    pub fn as_number(&self) -> Option<&str> {
        match self {
            Self::Number(value) => Some(value),
            _ => None,
        }
    }

    /// Returns the value as a boolean.
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            Self::Boolean(value) => Some(*value),
            _ => None,
        }
    }
}
