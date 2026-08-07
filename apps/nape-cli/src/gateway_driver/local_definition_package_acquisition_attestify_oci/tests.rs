use std::fs;

use kernel_oss::gateway::Gateway;
use nape_domain::{diagnostic::NapeOutcome, value::package::LocalPackageHandle};

use super::{AttestifyOciLocalPackageAcquisitionDriver, VerifiedPackageStore};

/// Requirement validation: exercises one bounded logical path.

#[test]
fn local_build_result_converges_on_common_verifier_success() {
    let root = std::env::temp_dir().join(format!("nape-local-acquisition-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    let source = root.join("source");
    fs::create_dir_all(source.join("action/database-connection")).expect("source");
    fs::write(
        source.join("verification-action.yaml"),
        br#"apiVersion: 2.0.0
kind: VerificationAction
spec:
  name: database-connection
  label: Database Connection
  claim: The endpoint is configured.
  evidence:
    name: application-configuration
    file: application-configuration.json
  test:
    name: database-connection
    file: database-connection.py
"#,
    )
    .expect("root");
    fs::write(
        source.join("action/database-connection/database-connection.py"),
        b"def evaluate(evidence, evaluations, metadata): return {}\n",
    )
    .expect("test");
    let output = root.join("package");
    attestify_oci::build_local_package(
        crate::gateway_driver::definition_package_profile_attestify_oci::definition_package_profile()
            .expect("profile"),
        "VerificationAction",
        &source,
        "pkg:attestify/acme.example/verification-action/database/database-connection@1.0.0",
        &output,
    )
    .expect("package");
    let driver = AttestifyOciLocalPackageAcquisitionDriver::new(
        VerifiedPackageStore::default(),
        root.join("staging"),
    );
    let outcome = Gateway::execute(
        &driver,
        LocalPackageHandle::try_new(output.to_string_lossy()).expect("handle"),
    )
    .expect("gateway");
    let NapeOutcome::Completed(closure) = outcome else {
        panic!("package rejected");
    };
    assert_eq!(closure.root().name(), "database-connection");
    fs::remove_dir_all(root).expect("cleanup");
}
