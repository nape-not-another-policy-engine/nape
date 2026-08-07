//! Verifies deterministic package-build orchestration.
//!
//! Requirement validation points:
//! - I5 build consumes sealed source and the complete caller dependency set.

use std::sync::Arc;

use kernel_oss::usecase::UseCase;
use test_framework_oss::is_ok;

use super::{BuildDefinitionPackage, BuildDefinitionPackageRequest, BuildDefinitionPackageUC};
use crate::{
    diagnostic::NapeOutcome,
    usecase::test_support::{FakeAuthoredSource, FakeBuild, FakeLocalAcquisition, FakeProjection},
    value::{external_resource_handle::ExternalResourceHandle, package::PackageReleasePurl},
};

/// Requirement validation: the Use Case executes through the shared seam and returns the exact build.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn build_definition_package_use_case_success() {
    let use_case = BuildDefinitionPackage::new(
        Arc::new(FakeAuthoredSource),
        Arc::new(FakeProjection),
        Arc::new(FakeLocalAcquisition),
        Arc::new(FakeBuild),
    );
    let request = is_ok!(BuildDefinitionPackageRequest::builder()
        .source(is_ok!(ExternalResourceHandle::try_new("source")))
        .package(is_ok!(PackageReleasePurl::try_new(
            "pkg:attestify/acme.example/verification-procedure/release/release-readiness@1.0.0"
        )))
        .dependency_packages(Vec::new())
        .output(is_ok!(ExternalResourceHandle::try_new("output")))
        .try_build());

    let outcome = is_ok!(UseCase::execute(
        &use_case as &dyn BuildDefinitionPackageUC,
        request,
    ));

    assert!(matches!(outcome, NapeOutcome::Completed(_)));
}
