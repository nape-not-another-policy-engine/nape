//! Collect and atomically associate one evidence payload with the current run.

#[cfg(test)]
mod tests;

use std::sync::Arc;

use kernel_oss::{
    error::{Error, Kind},
    gateway::Gateway,
    usecase::UseCase,
};

use crate::{
    diagnostic::NapeOutcome,
    gateway::{
        current_verification_acquisition::{
            CurrentVerificationAcquisitionGW, CurrentVerificationAcquisitionRequest,
        },
        evidence_acquisition::{EvidenceAcquisitionGW, EvidenceAcquisitionRequest},
        verification_evidence_commit::{
            EvidenceCommitDisposition, VerificationEvidenceCommitGW,
            VerificationEvidenceCommitRequest,
        },
    },
    service::evidence_admission::select_evidence_requirement,
    value::{
        effective_graph::CanonicalActionSelector,
        evidence::{EvidenceFileName, EvidenceSourceHandle},
        package::ManifestDigest,
    },
};

/// One-file staged Evidence request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectVerificationEvidenceRequest {
    action: CanonicalActionSelector,
    evidence: EvidenceSourceHandle,
    file_name: Option<EvidenceFileName>,
}

impl CollectVerificationEvidenceRequest {
    /// Starts a builder.
    pub fn builder() -> CollectVerificationEvidenceRequestBuilder {
        CollectVerificationEvidenceRequestBuilder::default()
    }
    /// Returns the selected occurrence.
    pub fn action(&self) -> &CanonicalActionSelector {
        &self.action
    }
    /// Returns the caller evidence source.
    pub fn evidence(&self) -> &EvidenceSourceHandle {
        &self.evidence
    }
    /// Returns the optional V1-compatible filename assertion.
    pub fn file_name(&self) -> Option<&EvidenceFileName> {
        self.file_name.as_ref()
    }
}

/// Builder for [`CollectVerificationEvidenceRequest`].
#[derive(Default)]
pub struct CollectVerificationEvidenceRequestBuilder {
    action: Option<CanonicalActionSelector>,
    evidence: Option<EvidenceSourceHandle>,
    file_name: Option<EvidenceFileName>,
}

impl CollectVerificationEvidenceRequestBuilder {
    /// Sets the contextual occurrence selector.
    pub fn action(mut self, value: CanonicalActionSelector) -> Self {
        self.action = Some(value);
        self
    }
    /// Sets the one source handle.
    pub fn evidence(mut self, value: EvidenceSourceHandle) -> Self {
        self.evidence = Some(value);
        self
    }
    /// Sets the optional definition-filename assertion.
    pub fn file_name(mut self, value: EvidenceFileName) -> Self {
        self.file_name = Some(value);
        self
    }
    /// Validates and builds the request.
    pub fn try_build(self) -> Result<CollectVerificationEvidenceRequest, Error> {
        Ok(CollectVerificationEvidenceRequest {
            action: self.action.ok_or_else(|| {
                Error::for_user(Kind::InvalidInput, "Action selector is required")
            })?,
            evidence: self.evidence.ok_or_else(|| {
                Error::for_user(Kind::InvalidInput, "Evidence source is required")
            })?,
            file_name: self.file_name,
        })
    }
}

/// Completed one-file evidence association.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CollectedVerificationEvidence {
    /// Current invocation ULID.
    pub invocation_id: String,
    /// Contextual Action selector.
    pub action: CanonicalActionSelector,
    /// Definition-owned evidence name.
    pub evidence_name: String,
    /// Definition-owned evidence filename.
    pub evidence_file: EvidenceFileName,
    /// Exact content digest.
    pub content_digest: ManifestDigest,
    /// Exact byte count.
    pub byte_count: u64,
    /// Atomic commitment disposition.
    pub disposition: EvidenceCommitDisposition,
    /// Unique payload count after commitment.
    pub unique_payload_count: u64,
}

/// Bounded Evidence outcome.
pub type CollectVerificationEvidenceOutcome = NapeOutcome<CollectedVerificationEvidence>;

/// Public marker seam for one-file staged Evidence.
pub trait CollectVerificationEvidenceUC:
    UseCase<Request = CollectVerificationEvidenceRequest, Response = CollectVerificationEvidenceOutcome>
{
}

/// Core one-file Evidence orchestration.
pub struct CollectVerificationEvidence {
    current: Arc<dyn CurrentVerificationAcquisitionGW>,
    acquisition: Arc<dyn EvidenceAcquisitionGW>,
    commit: Arc<dyn VerificationEvidenceCommitGW>,
}

impl CollectVerificationEvidence {
    /// Creates the use case with exact state, acquisition, and commit dependencies.
    pub fn new(
        current: Arc<dyn CurrentVerificationAcquisitionGW>,
        acquisition: Arc<dyn EvidenceAcquisitionGW>,
        commit: Arc<dyn VerificationEvidenceCommitGW>,
    ) -> Self {
        Self {
            current,
            acquisition,
            commit,
        }
    }
}

impl UseCase for CollectVerificationEvidence {
    type Request = CollectVerificationEvidenceRequest;
    type Response = CollectVerificationEvidenceOutcome;

    fn execute(&self, request: Self::Request) -> Result<Self::Response, Error> {
        let current = match Gateway::execute(
            self.current.as_ref() as &dyn CurrentVerificationAcquisitionGW,
            CurrentVerificationAcquisitionRequest,
        )? {
            NapeOutcome::Completed(value) => value,
            NapeOutcome::Rejected(value) => return Ok(NapeOutcome::Rejected(value)),
        };
        let requirement = select_evidence_requirement(
            current.evidence_requirements(),
            &request.action,
            request.file_name.as_ref(),
        )?;
        let payload = match Gateway::execute(
            self.acquisition.as_ref() as &dyn EvidenceAcquisitionGW,
            EvidenceAcquisitionRequest {
                requirement: requirement.clone(),
                source: request.evidence,
            },
        )? {
            NapeOutcome::Completed(value) => value,
            NapeOutcome::Rejected(value) => return Ok(NapeOutcome::Rejected(value)),
        };
        let commitment = match Gateway::execute(
            self.commit.as_ref() as &dyn VerificationEvidenceCommitGW,
            VerificationEvidenceCommitRequest {
                current: current.clone(),
                requirement: requirement.clone(),
                payload: payload.clone(),
            },
        )? {
            NapeOutcome::Completed(value) => value,
            NapeOutcome::Rejected(value) => return Ok(NapeOutcome::Rejected(value)),
        };
        Ok(NapeOutcome::Completed(CollectedVerificationEvidence {
            invocation_id: current.invocation_id().value().to_string(),
            action: request.action,
            evidence_name: requirement.name().to_string(),
            evidence_file: requirement.file().clone(),
            content_digest: payload.digest,
            byte_count: payload.bytes.len() as u64,
            disposition: commitment.disposition,
            unique_payload_count: commitment.unique_payload_count,
        }))
    }
}

impl CollectVerificationEvidenceUC for CollectVerificationEvidence {}
