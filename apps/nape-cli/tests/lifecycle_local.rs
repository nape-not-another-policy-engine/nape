#![cfg(unix)]

pub mod support;

/// Requirement validation: a local package build is exact and non-overwriting.
#[test]
fn local_package_build_lifecycle_success() {
    support::package_build_is_exact_non_overwriting_and_receipted();
}
