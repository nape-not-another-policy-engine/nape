//! Verification invocation metadata preserved from Start into the Report.

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;

use kernel_oss::error::{Error, Kind};

const RESERVED: [&str; 3] = ["id", "generated_at", "utc-start"];

/// One caller-supplied metadata entry before deterministic normalization.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct InvocationMetadataEntry {
    key: String,
    value: String,
}

impl InvocationMetadataEntry {
    /// Creates one normalized caller metadata entry.
    pub fn try_new(key: impl Into<String>, value: impl Into<String>) -> Result<Self, Error> {
        let key = key.into().to_lowercase();
        if key.is_empty()
            || key.len() > 64
            || key.starts_with('-')
            || key.ends_with('-')
            || !key
                .chars()
                .all(|character| character.is_alphanumeric() || character == '-')
        {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "metadata key is outside the VerificationReport caller-key profile",
            ));
        }
        if RESERVED.contains(&key.as_str()) {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "metadata key is reserved for the Verification Engine",
            ));
        }
        let raw = value.into();
        if raw.len() > 256 {
            return Err(Error::for_user(
                Kind::ExceedsMax,
                "metadata value exceeds 256 UTF-8 bytes",
            ));
        }
        let value = raw.trim().to_string();
        if value.is_empty() {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "metadata value must not be empty after trimming",
            ));
        }
        Ok(Self { key, value })
    }

    /// Returns the normalized key.
    pub fn key(&self) -> &str {
        &self.key
    }

    /// Returns the trimmed value.
    pub fn value(&self) -> &str {
        &self.value
    }
}

/// The frozen, deterministic caller metadata map.
#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct InvocationMetadata(BTreeMap<String, String>);

impl InvocationMetadata {
    /// Applies entries in order; the last normalized duplicate wins.
    pub fn from_entries(entries: Vec<InvocationMetadataEntry>) -> Self {
        let mut values = BTreeMap::new();
        for entry in entries {
            values.insert(entry.key, entry.value);
        }
        Self(values)
    }

    /// Returns deterministic accepted entries.
    pub fn values(&self) -> &BTreeMap<String, String> {
        &self.0
    }
}
