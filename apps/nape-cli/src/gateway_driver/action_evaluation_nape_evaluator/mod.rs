//! Exact NAPE Evaluator V2/V3 process driver.

pub mod contract_v2;
#[cfg(test)]
mod contract_v2_tests;
pub mod contract_v3;
#[cfg(test)]
mod contract_v3_tests;
#[cfg(test)]
mod tests;

use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicU64, Ordering},
};

use kernel_oss::{
    error::{Error, Kind},
    gateway::Gateway,
};
use nape_domain::{
    diagnostic::{NapeDiagnostic, NapeOutcome},
    gateway::action_evaluation::{
        ActionEvaluationContract, ActionEvaluationGW, ActionEvaluationImplementation,
        ActionEvaluationObservation, ActionEvaluationRequest,
    },
    value::{evidence::EvidenceArgument, verification_outcome::VerificationConclusion},
};
use serde_json::Value;
use sha2::{Digest, Sha256};

use self::{contract_v2 as v2, contract_v3 as v3};

use super::controlled_document_projection::{from_json, to_json};

static NEXT_INVOCATION: AtomicU64 = AtomicU64::new(1);

/// One exact preselected Evaluator executable per versioned contract.
pub enum SelectedEvaluatorExecutable {
    /// Evaluator V2 with the development-v1 Runner profile.
    V2(v2::EvaluatorExecutable),
    /// Evaluator V3 with the development-v2 Runner profile.
    V3(v2::EvaluatorExecutable),
    /// Both frozen boundaries; each occurrence request selects exactly one.
    V2AndV3 {
        /// Evaluator V2 executable.
        v2: v2::EvaluatorExecutable,
        /// Evaluator V3 executable.
        v3: v2::EvaluatorExecutable,
    },
}

/// Invokes one isolated Action occurrence and translates its governed result once.
pub struct NapeEvaluatorActionEvaluationDriver {
    executable: SelectedEvaluatorExecutable,
    staging_root: PathBuf,
}

impl NapeEvaluatorActionEvaluationDriver {
    /// Creates a driver around one already-admitted Evaluator executable.
    pub fn new(executable: SelectedEvaluatorExecutable, staging_root: impl Into<PathBuf>) -> Self {
        Self {
            executable,
            staging_root: staging_root.into(),
        }
    }
}

impl Gateway for NapeEvaluatorActionEvaluationDriver {
    type Request = ActionEvaluationRequest;
    type Response = NapeOutcome<ActionEvaluationObservation>;

    fn execute(&self, request: Self::Request) -> Result<Self::Response, Error> {
        let contract_matches = matches!(
            (&self.executable, request.contract),
            (
                SelectedEvaluatorExecutable::V2(_),
                ActionEvaluationContract::V2
            ) | (
                SelectedEvaluatorExecutable::V3(_),
                ActionEvaluationContract::V3
            ) | (SelectedEvaluatorExecutable::V2AndV3 { .. }, _)
        );
        if !contract_matches {
            return Ok(NapeOutcome::Rejected(evaluator_diagnostic(
                "evaluator_request_invalid",
                "evaluator-request-invalid-v1",
                "runner-call-validation",
                "the prepared Action selected a different Evaluator contract",
            )?));
        }
        let workspace = self.staging_root.join(format!(
            "action-{}-{}",
            std::process::id(),
            NEXT_INVOCATION.fetch_add(1, Ordering::Relaxed)
        ));
        create_private_workspace(&workspace)?;
        let outcome = self.invoke(request, &workspace);
        let _ = std::fs::remove_dir_all(&workspace);
        outcome
    }
}

fn create_private_workspace(path: &Path) -> Result<(), Error> {
    let parent = path.parent().ok_or_else(|| {
        Error::for_system(
            Kind::GatewayError,
            "Evaluator workspace parent is unavailable",
        )
    })?;
    std::fs::create_dir_all(parent).map_err(|_| {
        Error::for_system(
            Kind::GatewayError,
            "Evaluator workspace parent is unavailable",
        )
    })?;
    let mut builder = std::fs::DirBuilder::new();
    #[cfg(unix)]
    {
        use std::os::unix::fs::DirBuilderExt;
        builder.mode(0o700);
    }
    builder
        .create(path)
        .map_err(|_| Error::for_system(Kind::GatewayError, "Evaluator workspace is unavailable"))
}

impl ActionEvaluationGW for NapeEvaluatorActionEvaluationDriver {}

