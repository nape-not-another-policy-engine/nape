//! Atomic Verification result-set commitment seam.

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;

use kernel_oss::gateway::Gateway;

use crate::{
    diagnostic::NapeOutcome,
    value::{
        controlled_value::ControlledValue, external_resource_handle::ExternalResourceHandle,
        package::ManifestDigest,
    },
};

/// Complete accepted local result set.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationOutcomeCommitRequest {
    /// Complete VerificationReport value awaiting wire projection.
    pub report: ControlledValue,
    /// Complete Evidence Set relationship value awaiting wire projection.
    pub evidence_set_relationship: ControlledValue,
    /// Unique digest-addressed raw evidence payloads.
    pub evidence_payloads: BTreeMap<ManifestDigest, Vec<u8>>,
    /// NAPE-managed required-new result directory.
    pub output: ExternalResourceHandle,
}

/// Exact atomic output commitment observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationOutcomeCommitObservation {
    /// Committed local result-set handle.
    pub output: ExternalResourceHandle,
    /// Number of committed files.
    pub file_count: u64,
}

/// Atomically promotes a complete Report, relationship, and raw evidence set.
pub trait VerificationOutcomeCommitGW:
    Gateway<
    Request = VerificationOutcomeCommitRequest,
    Response = NapeOutcome<VerificationOutcomeCommitObservation>,
>
{
}
