//! Deterministic effective Procedure occurrence graph resolution.

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};

use kernel_oss::error::{Error, Kind};

use crate::{
    service::definition_admission::{
        self, ActionSpec, ActionUse, ActivitySpec, ActivityUse, DefinitionSpec,
        EvaluationAssignment, EvaluationDefinition, FunctionalDefinition, ProcedureSpec,
    },
    value::{
        controlled_value::ControlledValue,
        effective_graph::{
            CanonicalActionSelector, DefinitionOriginType, DefinitionOwner, EffectiveActionDetail,
            EffectiveActionOccurrence, EffectiveEvidenceSchema, EffectiveTestModule,
            EffectiveVerificationGraph,
        },
        evaluation::EffectiveEvaluation,
        package::{PackageReleaseIdentity, VerifiedPackage, VerifiedPackageClosure},
    },
};

const MAX_EVIDENCE_BYTES: u64 = 268_435_456;
const MAX_INVOCATION_EVIDENCE_BYTES: u64 = 2_147_483_648;
const MAXIMUM_EXECUTION_WORK_UNITS: u64 = 25_000_000;
const MAXIMUM_RESULT_BYTES: u64 = 131_072;

/// Admits a complete ordered occurrence graph under the profile ceiling.
pub fn admit_effective_graph(
    occurrences: Vec<EffectiveActionOccurrence>,
) -> Result<Vec<EffectiveActionOccurrence>, Error> {
    if occurrences.is_empty() || occurrences.len() > 4_096 {
        return Err(invalid(
            "effective graph must contain 1..=4096 Action occurrences",
        ));
    }
    let mut selectors = BTreeSet::new();
    for occurrence in &occurrences {
        if !selectors.insert(occurrence.selector().value()) {
            return Err(invalid(
                "effective graph contains a duplicate contextual Action selector",
            ));
        }
    }
    Ok(occurrences)
}

/// Resolves the exact already-verified package closure into one executable
/// Procedure occurrence graph. This service performs no acquisition or I/O.
pub fn resolve_effective_verification_graph(
    closure: &VerifiedPackageClosure,
) -> Result<EffectiveVerificationGraph, Error> {
    let root_definition = functional_definition(closure.root())?;
    let DefinitionSpec::Procedure(procedure) = &root_definition.document.spec else {
        return Err(invalid(
            "the selected root package is not a Verification Procedure",
        ));
    };
    let procedure_owner = owner(
        closure.root(),
        "VerificationProcedure",
        &procedure.name,
        DefinitionOriginType::Package,
    );
    let mut occurrences = Vec::new();
    for activity_use in &procedure.activity {
        match activity_use {
            ActivityUse::Embedded(activity) => expand_activity(
                closure,
                procedure,
                &activity.name,
                activity,
                closure.root(),
                DefinitionOwner {
                    origin_type: DefinitionOriginType::Embedded,
                    ..procedure_owner.clone()
                },
                &mut occurrences,
            )?,
            ActivityUse::Referenced(reference) => {
                admit_reference_edge(
                    closure,
                    closure.root().identity(),
                    &reference.package,
                    &reference.name,
                )?;
                let package = dependency(closure, &reference.package)?;
                let definition = functional_definition(package)?;
                let DefinitionSpec::Activity(activity) = &definition.document.spec else {
                    return Err(invalid(
                        "referenced Activity package has the wrong semantic kind",
                    ));
                };
                expand_activity(
                    closure,
                    procedure,
                    &reference.name,
                    activity,
                    package,
                    owner(
                        package,
                        "VerificationActivity",
                        &activity.name,
                        DefinitionOriginType::Package,
                    ),
                    &mut occurrences,
                )?;
            }
        }
    }
    let occurrences = admit_effective_graph(occurrences)?;
    let activity_count = occurrences
        .iter()
        .filter_map(|value| value.detail().map(|detail| detail.activity_name.as_str()))
        .collect::<BTreeSet<_>>()
        .len() as u64;
    Ok(EffectiveVerificationGraph {
        occurrences,
        maximum_invocation_evidence_bytes: procedure
            .evidence_limit
            .as_ref()
            .map_or(MAX_INVOCATION_EVIDENCE_BYTES, |limit| limit.maximum_bytes),
        invocation_evidence_limit_authored: procedure.evidence_limit.is_some(),
        activity_count,
    })
}

