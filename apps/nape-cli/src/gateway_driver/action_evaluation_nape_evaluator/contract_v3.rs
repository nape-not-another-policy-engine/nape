//! Strict NAPE consumer for the additive functional Evaluator V3 boundary.
//!
//! V3 adds validated structured Evidence, complete effective evaluations,
//! declared helper modules, and resource selection. It deliberately reuses
//! the V2 process supervisor while preserving V2 request/response admission.

use super::contract_v2::{
    absolute_normalized_path, canonical_json, exact_object, invoke_framed_value, media_type,
    relative_normalized_path, sha256_digest, validate_response as validate_v2_response,
    ConsumerFailure, ConsumerFailureKind, EvaluatorExecutable, EvaluatorLimits, EvidenceDescriptor,
    ProcessPolicy,
};
use serde::Serialize;
use serde_json::Value;
use std::collections::BTreeSet;
use std::path::PathBuf;

pub const CONTRACT: &str = "attestify.nape-evaluator.action-invocation/v3";
pub const RUNNER_PROFILE: &str = "attestify-python-test-development-v2";
pub const EVALUATOR_RELEASE: &str = "2.0.0";
pub const MAX_REQUEST_BYTES: usize = 1_048_576;

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TestModuleDescriptor {
    pub name: String,
    pub file: String,
    pub content_digest: String,
    pub byte_count: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct TestDescriptorV3 {
    pub file: String,
    pub content_digest: String,
    pub byte_count: u64,
    pub runner_profile: String,
    pub module: Vec<TestModuleDescriptor>,
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize)]
pub struct EvaluatorMetadataV3 {
    pub evidence_media_type: String,
    pub evidence_representation: String,
    pub evaluator_contract_version: String,
}

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct ActionInvocationRequestV3 {
    pub contract: String,
    pub workspace_root: String,
    pub evidence: EvidenceDescriptor,
    pub test: TestDescriptorV3,
    pub evaluations: Vec<Value>,
    pub metadata: EvaluatorMetadataV3,
    pub limits: EvaluatorLimits,
}

impl ActionInvocationRequestV3 {
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
            || !matches!(
                self.evidence.representation.as_str(),
                "opaque" | "validated-structured"
            )
            || self.test.runner_profile != RUNNER_PROFILE
            || self.evaluations.len() > 64
            || self.metadata.evidence_media_type != self.evidence.media_type
            || self.metadata.evidence_representation != self.evidence.representation
            || self.metadata.evaluator_contract_version != "3"
            || !(1..=100_000_000).contains(&self.limits.maximum_execution_work_units)
            || !(1..=1_048_576).contains(&self.limits.maximum_result_bytes)
            || !valid_modules(&self.test.module)
            || self.test.module.iter().any(|module| {
                module.name == "json"
                    || module.file == self.test.file
                    || module.file == self.evidence.file
            })
            || !self.evaluations.iter().all(valid_evaluation)
        {
            return Err(request_invalid());
        }
        Ok(())
    }

    fn framed_bytes(&self) -> Result<Vec<u8>, ConsumerFailure> {
        self.validate()?;
        let value = serde_json::to_value(self).map_err(|_| request_invalid())?;
        let mut bytes = canonical_json(&value)?.into_bytes();
        bytes.push(b'\n');
        if bytes.len() > MAX_REQUEST_BYTES {
            return Err(request_invalid());
        }
        Ok(bytes)
    }
}

pub fn select_evaluator(
    path: impl Into<PathBuf>,
    build_record_path: impl Into<PathBuf>,
    expected_build_record_digest: &str,
) -> Result<EvaluatorExecutable, ConsumerFailure> {
    EvaluatorExecutable::select_for(
        path,
        build_record_path,
        expected_build_record_digest,
        CONTRACT,
        RUNNER_PROFILE,
        EVALUATOR_RELEASE,
    )
}

#[derive(Clone, Debug, PartialEq)]
pub struct EvaluatorResponseV3 {
    value: Value,
}

impl EvaluatorResponseV3 {
    pub fn value(&self) -> &Value {
        &self.value
    }
}

pub fn invoke_action(
    executable: &EvaluatorExecutable,
    request: &ActionInvocationRequestV3,
) -> Result<EvaluatorResponseV3, ConsumerFailure> {
    let request_bytes = request.framed_bytes()?;
    let value = invoke_framed_value(
        executable,
        &request_bytes,
        ProcessPolicy::PROFILE,
        validate_response,
    )?;
    Ok(EvaluatorResponseV3 { value })
}

fn valid_modules(modules: &[TestModuleDescriptor]) -> bool {
    if modules.len() > 64 {
        return false;
    }
    let mut names = BTreeSet::new();
    let mut files = BTreeSet::new();
    let mut previous = None;
    modules.iter().all(|module| {
        let name = module.name.as_str();
        let valid = !name.is_empty()
            && name.len() <= 64
            && name.as_bytes()[0].is_ascii_lowercase()
            && name
                .bytes()
                .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'_')
            && previous.is_none_or(|prior: &str| prior < name)
            && names.insert(name)
            && files.insert(module.file.as_str())
            && relative_normalized_path(&module.file)
            && sha256_digest(&module.content_digest)
            && (1..=33_554_432).contains(&module.byte_count);
        previous = Some(name);
        valid
    })
}

