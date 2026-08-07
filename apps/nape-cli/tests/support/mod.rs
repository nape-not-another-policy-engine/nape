#![cfg(unix)]

use attestify_oci_oss::{
    sha256_digest, stage_and_verify_local_package, ExactPackageDependencyRule,
    ExactPackageKindProfile, ExactPackageProfile,
};
use serde_json::{json, Value};
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

pub const PACKAGE: &str =
    "pkg:attestify/acme.example/verification-procedure/release-readiness/database-endpoint@1.0.0";
pub const ACTION_PACKAGE: &str =
    "pkg:attestify/acme.example/verification-action/release-readiness/database-connection@1.0.0";
pub const L1_PACKAGE: &str =
    "pkg:attestify/acme.example/verification-procedure/release-readiness/database-endpoint@1.1.0";
pub const ACTIVITY_PACKAGE: &str =
    "pkg:attestify/acme.example/verification-activity/release-readiness/database-readiness@1.0.0";
pub const L2_PACKAGE: &str =
    "pkg:attestify/acme.example/verification-procedure/release-readiness/transitive-database-readiness@1.0.0";

pub fn definition_package_profile() -> ExactPackageProfile {
    let kinds = vec![
        ExactPackageKindProfile::try_new(
            "VerificationProcedure",
            "verification-procedure",
            "verification-procedure.yaml",
            "application/yaml",
        ),
        ExactPackageKindProfile::try_new(
            "VerificationActivity",
            "verification-activity",
            "verification-activity.yaml",
            "application/yaml",
        ),
        ExactPackageKindProfile::try_new(
            "VerificationAction",
            "verification-action",
            "verification-action.yaml",
            "application/yaml",
        ),
    ]
    .into_iter()
    .collect::<Result<Vec<_>, _>>()
    .expect("test package kinds should be valid");
    let dependencies = vec![
        ExactPackageDependencyRule::try_new("VerificationProcedure", "VerificationActivity"),
        ExactPackageDependencyRule::try_new("VerificationProcedure", "VerificationAction"),
        ExactPackageDependencyRule::try_new("VerificationActivity", "VerificationAction"),
    ]
    .into_iter()
    .collect::<Result<Vec<_>, _>>()
    .expect("test dependency rules should be valid");
    ExactPackageProfile::try_new(
        "application/vnd.attestify.definition-package.v1",
        "application/vnd.attestify.definition-package.config.v1+json",
        "application/vnd.attestify.package.v1.tar",
        "attestify-oci-repository-profile/1",
        "attestify",
        kinds,
        dependencies,
    )
    .expect("test package profile should be valid")
}

pub fn binary() -> &'static str {
    env!("CARGO_BIN_EXE_nape")
}

pub fn product() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "../../../../2-other-workspaces/attestify-design/attestify-product-specification/05-specs/products/verification-engine/schemas/i0-i4/conformance/golden/step-4",
    )
}

pub fn product_i5_source() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(
        "../../../../2-other-workspaces/attestify-design/attestify-product-specification/05-specs/products/verification-engine/schemas/i0-i5/conformance/source",
    )
}

pub fn current_v2_examples() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../docs/examples/verification-v2-oci")
}

pub fn temporary(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("test setup should succeed")
        .as_nanos();
    let path = std::env::temp_dir().join(format!("nape-i2-{name}-{}-{nanos}", std::process::id()));
    fs::create_dir(&path).expect("test setup should succeed");
    path
}

pub fn receipt(output: &Output) -> Value {
    assert_eq!(output.stdout.last(), Some(&b'\n'));
    assert_eq!(
        output.stdout.iter().filter(|byte| **byte == b'\n').count(),
        1
    );
    serde_json::from_slice(&output.stdout[..output.stdout.len() - 1])
        .expect("test setup should succeed")
}

pub fn normalized_result(mut report: Value, mut relationship: Value) -> (Value, Value) {
    report["invocation"]["id"] = json!("<invocation-id>");
    report["metadata"]["id"] = json!("<report-id>");
    report["metadata"]["generated_at"] = json!("<generated-at>");
    report["metadata"]["utc-start"] = json!("<utc-start>");
    relationship["report"]["metadata"]["id"] = json!("<report-id>");
    (report, relationship)
}

