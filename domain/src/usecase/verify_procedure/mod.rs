//! Execute the complete already-frozen current Verification run.

#[cfg(test)]
mod tests;

use std::{
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

use kernel_oss::{
    error::{Error, Kind},
    gateway::{
        current_utc_timestamp::CurrentUTCTimestampGW, new_identity::NewIdentityGW, Gateway,
        VoidGateway,
    },
    response::ResponseFuture,
    usecase::AsyncUseCase,
};

use crate::{
    diagnostic::{NapeDiagnostic, NapeOutcome},
    gateway::{
        action_evaluation::{
            ActionEvaluationContract, ActionEvaluationGW, ActionEvaluationRequest,
            PreparedTestModule,
        },
        controlled_document_projection::{
            ControlledDocumentProjectionGW, ControlledDocumentProjectionRequest,
            ControlledMediaProfile,
        },
        current_verification_acquisition::{
            CurrentVerificationAcquisitionGW, CurrentVerificationAcquisitionRequest,
        },
        restricted_schema_validation::{
            RestrictedSchemaProfile, RestrictedSchemaValidationGW,
            RestrictedSchemaValidationRequest,
        },
        verification_outcome_commit::{
            VerificationOutcomeCommitGW, VerificationOutcomeCommitObservation,
            VerificationOutcomeCommitRequest,
        },
    },
    service::{
        evidence_admission::evidence_media_type,
        report_construction::{
            construct_evidence_set_relationship, construct_verification_report, rfc3339_utc,
            EvaluatedActionRecord, EvidenceSchemaObservation,
        },
        test_preflight::{
            preflight_module_closure, preflight_test, EffectiveTestResources, PreflightTestModule,
        },
    },
    value::{
        controlled_value::ControlledValue,
        effective_graph::EffectiveActionOccurrence,
        evidence::EvidenceArgument,
        package::{PackageReleaseIdentity, VerifiedPackage},
        verification_outcome::{VerificationConclusion, VerificationSummary},
    },
};

const MAX_TEST_BYTES: u64 = 33_554_432;

/// Closed zero-field current-run Verify request.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct VerifyProcedureRequest;

/// Completed current-run Verification meaning.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CommittedVerificationOutcome {
    /// Invocation ULID established by Start.
    pub invocation_id: String,
    /// Fresh VerificationReport ULID.
    pub report_id: String,
    /// Exact Procedure package release.
    pub procedure: PackageReleaseIdentity,
    /// Exact acquisition source vocabulary.
    pub acquisition: String,
    /// Complete Report aggregate.
    pub summary: VerificationSummary,
    /// Number of unique evidence payloads.
    pub unique_evidence_payload_count: u64,
    /// Atomic local result commitment.
    pub commitment: VerificationOutcomeCommitObservation,
}

/// Bounded Verify outcome.
pub type VerifyProcedureOutcome = NapeOutcome<CommittedVerificationOutcome>;

/// Public marker seam for argument-free current-run Verify.
pub trait VerifyProcedureUC:
    AsyncUseCase<Request = VerifyProcedureRequest, Response = VerifyProcedureOutcome>
{
}

/// Core current-run execution orchestration.
pub struct VerifyProcedure {
    current: Arc<dyn CurrentVerificationAcquisitionGW>,
    evaluation: Arc<dyn ActionEvaluationGW>,
    projection: Option<Arc<dyn ControlledDocumentProjectionGW>>,
    schema: Option<Arc<dyn RestrictedSchemaValidationGW>>,
    commit: Arc<dyn VerificationOutcomeCommitGW>,
    new_identity: Arc<dyn NewIdentityGW>,
    current_time: Arc<dyn CurrentUTCTimestampGW>,
    engine_build_digest: String,
}

