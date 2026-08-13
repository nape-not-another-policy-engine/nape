use std::fs;

use kernel_oss::gateway::Gateway;
use nape_domain::{
    diagnostic::NapeOutcome,
    gateway::authored_definition_source::{
        AuthoredDefinitionSourceHandle, AuthoredDefinitionSourceRequest,
    },
    value::package::PackageReleasePurl,
};

use super::LocalAuthoredDefinitionSourceDriver;

/// Requirement validation: exercises one bounded logical path.

#[test]
fn source_is_sealed_with_the_kind_selected_root_success() {
    let root = std::env::temp_dir().join(format!("nape-authored-source-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir(&root).expect("source root");
    fs::write(
        root.join("verification-action.yaml"),
        b"kind: VerificationAction\n",
    )
    .expect("root file");
    let request = AuthoredDefinitionSourceRequest::new(
        AuthoredDefinitionSourceHandle::try_new(root.to_string_lossy()).expect("handle"),
        PackageReleasePurl::try_new(
            "pkg:attestify/acme.example/verification-action/database/connectivity@1.0.0",
        )
        .expect("PURL"),
    );
    let outcome = Gateway::execute(&LocalAuthoredDefinitionSourceDriver, request).expect("gateway");
    let NapeOutcome::Completed(source) = outcome else {
        panic!("source rejected");
    };
    assert_eq!(source.semantic_root(), "verification-action.yaml");
    assert_eq!(source.files().len(), 1);
    fs::remove_dir_all(root).expect("cleanup");
}