pub fn build(root: &Path, output: &Path) -> Output {
    Command::new(binary())
        .args([
            "package",
            "build",
            "--source",
            product_i5_source()
                .join("l0-procedure")
                .to_str()
                .expect("test setup should succeed"),
            "--package",
            PACKAGE,
            "--output",
            output.to_str().expect("test setup should succeed"),
        ])
        .current_dir(root)
        .output()
        .expect("test setup should succeed")
}

pub fn build_i5(
    root: &Path,
    source: &str,
    package: &str,
    dependencies: &[&Path],
    output: &Path,
) -> Output {
    let mut command = Command::new(binary());
    command
        .args(["package", "build", "--source"])
        .arg(product_i5_source().join(source))
        .args(["--package", package]);
    for dependency in dependencies {
        command.arg("--dependency-package").arg(dependency);
    }
    command
        .arg("--output")
        .arg(output)
        .current_dir(root)
        .output()
        .expect("test setup should succeed")
}

pub fn build_i5_from(
    root: &Path,
    source: &Path,
    package: &str,
    dependencies: &[&Path],
    output: &Path,
) -> Output {
    let mut command = Command::new(binary());
    command
        .args(["package", "build", "--source"])
        .arg(source)
        .args(["--package", package]);
    for dependency in dependencies {
        command.arg("--dependency-package").arg(dependency);
    }
    command
        .arg("--output")
        .arg(output)
        .current_dir(root)
        .output()
        .expect("test setup should succeed")
}

/// Requirement validation: exercises one bounded logical path.
pub fn checked_current_v2_examples_build_directly_and_deterministically() {
    let root = temporary("current-v2-examples");
    let examples = current_v2_examples();

    let minimal_a = root.join("minimal-a");
    let minimal_b = root.join("minimal-b");
    for output in [&minimal_a, &minimal_b] {
        let result = build_i5_from(
            &root,
            &examples.join("minimal/source"),
            PACKAGE,
            &[],
            output,
        );
        assert_eq!(result.status.code(), Some(0), "{:?}", result.stderr);
    }
    for name in ["config.json", "manifest.json", "package.tar"] {
        assert_eq!(
            fs::read(minimal_a.join(name)).expect("test setup should succeed"),
            fs::read(minimal_b.join(name)).expect("test setup should succeed")
        );
    }

    let action = root.join("action");
    let activity = root.join("activity");
    let procedure = root.join("procedure");
    let action_result = build_i5_from(
        &root,
        &examples.join("complete/source/action"),
        "pkg:attestify/acme.example/verification-action/release-readiness/database-connection@2.0.0",
        &[],
        &action,
    );
    assert_eq!(
        action_result.status.code(),
        Some(0),
        "{:?}",
        action_result.stderr
    );
    let activity_result = build_i5_from(
        &root,
        &examples.join("complete/source/activity"),
        "pkg:attestify/acme.example/verification-activity/release-readiness/database-readiness@2.0.0",
        &[&action],
        &activity,
    );
    assert_eq!(
        activity_result.status.code(),
        Some(0),
        "{:?}",
        activity_result.stderr
    );
    let procedure_result = build_i5_from(
        &root,
        &examples.join("complete/source/procedure"),
        "pkg:attestify/acme.example/verification-procedure/release-readiness/release-readiness@2.0.0",
        &[&action, &activity],
        &procedure,
    );
    assert_eq!(
        procedure_result.status.code(),
        Some(0),
        "{:?}",
        procedure_result.stderr
    );

    let verified = stage_and_verify_local_package(
        definition_package_profile(),
        &procedure,
        &root.join("verified-procedure"),
    )
    .expect("current complete Procedure package");
    assert_eq!(verified.kind, "VerificationProcedure");
    assert_eq!(verified.lock.node_count, 2);
    assert_eq!(verified.lock.edge_count, 2);
    assert_eq!(verified.lock.nodes.len(), 2);
    assert_eq!(verified.lock.edges.len(), 2);

    fs::remove_dir_all(root).expect("test setup should succeed");
}

