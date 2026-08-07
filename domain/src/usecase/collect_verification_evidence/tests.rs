//! Verifies one-file staged Evidence orchestration.
//!
//! Requirement validation points:
//! - Evidence supports source-name independence and atomic add/replace semantics.

use std::sync::Arc;

use kernel_oss::usecase::UseCase;
use test_framework_oss::is_ok;

use super::{
    CollectVerificationEvidence, CollectVerificationEvidenceRequest, CollectVerificationEvidenceUC,
};
use crate::{
    diagnostic::NapeOutcome,
    gateway::verification_evidence_commit::EvidenceCommitDisposition,
    usecase::test_support::{FakeCurrent, FakeEvidenceAcquisition, FakeEvidenceCommit},
    value::{
        effective_graph::CanonicalActionSelector, external_resource_handle::ExternalResourceHandle,
    },
};

/// Requirement validation: one arbitrary source filename maps to the definition-owned evidence file.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn collect_verification_evidence_use_case_success() {
    let use_case = CollectVerificationEvidence::new(
        Arc::new(FakeCurrent(false)),
        Arc::new(FakeEvidenceAcquisition),
        Arc::new(FakeEvidenceCommit(EvidenceCommitDisposition::Added, 1)),
    );
    let request = is_ok!(CollectVerificationEvidenceRequest::builder()
        .action(is_ok!(CanonicalActionSelector::try_new(
            "release-readiness.database-connection"
        )))
        .evidence(is_ok!(ExternalResourceHandle::try_new("prod-export.json")))
        .try_build());

    let outcome = is_ok!(UseCase::execute(
        &use_case as &dyn CollectVerificationEvidenceUC,
        request,
    ));

    assert!(
        matches!(outcome, NapeOutcome::Completed(value) if value.disposition == EvidenceCommitDisposition::Added && value.evidence_file.value() == "application-configuration.json")
    );
}

/// Requirement validation: a different repeat may replace one association while retaining shared payloads.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn repeated_evidence_atomic_replacement_success() {
    let use_case = CollectVerificationEvidence::new(
        Arc::new(FakeCurrent(false)),
        Arc::new(FakeEvidenceAcquisition),
        Arc::new(FakeEvidenceCommit(EvidenceCommitDisposition::Replaced, 1)),
    );
    let request = is_ok!(CollectVerificationEvidenceRequest::builder()
        .action(is_ok!(CanonicalActionSelector::try_new(
            "release-readiness.database-connection"
        )))
        .evidence(is_ok!(ExternalResourceHandle::try_new("replacement.bin")))
        .try_build());

    let outcome = is_ok!(UseCase::execute(
        &use_case as &dyn CollectVerificationEvidenceUC,
        request,
    ));

    assert!(
        matches!(outcome, NapeOutcome::Completed(value) if value.disposition == EvidenceCommitDisposition::Replaced && value.unique_payload_count == 1)
    );
}
