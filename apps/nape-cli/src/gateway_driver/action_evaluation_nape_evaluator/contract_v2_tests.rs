use super::contract_v2::*;
use serde_json::Value;
use sha2::Digest;
use std::fs;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::Duration;

static NEXT_TEMPORARY: AtomicU64 = AtomicU64::new(0);

struct TemporaryDirectory(PathBuf);

impl TemporaryDirectory {
    fn new(label: &str) -> Self {
        let serial = NEXT_TEMPORARY.fetch_add(1, Ordering::SeqCst);
        let path = std::env::temp_dir().join(format!(
            "attestify-nape-v2-{label}-{}-{serial}",
            std::process::id()
        ));
        fs::create_dir(&path).expect("test setup should succeed");
        Self(path)
    }
}

impl Drop for TemporaryDirectory {
    fn drop(&mut self) {
        let _ = fs::remove_dir_all(&self.0);
    }
}

fn request() -> ActionInvocationRequestV2 {
    ActionInvocationRequestV2 {
        contract: CONTRACT.to_string(),
        workspace_root: "/private/nape/invocation-0001".to_string(),
        evidence: EvidenceDescriptor {
            file: "evidence/input.json".to_string(),
            argument_digest: format!("sha256:{}", "e".repeat(64)),
            argument_byte_count: 81,
            media_type: "application/json".to_string(),
            representation: "opaque".to_string(),
        },
        test: TestDescriptor {
            file: "package/database-connection.py".to_string(),
            content_digest: format!("sha256:{}", "b".repeat(64)),
            byte_count: 4663,
            runner_profile: RUNNER_PROFILE.to_string(),
        },
        evaluations: Vec::new(),
        metadata: EvaluatorMetadata {
            evidence_media_type: "application/json".to_string(),
            evidence_representation: "opaque".to_string(),
            evaluator_contract_version: "2".to_string(),
        },
        limits: EvaluatorLimits {
            maximum_execution_work_units: 25_000_000,
            maximum_result_bytes: 131_072,
        },
    }
}

fn vectors() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../contracts/attestify-nape-evaluator-action-invocation-v2/conformance/vectors")
}

fn canonical_vector(name: &str) -> Vec<u8> {
    let raw = fs::read(vectors().join(name)).expect("test setup should succeed");
    let value: Value = serde_json::from_slice(&raw).expect("test setup should succeed");
    let mut output = canonical_json(&value)
        .expect("test setup should succeed")
        .into_bytes();
    output.push(b'\n');
    output
}

fn shell_literal(path: &Path) -> String {
    format!("'{}'", path.display().to_string().replace('\'', "'\\''"))
}

fn executable_with(
    label: &str,
    stdout: &[u8],
    stderr: &[u8],
    exit_code: i32,
    delay_seconds: Option<u64>,
) -> (TemporaryDirectory, EvaluatorExecutable) {
    let temporary = TemporaryDirectory::new(label);
    let stdout_path = temporary.0.join("stdout");
    let stderr_path = temporary.0.join("stderr");
    let executable_path = temporary.0.join("nape-eval");
    fs::write(&stdout_path, stdout).expect("test setup should succeed");
    fs::write(&stderr_path, stderr).expect("test setup should succeed");
    let delay = delay_seconds
        .map(|seconds| format!("/bin/sleep {seconds}\n"))
        .unwrap_or_default();
    let script = format!(
        "#!/bin/sh\n/bin/cat >/dev/null\n{delay}/bin/cat {}\n/bin/cat {} >&2\nexit {exit_code}\n",
        shell_literal(&stdout_path),
        shell_literal(&stderr_path)
    );
    fs::write(&executable_path, script).expect("test setup should succeed");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&executable_path, fs::Permissions::from_mode(0o700))
            .expect("test setup should succeed");
    }
    let executable = select_executable(&temporary, &executable_path);
    (temporary, executable)
}

fn build_record() -> Value {
    serde_json::json!({
        "contract": INSTALLED_BUILD_CONTRACT,
        "evaluator_contract": CONTRACT,
        "runner_profile": RUNNER_PROFILE,
        "evaluator_release": EVALUATOR_RELEASE,
        "implementation_digest": format!("sha256:{}", "a".repeat(64)),
        "contract_projection_digest": format!("sha256:{}", "b".repeat(64)),
        "dependency_set_digest": format!("sha256:{}", "c".repeat(64))
    })
}

fn write_build_record(temporary: &TemporaryDirectory, value: &Value) -> (PathBuf, String) {
    let path = temporary.0.join("evaluator-build.json");
    let bytes = canonical_json(value)
        .expect("test setup should succeed")
        .into_bytes();
    let digest = format!("sha256:{}", hex::encode(sha2::Sha256::digest(&bytes)));
    fs::write(&path, bytes).expect("test setup should succeed");
    (path, digest)
}

