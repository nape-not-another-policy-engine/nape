//! Serde-backed controlled document projection driver.

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;

use kernel_oss::{error::Error, gateway::Gateway};
use nape_domain::{
    diagnostic::{NapeDiagnostic, NapeOutcome},
    gateway::controlled_document_projection::{
        ControlledDocumentProjectionGW, ControlledDocumentProjectionRequest, ControlledMediaProfile,
    },
    value::controlled_value::ControlledValue,
};
use serde_json::Value;

/// The production controlled-document projection driver.
pub struct ControlledDocumentProjectionDriver;

impl Gateway for ControlledDocumentProjectionDriver {
    type Request = ControlledDocumentProjectionRequest;
    type Response = NapeOutcome<ControlledValue>;

    fn execute(&self, request: Self::Request) -> Result<Self::Response, Error> {
        let projected = match request.profile() {
            ControlledMediaProfile::Json => {
                crate::gateway_driver::action_evaluation_nape_evaluator::contract_v2::strict_json(
                    request.bytes(),
                )
                .map_err(|_| "controlled JSON is invalid".to_string())
            }
            ControlledMediaProfile::Yaml => project_yaml(request.bytes()),
            ControlledMediaProfile::Toml => project_toml(request.bytes()),
        };
        Ok(match projected {
            Ok(value) => NapeOutcome::Completed(from_json(value)),
            Err(detail) => NapeOutcome::Rejected(NapeDiagnostic::try_new(
                "evidence_format_validation_failed",
                "evidence-format-validation-failed-v1",
                "evidence-admission",
                detail,
            )?),
        })
    }
}

pub(crate) fn project_yaml(bytes: &[u8]) -> Result<Value, String> {
    if bytes.len() > 4 * 1_024 * 1_024 {
        return Err("YAML exceeds the 4 MiB root-document ceiling".to_string());
    }
    let text = std::str::from_utf8(bytes).map_err(|_| "YAML is not UTF-8".to_string())?;
    for line in text.lines() {
        let mut plain = String::new();
        let mut single = false;
        let mut double = false;
        let mut escaped = false;
        for character in line.chars() {
            if escaped {
                escaped = false;
                continue;
            }
            if double && character == '\\' {
                escaped = true;
                continue;
            }
            if !double && character == '\'' {
                single = !single;
                continue;
            }
            if !single && character == '"' {
                double = !double;
                continue;
            }
            if !single && !double && character == '#' {
                break;
            }
            if !single && !double {
                plain.push(character);
            }
        }
        let trimmed = plain.trim_start();
        if trimmed.starts_with("<<:")
            || plain.split_whitespace().any(|token| {
                token.starts_with('&') || token.starts_with('*') || token.starts_with('!')
            })
        {
            return Err("YAML anchors, aliases, tags, and merge keys are excluded".to_string());
        }
        if contains_plain_timestamp(trimmed) {
            return Err("YAML implicit timestamp scalars are excluded".to_string());
        }
    }
    let value: serde_yaml::Value =
        serde_yaml::from_slice(bytes).map_err(|error| format!("YAML is invalid: {error}"))?;
    validate_yaml_value(&value, 0)?;
    let value = serde_json::to_value(value)
        .map_err(|_| "YAML contains a scalar outside the JSON-compatible data model".to_string())?;
    validate_json_depth(&value, 0)?;
    Ok(value)
}

fn validate_yaml_value(value: &serde_yaml::Value, depth: usize) -> Result<(), String> {
    match value {
        serde_yaml::Value::Sequence(values) => {
            let next = depth + 1;
            if next > 64 {
                return Err("YAML exceeds the collection-depth ceiling of 64".to_string());
            }
            for value in values {
                validate_yaml_value(value, next)?;
            }
        }
        serde_yaml::Value::Mapping(values) => {
            let next = depth + 1;
            if next > 64 {
                return Err("YAML exceeds the collection-depth ceiling of 64".to_string());
            }
            for (key, value) in values {
                if !matches!(key, serde_yaml::Value::String(_)) {
                    return Err("YAML mapping keys must be strings".to_string());
                }
                validate_yaml_value(value, next)?;
            }
        }
        serde_yaml::Value::Null
        | serde_yaml::Value::Bool(_)
        | serde_yaml::Value::Number(_)
        | serde_yaml::Value::String(_) => {}
        serde_yaml::Value::Tagged(_) => return Err("YAML tags are excluded".to_string()),
    }
    Ok(())
}

