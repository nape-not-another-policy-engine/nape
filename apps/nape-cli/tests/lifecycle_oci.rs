#![cfg(unix)]

pub mod support;

/// Requirement validation: local mode cannot resolve an OCI dependency implicitly.
#[test]
fn local_nonempty_lock_does_not_cross_into_oci_error() {
    support::local_nonempty_lock_cannot_resolve_an_oci_dependency();
}
