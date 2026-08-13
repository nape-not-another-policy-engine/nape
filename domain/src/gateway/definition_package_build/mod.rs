//! Deterministic package construction and atomic commitment seam.

#[cfg(test)]
mod tests;

use kernel_oss::gateway::Gateway;

use crate::{
    diagnostic::NapeOutcome,
    gateway::authored_definition_source::{NewPackageOutputHandle, SealedAuthoredDefinition},
    value::{
        definition::{BuildDefinition, DefinitionKind},
        package::{
            LockSummary, PackageReleaseIdentity, PackageReleasePurl, VerifiedPackageClosure,
        },
    },
};

/// Complete admitted deterministic package build input.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefinitionPackageBuildRequest {
    source: SealedAuthoredDefinition,
    admitted_definition: BuildDefinition,
    package: PackageReleasePurl,
    dependencies: Vec<VerifiedPackageClosure>,
    output: NewPackageOutputHandle,
}

impl DefinitionPackageBuildRequest {
    /// Creates the exact immutable build input.
    pub fn new(
        source: SealedAuthoredDefinition,
        admitted_definition: BuildDefinition,
        package: PackageReleasePurl,
        dependencies: Vec<VerifiedPackageClosure>,
        output: NewPackageOutputHandle,
    ) -> Self {
        Self {
            source,
            admitted_definition,
            package,
            dependencies,
            output,
        }
    }

    /// Returns the sealed authored bytes.
    pub fn source(&self) -> &SealedAuthoredDefinition {
        &self.source
    }
    /// Returns the NAPE-admitted Product definition build projection.
    pub fn admitted_definition(&self) -> &BuildDefinition {
        &self.admitted_definition
    }
    /// Returns the exact intended PURL.
    pub fn package(&self) -> &PackageReleasePurl {
        &self.package
    }
    /// Returns the complete exact dependency set.
    pub fn dependencies(&self) -> &[VerifiedPackageClosure] {
        &self.dependencies
    }
    /// Returns the required-new output handle.
    pub fn output(&self) -> &NewPackageOutputHandle {
        &self.output
    }
}

/// Completed package build observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuiltDefinitionPackage {
    /// Semantic-root kind.
    pub kind: DefinitionKind,
    /// Exact package-release identity.
    pub identity: PackageReleaseIdentity,
    /// Canonical generated Lock aggregate.
    pub lock: LockSummary,
    /// Exact committed local build result.
    pub output: NewPackageOutputHandle,
    /// Common-verifier observation over the committed package.
    pub verified: VerifiedPackageClosure,
}

/// Builds and commits one exact package without rereading mutable source paths.
pub trait DefinitionPackageBuildGW:
    Gateway<Request = DefinitionPackageBuildRequest, Response = NapeOutcome<BuiltDefinitionPackage>>
{
}
