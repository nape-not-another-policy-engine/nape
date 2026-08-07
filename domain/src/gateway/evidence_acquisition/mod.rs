//! Stable one-source evidence acquisition seam.

#[cfg(test)]
mod tests;

use kernel_oss::gateway::Gateway;

use crate::{
    diagnostic::NapeOutcome,
    value::{
        evidence::{EvidenceRequirement, EvidenceSourceHandle},
        package::ManifestDigest,
    },
};

/// Requests one stable evidence payload for one occurrence requirement.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceAcquisitionRequest {
    /// Definition-owned occurrence requirement.
    pub requirement: EvidenceRequirement,
    /// Caller-supplied source handle.
    pub source: EvidenceSourceHandle,
}

/// One immutable content-addressed evidence payload.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdmittedEvidencePayload {
    /// Exact content digest.
    pub digest: ManifestDigest,
    /// Exact admitted bytes.
    pub bytes: Vec<u8>,
}

/// Performs one bounded stable read and digest computation.
pub trait EvidenceAcquisitionGW:
    Gateway<Request = EvidenceAcquisitionRequest, Response = NapeOutcome<AdmittedEvidencePayload>>
{
}
