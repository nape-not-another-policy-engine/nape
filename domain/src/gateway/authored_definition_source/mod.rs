//! Immutable authored-definition source acquisition seam.

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;

use kernel_oss::gateway::Gateway;

use crate::{
    diagnostic::NapeOutcome,
    value::{external_resource_handle::ExternalResourceHandle, package::PackageReleasePurl},
};

/// Opaque authored-source root interpreted only by its driver.
pub type AuthoredDefinitionSourceHandle = ExternalResourceHandle;

/// Opaque required-new package output interpreted only by its driver.
pub type NewPackageOutputHandle = ExternalResourceHandle;

/// Requests one stable authored-source snapshot.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AuthoredDefinitionSourceRequest {
    source: AuthoredDefinitionSourceHandle,
    package: PackageReleasePurl,
}

impl AuthoredDefinitionSourceRequest {
    /// Creates the request.
    pub fn new(source: AuthoredDefinitionSourceHandle, package: PackageReleasePurl) -> Self {
        Self { source, package }
    }

    /// Returns the opaque source handle.
    pub fn source(&self) -> &AuthoredDefinitionSourceHandle {
        &self.source
    }

    /// Returns the exact intended package PURL.
    pub fn package(&self) -> &PackageReleasePurl {
        &self.package
    }
}

/// One sealed stable authored file inventory.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SealedAuthoredDefinition {
    semantic_root: String,
    semantic_root_media_type: String,
    files: BTreeMap<String, Vec<u8>>,
}

impl SealedAuthoredDefinition {
    /// Creates a sealed source snapshot.
    pub fn new(
        semantic_root: impl Into<String>,
        semantic_root_media_type: impl Into<String>,
        files: BTreeMap<String, Vec<u8>>,
    ) -> Self {
        Self {
            semantic_root: semantic_root.into(),
            semantic_root_media_type: semantic_root_media_type.into(),
            files,
        }
    }

    /// Returns the entry-document path selected by package kind protocol.
    pub fn semantic_root(&self) -> &str {
        &self.semantic_root
    }

    /// Returns the semantic-root controlled media type.
    pub fn semantic_root_media_type(&self) -> &str {
        &self.semantic_root_media_type
    }

    /// Returns the exact sealed files.
    pub fn files(&self) -> &BTreeMap<String, Vec<u8>> {
        &self.files
    }
}

/// Acquires and seals authored definition bytes without semantic interpretation.
pub trait AuthoredDefinitionSourceGW:
    Gateway<Request = AuthoredDefinitionSourceRequest, Response = NapeOutcome<SealedAuthoredDefinition>>
{
}
