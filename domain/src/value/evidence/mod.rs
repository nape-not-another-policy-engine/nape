//! Verification evidence requirements, sources, and immutable payloads.

#[cfg(test)]
mod tests;

use kernel_oss::error::{Error, Kind};

use super::{
    bounded_nonempty, controlled_value::ControlledValue, effective_graph::CanonicalActionSelector,
    external_resource_handle::ExternalResourceHandle, package::ManifestDigest,
};

/// One definition-owned evidence filename.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceFileName(String);

impl EvidenceFileName {
    /// Creates a safe single-segment filename.
    pub fn try_new(value: impl Into<String>) -> Result<Self, Error> {
        let value = bounded_nonempty(value, "evidence filename", 255)?;
        if value == "." || value == ".." || value.contains('/') || value.contains('\\') {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "evidence filename must be one safe path segment",
            ));
        }
        Ok(Self(value))
    }

    /// Returns the filename.
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// A caller-supplied local source handle for one evidence payload.
pub type EvidenceSourceHandle = ExternalResourceHandle;

/// One immutable Test-visible evidence argument.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EvidenceArgument {
    /// Opaque evidence bytes passed without structural projection.
    Opaque(Vec<u8>),
    /// Selected controlled projection passed after optional schema validation.
    Controlled(ControlledValue),
}

/// One current-run occurrence-to-payload association.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceAssociation {
    /// Contextual Action occurrence.
    pub action: CanonicalActionSelector,
    /// Exact payload digest.
    pub digest: ManifestDigest,
    /// Exact payload byte count.
    pub byte_count: u64,
}

/// One complete effective evidence requirement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceRequirement {
    action: CanonicalActionSelector,
    name: String,
    file: EvidenceFileName,
    maximum_bytes: u64,
}

impl EvidenceRequirement {
    /// Creates one definition-owned requirement.
    pub fn try_new(
        action: CanonicalActionSelector,
        name: impl Into<String>,
        file: EvidenceFileName,
        maximum_bytes: u64,
    ) -> Result<Self, Error> {
        if maximum_bytes == 0 {
            return Err(Error::for_user(
                Kind::BelowMin,
                "evidence maximum bytes must be positive",
            ));
        }
        Ok(Self {
            action,
            name: bounded_nonempty(name, "evidence name", 128)?,
            file,
            maximum_bytes,
        })
    }

    /// Returns the occurrence selector.
    pub fn action(&self) -> &CanonicalActionSelector {
        &self.action
    }

    /// Returns the definition-owned evidence name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the definition-owned filename.
    pub fn file(&self) -> &EvidenceFileName {
        &self.file
    }

    /// Returns the effective payload ceiling.
    pub fn maximum_bytes(&self) -> u64 {
        self.maximum_bytes
    }
}