impl VerifyProcedure {
    /// Creates the use case with exact state, Evaluator, identity, time, and
    /// commit capabilities. This narrow constructor supports opaque fixtures.
    pub fn new(
        current: Arc<dyn CurrentVerificationAcquisitionGW>,
        evaluation: Arc<dyn ActionEvaluationGW>,
        commit: Arc<dyn VerificationOutcomeCommitGW>,
        new_identity: Arc<dyn NewIdentityGW>,
        current_time: Arc<dyn CurrentUTCTimestampGW>,
    ) -> Self {
        Self {
            current,
            evaluation,
            projection: None,
            schema: None,
            commit,
            new_identity,
            current_time,
            engine_build_digest: "sha256:fixture-engine-build".to_string(),
        }
    }

    /// Supplies the controlled Evidence gateways and exact NAPE build digest
    /// required by production Verification V2.
    pub fn with_controlled_evidence(
        mut self,
        projection: Arc<dyn ControlledDocumentProjectionGW>,
        schema: Arc<dyn RestrictedSchemaValidationGW>,
        engine_build_digest: impl Into<String>,
    ) -> Self {
        self.projection = Some(projection);
        self.schema = Some(schema);
        self.engine_build_digest = engine_build_digest.into();
        self
    }
}

impl AsyncUseCase for VerifyProcedure {
    type Request = VerifyProcedureRequest;
    type Response = VerifyProcedureOutcome;

    fn execute<'a>(&'a self, _request: Self::Request) -> ResponseFuture<'a, Self::Response> {
        Box::pin(async move {
            let current = match Gateway::execute(
                self.current.as_ref() as &dyn CurrentVerificationAcquisitionGW,
                CurrentVerificationAcquisitionRequest,
            )? {
                NapeOutcome::Completed(value) => value,
                NapeOutcome::Rejected(value) => return Ok(NapeOutcome::Rejected(value)),
            };
            let graph = current.effective_graph().ok_or_else(|| {
                Error::for_system(
                    Kind::ProcessingFailure,
                    "current Verification omitted its effective graph",
                )
            })?;
            for occurrence in &graph.occurrences {
                if !current
                    .evidence_associations()
                    .iter()
                    .any(|association| association.action == *occurrence.selector())
                {
                    return Ok(NapeOutcome::Rejected(NapeDiagnostic::try_new(
                        "evidence_association_invalid",
                        "evidence-association-invalid-v1",
                        "evidence-admission",
                        "required evidence has not been collected for every Action occurrence",
                    )?));
                }
            }

            let contract = if graph.occurrences.iter().any(requires_evaluator_v3) {
                ActionEvaluationContract::V3
            } else {
                ActionEvaluationContract::V2
            };
            let prepared = self.prepare_all(&current, contract)?;
            let mut summary = VerificationSummary {
                activity_count: graph.activity_count,
                action_count: graph.occurrences.len() as u64,
                ..VerificationSummary::default()
            };
            let mut evaluated = Vec::with_capacity(prepared.len());
            for prepared in prepared {
                let observation = match Gateway::execute(
                    self.evaluation.as_ref() as &dyn ActionEvaluationGW,
                    prepared.request.clone(),
                )? {
                    NapeOutcome::Completed(value) => value,
                    NapeOutcome::Rejected(value) => return Ok(NapeOutcome::Rejected(value)),
                };
                match observation.conclusion {
                    VerificationConclusion::True => summary.conclusion_true += 1,
                    VerificationConclusion::False => summary.conclusion_false += 1,
                    VerificationConclusion::Inconclusive => summary.conclusion_inconclusive += 1,
                }
                let status = observation
                    .response
                    .get("execution")
                    .and_then(|value| value.get("status"));
                match status {
                    Some(ControlledValue::String(value)) if value == "completed" => {
                        summary.actions_completed += 1;
                    }
                    Some(ControlledValue::String(value)) if value == "terminated" => {
                        summary.actions_terminated += 1;
                    }
                    Some(ControlledValue::String(value)) if value == "blocked" => {
                        summary.actions_blocked += 1;
                    }
                    _ => {
                        return Err(Error::for_system(
                            Kind::ProcessingFailure,
                            "Evaluator execution status escaped its closed vocabulary",
                        ));
                    }
                }
                if observation
                    .response
                    .get("diagnostic")
                    .is_some_and(|value| value != &ControlledValue::Null)
                {
                    summary.diagnostic_count += 1;
                }
                evaluated.push(EvaluatedActionRecord {
                    occurrence: prepared.occurrence,
                    evidence_digest: prepared.evidence_digest,
                    evidence_byte_count: prepared.evidence_byte_count,
                    evidence_representation: prepared.evidence_representation,
                    evidence_media_type: prepared.evidence_media_type,
                    schema: prepared.schema,
                    test_content_digest: prepared.test_content_digest,
                    test_byte_count: prepared.test_byte_count,
                    modules: prepared.request.modules,
                    evaluation: observation,
                });
            }
            let report_id =
                VoidGateway::execute(self.new_identity.as_ref() as &dyn NewIdentityGW)?.to_string();
            let generated_at = rfc3339_utc(
                VoidGateway::execute(self.current_time.as_ref() as &dyn CurrentUTCTimestampGW)?
                    .as_milli(),
            );
            let report = construct_verification_report(
                &current,
                report_id.clone(),
                generated_at,
                &self.engine_build_digest,
                summary.clone(),
                &evaluated,
            )?;
            let relationship =
                construct_evidence_set_relationship(&current, &report_id, &evaluated)?;
            let output = current.result_output().cloned().ok_or_else(|| {
                Error::for_system(
                    Kind::ProcessingFailure,
                    "current run has no managed result output",
                )
            })?;
            let commitment = match Gateway::execute(
                self.commit.as_ref() as &dyn VerificationOutcomeCommitGW,
                VerificationOutcomeCommitRequest {
                    report,
                    evidence_set_relationship: relationship,
                    evidence_payloads: current.evidence_payloads().clone(),
                    output,
                },
            )? {
                NapeOutcome::Completed(value) => value,
                NapeOutcome::Rejected(value) => return Ok(NapeOutcome::Rejected(value)),
            };
            Ok(NapeOutcome::Completed(CommittedVerificationOutcome {
                invocation_id: current.invocation_id().value().to_string(),
                report_id,
                procedure: current.package_closure().root().identity().clone(),
                acquisition: current.acquisition().source().to_string(),
                summary,
                unique_evidence_payload_count: current.evidence_payloads().len() as u64,
                commitment,
            }))
        })
    }
}

