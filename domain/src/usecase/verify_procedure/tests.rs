//! Verifies argument-free current-run Procedure execution.
//!
//! Requirement validation points:
//! - Verify acquires current state, performs no source resolution, and commits one local result set.

use std::sync::Arc;

use kernel_oss::usecase::AsyncUseCase;
use test_framework_oss::is_ok;

use super::{VerifyProcedure, VerifyProcedureRequest, VerifyProcedureUC};
use crate::{
    diagnostic::NapeOutcome,
    usecase::test_support::{
        block_on, FakeCurrent, FakeEvaluation, FakeIdentity, FakeOutcomeCommit, FakeTime,
    },
};

/// Requirement validation: complete frozen evidence executes and produces one true conclusion.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn verify_procedure_use_case_success_async() {
    let use_case = VerifyProcedure::new(
        Arc::new(FakeCurrent(true)),
        Arc::new(FakeEvaluation),
        Arc::new(FakeOutcomeCommit),
        Arc::new(FakeIdentity),
        Arc::new(FakeTime),
    );

    let outcome = is_ok!(block_on(AsyncUseCase::execute(
        &use_case as &dyn VerifyProcedureUC,
        VerifyProcedureRequest,
    )));

    assert!(matches!(outcome, NapeOutcome::Completed(value) if value.summary.conclusion_true == 1));
}

/// Requirement validation: incomplete evidence rejects before Test execution or output commitment.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn verify_procedure_incomplete_evidence_error_async() {
    let use_case = VerifyProcedure::new(
        Arc::new(FakeCurrent(false)),
        Arc::new(FakeEvaluation),
        Arc::new(FakeOutcomeCommit),
        Arc::new(FakeIdentity),
        Arc::new(FakeTime),
    );

    let outcome = is_ok!(block_on(AsyncUseCase::execute(
        &use_case as &dyn VerifyProcedureUC,
        VerifyProcedureRequest,
    )));

    assert!(
        matches!(outcome, NapeOutcome::Rejected(value) if value.code() == "evidence_association_invalid")
    );
}