fn validate_json_depth(value: &Value, depth: usize) -> Result<(), String> {
    match value {
        Value::Array(values) => {
            let next = depth + 1;
            if next > 64 {
                return Err("YAML exceeds the collection-depth ceiling of 64".to_string());
            }
            for value in values {
                validate_json_depth(value, next)?;
            }
        }
        Value::Object(values) => {
            let next = depth + 1;
            if next > 64 {
                return Err("YAML exceeds the collection-depth ceiling of 64".to_string());
            }
            for value in values.values() {
                validate_json_depth(value, next)?;
            }
        }
        _ => {}
    }
    Ok(())
}

fn contains_plain_timestamp(line: &str) -> bool {
    let scalar = line
        .strip_prefix("- ")
        .unwrap_or(line)
        .split_once(':')
        .map(|(_, value)| value.trim())
        .unwrap_or("")
        .split_whitespace()
        .next()
        .unwrap_or("")
        .trim_end_matches(',');
    let bytes = scalar.as_bytes();
    bytes.len() >= 10
        && bytes[0..4].iter().all(u8::is_ascii_digit)
        && bytes[4] == b'-'
        && bytes[5..7].iter().all(u8::is_ascii_digit)
        && bytes[7] == b'-'
        && bytes[8..10].iter().all(u8::is_ascii_digit)
        && (bytes.len() == 10 || matches!(bytes[10], b'T' | b't'))
}

impl ControlledDocumentProjectionGW for ControlledDocumentProjectionDriver {}

fn project_toml(bytes: &[u8]) -> Result<Value, String> {
    let text =
        std::str::from_utf8(bytes).map_err(|_| "controlled TOML is not UTF-8".to_string())?;
    let value: toml::Value =
        toml::from_str(text).map_err(|_| "controlled TOML is invalid".to_string())?;
    toml_to_json(value)
}

fn toml_to_json(value: toml::Value) -> Result<Value, String> {
    Ok(match value {
        toml::Value::String(value) => Value::String(value),
        toml::Value::Integer(value) => Value::Number(value.into()),
        toml::Value::Float(value) => serde_json::Number::from_f64(value)
            .map(Value::Number)
            .ok_or_else(|| "TOML infinity and NaN are excluded".to_string())?,
        toml::Value::Boolean(value) => Value::Bool(value),
        toml::Value::Datetime(_) => return Err("TOML date/time values are excluded".to_string()),
        toml::Value::Array(values) => Value::Array(
            values
                .into_iter()
                .map(toml_to_json)
                .collect::<Result<Vec<_>, _>>()?,
        ),
        toml::Value::Table(values) => Value::Object(
            values
                .into_iter()
                .map(|(key, value)| toml_to_json(value).map(|value| (key, value)))
                .collect::<Result<_, _>>()?,
        ),
    })
}

pub(crate) fn from_json(value: Value) -> ControlledValue {
    match value {
        Value::Null => ControlledValue::Null,
        Value::Bool(value) => ControlledValue::Boolean(value),
        Value::Number(value) => ControlledValue::Number(value.to_string()),
        Value::String(value) => ControlledValue::String(value),
        Value::Array(values) => ControlledValue::Array(values.into_iter().map(from_json).collect()),
        Value::Object(values) => ControlledValue::Object(
            values
                .into_iter()
                .map(|(key, value)| (key, from_json(value)))
                .collect::<BTreeMap<_, _>>(),
        ),
    }
}

pub(crate) fn to_json(value: &ControlledValue) -> Result<Value, Error> {
    Ok(match value {
        ControlledValue::Null => Value::Null,
        ControlledValue::Boolean(value) => Value::Bool(*value),
        ControlledValue::Number(value) => Value::Number(value.parse().map_err(|_| {
            Error::for_system(
                kernel_oss::error::Kind::ProcessingFailure,
                "controlled number is not a valid JSON number",
            )
        })?),
        ControlledValue::String(value) => Value::String(value.clone()),
        ControlledValue::Array(values) => {
            Value::Array(values.iter().map(to_json).collect::<Result<Vec<_>, _>>()?)
        }
        ControlledValue::Object(values) => Value::Object(
            values
                .iter()
                .map(|(key, value)| to_json(value).map(|value| (key.clone(), value)))
                .collect::<Result<_, _>>()?,
        ),
    })
}