fn select_executable(
    temporary: &TemporaryDirectory,
    executable_path: &Path,
) -> EvaluatorExecutable {
    let (record_path, record_digest) = write_build_record(temporary, &build_record());
    EvaluatorExecutable::select(executable_path, record_path, &record_digest)
        .expect("test setup should succeed")
}

/// Requirement validation: exercises one bounded logical path.

#[test]
fn request_is_closed_and_requires_empty_evaluations_success() {
    let mut candidate = request();
    assert_eq!(candidate.validate(), Ok(()));
    candidate.evaluations.push(Value::Null);
    assert_eq!(
        candidate
            .validate()
            .expect_err("test path should reject")
            .diagnostic_code,
        "evaluator_request_invalid"
    );
}

/// Requirement validation: exercises one bounded logical path.

#[test]
fn executable_selection_requires_one_absolute_regular_non_symlink_executable_success() {
    let invalid_digest = format!("sha256:{}", "0".repeat(64));
    assert_eq!(
        EvaluatorExecutable::select("nape-eval", "/tmp/build.json", &invalid_digest)
            .expect_err("test path should reject")
            .diagnostic_code,
        "runner_capacity_unavailable"
    );
    let temporary = TemporaryDirectory::new("selector");
    let target = temporary.0.join("target");
    fs::write(&target, "#!/bin/sh\n").expect("test setup should succeed");
    #[cfg(unix)]
    {
        use std::os::unix::fs::{symlink, PermissionsExt};
        fs::set_permissions(&target, fs::Permissions::from_mode(0o700))
            .expect("test setup should succeed");
        let link = temporary.0.join("link");
        symlink(&target, &link).expect("test setup should succeed");
        let (record_path, record_digest) = write_build_record(&temporary, &build_record());
        assert_eq!(
            EvaluatorExecutable::select(link, record_path, &record_digest)
                .expect_err("test path should reject")
                .diagnostic_code,
            "runner_capacity_unavailable"
        );
    }
}

/// Requirement validation: exercises one bounded logical path.

#[test]
fn installed_build_record_is_digest_verified_closed_and_compatible_success() {
    let temporary = TemporaryDirectory::new("build-record");
    let executable_path = temporary.0.join("nape-eval");
    fs::write(&executable_path, "#!/bin/sh\n").expect("test setup should succeed");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        fs::set_permissions(&executable_path, fs::Permissions::from_mode(0o700))
            .expect("test setup should succeed");
    }
    let (record_path, record_digest) = write_build_record(&temporary, &build_record());
    let selected = EvaluatorExecutable::select(&executable_path, &record_path, &record_digest)
        .expect("test setup should succeed");
    assert_eq!(selected.build().record_digest, record_digest);
    assert_eq!(selected.build().evaluator_release, EVALUATOR_RELEASE);
    assert_eq!(
        selected.build().implementation_digest,
        format!("sha256:{}", "a".repeat(64))
    );

    fs::write(&record_path, b"{}").expect("test setup should succeed");
    assert_eq!(
        EvaluatorExecutable::select(&executable_path, &record_path, &record_digest)
            .expect_err("test path should reject")
            .diagnostic_code,
        "runner_capacity_unavailable"
    );

    let mut incompatible = build_record();
    incompatible["runner_profile"] = Value::String("other-profile".to_string());
    let (incompatible_path, incompatible_digest) = write_build_record(&temporary, &incompatible);
    assert_eq!(
        EvaluatorExecutable::select(&executable_path, &incompatible_path, &incompatible_digest)
            .expect_err("test path should reject")
            .diagnostic_code,
        "runner_capacity_unavailable"
    );
}

/// Requirement validation: exercises one bounded logical path.

#[test]
fn all_ten_accepted_response_variants_validate_success() {
    let accepted = [
        "eval-response-v2-test-true.json",
        "eval-response-v2-test-false.json",
        "eval-response-v2-test-inconclusive.json",
        "eval-response-v2-invalid-completed.json",
        "eval-response-v2-over-limit.json",
        "eval-response-v2-blocked-activation.json",
        "eval-response-v2-terminated-initialization.json",
        "eval-response-v2-terminated-evaluate.json",
        "eval-response-v2-request-rejected.json",
        "eval-response-v2-integrity-failed.json",
    ];
    for name in accepted {
        validate_response_frame(&canonical_vector(name)).unwrap_or_else(|error| {
            panic!("{name} failed with {error:?}");
        });
    }
}

/// Requirement validation: exercises one bounded logical path.

#[test]
fn all_response_near_misses_are_rejected_error() {
    let rejected = [
        "eval-response-v2-invalid-canonical-scalar.json",
        "eval-response-v2-missing-required-field.json",
        "eval-response-v2-out-of-vocabulary.json",
        "eval-response-v2-prohibited-identity-or-compatibility-field.json",
        "eval-response-v2-unknown-field.json",
        "eval-response-v2-wrong-type.json",
        "response-boundary-extra-occurrence-result.json",
        "response-boundary-without-diagnostic.json",
        "response-missing-occurrence-result.json",
        "response-near-miss-blocked-activation.json",
        "response-near-miss-invalid-completed.json",
        "response-near-miss-over-limit.json",
        "response-near-miss-terminated-evaluate.json",
        "response-near-miss-terminated-initialization.json",
        "response-near-miss-valid-completed.json",
        "response-raw-output-field.json",
        "response-traceback-field.json",
        "response-wrong-semantic-owner-result.json",
    ];
    for name in rejected {
        let error =
            validate_response_frame(&canonical_vector(name)).expect_err("test path should reject");
        assert_eq!(
            error.diagnostic_code, "evaluator_response_contract_invalid",
            "{name}"
        );
    }
}

