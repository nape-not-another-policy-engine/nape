//! Controlled JSON, YAML, and TOML projection gateway.

#[cfg(test)]
mod tests;

use kernel_oss::{
    error::{Error, Kind},
    gateway::Gateway,
};

use crate::{diagnostic::NapeOutcome, value::controlled_value::ControlledValue};

/// The closed controlled-document media profiles.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ControlledMediaProfile {
    /// Strict JSON.
    Json,
    /// Controlled YAML projected to the JSON data model.
    Yaml,
    /// Controlled TOML projected to the JSON data model.
    Toml,
}

/// A bounded immutable document awaiting controlled projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ControlledDocumentProjectionRequest {
    profile: ControlledMediaProfile,
    bytes: Vec<u8>,
}

impl ControlledDocumentProjectionRequest {
    /// Creates a request under the profile-wide 256 MiB payload ceiling.
    pub fn try_new(
        profile: ControlledMediaProfile,
        bytes: Vec<u8>,
        effective_maximum_bytes: u64,
    ) -> Result<Self, Error> {
        if effective_maximum_bytes == 0 || effective_maximum_bytes > 268_435_456 {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "controlled-document maximum must be within 1..=256 MiB",
            ));
        }
        if bytes.len() as u64 > effective_maximum_bytes {
            return Err(Error::for_user(
                Kind::ExceedsMax,
                "controlled document exceeds its effective maximum",
            ));
        }
        Ok(Self { profile, bytes })
    }

    /// Returns the selected controlled profile.
    pub fn profile(&self) -> ControlledMediaProfile {
        self.profile
    }

    /// Returns the exact immutable source bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }
}

/// Projects controlled document bytes into one transport-neutral value.
pub trait ControlledDocumentProjectionGW:
    Gateway<Request = ControlledDocumentProjectionRequest, Response = NapeOutcome<ControlledValue>>
{
}
