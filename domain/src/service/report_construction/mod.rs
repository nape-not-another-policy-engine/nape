//! Deterministic transport-neutral VerificationReport construction.

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;

use kernel_oss::error::{Error, Kind};

use crate::{
    gateway::action_evaluation::{
        ActionEvaluationContract, ActionEvaluationObservation, PreparedTestModule,
    },
    value::{
        controlled_value::ControlledValue, current_verification::CurrentVerification,
        effective_graph::EffectiveActionOccurrence, verification_outcome::VerificationSummary,
    },
};

const RUNNER_PROFILE_V1: &str = "attestify-python-test-development-v1";
const RUNNER_PROFILE_V1_DIGEST: &str =
    "sha256:eec056aa51c2fd0afbe361c55a4310165d77bffdb28c5b49a83490c3d7ecdac9";
const RUNNER_PROFILE_V2: &str = "attestify-python-test-development-v2";
const RUNNER_PROFILE_V2_DIGEST: &str =
    "sha256:6d9eec4fafc156446c46da571c56f50945735e39bebbad9ccb448f1dfa312771";
const FUNCTIONAL_ENGINE_CONTRACT_DIGEST: &str =
    "sha256:9deef7a824a741fd313592703c866b4d854303ce7488365f35f9599fcf56f45d";

/// One successful schema-admission observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceSchemaObservation {
    /// Exact schema content digest.
    pub content_digest: String,
    /// Controlled schema media type.
    pub media_type: String,
    /// Exact package-owned schema path.
    pub file: String,
}

/// One prepared and evaluated Action occurrence used only for reporting.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvaluatedActionRecord {
    /// Exact resolved occurrence semantics.
    pub occurrence: EffectiveActionOccurrence,
    /// Exact raw Evidence digest and byte count.
    pub evidence_digest: String,
    pub evidence_byte_count: u64,
    /// Test-visible representation and admitted media type.
    pub evidence_representation: String,
    pub evidence_media_type: String,
    /// Optional selected schema observation.
    pub schema: Option<EvidenceSchemaObservation>,
    /// Exact Action-owned Test bytes.
    pub test_content_digest: String,
    pub test_byte_count: u64,
    /// Exact Action-owned helper modules.
    pub modules: Vec<PreparedTestModule>,
    /// Governed Evaluator observation.
    pub evaluation: ActionEvaluationObservation,
}

/// Formats one Kernel UTC timestamp as canonical whole-second RFC 3339 UTC.
pub fn rfc3339_utc(milliseconds: u64) -> String {
    let seconds = (milliseconds / 1_000) as i64;
    let days = seconds.div_euclid(86_400);
    let day_seconds = seconds.rem_euclid(86_400);
    let (year, month, day) = civil_from_days(days);
    let hour = day_seconds / 3_600;
    let minute = (day_seconds % 3_600) / 60;
    let second = day_seconds % 60;
    format!("{year:04}-{month:02}-{day:02}T{hour:02}:{minute:02}:{second:02}Z")
}

fn civil_from_days(days: i64) -> (i64, i64, i64) {
    let z = days + 719_468;
    let era = if z >= 0 { z } else { z - 146_096 } / 146_097;
    let day_of_era = z - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let mut year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let month_prime = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * month_prime + 2) / 5 + 1;
    let month = month_prime + if month_prime < 10 { 3 } else { -9 };
    year += i64::from(month <= 2);
    (year, month, day)
}

