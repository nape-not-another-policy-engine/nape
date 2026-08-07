//! Atomic current-run evidence association commitment seam.

#[cfg(test)]
mod tests;

use kernel_oss::gateway::Gateway;

use crate::{
    diagnostic::NapeOutcome,
    gateway::evidence_acquisition::AdmittedEvidencePayload,
    value::{current_verification::CurrentVerification, evidence::EvidenceRequirement},
};

/// Complete one-occurrence evidence mutation request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationEvidenceCommitRequest {
    /// Revalidated current state.
    pub current: CurrentVerification,
    /// Selected definition-owned requirement.
    pub requirement: EvidenceRequirement,
    /// Newly admitted immutable payload.
    pub payload: AdmittedEvidencePayload,
}

/// Deterministic evidence commitment disposition.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EvidenceCommitDisposition {
    /// First association for the occurrence.
    Added,
    /// Exact same digest was already associated.
    Unchanged,
    /// A different prior association was atomically replaced.
    Replaced,
}

/// Atomic evidence commitment observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationEvidenceCommitObservation {
    /// Commitment disposition.
    pub disposition: EvidenceCommitDisposition,
    /// Number of unique payloads after commitment.
    pub unique_payload_count: u64,
}

/// Commits one association or replacement without partial state mutation.
pub trait VerificationEvidenceCommitGW:
    Gateway<
    Request = VerificationEvidenceCommitRequest,
    Response = NapeOutcome<VerificationEvidenceCommitObservation>,
>
{
}