impl NapeEvaluatorActionEvaluationDriver {
    fn invoke(
        &self,
        request: ActionEvaluationRequest,
        workspace: &Path,
    ) -> Result<NapeOutcome<ActionEvaluationObservation>, Error> {
        let (evidence_bytes, representation) = match &request.evidence {
            EvidenceArgument::Opaque(bytes) => (bytes.clone(), "opaque"),
            EvidenceArgument::Controlled(value) => {
                let json = to_json(value)?;
                let bytes = v2::canonical_json(&json)
                    .map_err(|_| {
                        Error::for_system(
                            Kind::ProcessingFailure,
                            "controlled Evidence is not canonicalizable",
                        )
                    })?
                    .into_bytes();
                (bytes, "validated-structured")
            }
        };
        let evidence_file = format!("evidence/{}", request.evidence_file);
        let test_file = format!("test/{}", request.test_file);
        write_workspace_file(workspace, &evidence_file, &evidence_bytes)?;
        write_workspace_file(workspace, &test_file, &request.test_source)?;
        let evidence = v2::EvidenceDescriptor {
            file: evidence_file,
            argument_digest: digest(&evidence_bytes),
            argument_byte_count: evidence_bytes.len() as u64,
            media_type: request.evidence_media_type.clone(),
            representation: representation.to_string(),
        };
        let limits = v2::EvaluatorLimits {
            maximum_execution_work_units: request.maximum_execution_work_units,
            maximum_result_bytes: request.maximum_result_bytes,
        };
        let response = match (&self.executable, request.contract) {
            (SelectedEvaluatorExecutable::V2(executable), ActionEvaluationContract::V2)
            | (
                SelectedEvaluatorExecutable::V2AndV3 { v2: executable, .. },
                ActionEvaluationContract::V2,
            ) => {
                if representation != "opaque"
                    || !request.evaluations.is_empty()
                    || !request.modules.is_empty()
                {
                    return Ok(NapeOutcome::Rejected(evaluator_diagnostic(
                        "evaluator_request_invalid",
                        "evaluator-request-invalid-v1",
                        "runner-call-validation",
                        "Evaluator V2 accepts opaque Evidence without evaluations or helper modules",
                    )?));
                }
                let request = v2::ActionInvocationRequestV2 {
                    contract: v2::CONTRACT.to_string(),
                    workspace_root: workspace.to_string_lossy().to_string(),
                    evidence,
                    test: v2::TestDescriptor {
                        file: test_file,
                        content_digest: digest(&request.test_source),
                        byte_count: request.test_source.len() as u64,
                        runner_profile: v2::RUNNER_PROFILE.to_string(),
                    },
                    evaluations: Vec::new(),
                    metadata: v2::EvaluatorMetadata {
                        evidence_media_type: request.evidence_media_type,
                        evidence_representation: representation.to_string(),
                        evaluator_contract_version: "2".to_string(),
                    },
                    limits,
                };
                match v2::invoke_action(executable, &request) {
                    Ok(value) => value.value().clone(),
                    Err(error) => return Ok(NapeOutcome::Rejected(consumer_diagnostic(error)?)),
                }
            }
            (SelectedEvaluatorExecutable::V3(executable), ActionEvaluationContract::V3)
            | (
                SelectedEvaluatorExecutable::V2AndV3 { v3: executable, .. },
                ActionEvaluationContract::V3,
            ) => {
                let mut modules = Vec::with_capacity(request.modules.len());
                for module in &request.modules {
                    let file = format!("module/{}", module.file);
                    write_workspace_file(workspace, &file, &module.source)?;
                    modules.push(v3::TestModuleDescriptor {
                        name: module.name.clone(),
                        file,
                        content_digest: digest(&module.source),
                        byte_count: module.source.len() as u64,
                    });
                }
                modules.sort_by(|left, right| left.name.cmp(&right.name));
                let evaluations = request
                    .evaluations
                    .iter()
                    .map(|evaluation| {
                        evaluation
                            .wire()
                            .ok_or_else(|| {
                                Error::for_system(
                                    Kind::ProcessingFailure,
                                    "effective evaluation omitted its governed wire projection",
                                )
                            })
                            .and_then(to_json)
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                let request = v3::ActionInvocationRequestV3 {
                    contract: v3::CONTRACT.to_string(),
                    workspace_root: workspace.to_string_lossy().to_string(),
                    evidence,
                    test: v3::TestDescriptorV3 {
                        file: test_file,
                        content_digest: digest(&request.test_source),
                        byte_count: request.test_source.len() as u64,
                        runner_profile: v3::RUNNER_PROFILE.to_string(),
                        module: modules,
                    },
                    evaluations,
                    metadata: v3::EvaluatorMetadataV3 {
                        evidence_media_type: request.evidence_media_type,
                        evidence_representation: representation.to_string(),
                        evaluator_contract_version: "3".to_string(),
                    },
                    limits,
                };
                match v3::invoke_action(executable, &request) {
                    Ok(value) => value.value().clone(),
                    Err(error) => return Ok(NapeOutcome::Rejected(consumer_diagnostic(error)?)),
                }
            }
            _ => unreachable!("contract compatibility was admitted above"),
        };
        let (contract, build) = match (&self.executable, request.contract) {
            (SelectedEvaluatorExecutable::V2(value), ActionEvaluationContract::V2)
            | (
                SelectedEvaluatorExecutable::V2AndV3 { v2: value, .. },
                ActionEvaluationContract::V2,
            ) => (ActionEvaluationContract::V2, value.build()),
            (SelectedEvaluatorExecutable::V3(value), ActionEvaluationContract::V3)
            | (
                SelectedEvaluatorExecutable::V2AndV3 { v3: value, .. },
                ActionEvaluationContract::V3,
            ) => (ActionEvaluationContract::V3, value.build()),
            _ => unreachable!("contract compatibility was admitted above"),
        };
        project_response(&response, contract, build)
    }
}

fn project_response(
    value: &Value,
    contract: ActionEvaluationContract,
    build: &v2::EvaluatorBuildObservation,
) -> Result<NapeOutcome<ActionEvaluationObservation>, Error> {
    if value.get("disposition").and_then(Value::as_str) != Some("occurrence-result") {
        let diagnostic = value
            .get("diagnostic")
            .and_then(Value::as_object)
            .ok_or_else(|| {
                Error::for_system(
                    Kind::ProcessingFailure,
                    "Evaluator boundary result omitted its diagnostic",
                )
            })?;
        return Ok(NapeOutcome::Rejected(NapeDiagnostic::try_new(
            text(diagnostic, "code")?,
            text(diagnostic, "reason_template")?,
            text(diagnostic, "phase")?,
            "Evaluator returned a governed Action boundary result",
        )?));
    }
    let result = value
        .get("result")
        .and_then(Value::as_object)
        .ok_or_else(|| {
            Error::for_system(
                Kind::ProcessingFailure,
                "Evaluator result projection is absent",
            )
        })?;
    let conclusion = match text(result, "conclusion")? {
        "true" => VerificationConclusion::True,
        "false" => VerificationConclusion::False,
        "inconclusive" => VerificationConclusion::Inconclusive,
        _ => {
            return Err(Error::for_system(
                Kind::ProcessingFailure,
                "Evaluator conclusion escaped the closed vocabulary",
            ))
        }
    };
    let facts = result
        .get("facts")
        .and_then(Value::as_array)
        .ok_or_else(|| {
            Error::for_system(
                Kind::ProcessingFailure,
                "Evaluator facts projection is absent",
            )
        })?
        .iter()
        .cloned()
        .map(from_json)
        .collect();
    Ok(NapeOutcome::Completed(ActionEvaluationObservation {
        conclusion,
        facts,
        reason: text(result, "reason")?.to_string(),
        response: from_json(value.clone()),
        implementation: ActionEvaluationImplementation {
            contract,
            implementation_digest: build.implementation_digest.clone(),
            dependency_set_digest: build.dependency_set_digest.clone(),
        },
    }))
}

fn text<'a>(object: &'a serde_json::Map<String, Value>, name: &str) -> Result<&'a str, Error> {
    object.get(name).and_then(Value::as_str).ok_or_else(|| {
        Error::for_system(
            Kind::ProcessingFailure,
            format!("Evaluator response omitted {name}"),
        )
    })
}