/// Constructs the complete Product-owned Verification Report without I/O or
/// serialization.
pub fn construct_verification_report(
    current: &CurrentVerification,
    report_id: impl Into<String>,
    generated_at: impl Into<String>,
    engine_build_digest: &str,
    summary: VerificationSummary,
    evaluated: &[EvaluatedActionRecord],
) -> Result<ControlledValue, Error> {
    if !summary.is_complete() || evaluated.len() as u64 != summary.action_count {
        return Err(internal(
            "Report summary does not cover every Action occurrence",
        ));
    }
    let graph = current
        .effective_graph()
        .ok_or_else(|| internal("current Verification has no effective graph"))?;
    let first = evaluated
        .first()
        .and_then(|record| record.occurrence.detail())
        .ok_or_else(|| internal("Verification Report requires one resolved Action"))?;

    let mut diagnostics = Vec::new();
    let mut activities: Vec<ControlledValue> = Vec::new();
    for record in evaluated {
        let detail = record
            .occurrence
            .detail()
            .ok_or_else(|| internal("evaluated occurrence omitted detail"))?;
        if let Some(diagnostic) = record
            .evaluation
            .response
            .get("diagnostic")
            .filter(|value| **value != ControlledValue::Null)
        {
            diagnostics.push(object([
                ("action", string(record.occurrence.selector().value())),
                (
                    "code",
                    required(diagnostic, "code", "Evaluator diagnostic code")?,
                ),
                (
                    "phase",
                    required(diagnostic, "phase", "Evaluator diagnostic phase")?,
                ),
                (
                    "reason_template",
                    required(
                        diagnostic,
                        "reason_template",
                        "Evaluator diagnostic reason template",
                    )?,
                ),
            ]));
        }
        let owner = definition_owner(&detail.action_owner);
        let evaluations = detail
            .evaluations
            .iter()
            .enumerate()
            .map(|(index, evaluation)| {
                let mut value =
                    match evaluation.wire().cloned().ok_or_else(|| {
                        internal("effective Evaluation omitted its wire projection")
                    })? {
                        ControlledValue::Object(value) => value,
                        _ => return Err(internal("effective Evaluation wire is not an object")),
                    };
                value.insert(
                    "criterion_origin".to_string(),
                    detail
                        .criterion_origins
                        .get(index)
                        .cloned()
                        .ok_or_else(|| internal("criterion origin is absent"))?,
                );
                Ok(ControlledValue::Object(value))
            })
            .collect::<Result<Vec<_>, Error>>()?;

        let mut declaration = BTreeMap::from([
            ("applied_constraints".to_string(), array([])),
            ("file".to_string(), string(&detail.evidence_file)),
            (
                "maximum_bytes".to_string(),
                number(detail.maximum_evidence_bytes),
            ),
            (
                "maximum_bytes_origin".to_string(),
                string(if detail.evidence_maximum_authored {
                    "action-authored"
                } else {
                    "profile-default"
                }),
            ),
            (
                "media_type".to_string(),
                string(&record.evidence_media_type),
            ),
            (
                "media_type_origin".to_string(),
                string(if detail.evidence_media_type.is_some() {
                    "action-authored"
                } else {
                    "registered-file-suffix"
                }),
            ),
            ("name".to_string(), string(&detail.evidence_name)),
            (
                "representation".to_string(),
                string(&record.evidence_representation),
            ),
            (
                "representation_origin".to_string(),
                string(if detail.evidence_schema.is_some() {
                    "schema-presence"
                } else {
                    "schema-absent"
                }),
            ),
        ]);
        let mut evidence_occurrence = BTreeMap::from([
            ("admission".to_string(), string("passed")),
            ("byte_count".to_string(), number(record.evidence_byte_count)),
            (
                "canonical_path".to_string(),
                string(format!("evidence/{}", detail.evidence_input_path)),
            ),
            (
                "content_digest".to_string(),
                string(&record.evidence_digest),
            ),
        ]);
        if let Some(schema) = &record.schema {
            declaration.insert(
                "schema".to_string(),
                object([
                    ("content_digest", string(&schema.content_digest)),
                    ("media_type", string(&schema.media_type)),
                    ("path", string(&schema.file)),
                ]),
            );
            declaration.insert(
                "validation_profile".to_string(),
                object([
                    (
                        "id",
                        string(validation_profile(&record.evidence_media_type)?),
                    ),
                    (
                        "normative_assets_digest",
                        string(FUNCTIONAL_ENGINE_CONTRACT_DIGEST),
                    ),
                ]),
            );
            evidence_occurrence.insert(
                "validation".to_string(),
                object([
                    ("status", string("passed")),
                    (
                        "validator_implementation_digest",
                        string(engine_build_digest),
                    ),
                ]),
            );
        }

        let (evaluator_id, runner_id, runner_profile, runner_digest) =
            match record.evaluation.implementation.contract {
                ActionEvaluationContract::V2 => (
                    "nape-evaluator-v2",
                    "attestify-python-runner-development-v1",
                    RUNNER_PROFILE_V1,
                    RUNNER_PROFILE_V1_DIGEST,
                ),
                ActionEvaluationContract::V3 => (
                    "nape-evaluator-v3",
                    "attestify-python-runner-development-v2",
                    RUNNER_PROFILE_V2,
                    RUNNER_PROFILE_V2_DIGEST,
                ),
            };
        let resolved_modules = record
            .modules
            .iter()
            .map(|module| {
                object([
                    ("byte_count", number(module.source.len() as u64)),
                    ("content_digest", string(&module.content_digest)),
                    ("name", string(&module.name)),
                    ("owner", owner.clone()),
                    ("path", string(&module.file)),
                ])
            })
            .collect::<Vec<_>>();
        let execution = required(
            &record.evaluation.response,
            "execution",
            "Evaluator execution observation",
        )?;
        let result_validation = required(
            &record.evaluation.response,
            "result_validation",
            "Evaluator result validation",
        )?;
        let semantic_owner = required(
            &record.evaluation.response,
            "semantic_owner",
            "Evaluator semantic owner",
        )?;
        let conclusion = match record.evaluation.conclusion {
            crate::value::verification_outcome::VerificationConclusion::True => "true",
            crate::value::verification_outcome::VerificationConclusion::False => "false",
            crate::value::verification_outcome::VerificationConclusion::Inconclusive => {
                "inconclusive"
            }
        };
        let mut action = BTreeMap::from([
            (
                "action".to_string(),
                string(record.occurrence.selector().value()),
            ),
            ("claim".to_string(), string(&detail.claim)),
            ("conclusion".to_string(), string(conclusion)),
            (
                "definition_origin".to_string(),
                definition_origin(&detail.action_owner, record.occurrence.selector().value()),
            ),
            (
                "evaluations".to_string(),
                ControlledValue::Array(evaluations),
            ),
            (
                "evidence".to_string(),
                object([
                    ("declaration", ControlledValue::Object(declaration)),
                    ("occurrence", ControlledValue::Object(evidence_occurrence)),
                ]),
            ),
            (
                "execution".to_string(),
                object([
                    (
                        "automatic_retry_count",
                        required(&execution, "automatic_retry_count", "retry count")?,
                    ),
                    (
                        "evaluate_call_count",
                        required(&execution, "evaluate_call_count", "call count")?,
                    ),
                    ("executed", required(&execution, "executed", "executed")?),
                    ("phase", required(&execution, "phase", "phase")?),
                    ("result_validation", result_validation),
                    ("semantic_owner", semantic_owner),
                    ("status", required(&execution, "status", "status")?),
                ]),
            ),
            (
                "facts".to_string(),
                ControlledValue::Array(record.evaluation.facts.clone()),
            ),
            ("label".to_string(), string(&detail.action_label)),
            ("name".to_string(), string(&detail.action_name)),
            ("reason".to_string(), string(&record.evaluation.reason)),
            (
                "test".to_string(),
                object([
                    (
                        "authored_file",
                        object([
                            (
                                "form",
                                string(if detail.test_file.contains('/') {
                                    "explicit"
                                } else {
                                    "standard"
                                }),
                            ),
                            ("value", string(&detail.test_file)),
                        ]),
                    ),
                    (
                        "implementation",
                        object([
                            ("engine_build_digest", string(engine_build_digest)),
                            (
                                "evaluator",
                                object([
                                    (
                                        "digest",
                                        string(
                                            &record.evaluation.implementation.implementation_digest,
                                        ),
                                    ),
                                    ("id", string(evaluator_id)),
                                ]),
                            ),
                            ("integrity", string("digest-verified")),
                            (
                                "runner",
                                object([
                                    (
                                        "digest",
                                        string(
                                            &record.evaluation.implementation.dependency_set_digest,
                                        ),
                                    ),
                                    ("id", string(runner_id)),
                                ]),
                            ),
                            ("trust", string("not-evaluated")),
                        ]),
                    ),
                    ("name", string(&detail.test_name)),
                    ("owner", owner),
                    (
                        "resolved_file",
                        object([
                            ("byte_count", number(record.test_byte_count)),
                            ("content_digest", string(&record.test_content_digest)),
                            ("path", string(&detail.package_test_path)),
                        ]),
                    ),
                    ("resolved_module", ControlledValue::Array(resolved_modules)),
                    (
                        "resource",
                        object([
                            (
                                "maximum_execution_work_units",
                                number(detail.maximum_execution_work_units),
                            ),
                            (
                                "maximum_execution_work_units_origin",
                                string(if detail.execution_work_authored {
                                    "action-authored"
                                } else {
                                    "profile-default"
                                }),
                            ),
                            ("maximum_result_bytes", number(detail.maximum_result_bytes)),
                            (
                                "maximum_result_bytes_origin",
                                string(if detail.result_bytes_authored {
                                    "action-authored"
                                } else {
                                    "profile-default"
                                }),
                            ),
                        ]),
                    ),
                    (
                        "runner_profile",
                        object([
                            ("id", string(runner_profile)),
                            ("manifestDigest", string(runner_digest)),
                        ]),
                    ),
                ]),
            ),
        ]);
        if let Some(description) = &detail.action_description {
            action.insert("description".to_string(), string(description));
        }
        if let Some(example) = &detail.action_example {
            action.insert("example".to_string(), string(example));
        }

        let new_activity = activities.last().and_then(|value| value.get("name"))
            != Some(&string(&detail.activity_name));
        if new_activity {
            let mut activity = BTreeMap::from([
                ("action".to_string(), ControlledValue::Array(Vec::new())),
                (
                    "definition_origin".to_string(),
                    definition_origin(&detail.activity_owner, &detail.activity_name),
                ),
                ("label".to_string(), string(&detail.activity_label)),
                ("name".to_string(), string(&detail.activity_name)),
            ]);
            if let Some(description) = &detail.activity_description {
                activity.insert("description".to_string(), string(description));
            }
            if let Some(example) = &detail.activity_example {
                activity.insert("example".to_string(), string(example));
            }
            activities.push(ControlledValue::Object(activity));
        }
        match activities.last_mut() {
            Some(ControlledValue::Object(activity)) => match activity.get_mut("action") {
                Some(ControlledValue::Array(actions)) => {
                    actions.push(ControlledValue::Object(action));
                }
                _ => return Err(internal("Report Activity Action array is absent")),
            },
            _ => return Err(internal("Report Activity is absent")),
        }
    }

    let mut metadata = current
        .metadata()
        .values()
        .iter()
        .map(|(key, value)| (key.clone(), string(value)))
        .collect::<BTreeMap<_, _>>();
    metadata.insert("id".to_string(), string(report_id.into()));
    metadata.insert("generated_at".to_string(), string(generated_at.into()));
    metadata.insert(
        "utc-start".to_string(),
        string(current.utc_start_milliseconds().to_string()),
    );
    let root = current.package_closure().root().identity();
    Ok(object([
        ("activity", ControlledValue::Array(activities)),
        ("apiVersion", string("2.0.0")),
        (
            "invocation",
            object([("id", string(current.invocation_id().value()))]),
        ),
        ("kind", string("VerificationReport")),
        ("metadata", ControlledValue::Object(metadata)),
        (
            "procedure",
            object([
                (
                    "acquisition",
                    object([
                        ("binding", string(current.acquisition().binding())),
                        ("integrity", string("digest-verified")),
                        ("source", string(current.acquisition().source())),
                        ("trust", string("not-evaluated")),
                    ]),
                ),
                ("kind", string("VerificationProcedure")),
                ("label", string(&first.procedure_label)),
                ("manifestDigest", string(root.manifest_digest().value())),
                ("name", string(&first.procedure_name)),
                ("package", string(root.purl().value())),
            ]),
        ),
        (
            "processing_detail",
            object([("diagnostics", ControlledValue::Array(diagnostics))]),
        ),
        ("subject", subject_value(current)),
        ("summary", summary_value(&summary, graph.activity_count)),
    ]))
}

