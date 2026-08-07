#![cfg(unix)]

pub mod support;

/// Requirement validation: dependency inputs produce exact direct and transitive Locks.
#[test]
fn staged_dependency_package_construction_is_exact_success() {
    support::package_build_constructs_exact_i5_direct_and_transitive_locks();
}
