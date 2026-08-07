//! Verifies staged Verification Start orchestration.
//!
//! Requirement validation points:
//! - Start freezes package closure, subject, metadata, invocation ULID, and UTC start before selection.

use std::sync::Arc;

use kernel_oss::usecase::AsyncUseCase;
use test_framework_oss::is_ok;

use super::{
    StartVerification, StartVerificationRequest, StartVerificationUC, VerificationProcedureSource,
};
use crate::{
    diagnostic::NapeOutcome,
    usecase::test_support::{
        block_on, FakeIdentity, FakeLocalAcquisition, FakeOciResolution, FakeStartCommit,
        FakeSubject, FakeTime,
    },
    value::{
        external_resource_handle::ExternalResourceHandle,
        invocation_metadata::InvocationMetadataEntry,
    },
};

/// Requirement validation: local Start returns one committed current run and preserves metadata.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn start_verification_use_case_success_async() {
    let use_case = StartVerification::new(
        Arc::new(FakeLocalAcquisition),
        Arc::new(FakeOciResolution),
        Arc::new(FakeSubject),
        Arc::new(FakeStartCommit),
        Arc::new(FakeIdentity),
        Arc::new(FakeTime),
    );
    let request = is_ok!(StartVerificationRequest::builder()
        .source(VerificationProcedureSource::Local(is_ok!(
            ExternalResourceHandle::try_new("package")
        )))
        .subject(is_ok!(ExternalResourceHandle::try_new("subject.json")))
        .metadata(vec![is_ok!(InvocationMetadataEntry::try_new(
            "build-id", "42"
        ))])
        .try_build());

    let outcome = is_ok!(block_on(AsyncUseCase::execute(
        &use_case as &dyn StartVerificationUC,
        request,
    )));

    assert!(
        matches!(outcome, NapeOutcome::Completed(value) if value.current.metadata().values().get("build-id").is_some())
    );
}
