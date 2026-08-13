//! Exact local package build-result acquisition seam.

#[cfg(test)]
mod tests;

use kernel_oss::gateway::Gateway;

use crate::{
    diagnostic::NapeOutcome,
    value::package::{LocalPackageHandle, VerifiedPackageClosure},
};

/// Acquires one exact local package build result through the common verifier.
pub trait LocalDefinitionPackageAcquisitionGW:
    Gateway<Request = LocalPackageHandle, Response = NapeOutcome<VerifiedPackageClosure>>
{
}
