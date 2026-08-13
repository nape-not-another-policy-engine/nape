//! Verifies exact OCI resolve-plan orchestration.
//!
//! Requirement validation points:
//! - Resolve-plan contains no evidence or execution state.

use std::sync::Arc;

use kernel_oss::usecase::AsyncUseCase;
use test_framework_oss::is_ok;

use super::{
    ResolveDefinitionPackagePlan, ResolveDefinitionPackagePlanRequest,
    ResolveDefinitionPackagePlanUC,
};
use crate::{
    diagnostic::NapeOutcome,
    usecase::test_support::{block_on, package_identity, FakeOciResolution, FakeProjection},
};

/// Requirement validation: the shared async Use Case returns a validated root-only plan.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn resolve_definition_package_plan_use_case_success_async() {
    let use_case =
        ResolveDefinitionPackagePlan::new(Arc::new(FakeOciResolution), Arc::new(FakeProjection));
    let request = ResolveDefinitionPackagePlanRequest::new(package_identity());

    let outcome = is_ok!(block_on(AsyncUseCase::execute(
        &use_case as &dyn ResolveDefinitionPackagePlanUC,
        request,
    )));

    assert!(matches!(outcome, NapeOutcome::Completed(value) if value.dependency_count == 0));
}