impl VerifyProcedure {
    fn prepare_all(
        &self,
        current: &crate::value::current_verification::CurrentVerification,
        contract: ActionEvaluationContract,
    ) -> Result<Vec<PreparedAction>, Error> {
        let graph = current
            .effective_graph()
            .ok_or_else(|| internal("effective graph is absent"))?;
        let mut unique = BTreeSet::new();
        let mut aggregate = 0u64;
        let mut prepared = Vec::with_capacity(graph.occurrences.len());
        for occurrence in &graph.occurrences {
            let detail = occurrence
                .detail()
                .ok_or_else(|| internal("effective occurrence omitted detail"))?;
            let owner = package_by_identity(current.package_closure(), occurrence.package_owner())?;
            let test_source = owner
                .files()
                .get(&detail.package_test_path)
                .cloned()
                .ok_or_else(|| invalid("Action-owned Test file is absent"))?;
            if test_source.len() as u64 > MAX_TEST_BYTES {
                return Err(invalid("Action-owned Test exceeds the profile ceiling"));
            }
            preflight_test(
                &test_source,
                EffectiveTestResources {
                    maximum_execution_work_units: detail.maximum_execution_work_units,
                    maximum_result_bytes: detail.maximum_result_bytes,
                },
            )?;
            let modules = detail
                .test_modules
                .iter()
                .map(|module| {
                    let source =
                        owner.files().get(&module.file).cloned().ok_or_else(|| {
                            invalid("declared Action-owned helper module is absent")
                        })?;
                    if source.len() as u64 > MAX_TEST_BYTES {
                        return Err(invalid("Action-owned helper exceeds the profile ceiling"));
                    }
                    Ok(PreparedTestModule {
                        name: module.name.clone(),
                        file: module.file.clone(),
                        content_digest: owner
                            .file_digest(&module.file)
                            .ok_or_else(|| internal("helper module digest is absent"))?
                            .to_string(),
                        source,
                    })
                })
                .collect::<Result<Vec<_>, Error>>()?;
            preflight_module_closure(
                &test_source,
                &modules
                    .iter()
                    .map(|module| PreflightTestModule {
                        name: module.name.clone(),
                        source: module.source.clone(),
                    })
                    .collect::<Vec<_>>(),
            )?;
            let association = current
                .evidence_associations()
                .iter()
                .find(|association| association.action == *occurrence.selector())
                .ok_or_else(|| internal("evidence readiness changed after preflight"))?;
            let evidence = current
                .evidence_payloads()
                .get(&association.digest)
                .cloned()
                .ok_or_else(|| internal("Evidence association has no immutable payload"))?;
            if unique.insert(association.digest.clone()) {
                aggregate = aggregate
                    .checked_add(evidence.len() as u64)
                    .ok_or_else(|| invalid("Evidence aggregate byte count overflowed"))?;
                if aggregate > graph.maximum_invocation_evidence_bytes {
                    return Err(invalid(if graph.invocation_evidence_limit_authored {
                        "unique Evidence exceeds the authored Procedure aggregate limit"
                    } else {
                        "unique Evidence exceeds the Profile aggregate ceiling"
                    }));
                }
            }
            let media_type = detail
                .evidence_media_type
                .clone()
                .or_else(|| evidence_media_type(&detail.evidence_file).map(str::to_string))
                .ok_or_else(|| invalid("Evidence media type is neither authored nor registered"))?;
            let (argument, representation, schema) =
                if let Some(selection) = &detail.evidence_schema {
                    let projection = self.projection.as_ref().ok_or_else(|| {
                        internal("controlled Evidence projection gateway is unavailable")
                    })?;
                    let schema_gateway = self.schema.as_ref().ok_or_else(|| {
                        internal("restricted schema validation gateway is unavailable")
                    })?;
                    let profile = match media_type.as_str() {
                        "application/json" => ControlledMediaProfile::Json,
                        "application/yaml" => ControlledMediaProfile::Yaml,
                        "application/toml" => ControlledMediaProfile::Toml,
                        _ => {
                            return Err(invalid(
                                "schema-selected Evidence media type is unsupported",
                            ))
                        }
                    };
                    let instance = match Gateway::execute(
                        projection.as_ref() as &dyn ControlledDocumentProjectionGW,
                        ControlledDocumentProjectionRequest::try_new(
                            profile,
                            evidence.clone(),
                            detail.maximum_evidence_bytes,
                        )?,
                    )? {
                        NapeOutcome::Completed(value) => value,
                        NapeOutcome::Rejected(value) => return Err(diagnostic_error(value)),
                    };
                    let schema_bytes =
                        owner.files().get(&selection.file).cloned().ok_or_else(|| {
                            invalid("selected Evidence schema is absent from its definition owner")
                        })?;
                    let schema_value = match Gateway::execute(
                        projection.as_ref() as &dyn ControlledDocumentProjectionGW,
                        ControlledDocumentProjectionRequest::try_new(
                            ControlledMediaProfile::Json,
                            schema_bytes.clone(),
                            MAX_TEST_BYTES,
                        )?,
                    )? {
                        NapeOutcome::Completed(value) => value,
                        NapeOutcome::Rejected(value) => return Err(diagnostic_error(value)),
                    };
                    match Gateway::execute(
                        schema_gateway.as_ref() as &dyn RestrictedSchemaValidationGW,
                        RestrictedSchemaValidationRequest::new(
                            RestrictedSchemaProfile::AttestifyJsonSchema202012Profile1,
                            schema_value,
                            instance.clone(),
                        ),
                    )? {
                        NapeOutcome::Completed(_) => {}
                        NapeOutcome::Rejected(value) => return Err(diagnostic_error(value)),
                    }
                    (
                        EvidenceArgument::Controlled(instance),
                        "validated-structured".to_string(),
                        Some(EvidenceSchemaObservation {
                            content_digest: owner
                                .file_digest(&selection.file)
                                .ok_or_else(|| internal("Evidence schema digest is absent"))?
                                .to_string(),
                            media_type: selection.media_type.clone(),
                            file: selection.file.clone(),
                        }),
                    )
                } else {
                    (
                        EvidenceArgument::Opaque(evidence.clone()),
                        "opaque".to_string(),
                        None,
                    )
                };
            let metadata = ControlledValue::Object(BTreeMap::from([
                (
                    "evidence_media_type".to_string(),
                    ControlledValue::String(media_type.clone()),
                ),
                (
                    "evidence_representation".to_string(),
                    ControlledValue::String(representation.clone()),
                ),
            ]));
            prepared.push(PreparedAction {
                occurrence: occurrence.clone(),
                evidence_digest: association.digest.value().to_string(),
                evidence_byte_count: evidence.len() as u64,
                evidence_representation: representation,
                evidence_media_type: media_type.clone(),
                schema,
                test_content_digest: owner
                    .file_digest(&detail.package_test_path)
                    .ok_or_else(|| internal("Action-owned Test digest is absent"))?
                    .to_string(),
                test_byte_count: test_source.len() as u64,
                request: ActionEvaluationRequest {
                    action: occurrence.selector().clone(),
                    evidence: argument,
                    evidence_file: detail.evidence_file.clone(),
                    evidence_media_type: media_type,
                    evaluations: detail.evaluations.clone(),
                    metadata,
                    test_file: detail.test_file.clone(),
                    test_source,
                    contract,
                    modules,
                    maximum_execution_work_units: detail.maximum_execution_work_units,
                    maximum_result_bytes: detail.maximum_result_bytes,
                },
            });
        }
        Ok(prepared)
    }
}