/// Constructs the exact Report-to-Evidence-Set relationship.
pub fn construct_evidence_set_relationship(
    current: &CurrentVerification,
    report_id: &str,
    evaluated: &[EvaluatedActionRecord],
) -> Result<ControlledValue, Error> {
    let occurrence = evaluated
        .iter()
        .map(|record| {
            let detail = record
                .occurrence
                .detail()
                .ok_or_else(|| internal("evaluated occurrence omitted detail"))?;
            Ok(object([
                ("action", string(record.occurrence.selector().value())),
                ("byte_count", number(record.evidence_byte_count)),
                (
                    "canonical_path",
                    string(format!("evidence/{}", detail.evidence_input_path)),
                ),
                ("content_digest", string(&record.evidence_digest)),
            ]))
        })
        .collect::<Result<Vec<_>, Error>>()?;
    let mut payloads = BTreeMap::<String, u64>::new();
    for record in evaluated {
        payloads
            .entry(record.evidence_digest.clone())
            .or_insert(record.evidence_byte_count);
    }
    let payload = payloads
        .into_iter()
        .map(|(digest, byte_count)| {
            object([
                ("byte_count", number(byte_count)),
                ("content_digest", string(digest)),
            ])
        })
        .collect::<Vec<_>>();
    Ok(object([
        ("apiVersion", string("2.0.0")),
        ("kind", string("VerificationEvidenceSetRelationship")),
        ("occurrence", ControlledValue::Array(occurrence)),
        ("payload", ControlledValue::Array(payload)),
        (
            "report",
            object([
                ("kind", string("VerificationReport")),
                ("metadata", object([("id", string(report_id))])),
            ]),
        ),
        (
            "subject",
            object([("arn", string(current.subject().arn()))]),
        ),
    ]))
}