fn consumer_diagnostic(error: v2::ConsumerFailure) -> Result<NapeDiagnostic, Error> {
    let (template, phase) = match error.diagnostic_code {
        "runner_capacity_unavailable" => ("runner-capacity-unavailable-v1", "invocation-admission"),
        "evaluator_request_invalid" => ("evaluator-request-invalid-v1", "runner-call-validation"),
        "evaluator_response_contract_invalid" => (
            "evaluator-response-contract-invalid-v1",
            "evaluator-response-validation",
        ),
        "engine_execution_integrity_failed" => (
            "engine-execution-integrity-failed-v1",
            "execution-supervision",
        ),
        _ => (
            "verification-engine-unexpected-internal-failure-v1",
            "engine-process",
        ),
    };
    evaluator_diagnostic(
        error.diagnostic_code,
        template,
        phase,
        "Evaluator invocation failed at its governed boundary",
    )
}

fn evaluator_diagnostic(
    code: &str,
    template: &str,
    phase: &str,
    detail: &str,
) -> Result<NapeDiagnostic, Error> {
    NapeDiagnostic::try_new(code, template, phase, detail)
}

fn write_workspace_file(root: &Path, relative: &str, bytes: &[u8]) -> Result<(), Error> {
    let destination = root.join(relative);
    let parent = destination.parent().ok_or_else(|| {
        Error::for_system(Kind::ProcessingFailure, "Evaluator file has no parent")
    })?;
    std::fs::create_dir_all(parent).map_err(|_| {
        Error::for_system(
            Kind::GatewayError,
            "Evaluator workspace directory is unavailable",
        )
    })?;
    std::fs::write(destination, bytes).map_err(|_| {
        Error::for_system(
            Kind::GatewayError,
            "Evaluator workspace file cannot be written",
        )
    })
}

fn digest(bytes: &[u8]) -> String {
    format!("sha256:{}", hex::encode(Sha256::digest(bytes)))
}