impl VerifyProcedureUC for VerifyProcedure {}

#[derive(Clone)]
struct PreparedAction {
    occurrence: EffectiveActionOccurrence,
    evidence_digest: String,
    evidence_byte_count: u64,
    evidence_representation: String,
    evidence_media_type: String,
    schema: Option<EvidenceSchemaObservation>,
    test_content_digest: String,
    test_byte_count: u64,
    request: ActionEvaluationRequest,
}

fn requires_evaluator_v3(occurrence: &EffectiveActionOccurrence) -> bool {
    occurrence.detail().is_some_and(|detail| {
        detail.evidence_schema.is_some()
            || !detail.evaluations.is_empty()
            || !detail.test_modules.is_empty()
            || detail.execution_work_authored
            || detail.result_bytes_authored
    })
}

fn package_by_identity<'a>(
    closure: &'a crate::value::package::VerifiedPackageClosure,
    identity: &PackageReleaseIdentity,
) -> Result<&'a VerifiedPackage, Error> {
    std::iter::once(closure.root())
        .chain(closure.dependencies().iter())
        .find(|package| package.identity() == identity)
        .ok_or_else(|| invalid("Action definition owner is absent from the verified closure"))
}

fn diagnostic_error(value: NapeDiagnostic) -> Error {
    Error::for_user(
        Kind::InvalidInput,
        format!("{}: {}", value.code(), value.detail()),
    )
}

fn invalid(detail: impl Into<String>) -> Error {
    Error::for_user(Kind::InvalidInput, detail)
}

fn internal(detail: impl Into<String>) -> Error {
    Error::for_system(Kind::ProcessingFailure, detail)
}