pub fn evaluator(root: &Path) -> (PathBuf, PathBuf, String) {
    let executable = root.join("nape-eval");
    fs::write(
        &executable,
        br#"#!/usr/bin/env python3
import json, os, pathlib, sys
r=json.load(sys.stdin)
expected="postgresql://orders-db.prod.acme.example:5432/orders"
diagnostic=None
execution={"automatic_retry_count":0,"evaluate_call_count":1,"executed":True,"phase":"evaluate-call","status":"completed"}
validation={"contract":"passed","limit":"passed"}
owner="test-of-detail"
if os.environ.get("NAPE_I2_TEST_GF05") == "1":
    result={"conclusion":"inconclusive","facts":[],"reason":"The Test returned an invalid result contract."}
    diagnostic={"code":"completed_invalid_result_contract","phase":"result-validation","reason_template":"completed-invalid-result-contract-v1"}
    execution["phase"]="result-validation"
    validation={"contract":"failed","limit":"not-evaluated"}
    owner="verification-engine"
else:
    try:
        e=json.loads((pathlib.Path(r["workspace_root"])/r["evidence"]["file"]).read_bytes())
    except Exception:
        result={"conclusion":"inconclusive","facts":[{"name":"database_endpoint","status":"invalid","value":None,"value_type":None}],"reason":"The database endpoint could not be extracted from malformed evidence."}
    else:
        endpoint=e.get("database",{}).get("endpoint") if isinstance(e,dict) and isinstance(e.get("database",{}),dict) else None
        if endpoint is None:
            result={"conclusion":"inconclusive","facts":[{"name":"database_endpoint","status":"not_found","value":None,"value_type":None}],"reason":"The database endpoint fact was not found."}
        else:
            result={"conclusion":"true" if endpoint == expected else "false","facts":[{"name":"database_endpoint","status":"found","value":endpoint,"value_type":"text"}],"reason":"The observed database endpoint was compared with the approved endpoint."}
response={"contract":"attestify.nape-evaluator.action-invocation/v2","diagnostic":diagnostic,"disposition":"occurrence-result","execution":execution,"result":result,"result_validation":validation,"semantic_owner":owner}
print(json.dumps(response,sort_keys=True,separators=(",",":")))
"#,
    )
    .expect("test setup should succeed");
    let mut permissions = fs::metadata(&executable)
        .expect("test setup should succeed")
        .permissions();
    permissions.set_mode(0o700);
    fs::set_permissions(&executable, permissions).expect("test setup should succeed");

    let record = root.join("evaluator-build.json");
    let record_bytes = serde_json::to_vec(&json!({
        "contract": "attestify.nape-evaluator.installed-build/v1",
        "contract_projection_digest": format!("sha256:{}", "1".repeat(64)),
        "dependency_set_digest": format!("sha256:{}", "2".repeat(64)),
        "evaluator_contract": "attestify.nape-evaluator.action-invocation/v2",
        "evaluator_release": "2.0.0",
        "implementation_digest": format!("sha256:{}", "3".repeat(64)),
        "runner_profile": "attestify-python-test-development-v1",
    }))
    .expect("test setup should succeed");
    fs::write(&record, &record_bytes).expect("test setup should succeed");
    let digest = sha256_digest(&record_bytes);
    (executable, record, digest)
}

pub fn start_local(root: &Path, home: &Path, package: &Path, subject: &Path) -> Output {
    Command::new(binary())
        .args(["start", "--local-package"])
        .arg(package)
        .arg("--subject-file")
        .arg(subject)
        .args(["--meta", "environment", "production"])
        .env("HOME", home)
        .current_dir(root)
        .output()
        .expect("test setup should succeed")
}

