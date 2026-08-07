//! Atomic new-current-run commitment seam.

#[cfg(test)]
mod tests;

use kernel_oss::gateway::Gateway;

use crate::{
    diagnostic::NapeOutcome,
    value::{
        current_verification::{CurrentVerification, CurrentVerificationHandle},
        external_resource_handle::ExternalResourceHandle,
    },
};

/// Complete prepared Start state.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationStartCommitRequest {
    /// Complete prepared current state.
    pub verification: CurrentVerification,
}

/// Atomic Start commitment observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationStartCommitObservation {
    /// Selected current-run handle.
    pub current_run: CurrentVerificationHandle,
    /// Opaque current-state file handle.
    pub state: ExternalResourceHandle,
    /// NAPE-managed required-new result-set handle.
    pub result_output: ExternalResourceHandle,
}

/// Creates a new run and selects it only after complete preparation.
pub trait VerificationStartCommitGW:
    Gateway<
    Request = VerificationStartCommitRequest,
    Response = NapeOutcome<VerificationStartCommitObservation>,
>
{
}
