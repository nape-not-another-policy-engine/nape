//! Exact OCI package-closure resolution seam.

#[cfg(test)]
mod tests;

use kernel_oss::gateway::AsyncGateway;

use crate::{
    diagnostic::NapeOutcome,
    value::package::{PackageReleaseIdentity, VerifiedPackageClosure},
};

/// Resolves one exact root and its complete locked OCI closure.
pub trait OciDefinitionPackageResolutionGW:
    AsyncGateway<Request = PackageReleaseIdentity, Response = NapeOutcome<VerifiedPackageClosure>>
{
}
