//! Strict NAPE-owned consumer for one versioned Evaluator Action occurrence.
//!
//! This module is intentionally separate from the historical V1 gateway. It
//! owns process selection, bounded transport, and complete response
//! validation; it owns no Procedure, OCI, continuation, or reporting policy.

use serde::de::{self, DeserializeSeed, MapAccess, SeqAccess, Visitor};
use serde::Serialize;
use serde_json::{Map, Number, Value};
use sha2::{Digest, Sha256};
use std::collections::BTreeSet;
use std::fmt;
use std::fs;
use std::io::{Read, Write};
use std::path::{Component, Path, PathBuf};
use std::process::{Child, ChildStderr, ChildStdout, Command, Stdio};
use std::thread::{self, JoinHandle};
use std::time::{Duration, Instant};

pub const CONTRACT: &str = "attestify.nape-evaluator.action-invocation/v2";
pub const RUNNER_PROFILE: &str = "attestify-python-test-development-v1";
pub const INSTALLED_BUILD_CONTRACT: &str = "attestify.nape-evaluator.installed-build/v1";
pub const EVALUATOR_RELEASE: &str = "2.0.0";
pub const MAX_BUILD_RECORD_BYTES: usize = 4_096;
pub const MAX_REQUEST_BYTES: usize = 1_048_576;
pub const MAX_STDOUT_BYTES: usize = 1_114_112;
pub const MAX_STDERR_BYTES: usize = 65_536;
pub const OUTER_WATCHDOG: Duration = Duration::from_secs(65);

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EvidenceDescriptor {
    pub file: String,
    pub argument_digest: String,
    pub argument_byte_count: u64,
    pub media_type: String,
    pub representation: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TestDescriptor {
    pub file: String,
    pub content_digest: String,
    pub byte_count: u64,
    pub runner_profile: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EvaluatorMetadata {
    pub evidence_media_type: String,
    pub evidence_representation: String,
    pub evaluator_contract_version: String,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EvaluatorLimits {
    pub maximum_execution_work_units: u64,
    pub maximum_result_bytes: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct ActionInvocationRequestV2 {
    pub contract: String,
    pub workspace_root: String,
    pub evidence: EvidenceDescriptor,
    pub test: TestDescriptor,
    pub evaluations: Vec<Value>,
    pub metadata: EvaluatorMetadata,
    pub limits: EvaluatorLimits,
}

impl ActionInvocationRequestV2 {
    pub fn validate(&self) -> Result<(), ConsumerFailure> {
        if self.contract != CONTRACT
            || !absolute_normalized_path(&self.workspace_root)
            || !relative_normalized_path(&self.evidence.file)
            || !relative_normalized_path(&self.test.file)
            || !sha256_digest(&self.evidence.argument_digest)
            || !sha256_digest(&self.test.content_digest)
            || self.evidence.argument_byte_count > 268_435_456
            || self.test.byte_count > 33_554_432
            || !media_type(&self.evidence.media_type)
            || self.evidence.representation != "opaque"
            || self.test.runner_profile != RUNNER_PROFILE
            || !self.evaluations.is_empty()
            || self.metadata.evidence_media_type != self.evidence.media_type
            || self.metadata.evidence_representation != "opaque"
            || self.metadata.evaluator_contract_version != "2"
            || !(1..=100_000_000).contains(&self.limits.maximum_execution_work_units)
            || !(1..=1_048_576).contains(&self.limits.maximum_result_bytes)
        {
            return Err(ConsumerFailure::new(
                ConsumerFailureKind::RequestInvalid,
                "evaluator_request_invalid",
            ));
        }
        Ok(())
    }

    fn framed_bytes(&self) -> Result<Vec<u8>, ConsumerFailure> {
        self.validate()?;
        let value = serde_json::to_value(self).map_err(|_| {
            ConsumerFailure::new(
                ConsumerFailureKind::RequestInvalid,
                "evaluator_request_invalid",
            )
        })?;
        let mut bytes = canonical_json(&value)?.into_bytes();
        bytes.push(b'\n');
        if bytes.len() > MAX_REQUEST_BYTES {
            return Err(ConsumerFailure::new(
                ConsumerFailureKind::RequestInvalid,
                "evaluator_request_invalid",
            ));
        }
        Ok(bytes)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvaluatorBuildObservation {
    pub record_digest: String,
    pub evaluator_release: String,
    pub implementation_digest: String,
    pub contract_projection_digest: String,
    pub dependency_set_digest: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvaluatorExecutable {
    path: PathBuf,
    build: EvaluatorBuildObservation,
}

impl EvaluatorExecutable {
    pub fn select(
        path: impl Into<PathBuf>,
        build_record_path: impl Into<PathBuf>,
        expected_build_record_digest: &str,
    ) -> Result<Self, ConsumerFailure> {
        Self::select_for(
            path,
            build_record_path,
            expected_build_record_digest,
            CONTRACT,
            RUNNER_PROFILE,
            EVALUATOR_RELEASE,
        )
    }

    pub(super) fn select_for(
        path: impl Into<PathBuf>,
        build_record_path: impl Into<PathBuf>,
        expected_build_record_digest: &str,
        evaluator_contract: &str,
        runner_profile: &str,
        evaluator_release: &str,
    ) -> Result<Self, ConsumerFailure> {
        let path = path.into();
        let build_record_path = build_record_path.into();
        if !path.is_absolute()
            || !build_record_path.is_absolute()
            || !sha256_digest(expected_build_record_digest)
        {
            return Err(ConsumerFailure::new(
                ConsumerFailureKind::ExecutableUnavailable,
                "runner_capacity_unavailable",
            ));
        }
        let metadata = fs::symlink_metadata(&path).map_err(|_| {
            ConsumerFailure::new(
                ConsumerFailureKind::ExecutableUnavailable,
                "runner_capacity_unavailable",
            )
        })?;
        if metadata.file_type().is_symlink() || !metadata.is_file() {
            return Err(ConsumerFailure::new(
                ConsumerFailureKind::ExecutableUnavailable,
                "runner_capacity_unavailable",
            ));
        }
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            if metadata.permissions().mode() & 0o111 == 0 {
                return Err(ConsumerFailure::new(
                    ConsumerFailureKind::ExecutableUnavailable,
                    "runner_capacity_unavailable",
                ));
            }
        }
        let build_record_metadata = fs::symlink_metadata(&build_record_path).map_err(|_| {
            ConsumerFailure::new(
                ConsumerFailureKind::ExecutableUnavailable,
                "runner_capacity_unavailable",
            )
        })?;
        if build_record_metadata.file_type().is_symlink()
            || !build_record_metadata.is_file()
            || build_record_metadata.len() as usize > MAX_BUILD_RECORD_BYTES
        {
            return Err(ConsumerFailure::new(
                ConsumerFailureKind::ExecutableUnavailable,
                "runner_capacity_unavailable",
            ));
        }
        let record_bytes = fs::read(&build_record_path).map_err(|_| {
            ConsumerFailure::new(
                ConsumerFailureKind::ExecutableUnavailable,
                "runner_capacity_unavailable",
            )
        })?;
        let actual_record_digest = format!("sha256:{}", hex::encode(Sha256::digest(&record_bytes)));
        if actual_record_digest != expected_build_record_digest {
            return Err(ConsumerFailure::new(
                ConsumerFailureKind::ExecutableUnavailable,
                "runner_capacity_unavailable",
            ));
        }
        let value = strict_json(&record_bytes).map_err(|_| {
            ConsumerFailure::new(
                ConsumerFailureKind::ExecutableUnavailable,
                "runner_capacity_unavailable",
            )
        })?;
        let canonical = canonical_json(&value).map_err(|_| {
            ConsumerFailure::new(
                ConsumerFailureKind::ExecutableUnavailable,
                "runner_capacity_unavailable",
            )
        })?;
        if canonical.as_bytes() != record_bytes {
            return Err(ConsumerFailure::new(
                ConsumerFailureKind::ExecutableUnavailable,
                "runner_capacity_unavailable",
            ));
        }
        let object = exact_object(
            &value,
            &[
                "contract",
                "evaluator_contract",
                "runner_profile",
                "evaluator_release",
                "implementation_digest",
                "contract_projection_digest",
                "dependency_set_digest",
            ],
        )
        .ok_or_else(|| {
            ConsumerFailure::new(
                ConsumerFailureKind::ExecutableUnavailable,
                "runner_capacity_unavailable",
            )
        })?;
        let field = |name: &str| object.get(name).and_then(Value::as_str);
        if field("contract") != Some(INSTALLED_BUILD_CONTRACT)
            || field("evaluator_contract") != Some(evaluator_contract)
            || field("runner_profile") != Some(runner_profile)
            || field("evaluator_release") != Some(evaluator_release)
            || !field("implementation_digest").is_some_and(sha256_digest)
            || !field("contract_projection_digest").is_some_and(sha256_digest)
            || !field("dependency_set_digest").is_some_and(sha256_digest)
        {
            return Err(ConsumerFailure::new(
                ConsumerFailureKind::ExecutableUnavailable,
                "runner_capacity_unavailable",
            ));
        }
        Ok(Self {
            path,
            build: EvaluatorBuildObservation {
                record_digest: actual_record_digest,
                evaluator_release: evaluator_release.to_string(),
                implementation_digest: field("implementation_digest")
                    .unwrap_or_default()
                    .to_string(),
                contract_projection_digest: field("contract_projection_digest")
                    .unwrap_or_default()
                    .to_string(),
                dependency_set_digest: field("dependency_set_digest")
                    .unwrap_or_default()
                    .to_string(),
            },
        })
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn build(&self) -> &EvaluatorBuildObservation {
        &self.build
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ConsumerFailureKind {
    RequestInvalid,
    ExecutableUnavailable,
    LaunchFailed,
    RequestWriteFailed,
    Timeout,
    AbnormalExit,
    StdoutLimit,
    StderrLimit,
    OutputFraming,
    OutputMalformed,
    ResponseInvalid,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ConsumerFailure {
    pub kind: ConsumerFailureKind,
    pub diagnostic_code: &'static str,
}

impl ConsumerFailure {
    pub(super) fn new(kind: ConsumerFailureKind, diagnostic_code: &'static str) -> Self {
        Self {
            kind,
            diagnostic_code,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvaluatorResponseV2 {
    value: Value,
}

impl EvaluatorResponseV2 {
    pub fn value(&self) -> &Value {
        &self.value
    }
}

#[derive(Clone, Copy)]
pub(super) struct ProcessPolicy {
    pub(super) timeout: Duration,
    pub(super) stdout_limit: usize,
    pub(super) stderr_limit: usize,
}

impl ProcessPolicy {
    pub(super) const PROFILE: Self = Self {
        timeout: OUTER_WATCHDOG,
        stdout_limit: MAX_STDOUT_BYTES,
        stderr_limit: MAX_STDERR_BYTES,
    };
}

pub fn invoke_action(
    executable: &EvaluatorExecutable,
    request: &ActionInvocationRequestV2,
) -> Result<EvaluatorResponseV2, ConsumerFailure> {
    invoke_with_policy(executable, request, ProcessPolicy::PROFILE)
}

pub(super) fn invoke_with_policy(
    executable: &EvaluatorExecutable,
    request: &ActionInvocationRequestV2,
    policy: ProcessPolicy,
) -> Result<EvaluatorResponseV2, ConsumerFailure> {
    let request_bytes = request.framed_bytes()?;
    let value = invoke_framed_value(executable, &request_bytes, policy, validate_response)?;
    Ok(EvaluatorResponseV2 { value })
}

pub(super) fn invoke_framed_value(
    executable: &EvaluatorExecutable,
    request_bytes: &[u8],
    policy: ProcessPolicy,
    validator: fn(&Value) -> Result<(), ConsumerFailure>,
) -> Result<Value, ConsumerFailure> {
    let mut child = Command::new(executable.path())
        .arg("--request-file")
        .arg("-")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| {
            ConsumerFailure::new(
                ConsumerFailureKind::LaunchFailed,
                "runner_capacity_unavailable",
            )
        })?;

    let mut stdin = child.stdin.take().ok_or_else(|| {
        ConsumerFailure::new(
            ConsumerFailureKind::RequestWriteFailed,
            "engine_execution_integrity_failed",
        )
    })?;
    if stdin.write_all(request_bytes).is_err() || stdin.flush().is_err() {
        terminate(&mut child);
        return Err(ConsumerFailure::new(
            ConsumerFailureKind::RequestWriteFailed,
            "engine_execution_integrity_failed",
        ));
    }
    drop(stdin);

    let stdout = child.stdout.take().ok_or_else(|| {
        ConsumerFailure::new(
            ConsumerFailureKind::OutputFraming,
            "engine_execution_integrity_failed",
        )
    })?;
    let stderr = child.stderr.take().ok_or_else(|| {
        ConsumerFailure::new(
            ConsumerFailureKind::OutputFraming,
            "engine_execution_integrity_failed",
        )
    })?;
    let mut stdout_reader = Some(read_bounded_stdout(stdout, policy.stdout_limit));
    let mut stderr_reader = Some(read_bounded_stderr(stderr, policy.stderr_limit));
    let mut completed_stdout = None;
    let mut completed_stderr = None;
    let started = Instant::now();

    loop {
        if started.elapsed() >= policy.timeout {
            terminate(&mut child);
            let _ = join_reader(stdout_reader.take());
            let _ = join_reader(stderr_reader.take());
            return Err(ConsumerFailure::new(
                ConsumerFailureKind::Timeout,
                "engine_execution_integrity_failed",
            ));
        }
        if stdout_reader.as_ref().is_some_and(JoinHandle::is_finished) {
            let output = join_reader(stdout_reader.take())?;
            if output.len() > policy.stdout_limit {
                terminate(&mut child);
                let _ = join_reader(stderr_reader.take());
                return Err(ConsumerFailure::new(
                    ConsumerFailureKind::StdoutLimit,
                    "engine_execution_integrity_failed",
                ));
            }
            completed_stdout = Some(output);
        }
        if stderr_reader.as_ref().is_some_and(JoinHandle::is_finished) {
            let output = join_reader(stderr_reader.take())?;
            if output.len() > policy.stderr_limit {
                terminate(&mut child);
                let _ = join_reader(stdout_reader.take());
                return Err(ConsumerFailure::new(
                    ConsumerFailureKind::StderrLimit,
                    "engine_execution_integrity_failed",
                ));
            }
            completed_stderr = Some(output);
        }
        match child.try_wait() {
            Ok(Some(status)) => {
                let stdout = match completed_stdout.take() {
                    Some(output) => output,
                    None => join_reader(stdout_reader.take())?,
                };
                let stderr = match completed_stderr.take() {
                    Some(output) => output,
                    None => join_reader(stderr_reader.take())?,
                };
                if stdout.len() > policy.stdout_limit {
                    return Err(ConsumerFailure::new(
                        ConsumerFailureKind::StdoutLimit,
                        "engine_execution_integrity_failed",
                    ));
                }
                if stderr.len() > policy.stderr_limit {
                    return Err(ConsumerFailure::new(
                        ConsumerFailureKind::StderrLimit,
                        "engine_execution_integrity_failed",
                    ));
                }
                if !status.success() {
                    return Err(ConsumerFailure::new(
                        ConsumerFailureKind::AbnormalExit,
                        "engine_execution_integrity_failed",
                    ));
                }
                return validate_response_frame_with(&stdout, validator);
            }
            Ok(None) => thread::sleep(Duration::from_millis(2)),
            Err(_) => {
                terminate(&mut child);
                return Err(ConsumerFailure::new(
                    ConsumerFailureKind::AbnormalExit,
                    "engine_execution_integrity_failed",
                ));
            }
        }
    }
}

type Reader = JoinHandle<Result<Vec<u8>, ConsumerFailure>>;

fn read_bounded_stdout(mut reader: ChildStdout, limit: usize) -> Reader {
    thread::spawn(move || read_limit(&mut reader, limit))
}

fn read_bounded_stderr(mut reader: ChildStderr, limit: usize) -> Reader {
    thread::spawn(move || read_limit(&mut reader, limit))
}

fn read_limit(reader: &mut dyn Read, limit: usize) -> Result<Vec<u8>, ConsumerFailure> {
    let mut output = Vec::with_capacity(limit.min(64 * 1024) + 1);
    reader
        .take((limit + 1) as u64)
        .read_to_end(&mut output)
        .map_err(|_| {
            ConsumerFailure::new(
                ConsumerFailureKind::OutputFraming,
                "engine_execution_integrity_failed",
            )
        })?;
    Ok(output)
}

fn join_reader(reader: Option<Reader>) -> Result<Vec<u8>, ConsumerFailure> {
    reader
        .ok_or_else(|| {
            ConsumerFailure::new(
                ConsumerFailureKind::OutputFraming,
                "engine_execution_integrity_failed",
            )
        })?
        .join()
        .map_err(|_| {
            ConsumerFailure::new(
                ConsumerFailureKind::OutputFraming,
                "engine_execution_integrity_failed",
            )
        })?
}

fn terminate(child: &mut Child) {
    let _ = child.kill();
    let _ = child.wait();
}

#[cfg(test)]
pub(super) fn validate_response_frame(
    stdout: &[u8],
) -> Result<EvaluatorResponseV2, ConsumerFailure> {
    let value = validate_response_frame_with(stdout, validate_response)?;
    Ok(EvaluatorResponseV2 { value })
}

fn validate_response_frame_with(
    stdout: &[u8],
    validator: fn(&Value) -> Result<(), ConsumerFailure>,
) -> Result<Value, ConsumerFailure> {
    if stdout.is_empty()
        || !stdout.ends_with(b"\n")
        || stdout[..stdout.len() - 1].contains(&b'\n')
        || stdout[..stdout.len() - 1].contains(&b'\r')
    {
        return Err(ConsumerFailure::new(
            ConsumerFailureKind::OutputFraming,
            "engine_execution_integrity_failed",
        ));
    }
    let object_bytes = &stdout[..stdout.len() - 1];
    let value = strict_json(object_bytes)?;
    validator(&value)?;
    let expected = canonical_json(&value)?;
    if expected.as_bytes() != object_bytes {
        return Err(ConsumerFailure::new(
            ConsumerFailureKind::ResponseInvalid,
            "evaluator_response_contract_invalid",
        ));
    }
    Ok(value)
}

pub(crate) fn strict_json(bytes: &[u8]) -> Result<Value, ConsumerFailure> {
    let mut deserializer = serde_json::Deserializer::from_slice(bytes);
    let value = StrictValueSeed
        .deserialize(&mut deserializer)
        .map_err(|_| {
            ConsumerFailure::new(
                ConsumerFailureKind::OutputMalformed,
                "evaluator_response_contract_invalid",
            )
        })?;
    deserializer.end().map_err(|_| {
        ConsumerFailure::new(
            ConsumerFailureKind::OutputMalformed,
            "evaluator_response_contract_invalid",
        )
    })?;
    Ok(value)
}

struct StrictValueSeed;

impl<'de> DeserializeSeed<'de> for StrictValueSeed {
    type Value = Value;

    fn deserialize<D>(self, deserializer: D) -> Result<Self::Value, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        deserializer.deserialize_any(StrictValueVisitor)
    }
}

struct StrictValueVisitor;

impl<'de> Visitor<'de> for StrictValueVisitor {
    type Value = Value;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("one strict JSON value")
    }

    fn visit_bool<E>(self, value: bool) -> Result<Value, E> {
        Ok(Value::Bool(value))
    }

    fn visit_i64<E>(self, value: i64) -> Result<Value, E> {
        Ok(Value::Number(Number::from(value)))
    }

    fn visit_u64<E>(self, value: u64) -> Result<Value, E> {
        Ok(Value::Number(Number::from(value)))
    }

    fn visit_f64<E>(self, value: f64) -> Result<Value, E>
    where
        E: de::Error,
    {
        Number::from_f64(value)
            .map(Value::Number)
            .ok_or_else(|| E::custom("non-finite number"))
    }

    fn visit_str<E>(self, value: &str) -> Result<Value, E> {
        Ok(Value::String(value.to_owned()))
    }

    fn visit_string<E>(self, value: String) -> Result<Value, E> {
        Ok(Value::String(value))
    }

    fn visit_none<E>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }

    fn visit_unit<E>(self) -> Result<Value, E> {
        Ok(Value::Null)
    }

    fn visit_seq<A>(self, mut sequence: A) -> Result<Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut values = Vec::new();
        while let Some(value) = sequence.next_element_seed(StrictValueSeed)? {
            values.push(value);
        }
        Ok(Value::Array(values))
    }

    fn visit_map<A>(self, mut map: A) -> Result<Value, A::Error>
    where
        A: MapAccess<'de>,
    {
        let mut keys = BTreeSet::new();
        let mut values = Map::new();
        while let Some(key) = map.next_key::<String>()? {
            if !keys.insert(key.clone()) {
                return Err(de::Error::custom("duplicate member"));
            }
            let value = map.next_value_seed(StrictValueSeed)?;
            values.insert(key, value);
        }
        Ok(Value::Object(values))
    }
}

pub(crate) fn canonical_json(value: &Value) -> Result<String, ConsumerFailure> {
    fn encode(value: &Value, output: &mut String) -> Result<(), ()> {
        match value {
            Value::Null => output.push_str("null"),
            Value::Bool(true) => output.push_str("true"),
            Value::Bool(false) => output.push_str("false"),
            Value::Number(number) => output.push_str(&number.to_string()),
            Value::String(text) => output.push_str(&serde_json::to_string(text).map_err(|_| ())?),
            Value::Array(values) => {
                output.push('[');
                for (index, item) in values.iter().enumerate() {
                    if index > 0 {
                        output.push(',');
                    }
                    encode(item, output)?;
                }
                output.push(']');
            }
            Value::Object(values) => {
                let mut entries: Vec<_> = values.iter().collect();
                entries.sort_by_key(|(key, _)| key.encode_utf16().collect::<Vec<u16>>());
                output.push('{');
                for (index, (key, item)) in entries.into_iter().enumerate() {
                    if index > 0 {
                        output.push(',');
                    }
                    output.push_str(&serde_json::to_string(key).map_err(|_| ())?);
                    output.push(':');
                    encode(item, output)?;
                }
                output.push('}');
            }
        }
        Ok(())
    }
    let mut output = String::new();
    encode(value, &mut output).map_err(|_| {
        ConsumerFailure::new(
            ConsumerFailureKind::ResponseInvalid,
            "evaluator_response_contract_invalid",
        )
    })?;
    Ok(output)
}

pub(super) fn validate_response(value: &Value) -> Result<(), ConsumerFailure> {
    let object = exact_object(value, &["contract", "disposition", "diagnostic"])
        .or_else(|| {
            exact_object(
                value,
                &[
                    "contract",
                    "disposition",
                    "execution",
                    "result_validation",
                    "semantic_owner",
                    "result",
                    "diagnostic",
                ],
            )
        })
        .ok_or_else(response_invalid)?;
    if object.get("contract").and_then(Value::as_str) != Some(CONTRACT) {
        return Err(response_invalid());
    }
    match object.get("disposition").and_then(Value::as_str) {
        Some("request-rejected") => validate_boundary_diagnostic(
            object.get("diagnostic"),
            &[
                (
                    "evaluator_request_invalid",
                    "evaluator-request-invalid-v1",
                    "runner-call-validation",
                ),
                (
                    "invalid_metadata_contract",
                    "invalid-metadata-contract-v1",
                    "runner-call-validation",
                ),
                (
                    "runner_profile_unsupported",
                    "runner-profile-unsupported-v1",
                    "runner-profile-selection",
                ),
            ],
        ),
        Some("integrity-failed") => validate_boundary_diagnostic(
            object.get("diagnostic"),
            &[
                (
                    "evidence_argument_integrity_failed",
                    "evidence-argument-integrity-failed-v1",
                    "runner-call-validation",
                ),
                (
                    "test_digest_mismatch",
                    "test-digest-mismatch-v1",
                    "runner-call-validation",
                ),
            ],
        ),
        Some("occurrence-result") => validate_occurrence(object),
        _ => Err(response_invalid()),
    }
}

fn validate_occurrence(object: &Map<String, Value>) -> Result<(), ConsumerFailure> {
    let execution = exact_object(
        object.get("execution").unwrap_or(&Value::Null),
        &[
            "executed",
            "status",
            "phase",
            "evaluate_call_count",
            "automatic_retry_count",
        ],
    )
    .ok_or_else(response_invalid)?;
    let validation = exact_object(
        object.get("result_validation").unwrap_or(&Value::Null),
        &["contract", "limit"],
    )
    .ok_or_else(response_invalid)?;
    let result = validate_semantic_result(object.get("result").unwrap_or(&Value::Null))?;
    if execution
        .get("automatic_retry_count")
        .and_then(Value::as_u64)
        != Some(0)
    {
        return Err(response_invalid());
    }
    let observed = (
        execution.get("executed").and_then(Value::as_bool),
        execution.get("status").and_then(Value::as_str),
        execution.get("phase").and_then(Value::as_str),
        execution.get("evaluate_call_count").and_then(Value::as_u64),
        validation.get("contract").and_then(Value::as_str),
        validation.get("limit").and_then(Value::as_str),
        object.get("semantic_owner").and_then(Value::as_str),
    );
    match observed {
        (
            Some(true),
            Some("completed"),
            Some("evaluate-call"),
            Some(1),
            Some("passed"),
            Some("passed"),
            Some("test-of-detail"),
        ) => {
            if object.get("diagnostic") != Some(&Value::Null) {
                return Err(response_invalid());
            }
            Ok(())
        }
        (
            Some(true),
            Some("completed"),
            Some("result-validation"),
            Some(1),
            Some("failed"),
            Some("not-evaluated"),
            Some("verification-engine"),
        ) => validate_engine_owned(
            object,
            result,
            "completed_invalid_result_contract",
            "completed-invalid-result-contract-v1",
            "result-validation",
        ),
        (
            Some(true),
            Some("completed"),
            Some("result-validation"),
            Some(1),
            Some("passed"),
            Some("failed"),
            Some("verification-engine"),
        ) => validate_engine_owned(
            object,
            result,
            "completed_result_limit_exceeded",
            "completed-result-limit-exceeded-v1",
            "result-validation",
        ),
        (
            Some(false),
            Some("blocked"),
            Some("realm-activation"),
            Some(0),
            Some("not-applicable"),
            Some("not-applicable"),
            Some("verification-engine"),
        ) => validate_engine_owned(
            object,
            result,
            "runner_activation_failed",
            "runner-activation-failed-v1",
            "realm-activation",
        ),
        (
            Some(true),
            Some("terminated"),
            Some("module-initialization"),
            Some(0),
            Some("not-applicable"),
            Some("not-applicable"),
            Some("verification-engine"),
        ) => validate_engine_owned(
            object,
            result,
            "test_exception",
            "test-exception-v1",
            "module-initialization",
        ),
        (
            Some(true),
            Some("terminated"),
            Some("evaluate-call"),
            Some(1),
            Some("not-applicable"),
            Some("not-applicable"),
            Some("verification-engine"),
        ) => {
            let allowed = [
                ("test_exception", "test-exception-v1", "evaluate-call"),
                (
                    "test_sandbox_capability_violation",
                    "test-sandbox-capability-violation-v1",
                    "evaluate-call",
                ),
                (
                    "test_execution_work_limit_exceeded",
                    "test-execution-work-limit-exceeded-v1",
                    "evaluate-call",
                ),
                (
                    "test_call_depth_exceeded",
                    "test-call-depth-exceeded-v1",
                    "evaluate-call",
                ),
                (
                    "test_cpu_backstop_exceeded",
                    "test-cpu-backstop-exceeded-v1",
                    "worker-supervision",
                ),
                (
                    "test_wall_watchdog_expired",
                    "test-wall-watchdog-expired-v1",
                    "worker-supervision",
                ),
                (
                    "test_memory_backstop_exceeded",
                    "test-memory-backstop-exceeded-v1",
                    "worker-supervision",
                ),
                (
                    "test_worker_lost",
                    "test-worker-lost-v1",
                    "worker-supervision",
                ),
            ];
            validate_engine_result(result)?;
            validate_boundary_diagnostic(object.get("diagnostic"), &allowed)
        }
        _ => Err(response_invalid()),
    }
}

fn validate_engine_owned(
    object: &Map<String, Value>,
    result: &Map<String, Value>,
    code: &'static str,
    template: &'static str,
    phase: &'static str,
) -> Result<(), ConsumerFailure> {
    validate_engine_result(result)?;
    validate_boundary_diagnostic(object.get("diagnostic"), &[(code, template, phase)])
}

fn validate_engine_result(result: &Map<String, Value>) -> Result<(), ConsumerFailure> {
    if result.get("conclusion").and_then(Value::as_str) != Some("inconclusive")
        || result.get("facts").and_then(Value::as_array) != Some(&Vec::new())
    {
        return Err(response_invalid());
    }
    Ok(())
}

fn validate_semantic_result(value: &Value) -> Result<&Map<String, Value>, ConsumerFailure> {
    let object =
        exact_object(value, &["conclusion", "facts", "reason"]).ok_or_else(response_invalid)?;
    if !matches!(
        object.get("conclusion").and_then(Value::as_str),
        Some("true" | "false" | "inconclusive")
    ) {
        return Err(response_invalid());
    }
    let reason = object
        .get("reason")
        .and_then(Value::as_str)
        .ok_or_else(response_invalid)?;
    if reason.is_empty() || reason.chars().count() > 65_536 {
        return Err(response_invalid());
    }
    let facts = object
        .get("facts")
        .and_then(Value::as_array)
        .ok_or_else(response_invalid)?;
    if facts.len() > 256 {
        return Err(response_invalid());
    }
    for fact in facts {
        let fact = exact_object(fact, &["name", "value", "value_type", "status"])
            .ok_or_else(response_invalid)?;
        let name = fact
            .get("name")
            .and_then(Value::as_str)
            .ok_or_else(response_invalid)?;
        if name.is_empty()
            || name.len() > 128
            || !name.bytes().enumerate().all(|(index, byte)| {
                byte.is_ascii_lowercase() || (index > 0 && (byte.is_ascii_digit() || byte == b'_'))
            })
        {
            return Err(response_invalid());
        }
        match fact.get("status").and_then(Value::as_str) {
            Some("found") => {
                if !matches!(
                    fact.get("value_type").and_then(Value::as_str),
                    Some(
                        "text"
                            | "integer"
                            | "number"
                            | "boolean"
                            | "date"
                            | "datetime"
                            | "duration"
                            | "array"
                            | "object"
                    )
                ) {
                    return Err(response_invalid());
                }
            }
            Some("not_found" | "invalid") => {
                if fact.get("value") != Some(&Value::Null)
                    || fact.get("value_type") != Some(&Value::Null)
                {
                    return Err(response_invalid());
                }
            }
            _ => return Err(response_invalid()),
        }
    }
    Ok(object)
}

fn validate_boundary_diagnostic(
    value: Option<&Value>,
    allowed: &[(&str, &str, &str)],
) -> Result<(), ConsumerFailure> {
    let diagnostic = exact_object(
        value.unwrap_or(&Value::Null),
        &["code", "phase", "reason_template"],
    )
    .ok_or_else(response_invalid)?;
    let observed = (
        diagnostic.get("code").and_then(Value::as_str),
        diagnostic.get("reason_template").and_then(Value::as_str),
        diagnostic.get("phase").and_then(Value::as_str),
    );
    if allowed
        .iter()
        .any(|expected| observed == (Some(expected.0), Some(expected.1), Some(expected.2)))
    {
        Ok(())
    } else {
        Err(response_invalid())
    }
}

pub(super) fn exact_object<'a>(value: &'a Value, keys: &[&str]) -> Option<&'a Map<String, Value>> {
    let object = value.as_object()?;
    if object.len() != keys.len() || !keys.iter().all(|key| object.contains_key(*key)) {
        return None;
    }
    Some(object)
}

fn response_invalid() -> ConsumerFailure {
    ConsumerFailure::new(
        ConsumerFailureKind::ResponseInvalid,
        "evaluator_response_contract_invalid",
    )
}

pub(super) fn sha256_digest(value: &str) -> bool {
    value.len() == 71
        && value.starts_with("sha256:")
        && value[7..]
            .bytes()
            .all(|byte| byte.is_ascii_digit() || (b'a'..=b'f').contains(&byte))
}

pub(super) fn media_type(value: &str) -> bool {
    if value.len() < 3 || value.len() > 255 || value.matches('/').count() != 1 {
        return false;
    }
    value.split('/').all(|part| {
        !part.is_empty()
            && part.bytes().enumerate().all(|(index, byte)| {
                (index == 0 && (byte.is_ascii_lowercase() || byte.is_ascii_digit()))
                    || (index > 0
                        && (byte.is_ascii_lowercase()
                            || byte.is_ascii_digit()
                            || b"!#$&^_.+-".contains(&byte)))
            })
    })
}

pub(super) fn relative_normalized_path(value: &str) -> bool {
    if value.is_empty() || value.contains('\\') || value.contains('\0') {
        return false;
    }
    let path = Path::new(value);
    !path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, Component::Normal(_)))
}

pub(super) fn absolute_normalized_path(value: &str) -> bool {
    if value.contains('\\') || value.contains('\0') {
        return false;
    }
    let path = Path::new(value);
    path.is_absolute()
        && path
            .components()
            .all(|component| matches!(component, Component::RootDir | Component::Normal(_)))
}