/// Requirement validation: exercises one bounded logical path.

#[test]
fn duplicate_response_member_is_rejected_by_strict_json_error() {
    let frame = format!(
        "{{\"contract\":\"{CONTRACT}\",\"contract\":\"{CONTRACT}\",\"disposition\":\"request-rejected\",\"diagnostic\":{{\"code\":\"evaluator_request_invalid\",\"phase\":\"runner-call-validation\",\"reason_template\":\"evaluator-request-invalid-v1\"}}}}\n"
    );
    let error = validate_response_frame(frame.as_bytes()).expect_err("test path should reject");
    assert_eq!(error.kind, ConsumerFailureKind::OutputMalformed);
}

/// Requirement validation: exercises one bounded logical path.

#[test]
fn multiple_or_noncanonical_stdout_is_rejected_error() {
    let mut multiple = canonical_vector("eval-response-v2-test-true.json");
    multiple.extend_from_slice(b"{}\n");
    assert_eq!(
        validate_response_frame(&multiple)
            .expect_err("test path should reject")
            .kind,
        ConsumerFailureKind::OutputFraming
    );
    let pretty = fs::read(vectors().join("eval-response-v2-test-true.json"))
        .expect("test setup should succeed");
    let mut pretty_frame = pretty;
    pretty_frame.push(b'\n');
    assert_eq!(
        validate_response_frame(&pretty_frame)
            .expect_err("test path should reject")
            .kind,
        ConsumerFailureKind::OutputFraming
    );
}

/// Requirement validation: exercises one bounded logical path.

#[test]
fn process_adapter_returns_one_valid_response_and_ignores_bounded_stderr_success() {
    let response = canonical_vector("eval-response-v2-test-true.json");
    let (_temporary, executable) =
        executable_with("success", &response, b"protected progress\n", 0, None);
    let actual = invoke_action(&executable, &request()).expect("test setup should succeed");
    assert_eq!(actual.value()["disposition"], "occurrence-result");
    assert_eq!(actual.value()["result"]["conclusion"], "true");
}

/// Requirement validation: exercises one bounded logical path.

#[test]
fn nonzero_exit_never_promotes_valid_looking_stdout_error() {
    let response = canonical_vector("eval-response-v2-test-true.json");
    let (_temporary, executable) = executable_with("exit", &response, b"", 1, None);
    let error = invoke_action(&executable, &request()).expect_err("test path should reject");
    assert_eq!(error.kind, ConsumerFailureKind::AbnormalExit);
    assert_eq!(error.diagnostic_code, "engine_execution_integrity_failed");
}

/// Requirement validation: exercises one bounded logical path.

#[test]
fn timeout_is_terminal_and_creates_no_occurrence_result_error() {
    let response = canonical_vector("eval-response-v2-test-true.json");
    let (_temporary, executable) = executable_with("timeout", &response, b"", 0, Some(2));
    let error = invoke_with_policy(
        &executable,
        &request(),
        ProcessPolicy {
            timeout: Duration::from_millis(20),
            stdout_limit: MAX_STDOUT_BYTES,
            stderr_limit: MAX_STDERR_BYTES,
        },
    )
    .expect_err("test path should reject");
    assert_eq!(error.kind, ConsumerFailureKind::Timeout);
}

/// Requirement validation: exercises one bounded logical path.

#[test]
fn stdout_and_stderr_are_incrementally_bounded_success() {
    let oversized = vec![b'x'; 129];
    let (_temporary, executable) = executable_with("stdout-limit", &oversized, b"", 0, None);
    let error = invoke_with_policy(
        &executable,
        &request(),
        ProcessPolicy {
            timeout: Duration::from_secs(1),
            stdout_limit: 128,
            stderr_limit: 128,
        },
    )
    .expect_err("test path should reject");
    assert_eq!(error.kind, ConsumerFailureKind::StdoutLimit);

    let response = canonical_vector("eval-response-v2-test-true.json");
    let (_temporary, executable) = executable_with("stderr-limit", &response, &oversized, 0, None);
    let error = invoke_with_policy(
        &executable,
        &request(),
        ProcessPolicy {
            timeout: Duration::from_secs(1),
            stdout_limit: MAX_STDOUT_BYTES,
            stderr_limit: 128,
        },
    )
    .expect_err("test path should reject");
    assert_eq!(error.kind, ConsumerFailureKind::StderrLimit);
}