#[allow(clippy::too_many_arguments)]
fn expand_activity(
    closure: &VerifiedPackageClosure,
    procedure: &ProcedureSpec,
    activity_alias: &str,
    activity: &ActivitySpec,
    activity_package: &VerifiedPackage,
    activity_owner: DefinitionOwner,
    output: &mut Vec<EffectiveActionOccurrence>,
) -> Result<(), Error> {
    for action_use in &activity.action {
        match action_use {
            ActionUse::Embedded(action) => {
                let static_base = if activity_package.kind()
                    == crate::value::definition::DefinitionKind::VerificationProcedure
                {
                    format!("activity/{activity_alias}/{}", action.name)
                } else {
                    format!("activity/{}/{}", activity.name, action.name)
                };
                let action_owner = DefinitionOwner {
                    origin_type: DefinitionOriginType::Embedded,
                    ..activity_owner.clone()
                };
                output.push(resolve_occurrence(
                    procedure,
                    activity_alias,
                    activity,
                    &action.name,
                    action,
                    &[],
                    action_owner,
                    activity_owner.clone(),
                    static_base,
                )?);
            }
            ActionUse::Referenced(reference) => {
                let selector = format!("{activity_alias}.{}", reference.name);
                admit_reference_edge(
                    closure,
                    activity_package.identity(),
                    &reference.package,
                    &selector,
                )?;
                let package = dependency(closure, &reference.package)?;
                let definition = functional_definition(package)?;
                let DefinitionSpec::Action(action) = &definition.document.spec else {
                    return Err(invalid(
                        "referenced Action package has the wrong semantic kind",
                    ));
                };
                output.push(resolve_occurrence(
                    procedure,
                    activity_alias,
                    activity,
                    &reference.name,
                    action,
                    &reference.evaluations,
                    owner(
                        package,
                        "VerificationAction",
                        &action.name,
                        DefinitionOriginType::Package,
                    ),
                    activity_owner.clone(),
                    format!("action/{}", action.name),
                )?);
            }
        }
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn resolve_occurrence(
    procedure: &ProcedureSpec,
    activity_alias: &str,
    activity: &ActivitySpec,
    action_alias: &str,
    action: &ActionSpec,
    assignments: &[EvaluationAssignment],
    action_owner: DefinitionOwner,
    activity_owner: DefinitionOwner,
    static_base: String,
) -> Result<EffectiveActionOccurrence, Error> {
    let selector = CanonicalActionSelector::try_new(format!("{activity_alias}.{action_alias}"))?;
    let (evaluations, criterion_origins) =
        effective_evaluations(action, assignments, &action_owner, &activity_owner)?;
    let resource = action.test.resource.as_ref();
    let detail = EffectiveActionDetail {
        procedure_name: procedure.name.clone(),
        procedure_label: procedure.label.clone(),
        activity_name: activity_alias.to_string(),
        activity_label: activity.label.clone(),
        activity_description: activity.description.clone(),
        activity_example: activity.example.clone(),
        action_name: action_alias.to_string(),
        action_label: action.label.clone(),
        action_description: action.description.clone(),
        action_example: action.example.clone(),
        claim: action.claim.clone(),
        action_owner: action_owner.clone(),
        activity_owner,
        test_name: action.test.name.clone(),
        test_file: action.test.file.clone(),
        package_test_path: if action.test.file.contains('/') {
            action.test.file.clone()
        } else {
            format!("{static_base}/{}", action.test.file)
        },
        test_modules: action
            .test
            .module
            .iter()
            .map(|module| EffectiveTestModule {
                name: module.name.clone(),
                file: module.file.clone(),
            })
            .collect(),
        evidence_name: action.evidence.name.clone(),
        evidence_file: action.evidence.file.clone(),
        evidence_input_path: if action.evidence.file.contains('/') {
            action.evidence.file.clone()
        } else {
            format!("{activity_alias}/{action_alias}/{}", action.evidence.file)
        },
        evidence_media_type: action.evidence.media_type.clone(),
        evidence_schema: action
            .evidence
            .schema
            .as_ref()
            .map(|schema| EffectiveEvidenceSchema {
                media_type: schema.media_type.clone(),
                file: schema.file.clone(),
            }),
        evaluations,
        criterion_origins,
        maximum_evidence_bytes: action.evidence.maximum_bytes.unwrap_or(MAX_EVIDENCE_BYTES),
        maximum_execution_work_units: resource
            .and_then(|value| value.maximum_execution_work_units)
            .unwrap_or(MAXIMUM_EXECUTION_WORK_UNITS),
        maximum_result_bytes: resource
            .and_then(|value| value.maximum_result_bytes)
            .unwrap_or(MAXIMUM_RESULT_BYTES),
        evidence_maximum_authored: action.evidence.maximum_bytes.is_some(),
        execution_work_authored: resource
            .is_some_and(|value| value.maximum_execution_work_units.is_some()),
        result_bytes_authored: resource.is_some_and(|value| value.maximum_result_bytes.is_some()),
    };
    Ok(EffectiveActionOccurrence::with_detail(
        selector,
        action_owner.package.clone(),
        detail,
    ))
}

fn effective_evaluations(
    action: &ActionSpec,
    assignments: &[EvaluationAssignment],
    default_owner: &DefinitionOwner,
    assignment_owner: &DefinitionOwner,
) -> Result<(Vec<EffectiveEvaluation>, Vec<ControlledValue>), Error> {
    let mut effective = action.evaluations.clone();
    let mut previous_position = None;
    for assignment in assignments {
        let position = effective
            .iter()
            .position(|evaluation| evaluation.name == assignment.name)
            .ok_or_else(|| invalid("assignment names an absent package Evaluation"))?;
        if previous_position.is_some_and(|previous| previous >= position) {
            return Err(invalid(
                "assigned evaluations do not preserve package Evaluation order",
            ));
        }
        previous_position = Some(position);
        let declaration = &mut effective[position];
        for (operator, selected) in &assignment.criteria {
            let default = declaration
                .criteria
                .get(operator)
                .ok_or_else(|| invalid("assignment names an absent criterion"))?;
            if selected == default {
                return Err(invalid(
                    "assignment redundantly repeats the package default",
                ));
            }
            let authorization = declaration
                .criteria_assignment
                .iter()
                .find(|candidate| candidate.name == *operator)
                .ok_or_else(|| invalid("criterion operator is not assignable"))?;
            if !authorization.authorized_values.contains(selected) {
                return Err(invalid("criterion value is not authorized by the Action"));
            }
            declaration
                .criteria
                .insert(operator.clone(), selected.clone());
        }
    }

    let mut values = Vec::with_capacity(effective.len());
    let mut origins = Vec::with_capacity(effective.len());
    for evaluation in effective {
        let origin = criterion_origin(&evaluation, assignments, default_owner, assignment_owner)?;
        let criteria = evaluation.criteria.clone();
        let mut wire = BTreeMap::from([
            (
                "criteria".to_string(),
                ControlledValue::Object(criteria.clone()),
            ),
            (
                "label".to_string(),
                ControlledValue::String(evaluation.label.clone()),
            ),
            (
                "name".to_string(),
                ControlledValue::String(evaluation.name.clone()),
            ),
            (
                "subject".to_string(),
                ControlledValue::Object(BTreeMap::from([
                    (
                        "data_type".to_string(),
                        ControlledValue::String(evaluation.subject.data_type.clone()),
                    ),
                    (
                        "label".to_string(),
                        ControlledValue::String(evaluation.subject.label.clone()),
                    ),
                    (
                        "name".to_string(),
                        ControlledValue::String(evaluation.subject.name.clone()),
                    ),
                ])),
            ),
        ]);
        if let Some(description) = &evaluation.description {
            wire.insert(
                "description".to_string(),
                ControlledValue::String(description.clone()),
            );
        }
        if let Some(example) = &evaluation.example {
            wire.insert(
                "example".to_string(),
                ControlledValue::String(example.clone()),
            );
        }
        values.push(EffectiveEvaluation::try_with_wire(
            evaluation.name,
            criteria,
            ControlledValue::Object(wire),
        )?);
        origins.push(origin);
    }
    Ok((values, origins))
}

fn criterion_origin(
    evaluation: &EvaluationDefinition,
    assignments: &[EvaluationAssignment],
    default_owner: &DefinitionOwner,
    assignment_owner: &DefinitionOwner,
) -> Result<ControlledValue, Error> {
    let mut origins = BTreeMap::new();
    for operator in evaluation.criteria.keys() {
        let assigned = assignments.iter().any(|assignment| {
            assignment.name == evaluation.name && assignment.criteria.contains_key(operator)
        });
        let owner = if assigned {
            assignment_owner
        } else {
            default_owner
        };
        origins.insert(
            operator.clone(),
            ControlledValue::Object(BTreeMap::from([
                (
                    "type".to_string(),
                    ControlledValue::String(
                        if assigned {
                            "parent-assignment"
                        } else {
                            "package-default"
                        }
                        .to_string(),
                    ),
                ),
                (
                    "owner".to_string(),
                    ControlledValue::Object(BTreeMap::from([
                        (
                            "manifestDigest".to_string(),
                            ControlledValue::String(
                                owner.package.manifest_digest().value().to_string(),
                            ),
                        ),
                        (
                            "package".to_string(),
                            ControlledValue::String(owner.package.purl().value().to_string()),
                        ),
                    ])),
                ),
            ])),
        );
    }
    Ok(ControlledValue::Object(origins))
}

fn functional_definition(package: &VerifiedPackage) -> Result<FunctionalDefinition, Error> {
    definition_admission::admit_build_definition(
        package.semantic_root(),
        package.identity().purl().value(),
    )
    .map_err(|error| invalid(error.detail))
}

fn dependency<'a>(
    closure: &'a VerifiedPackageClosure,
    purl: &str,
) -> Result<&'a VerifiedPackage, Error> {
    closure
        .dependencies()
        .iter()
        .find(|candidate| candidate.identity().purl().value() == purl)
        .ok_or_else(|| invalid("referenced package is absent from the exact Lock closure"))
}

fn admit_reference_edge(
    closure: &VerifiedPackageClosure,
    from: &PackageReleaseIdentity,
    to: &str,
    selector: &str,
) -> Result<(), Error> {
    let matches = closure
        .root()
        .lock()
        .edges()
        .iter()
        .filter(|edge| {
            edge.from.value() == from.purl().value()
                && edge.to.value() == to
                && (edge.use_selector == selector
                    || selector
                        .split_once('.')
                        .is_some_and(|(_, action)| edge.use_selector == action))
        })
        .count();
    if matches != 1 {
        return Err(invalid(
            "authored reference does not equal one exact verified Lock edge",
        ));
    }
    Ok(())
}

fn owner(
    package: &VerifiedPackage,
    kind: &str,
    name: &str,
    origin_type: DefinitionOriginType,
) -> DefinitionOwner {
    DefinitionOwner {
        kind: kind.to_string(),
        name: name.to_string(),
        package: package.identity().clone(),
        origin_type,
    }
}

fn invalid(detail: impl Into<String>) -> Error {
    Error::for_user(Kind::InvalidInput, detail)
}
