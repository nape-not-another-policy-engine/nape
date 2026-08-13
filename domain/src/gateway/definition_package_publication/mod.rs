//! Exact package publication and clean-repull observation seam.

#[cfg(test)]
mod tests;

use kernel_oss::gateway::AsyncGateway;

use crate::{diagnostic::NapeOutcome, value::package::VerifiedPackageClosure};

/// One exact registry location observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RegistryLocationObservation {
    /// Registry-map profile version.
    pub profile_version: String,
    /// Publisher selected from the package PURL.
    pub publisher: String,
    /// Controlled transport scheme.
    pub scheme: String,
    /// Selected registry origin.
    pub registry: String,
    /// Exact mapped repository.
    pub repository: String,
    /// Exact pushed reference.
    pub reference: String,
    /// Tag or digest reference class.
    pub reference_class: String,
}

/// Completed publication meaning.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PublishedDefinitionPackage {
    /// Exact published package closure.
    pub package: VerifiedPackageClosure,
    /// Exact mapped registry location.
    pub location: RegistryLocationObservation,
    /// Publication disposition.
    pub disposition: String,
    /// Whether the clean re-pull was exactly equal.
    pub clean_repull_equal: bool,
}

/// Publishes an exact verified build result without rebuilding it.
pub trait DefinitionPackagePublicationGW:
    AsyncGateway<Request = VerifiedPackageClosure, Response = NapeOutcome<PublishedDefinitionPackage>>
{
}