pub fn collect_evidence(root: &Path, home: &Path, evidence: &Path) -> Output {
    Command::new(binary())
        .args([
            "evidence",
            "--action",
            "release-readiness.database-connection",
            "--file",
        ])
        .arg(evidence)
        .env("HOME", home)
        .current_dir(root)
        .output()
        .expect("test setup should succeed")
}

pub fn verify_current(
    root: &Path,
    home: &Path,
    evaluator: &Path,
    record: &Path,
    record_digest: &str,
    invalid_result: bool,
) -> Output {
    let mut command = Command::new(binary());
    command
        .arg("verify")
        .env("HOME", home)
        .env("NAPE_EVALUATOR_V2_EXECUTABLE", evaluator)
        .env("NAPE_EVALUATOR_V2_BUILD_RECORD", record)
        .env("NAPE_EVALUATOR_V2_BUILD_RECORD_SHA256", record_digest)
        .current_dir(root);
    if invalid_result {
        command.env("NAPE_I2_TEST_GF05", "1");
    }
    command.output().expect("test setup should succeed")
}

pub fn receipt_output_path(output: &Output) -> PathBuf {
    let value = receipt(output);
    let uri = value["result"]["output"].as_str().expect("output URI");
    PathBuf::from(uri.strip_prefix("file://").expect("file URI"))
}

pub fn current_result_output(home: &Path) -> PathBuf {
    let state: serde_yaml::Value = serde_yaml::from_slice(
        &fs::read(home.join("nape/.nape_cli_config")).expect("test setup should succeed"),
    )
    .expect("test setup should succeed");
    PathBuf::from(state["result_output"].as_str().expect("result output"))
}

/// Requirement validation: exercises one bounded logical path.
pub fn package_build_is_exact_non_overwriting_and_receipted() {
    let root = temporary("package-command");
    let output = root.join("a0-package");
    let first = build(&root, &output);
    assert_eq!(first.status.code(), Some(0));
    assert!(first.stderr.is_empty());
    let first_receipt = receipt(&first);
    assert_eq!(first_receipt["command"], "package-build");
    assert_eq!(first_receipt["status"], "succeeded");
    assert_eq!(first_receipt["result"]["package"], PACKAGE);
    assert_eq!(
        first_receipt["producer"]["qualification"],
        "attestify-verification-engine-v2-development"
    );

    let expected = product().join("generated/package/a0-procedure");
    for name in ["config.json", "manifest.json", "package.tar"] {
        assert_eq!(
            fs::read(output.join(name)).expect("test setup should succeed"),
            fs::read(expected.join(name)).expect("test setup should succeed")
        );
    }
    let manifest_before =
        fs::read(output.join("manifest.json")).expect("test setup should succeed");
    let second = build(&root, &output);
    assert_eq!(second.status.code(), Some(1));
    let second_receipt = receipt(&second);
    assert_eq!(second_receipt["status"], "failed");
    assert_eq!(
        second_receipt["diagnostic"]["code"],
        "package_atomic_commit_violation"
    );
    assert_eq!(
        manifest_before,
        fs::read(output.join("manifest.json")).expect("test setup should succeed")
    );

    let dangling = root.join("dangling-package-output");
    std::os::unix::fs::symlink(root.join("absent-package-target"), &dangling)
        .expect("test setup should succeed");
    let dangling_result = build(&root, &dangling);
    assert_eq!(dangling_result.status.code(), Some(1));
    assert_eq!(
        receipt(&dangling_result)["diagnostic"]["code"],
        "package_atomic_commit_violation"
    );
    assert!(fs::symlink_metadata(&dangling)
        .expect("test setup should succeed")
        .file_type()
        .is_symlink());

    let grammar = Command::new(binary())
        .arg("package")
        .output()
        .expect("test setup should succeed");
    assert_eq!(grammar.status.code(), Some(2));
    assert!(grammar.stdout.is_empty());
    fs::remove_dir_all(root).expect("test setup should succeed");
}