/// Backward-compatible narrow envelope helper retained for focused Domain
/// unit tests; production Verify uses [`construct_verification_report`].
pub fn construct_report_envelope(
    current: &CurrentVerification,
    report_id: impl Into<String>,
    generated_at: impl Into<String>,
    summary: VerificationSummary,
) -> Result<ControlledValue, Error> {
    if !summary.is_complete() {
        return Err(internal(
            "Report summary does not cover every Action occurrence",
        ));
    }
    let mut metadata = current
        .metadata()
        .values()
        .iter()
        .map(|(key, value)| (key.clone(), string(value)))
        .collect::<BTreeMap<_, _>>();
    metadata.insert("id".to_string(), string(report_id.into()));
    metadata.insert("generated_at".to_string(), string(generated_at.into()));
    metadata.insert(
        "utc-start".to_string(),
        string(current.utc_start_milliseconds().to_string()),
    );
    Ok(object([
        ("apiVersion", string("2.0.0")),
        ("kind", string("VerificationReport")),
        ("metadata", ControlledValue::Object(metadata)),
    ]))
}

fn summary_value(summary: &VerificationSummary, activity_count: u64) -> ControlledValue {
    object([
        ("action_count", number(summary.action_count)),
        ("actions_blocked", number(summary.actions_blocked)),
        ("actions_completed", number(summary.actions_completed)),
        ("actions_terminated", number(summary.actions_terminated)),
        ("activity_count", number(activity_count)),
        ("conclusion_false", number(summary.conclusion_false)),
        (
            "conclusion_inconclusive",
            number(summary.conclusion_inconclusive),
        ),
        ("conclusion_true", number(summary.conclusion_true)),
        ("diagnostic_count", number(summary.diagnostic_count)),
    ])
}

