//! Verifies exact package publication orchestration.
//!
//! Requirement validation points:
//! - Publication pushes a verified build result without rebuilding it.

use std::sync::Arc;

use kernel_oss::usecase::AsyncUseCase;
use test_framework_oss::is_ok;

use super::{
    PublishDefinitionPackage, PublishDefinitionPackageRequest, PublishDefinitionPackageUC,
};
use crate::{
    diagnostic::NapeOutcome,
    usecase::test_support::{block_on, FakeLocalAcquisition, FakeProjection, FakePublication},
    value::external_resource_handle::ExternalResourceHandle,
};

/// Requirement validation: the shared async Use Case returns a clean-repull-equal publication.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn publish_definition_package_use_case_success_async() {
    let use_case = PublishDefinitionPackage::new(
        Arc::new(FakeLocalAcquisition),
        Arc::new(FakeProjection),
        Arc::new(FakePublication),
    );
    let request =
        PublishDefinitionPackageRequest::new(is_ok!(ExternalResourceHandle::try_new("package")));

    let outcome = is_ok!(block_on(AsyncUseCase::execute(
        &use_case as &dyn PublishDefinitionPackageUC,
        request,
    )));

    assert!(matches!(outcome, NapeOutcome::Completed(value) if value.clean_repull_equal));
}
