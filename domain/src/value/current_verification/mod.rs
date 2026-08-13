//! NAPE-managed staged Verification state values.

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;

use kernel_oss::error::{Error, Kind};

use super::{
    effective_graph::EffectiveVerificationGraph,
    evidence::{EvidenceAssociation, EvidenceRequirement},
    external_resource_handle::ExternalResourceHandle,
    invocation_metadata::InvocationMetadata,
    package::VerifiedPackageClosure,
    subject::VerificationSubject,
};

/// How the exact Procedure closure entered the current invocation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum VerificationAcquisition {
    /// Exact verified local build result.
    LocalBuild,
    /// Exact digest-verified OCI pull.
    OciPull,
}

impl VerificationAcquisition {
    /// Stable report/receipt source vocabulary.
    pub fn source(self) -> &'static str {
        match self {
            Self::LocalBuild => "local-build",
            Self::OciPull => "oci-pull",
        }
    }

    /// Stable report/receipt binding vocabulary.
    pub fn binding(self) -> &'static str {
        match self {
            Self::LocalBuild => "local-build-result",
            Self::OciPull => "invocation",
        }
    }
}

/// One NAPE-generated staged invocation identity.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationInvocationId(String);

impl VerificationInvocationId {
    /// Creates a canonical 26-character ULID string.
    pub fn try_new(value: impl Into<String>) -> Result<Self, Error> {
        let value = value.into();
        kernel_oss::ulid::ULID::from_string(&value).map_err(|_| {
            Error::for_user(
                Kind::InvalidInput,
                "verification invocation identity must be a canonical ULID",
            )
        })?;
        Ok(Self(value))
    }

    /// Returns the ULID string.
    pub fn value(&self) -> &str {
        &self.0
    }
}

/// Opaque NAPE-managed run location.
pub type CurrentVerificationHandle = ExternalResourceHandle;

/// Complete immutable identity, subject, package, and time state frozen by Start.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationStartState {
    invocation_id: VerificationInvocationId,
    subject: VerificationSubject,
    metadata: InvocationMetadata,
    package_closure: VerifiedPackageClosure,
    utc_start_milliseconds: u64,
}

impl VerificationStartState {
    /// Creates the complete pre-evidence Start state.
    pub fn new(
        invocation_id: VerificationInvocationId,
        subject: VerificationSubject,
        metadata: InvocationMetadata,
        package_closure: VerifiedPackageClosure,
        utc_start_milliseconds: u64,
    ) -> Self {
        Self {
            invocation_id,
            subject,
            metadata,
            package_closure,
            utc_start_milliseconds,
        }
    }
}

/// Complete immutable evidence state associated with one current run.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationEvidenceState {
    requirements: Vec<EvidenceRequirement>,
    associations: Vec<EvidenceAssociation>,
    payloads: BTreeMap<crate::value::package::ManifestDigest, Vec<u8>>,
}

impl VerificationEvidenceState {
    /// Creates one complete evidence-state observation.
    pub fn new(
        requirements: Vec<EvidenceRequirement>,
        associations: Vec<EvidenceAssociation>,
        payloads: BTreeMap<crate::value::package::ManifestDigest, Vec<u8>>,
    ) -> Self {
        Self {
            requirements,
            associations,
            payloads,
        }
    }
}

/// The immutable state selected by Start and incrementally enriched by Evidence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CurrentVerification {
    invocation_id: VerificationInvocationId,
    subject: VerificationSubject,
    metadata: InvocationMetadata,
    package_closure: VerifiedPackageClosure,
    effective_graph: Option<EffectiveVerificationGraph>,
    acquisition: VerificationAcquisition,
    evidence_requirements: Vec<EvidenceRequirement>,
    evidence_associations: Vec<EvidenceAssociation>,
    evidence_payloads: BTreeMap<crate::value::package::ManifestDigest, Vec<u8>>,
    run: Option<CurrentVerificationHandle>,
    result_output: Option<ExternalResourceHandle>,
    utc_start_milliseconds: u64,
}

impl CurrentVerification {
    /// Creates one frozen Start result.
    pub fn new(start: VerificationStartState, evidence: VerificationEvidenceState) -> Self {
        Self {
            invocation_id: start.invocation_id,
            subject: start.subject,
            metadata: start.metadata,
            package_closure: start.package_closure,
            effective_graph: None,
            acquisition: VerificationAcquisition::LocalBuild,
            evidence_requirements: evidence.requirements,
            evidence_associations: evidence.associations,
            evidence_payloads: evidence.payloads,
            run: None,
            result_output: None,
            utc_start_milliseconds: start.utc_start_milliseconds,
        }
    }

    /// Adds the complete resolved graph and exact acquisition mode before
    /// Start commitment.
    pub fn with_resolution(
        mut self,
        graph: EffectiveVerificationGraph,
        acquisition: VerificationAcquisition,
    ) -> Self {
        self.effective_graph = Some(graph);
        self.acquisition = acquisition;
        self
    }

    /// Adds driver-produced NAPE-managed run and result handles after atomic Start commitment.
    pub fn with_managed_locations(
        mut self,
        run: CurrentVerificationHandle,
        result_output: ExternalResourceHandle,
    ) -> Self {
        self.run = Some(run);
        self.result_output = Some(result_output);
        self
    }

    /// Replaces the complete admitted evidence state after one atomic
    /// application-side commitment.
    pub fn with_evidence_state(
        mut self,
        evidence_associations: Vec<EvidenceAssociation>,
        evidence_payloads: BTreeMap<crate::value::package::ManifestDigest, Vec<u8>>,
    ) -> Self {
        self.evidence_associations = evidence_associations;
        self.evidence_payloads = evidence_payloads;
        self
    }

    /// Returns the invocation identity.
    pub fn invocation_id(&self) -> &VerificationInvocationId {
        &self.invocation_id
    }

    /// Returns the admitted subject.
    pub fn subject(&self) -> &VerificationSubject {
        &self.subject
    }

    /// Returns frozen caller metadata.
    pub fn metadata(&self) -> &InvocationMetadata {
        &self.metadata
    }

    /// Returns the complete Start-frozen verified package closure.
    pub fn package_closure(&self) -> &VerifiedPackageClosure {
        &self.package_closure
    }

    /// Returns the complete Start-frozen effective graph.
    pub fn effective_graph(&self) -> Option<&EffectiveVerificationGraph> {
        self.effective_graph.as_ref()
    }

    /// Returns the exact Procedure acquisition mode.
    pub fn acquisition(&self) -> VerificationAcquisition {
        self.acquisition
    }

    /// Returns complete ordered evidence requirements.
    pub fn evidence_requirements(&self) -> &[EvidenceRequirement] {
        &self.evidence_requirements
    }

    /// Returns current occurrence-to-payload associations.
    pub fn evidence_associations(&self) -> &[EvidenceAssociation] {
        &self.evidence_associations
    }

    /// Returns unique current digest-addressed payload bytes.
    pub fn evidence_payloads(&self) -> &BTreeMap<crate::value::package::ManifestDigest, Vec<u8>> {
        &self.evidence_payloads
    }

    /// Returns the selected NAPE-managed run handle after Start commitment.
    pub fn run(&self) -> Option<&CurrentVerificationHandle> {
        self.run.as_ref()
    }

    /// Returns the NAPE-managed required-new result handle.
    pub fn result_output(&self) -> Option<&ExternalResourceHandle> {
        self.result_output.as_ref()
    }

    /// Returns the Engine-owned Start time in epoch milliseconds.
    pub fn utc_start_milliseconds(&self) -> u64 {
        self.utc_start_milliseconds
    }
}
