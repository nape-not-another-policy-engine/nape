//! Closed Verification subject document driver.

#[cfg(test)]
mod tests;

use kernel_oss::{error::Error, gateway::Gateway};
use nape_domain::{
    diagnostic::{NapeDiagnostic, NapeOutcome},
    gateway::verification_subject_acquisition::VerificationSubjectAcquisitionGW,
    value::subject::{VerificationSubject, VerificationSubjectHandle},
};
use serde_json::Value;

use super::local_file::read_regular_stable;

/// Reads and admits one closed local subject document.
pub struct LocalVerificationSubjectAcquisitionDriver;

impl Gateway for LocalVerificationSubjectAcquisitionDriver {
    type Request = VerificationSubjectHandle;
    type Response = NapeOutcome<VerificationSubject>;

    fn execute(&self, request: Self::Request) -> Result<Self::Response, Error> {
        let admitted = read_regular_stable(std::path::Path::new(request.value()), 1_048_576)
            .map_err(|_| "subject input is unavailable, unstable, or too large".to_string())
            .and_then(|bytes| {
                crate::gateway_driver::controlled_document_projection::project_yaml(&bytes)
                    .map_err(|_| "subject input is not controlled YAML".to_string())
            })
            .and_then(admit_subject);
        Ok(match admitted {
            Ok(value) => NapeOutcome::Completed(value),
            Err(detail) => NapeOutcome::Rejected(NapeDiagnostic::try_new(
                "verification_subject_invalid",
                "verification-subject-invalid-v1",
                "invocation-admission",
                detail,
            )?),
        })
    }
}

impl VerificationSubjectAcquisitionGW for LocalVerificationSubjectAcquisitionDriver {}

fn admit_subject(value: Value) -> Result<VerificationSubject, String> {
    let object = value
        .as_object()
        .ok_or_else(|| "subject input must be an object".to_string())?;
    if object
        .keys()
        .any(|key| !matches!(key.as_str(), "arn" | "label" | "description"))
    {
        return Err("subject input contains an unknown field".to_string());
    }
    let arn = object
        .get("arn")
        .and_then(Value::as_str)
        .ok_or_else(|| "subject arn is required".to_string())?;
    let optional = |name: &str| -> Result<Option<String>, String> {
        object
            .get(name)
            .map(|value| {
                value
                    .as_str()
                    .map(str::to_string)
                    .ok_or_else(|| format!("subject {name} must be text"))
            })
            .transpose()
    };
    VerificationSubject::try_new(arn, optional("label")?, optional("description")?)
        .map_err(|_| "subject values are outside their bounds".to_string())
}
