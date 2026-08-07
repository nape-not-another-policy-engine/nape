use std::{fs, os::unix::fs::PermissionsExt};

use kernel_oss::gateway::Gateway;
use nape_domain::{
    diagnostic::NapeOutcome,
    gateway::action_evaluation::{ActionEvaluationContract, ActionEvaluationRequest},
    value::{
        controlled_value::ControlledValue, effective_graph::CanonicalActionSelector,
        evidence::EvidenceArgument, verification_outcome::VerificationConclusion,
    },
};
use serde_json::json;
use sha2::Digest;

use super::contract_v2::{canonical_json, EvaluatorExecutable};

use super::{
    create_private_workspace, NapeEvaluatorActionEvaluationDriver, SelectedEvaluatorExecutable,
};

#[cfg(unix)]
/// Requirement validation: caller-created Evaluator workspaces satisfy provider custody.
#[test]
fn evaluator_workspace_is_private_and_cannot_be_reused_success() {
    let root = std::env::temp_dir().join(format!(
        "nape-evaluator-private-workspace-{}",
        std::process::id()
    ));
    let _ = fs::remove_dir_all(&root);
    let workspace = root.join("parent/action");
    create_private_workspace(&workspace).expect("test setup should succeed");
    assert_eq!(
        fs::metadata(&workspace)
            .expect("test setup should succeed")
            .permissions()
            .mode()
            & 0o777,
        0o700
    );
    assert!(create_private_workspace(&workspace).is_err());
    fs::remove_dir_all(root).expect("test setup should succeed");
}

#[cfg(unix)]
/// Requirement validation: exercises one bounded logical path.
#[test]
fn v2_driver_returns_only_the_semantic_action_observation_success() {
    let root = std::env::temp_dir().join(format!("nape-evaluator-driver-{}", std::process::id()));
    let _ = fs::remove_dir_all(&root);
    fs::create_dir(&root).expect("root");
    let executable = root.join("evaluator.py");
    fs::write(&executable, r#"#!/usr/bin/env python3
import json, sys
r=json.load(sys.stdin)
print(json.dumps({"contract":r["contract"],"diagnostic":None,"disposition":"occurrence-result","execution":{"automatic_retry_count":0,"evaluate_call_count":1,"executed":True,"phase":"evaluate-call","status":"completed"},"result":{"conclusion":"true","facts":[],"reason":"Ready."},"result_validation":{"contract":"passed","limit":"passed"},"semantic_owner":"test-of-detail"},sort_keys=True,separators=(",",":")))
"#).expect("evaluator");
    let mut permissions = fs::metadata(&executable).expect("metadata").permissions();
    permissions.set_mode(0o700);
    fs::set_permissions(&executable, permissions).expect("permissions");
    let record = root.join("build.json");
    let bytes = canonical_json(&json!({
        "contract":"attestify.nape-evaluator.installed-build/v1",
        "contract_projection_digest":format!("sha256:{}", "1".repeat(64)),
        "dependency_set_digest":format!("sha256:{}", "2".repeat(64)),
        "evaluator_contract":"attestify.nape-evaluator.action-invocation/v2",
        "evaluator_release":"2.0.0",
        "implementation_digest":format!("sha256:{}", "3".repeat(64)),
        "runner_profile":"attestify-python-test-development-v1"
    }))
    .expect("canonical")
    .into_bytes();
    fs::write(&record, &bytes).expect("record");
    let digest = format!("sha256:{}", hex::encode(sha2::Sha256::digest(&bytes)));
    let selected = EvaluatorExecutable::select(&executable, &record, &digest).expect("selection");
    let driver = NapeEvaluatorActionEvaluationDriver::new(
        SelectedEvaluatorExecutable::V2(selected),
        root.join("workspace"),
    );
    let outcome = Gateway::execute(
        &driver,
        ActionEvaluationRequest {
            action: CanonicalActionSelector::try_new("release-readiness.database-connection")
                .expect("selector"),
            evidence: EvidenceArgument::Opaque(b"{}".to_vec()),
            evidence_file: "application-configuration.json".to_string(),
            evidence_media_type: "application/json".to_string(),
            evaluations: Vec::new(),
            metadata: ControlledValue::Object(Default::default()),
            test_file: "database-connection.py".to_string(),
            test_source: b"def evaluate(evidence, evaluations, metadata): return {}\n".to_vec(),
            contract: ActionEvaluationContract::V2,
            modules: Vec::new(),
            maximum_execution_work_units: 1_000_000,
            maximum_result_bytes: 65_536,
        },
    )
    .expect("gateway");
    let NapeOutcome::Completed(observation) = outcome else {
        panic!("evaluation rejected");
    };
    assert_eq!(observation.conclusion, VerificationConclusion::True);
    assert_eq!(observation.reason, "Ready.");
    fs::remove_dir_all(root).expect("cleanup");
}