fn valid_evaluation(value: &Value) -> bool {
    let Some(object) = exact_object(value, &["name", "label", "subject", "criteria"])
        .or_else(|| {
            exact_object(
                value,
                &["name", "label", "description", "subject", "criteria"],
            )
        })
        .or_else(|| exact_object(value, &["name", "label", "example", "subject", "criteria"]))
        .or_else(|| {
            exact_object(
                value,
                &[
                    "name",
                    "label",
                    "description",
                    "example",
                    "subject",
                    "criteria",
                ],
            )
        })
    else {
        return false;
    };
    let Some(name) = object.get("name").and_then(Value::as_str) else {
        return false;
    };
    let Some(label) = object.get("label").and_then(Value::as_str) else {
        return false;
    };
    let Some(subject) = object
        .get("subject")
        .and_then(|value| exact_object(value, &["name", "label", "data_type"]))
    else {
        return false;
    };
    let Some(criteria) = object.get("criteria").and_then(Value::as_object) else {
        return false;
    };
    valid_kebab_name(name)
        && !label.is_empty()
        && label.len() <= 256
        && object.get("description").is_none_or(|value| {
            value
                .as_str()
                .is_some_and(|text| !text.is_empty() && text.len() <= 4096)
        })
        && object.get("example").is_none_or(|value| {
            value
                .as_str()
                .is_some_and(|text| !text.is_empty() && text.len() <= 4096)
        })
        && subject
            .get("name")
            .and_then(Value::as_str)
            .is_some_and(valid_fact_name)
        && subject
            .get("label")
            .and_then(Value::as_str)
            .is_some_and(|text| !text.is_empty() && text.len() <= 256)
        && subject
            .get("data_type")
            .and_then(Value::as_str)
            .is_some_and(|data_type| valid_criterion_types(criteria, data_type))
        && !criteria.is_empty()
        && criteria.len() <= 64
        && criteria.keys().all(|key| {
            matches!(
                key.as_str(),
                "allowed_values"
                    | "disallowed_values"
                    | "equals"
                    | "maximum"
                    | "minimum"
                    | "required"
            )
        })
}

fn valid_criterion_types(criteria: &serde_json::Map<String, Value>, data_type: &str) -> bool {
    if !matches!(
        data_type,
        "text"
            | "integer"
            | "number"
            | "boolean"
            | "date"
            | "datetime"
            | "duration"
            | "array"
            | "object"
            | "null"
    ) {
        return false;
    }
    criteria
        .iter()
        .all(|(operator, value)| match operator.as_str() {
            "allowed_values" | "disallowed_values" => value.as_array().is_some_and(|values| {
                (1..=64).contains(&values.len())
                    && values
                        .iter()
                        .all(|candidate| value_matches_data_type(candidate, data_type))
            }),
            "required" => value.is_boolean(),
            _ => value_matches_data_type(value, data_type),
        })
}

fn value_matches_data_type(value: &Value, data_type: &str) -> bool {
    match data_type {
        "text" | "date" | "datetime" | "duration" => value.is_string(),
        "integer" => value.as_i64().is_some() || value.as_u64().is_some(),
        "number" => value.is_number(),
        "boolean" => value.is_boolean(),
        "array" => value.is_array(),
        "object" => value.is_object(),
        "null" => value.is_null(),
        _ => false,
    }
}

fn valid_kebab_name(value: &str) -> bool {
    let bytes = value.as_bytes();
    !bytes.is_empty()
        && bytes.len() <= 128
        && (bytes[0].is_ascii_lowercase() || bytes[0].is_ascii_digit())
        && (bytes[bytes.len() - 1].is_ascii_lowercase() || bytes[bytes.len() - 1].is_ascii_digit())
        && !value.contains("--")
        && bytes
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-')
}

fn valid_fact_name(value: &str) -> bool {
    let bytes = value.as_bytes();
    !bytes.is_empty()
        && bytes.len() <= 128
        && bytes[0].is_ascii_lowercase()
        && bytes
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'_')
}

pub(super) fn validate_response(value: &Value) -> Result<(), ConsumerFailure> {
    let Some(object) = value.as_object() else {
        return Err(response_invalid());
    };
    if object.get("contract").and_then(Value::as_str) != Some(CONTRACT) {
        return Err(response_invalid());
    }
    let mut translated = value.clone();
    translated
        .as_object_mut()
        .expect("response was established as an object")
        .insert(
            "contract".to_string(),
            Value::String(super::contract_v2::CONTRACT.to_string()),
        );
    // The current V3 Source Binding corrected only this diagnostic phase.
    // Translate the private validation copy back to the frozen V2 vocabulary;
    // the caller-visible V3 response remains unchanged.
    if value["disposition"] == "request-rejected"
        && value["diagnostic"]["code"] == "runner_profile_unsupported"
    {
        if value["diagnostic"]["reason_template"] != "runner-profile-unsupported-v1"
            || value["diagnostic"]["phase"] != "runner-call-validation"
        {
            return Err(response_invalid());
        }
        translated["diagnostic"]["phase"] = Value::String("runner-profile-selection".to_string());
    }
    validate_v2_response(&translated)
}

fn request_invalid() -> ConsumerFailure {
    ConsumerFailure::new(
        ConsumerFailureKind::RequestInvalid,
        "evaluator_request_invalid",
    )
}

fn response_invalid() -> ConsumerFailure {
    ConsumerFailure::new(
        ConsumerFailureKind::ResponseInvalid,
        "evaluator_response_contract_invalid",
    )
}