/// Requirement validation: exercises one bounded logical path.
pub fn package_build_constructs_exact_i5_direct_and_transitive_locks() {
    let root = temporary("i5-package-construction");
    let action = root.join("action");
    assert_eq!(
        build_i5(&root, "l1-action", ACTION_PACKAGE, &[], &action)
            .status
            .code(),
        Some(0)
    );
    let action_verified = stage_and_verify_local_package(
        definition_package_profile(),
        &action,
        &root.join("stage-action"),
    )
    .expect("test setup should succeed");
    assert_eq!(
        action_verified.identity.manifest_digest,
        "sha256:040e5e99ba3c640af33aac02c8cf7ff256a52e7f515cd094389d2885d4668339"
    );
    assert!(action_verified.lock.is_empty());

    let l1 = root.join("l1");
    assert_eq!(
        build_i5(&root, "l1-procedure", L1_PACKAGE, &[&action], &l1)
            .status
            .code(),
        Some(0)
    );
    let l1_verified =
        stage_and_verify_local_package(definition_package_profile(), &l1, &root.join("stage-l1"))
            .expect("test setup should succeed");
    assert_eq!(
        l1_verified.identity.manifest_digest,
        "sha256:383f12d9b3687c0e6bf7b80d07eb169759a6525ece6f67a07e3b85ef6bb6cb72"
    );
    assert_eq!(
        (l1_verified.lock.node_count, l1_verified.lock.edge_count),
        (1, 1)
    );

    let activity = root.join("activity");
    assert_eq!(
        build_i5(
            &root,
            "l2-activity",
            ACTIVITY_PACKAGE,
            &[&action],
            &activity
        )
        .status
        .code(),
        Some(0)
    );
    let activity_verified = stage_and_verify_local_package(
        definition_package_profile(),
        &activity,
        &root.join("stage-activity"),
    )
    .expect("test setup should succeed");
    assert_eq!(
        (
            activity_verified.lock.node_count,
            activity_verified.lock.edge_count
        ),
        (1, 1)
    );
    assert_eq!(
        activity_verified.lock.edges[0].use_selector,
        "primary-database"
    );

    let l2_first = root.join("l2-first");
    let l2_second = root.join("l2-second");
    assert_eq!(
        build_i5(
            &root,
            "l2-procedure",
            L2_PACKAGE,
            &[&activity, &action],
            &l2_first
        )
        .status
        .code(),
        Some(0)
    );
    assert_eq!(
        build_i5(
            &root,
            "l2-procedure",
            L2_PACKAGE,
            &[&action, &activity],
            &l2_second
        )
        .status
        .code(),
        Some(0)
    );
    for name in ["config.json", "manifest.json", "package.tar"] {
        assert_eq!(
            fs::read(l2_first.join(name)).expect("test setup should succeed"),
            fs::read(l2_second.join(name)).expect("test setup should succeed")
        );
    }
    let temporary_path = root.to_string_lossy();
    for name in ["config.json", "manifest.json", "package.tar"] {
        let bytes = fs::read(l2_first.join(name)).expect("test setup should succeed");
        assert!(!bytes
            .windows(temporary_path.len())
            .any(|window| window == temporary_path.as_bytes()));
    }
    let l2_verified = stage_and_verify_local_package(
        definition_package_profile(),
        &l2_first,
        &root.join("stage-l2"),
    )
    .expect("test setup should succeed");
    assert_eq!(
        (l2_verified.lock.node_count, l2_verified.lock.edge_count),
        (2, 2)
    );
    assert_eq!(l2_verified.lock.edges[0].use_selector, "primary-database");
    assert_eq!(
        l2_verified.lock.edges[1].use_selector,
        "infrastructure-readiness"
    );

    let missing = build_i5(
        &root,
        "l2-procedure",
        L2_PACKAGE,
        &[&activity],
        &root.join("missing-transitive"),
    );
    assert_eq!(missing.status.code(), Some(1));
    assert!(!root.join("missing-transitive").exists());
    let duplicate = build_i5(
        &root,
        "l1-procedure",
        L1_PACKAGE,
        &[&action, &action],
        &root.join("duplicate"),
    );
    assert_eq!(duplicate.status.code(), Some(1));
    assert!(!root.join("duplicate").exists());

    let l0 = root.join("l0");
    assert_eq!(build(&root, &l0).status.code(), Some(0));
    let extra = build_i5(
        &root,
        "l1-procedure",
        L1_PACKAGE,
        &[&action, &l0],
        &root.join("extra"),
    );
    assert_eq!(extra.status.code(), Some(1));
    assert!(!root.join("extra").exists());

    let authored_lock = build_i5_from(
        &root,
        &product().join("source/package/a0-procedure"),
        PACKAGE,
        &[],
        &root.join("authored-lock"),
    );
    assert_eq!(authored_lock.status.code(), Some(1));
    assert_eq!(
        receipt(&authored_lock)["diagnostic"]["code"],
        "package_archive_invalid"
    );
    assert!(!root.join("authored-lock").exists());

    let shadow_source = root.join("shadow-source");
    fs::create_dir_all(shadow_source.join("action/database-connection"))
        .expect("test setup should succeed");
    fs::copy(
        product_i5_source().join("l1-action/verification-action.yaml"),
        shadow_source.join("verification-action.yaml"),
    )
    .expect("test setup should succeed");
    fs::copy(
        product_i5_source().join("l1-action/action/database-connection/database-connection.py"),
        shadow_source.join("action/database-connection/database-connection.py"),
    )
    .expect("test setup should succeed");
    fs::write(shadow_source.join("shadow.txt"), b"not admitted\n")
        .expect("test setup should succeed");
    let shadow = build_i5_from(
        &root,
        &shadow_source,
        ACTION_PACKAGE,
        &[],
        &root.join("shadow"),
    );
    assert_eq!(shadow.status.code(), Some(1));
    assert_eq!(
        receipt(&shadow)["diagnostic"]["code"],
        "package_archive_invalid"
    );
    assert!(!root.join("shadow").exists());
    fs::remove_dir_all(root).expect("test setup should succeed");
}

