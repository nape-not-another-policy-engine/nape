use std::{collections::BTreeMap, fs};

use kernel_oss::gateway::Gateway;
use nape_domain::{
    diagnostic::NapeOutcome,
    gateway::{
        authored_definition_source::{NewPackageOutputHandle, SealedAuthoredDefinition},
        definition_package_build::DefinitionPackageBuildRequest,
    },
    service::definition_admission::admit_build_definition,
    value::package::PackageReleasePurl,
};

use super::{AttestifyOciDefinitionPackageBuildDriver, VerifiedPackageStore};

/// Requirement validation: exercises one bounded logical path.

#[test]
fn sealed_source_builds_and_commits_one_exact_package_success() {
    let root = std::env::temp_dir().join(format!("nape-package-driver-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir(&root).expect("root");
    let yaml = br#"apiVersion: 2.0.0
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
"#
    .to_vec();
    let files = BTreeMap::from([
        ("verification-action.yaml".to_string(), yaml.clone()),
        (
            "action/database-connection/database-connection.py".to_string(),
            b"def evaluate(evidence, evaluations, metadata): return {}\n".to_vec(),
        ),
    ]);
    let output = root.join("package");
    let projection = crate::gateway_driver::controlled_document_projection::project_yaml(&yaml)
        .map(crate::gateway_driver::controlled_document_projection::from_json)
        .expect("projection");
    let request = DefinitionPackageBuildRequest::new(
        SealedAuthoredDefinition::new("verification-action.yaml", "application/yaml", files),
        admit_build_definition(
            &projection,
            "pkg:attestify/acme.example/verification-action/database/database-connection@1.0.0",
        )
        .expect("definition")
        .build,
        PackageReleasePurl::try_new(
            "pkg:attestify/acme.example/verification-action/database/database-connection@1.0.0",
        )
        .expect("PURL"),
        Vec::new(),
        NewPackageOutputHandle::try_new(output.to_string_lossy()).expect("output"),
    );
    let driver = AttestifyOciDefinitionPackageBuildDriver::new(VerifiedPackageStore::default());
    let outcome = Gateway::execute(&driver, request).expect("gateway");
    let NapeOutcome::Completed(package) = outcome else {
        panic!("build rejected");
    };
    assert_eq!(package.lock.node_count, 0);
    assert!(output.join("manifest.json").is_file());
    fs::remove_dir_all(root).expect("cleanup");
}