fn definition_owner(owner: &crate::value::effective_graph::DefinitionOwner) -> ControlledValue {
    object([
        ("kind", string(&owner.kind)),
        (
            "manifestDigest",
            string(owner.package.manifest_digest().value()),
        ),
        ("name", string(&owner.name)),
        ("package", string(owner.package.purl().value())),
    ])
}

fn definition_origin(
    owner: &crate::value::effective_graph::DefinitionOwner,
    selector: &str,
) -> ControlledValue {
    match owner.origin_type {
        crate::value::effective_graph::DefinitionOriginType::Package => object([
            ("kind", string(&owner.kind)),
            (
                "manifestDigest",
                string(owner.package.manifest_digest().value()),
            ),
            ("name", string(&owner.name)),
            ("package", string(owner.package.purl().value())),
            ("selector", string(selector)),
            ("type", string("package")),
        ]),
        crate::value::effective_graph::DefinitionOriginType::Embedded => object([
            ("owner", definition_owner(owner)),
            ("selector", string(selector)),
            ("type", string("embedded")),
        ]),
    }
}

fn subject_value(current: &CurrentVerification) -> ControlledValue {
    let mut value = BTreeMap::from([("arn".to_string(), string(current.subject().arn()))]);
    if let Some(label) = current.subject().label() {
        value.insert("label".to_string(), string(label));
    }
    if let Some(description) = current.subject().description() {
        value.insert("description".to_string(), string(description));
    }
    ControlledValue::Object(value)
}

fn validation_profile(media_type: &str) -> Result<&'static str, Error> {
    match media_type {
        "application/json" => Ok("attestify-json-evidence-schema-v1"),
        "application/yaml" => Ok("attestify-yaml-evidence-schema-v1"),
        "application/toml" => Ok("attestify-toml-evidence-schema-v1"),
        _ => Err(internal("structured Evidence media type is unsupported")),
    }
}

fn required(value: &ControlledValue, name: &str, detail: &str) -> Result<ControlledValue, Error> {
    value.get(name).cloned().ok_or_else(|| internal(detail))
}

fn object<const N: usize>(values: [(&str, ControlledValue); N]) -> ControlledValue {
    ControlledValue::Object(
        values
            .into_iter()
            .map(|(name, value)| (name.to_string(), value))
            .collect(),
    )
}

fn array<const N: usize>(values: [ControlledValue; N]) -> ControlledValue {
    ControlledValue::Array(Vec::from(values))
}

fn string(value: impl Into<String>) -> ControlledValue {
    ControlledValue::String(value.into())
}

fn number(value: u64) -> ControlledValue {
    ControlledValue::Number(value.to_string())
}

fn internal(detail: impl Into<String>) -> Error {
    Error::for_system(Kind::ProcessingFailure, detail)
}