/// Requirement validation: exercises one bounded logical path.
pub fn verify_runs_one_occurrence_and_commits_or_leaves_no_output() {
    let root = temporary("verify-command");
    let home = root.join("home");
    fs::create_dir(&home).expect("test setup should succeed");
    let package = root.join("a0-package");
    assert!(build(&root, &package).status.success());
    let evidence_source = product().join("source/evidence/true/application-configuration.json");
    let subject = root.join("subject.json");
    fs::copy(product().join("source/subject.json"), &subject).expect("test setup should succeed");
    let (evaluator, record, record_digest) = evaluator(&root);

    let started = start_local(&root, &home, &package, &subject);
    assert_eq!(started.status.code(), Some(0), "{:?}", started.stderr);
    assert!(started.stdout.is_empty());
    assert!(started.stderr.is_empty());
    let collected = collect_evidence(&root, &home, &evidence_source);
    assert_eq!(collected.status.code(), Some(0), "{:?}", collected.stderr);
    assert!(collected.stdout.is_empty());
    assert!(collected.stderr.is_empty());
    let completed = verify_current(&root, &home, &evaluator, &record, &record_digest, false);
    assert_eq!(completed.status.code(), Some(0));
    assert!(completed.stderr.is_empty());
    let success = receipt(&completed);
    assert_eq!(success["command"], "verify");
    assert_eq!(success["status"], "succeeded");
    assert_eq!(success["result"]["report"]["summary"]["conclusion_true"], 1);
    let output = receipt_output_path(&completed);

    let report: Value = serde_json::from_slice(
        &fs::read(output.join("verification-report.json")).expect("test setup should succeed"),
    )
    .expect("test setup should succeed");
    let relationship: Value = serde_json::from_slice(
        &fs::read(output.join("evidence-set-relationship.json"))
            .expect("test setup should succeed"),
    )
    .expect("test setup should succeed");
    let digest = sha256_digest(&fs::read(&evidence_source).expect("test setup should succeed"));
    assert_eq!(report["activity"][0]["action"][0]["conclusion"], "true");
    assert_eq!(
        report["activity"][0]["definition_origin"],
        json!({
            "owner": {
                "kind": "VerificationProcedure",
                "manifestDigest": "sha256:58d5d09c2a74a8dd9a065fa5723dcdd591dac1b1126eaea924dcd70c6be5462d",
                "name": "database-endpoint",
                "package": PACKAGE,
            },
            "selector": "release-readiness",
            "type": "embedded",
        })
    );
    assert_eq!(report["procedure"]["acquisition"]["source"], "local-build");
    assert_eq!(report["metadata"]["environment"], "production");
    assert_eq!(report["test"], Value::Null);
    assert_eq!(
        report["metadata"]["id"],
        relationship["report"]["metadata"]["id"]
    );
    assert!(relationship["payload"][0].get("path").is_none());
    assert_eq!(
        fs::read(output.join(format!(
            "evidence/sha256/{}",
            digest.strip_prefix("sha256:").expect("test setup should succeed")
        )))
        .expect("test setup should succeed"),
        fs::read(&evidence_source).expect("test setup should succeed")
    );

    let clean_start = start_local(&root, &home, &package, &subject);
    assert_eq!(clean_start.status.code(), Some(0));
    assert_eq!(
        collect_evidence(&root, &home, &evidence_source)
            .status
            .code(),
        Some(0)
    );
    let clean_repeat = verify_current(&root, &home, &evaluator, &record, &record_digest, false);
    assert_eq!(clean_repeat.status.code(), Some(0));
    assert_eq!(
        receipt(&clean_repeat)["result"]["report"]["summary"]["conclusion_true"],
        1
    );
    let clean_output = receipt_output_path(&clean_repeat);
    let repeated_report: Value = serde_json::from_slice(
        &fs::read(clean_output.join("verification-report.json"))
            .expect("test setup should succeed"),
    )
    .expect("test setup should succeed");
    let repeated_relationship: Value = serde_json::from_slice(
        &fs::read(clean_output.join("evidence-set-relationship.json"))
            .expect("test setup should succeed"),
    )
    .expect("test setup should succeed");
    assert_eq!(
        normalized_result(report.clone(), relationship.clone()),
        normalized_result(repeated_report, repeated_relationship)
    );

    assert_eq!(
        start_local(&root, &home, &package, &subject).status.code(),
        Some(0)
    );
    assert_eq!(
        collect_evidence(&root, &home, &evidence_source)
            .status
            .code(),
        Some(0)
    );
    let dangling_output = current_result_output(&home);
    std::os::unix::fs::symlink(root.join("absent-verification-target"), &dangling_output)
        .expect("test setup should succeed");
    let dangling_failure = verify_current(&root, &home, &evaluator, &record, &record_digest, false);
    assert_eq!(dangling_failure.status.code(), Some(1));
    assert_eq!(
        receipt(&dangling_failure)["diagnostic"]["code"],
        "engine_execution_integrity_failed"
    );
    assert!(fs::symlink_metadata(&dangling_output)
        .expect("test setup should succeed")
        .file_type()
        .is_symlink());

    let committed_report =
        fs::read(output.join("verification-report.json")).expect("test setup should succeed");
    let repeated = verify_current(&root, &home, &evaluator, &record, &record_digest, false);
    assert_eq!(repeated.status.code(), Some(1));
    assert_eq!(
        receipt(&repeated)["diagnostic"]["code"],
        "engine_execution_integrity_failed"
    );
    assert_eq!(
        committed_report,
        fs::read(output.join("verification-report.json")).expect("test setup should succeed")
    );

    for (scenario, expected) in [
        ("false", "false"),
        ("missing", "inconclusive"),
        ("malformed", "inconclusive"),
    ] {
        let scenario_evidence = product().join(format!(
            "source/evidence/{scenario}/application-configuration.json"
        ));
        assert_eq!(
            start_local(&root, &home, &package, &subject).status.code(),
            Some(0)
        );
        assert_eq!(
            collect_evidence(&root, &home, &scenario_evidence)
                .status
                .code(),
            Some(0)
        );
        let completed = verify_current(&root, &home, &evaluator, &record, &record_digest, false);
        assert_eq!(completed.status.code(), Some(0));
        let completed_receipt = receipt(&completed);
        let scenario_output = receipt_output_path(&completed);
        let report: Value = serde_json::from_slice(
            &fs::read(scenario_output.join("verification-report.json"))
                .expect("test setup should succeed"),
        )
        .expect("test setup should succeed");
        assert_eq!(report["activity"][0]["action"][0]["conclusion"], expected);
        assert_eq!(
            completed_receipt["result"]["report"]["summary"],
            report["summary"]
        );
    }

    assert_eq!(
        start_local(&root, &home, &package, &subject).status.code(),
        Some(0)
    );
    assert_eq!(
        collect_evidence(&root, &home, &evidence_source)
            .status
            .code(),
        Some(0)
    );
    let invalid = verify_current(&root, &home, &evaluator, &record, &record_digest, true);
    assert_eq!(invalid.status.code(), Some(0));
    assert_eq!(
        receipt(&invalid)["result"]["report"]["summary"]["conclusion_inconclusive"],
        1
    );
    let invalid_output = receipt_output_path(&invalid);
    let invalid_report: Value = serde_json::from_slice(
        &fs::read(invalid_output.join("verification-report.json"))
            .expect("test setup should succeed"),
    )
    .expect("test setup should succeed");
    assert_eq!(
        invalid_report["processing_detail"]["diagnostics"][0]["code"],
        "completed_invalid_result_contract"
    );

    assert_eq!(
        start_local(&root, &home, &package, &subject).status.code(),
        Some(0)
    );
    let failed_output = current_result_output(&home);
    let failed = verify_current(&root, &home, &evaluator, &record, &record_digest, false);
    assert_eq!(failed.status.code(), Some(1));
    let failure = receipt(&failed);
    assert_eq!(
        failure["diagnostic"]["code"],
        "evidence_association_invalid"
    );
    assert_eq!(failure["source"], "local-package");
    assert!(!failed_output.exists());

    assert_eq!(
        start_local(&root, &home, &package, &subject).status.code(),
        Some(0)
    );
    let symlink = root.join("evidence-symlink.json");
    std::os::unix::fs::symlink(&evidence_source, &symlink).expect("test setup should succeed");
    let symlink_failure = collect_evidence(&root, &home, &symlink);
    assert_eq!(symlink_failure.status.code(), Some(1));
    assert!(symlink_failure.stdout.is_empty());

    let empty_home = root.join("empty-home");
    fs::create_dir(&empty_home).expect("test setup should succeed");
    let grammar = Command::new(binary())
        .arg("verify")
        .env("HOME", empty_home)
        .current_dir(&root)
        .output()
        .expect("test setup should succeed");
    assert_eq!(grammar.status.code(), Some(1));
    assert!(grammar.stdout.is_empty());
    if std::env::var_os("NAPE_I2_KEEP_TEST_OUTPUT").is_some() {
        eprintln!("I2_TEST_OUTPUT={}", root.display());
    } else {
        fs::remove_dir_all(root).expect("test setup should succeed");
    }
}

/// Requirement validation: exercises one bounded logical path.
pub fn local_nonempty_lock_cannot_resolve_an_oci_dependency() {
    let root = temporary("local-nonempty-lock");
    let home = root.join("home");
    fs::create_dir(&home).expect("test setup should succeed");
    let subject = product().join("source/subject.json");
    let completed = Command::new(binary())
        .args([
            "start",
            "--local-package",
            product()
                .join("generated/package/a3-procedure")
                .to_str()
                .expect("test setup should succeed"),
            "--subject-file",
            subject.to_str().expect("test setup should succeed"),
        ])
        .env("HOME", &home)
        .current_dir(&root)
        .output()
        .expect("test setup should succeed");
    assert_eq!(completed.status.code(), Some(1));
    assert!(completed.stdout.is_empty());
    assert!(
        String::from_utf8_lossy(&completed.stderr).contains("empty Lock"),
        "{completed:?}"
    );
    fs::remove_dir_all(root).expect("test setup should succeed");
}
