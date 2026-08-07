//! Additive I6 functional Verification V2 definition admission.
//!
//! The historical I2-I5 entry points in the parent module remain intentionally
//! narrow. This module admits the closed F01-F06 capability set without making
//! those older compatibility profiles silently broader.

use crate::value::{
    controlled_value::ControlledValue,
    definition::{BuildDefinition, BuildReference},
};
use std::collections::{BTreeMap, BTreeSet};
use std::fmt;

pub const RUNNER_PROFILE: &str = "attestify-python-test-development-v2";
const PROCEDURE_API_VERSION: &str = "2.0.0";

/// Product-semantic definition rejection retained until the diagnostic is
/// translated at the Use Case boundary.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationError {
    /// Stable human-readable rejection detail.
    pub detail: String,
}

impl VerificationError {
    fn invalid(detail: impl Into<String>) -> Self {
        Self {
            detail: detail.into(),
        }
    }
}

impl fmt::Display for VerificationError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.detail)
    }
}

impl std::error::Error for VerificationError {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FunctionalDefinition {
    pub build: BuildDefinition,
    pub document: DefinitionDocument,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefinitionDocument {
    pub api_version: String,
    pub kind: String,
    pub spec: DefinitionSpec,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DefinitionSpec {
    Procedure(ProcedureSpec),
    Activity(ActivitySpec),
    Action(Box<ActionSpec>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcedureSpec {
    pub name: String,
    pub label: String,
    pub description: Option<String>,
    pub example: Option<String>,
    pub evidence_limit: Option<ProcedureEvidenceLimit>,
    pub activity: Vec<ActivityUse>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProcedureEvidenceLimit {
    pub maximum_bytes: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActivityUse {
    Referenced(ActivityReference),
    Embedded(ActivitySpec),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivityReference {
    pub name: String,
    pub package: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActivitySpec {
    pub name: String,
    pub label: String,
    pub description: Option<String>,
    pub example: Option<String>,
    pub action: Vec<ActionUse>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ActionUse {
    Referenced(ActionReference),
    Embedded(Box<ActionSpec>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionReference {
    pub name: String,
    pub package: String,
    pub evaluations: Vec<EvaluationAssignment>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvaluationAssignment {
    pub name: String,
    pub criteria: BTreeMap<String, ControlledValue>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ActionSpec {
    pub name: String,
    pub label: String,
    pub claim: String,
    pub description: Option<String>,
    pub example: Option<String>,
    pub evidence: EvidenceDefinition,
    pub evaluations: Vec<EvaluationDefinition>,
    pub test: TestDefinition,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceDefinition {
    pub name: String,
    pub file: String,
    pub media_type: Option<String>,
    pub maximum_bytes: Option<u64>,
    pub schema: Option<SchemaSelection>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SchemaSelection {
    pub media_type: String,
    pub file: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvaluationDefinition {
    pub name: String,
    pub label: String,
    pub description: Option<String>,
    pub example: Option<String>,
    pub subject: EvaluationSubject,
    pub criteria: BTreeMap<String, ControlledValue>,
    pub criteria_assignment: Vec<CriteriaAssignment>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvaluationSubject {
    pub name: String,
    pub label: String,
    pub data_type: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CriteriaAssignment {
    pub name: String,
    pub authorized_values: Vec<ControlledValue>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestDefinition {
    pub name: String,
    pub label: Option<String>,
    pub description: Option<String>,
    pub example: Option<String>,
    pub file: String,
    pub module: Vec<TestModule>,
    pub resource: Option<TestResource>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestModule {
    pub name: String,
    pub file: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct TestResource {
    pub maximum_execution_work_units: Option<u64>,
    pub maximum_result_bytes: Option<u64>,
}

/// Admit the complete F01-F06 authored definition projection and derive the
/// exact package-owned static paths and direct reference edges.
pub fn admit_build_definition(
    value: &ControlledValue,
    package: &str,
) -> Result<FunctionalDefinition, VerificationError> {
    let document = project_definition(value)?;
    if document.api_version != PROCEDURE_API_VERSION {
        return Err(VerificationError::invalid(
            "functional authored apiVersion must be 2.0.0",
        ));
    }

    let (kind, name, references, paths) = match (&*document.kind, &document.spec) {
        ("VerificationProcedure", DefinitionSpec::Procedure(spec)) => {
            validate_procedure(spec)?;
            let (references, paths) = procedure_projection(spec);
            (
                "VerificationProcedure",
                spec.name.clone(),
                references,
                paths,
            )
        }
        ("VerificationActivity", DefinitionSpec::Activity(spec)) => {
            validate_activity(spec, "Activity")?;
            let (references, paths) = activity_projection(spec);
            ("VerificationActivity", spec.name.clone(), references, paths)
        }
        ("VerificationAction", DefinitionSpec::Action(spec)) => {
            validate_action(spec, "Action", &format!("action/{}", spec.name), true)?;
            (
                "VerificationAction",
                spec.name.clone(),
                Vec::new(),
                action_paths(spec, &format!("action/{}", spec.name)),
            )
        }
        _ => {
            return Err(VerificationError::invalid(
                "definition kind and spec shape do not agree",
            ))
        }
    };
    validate_functional_package_purl(
        package,
        match kind {
            "VerificationProcedure" => "verification-procedure",
            "VerificationActivity" => "verification-activity",
            _ => "verification-action",
        },
        "root",
    )?;
    if package_name(package).as_deref() != Some(name.as_str()) {
        return Err(VerificationError::invalid(
            "definition root name does not equal the package name",
        ));
    }
    let mut authored_paths = vec![match kind {
        "VerificationProcedure" => "verification-procedure.yaml".to_string(),
        "VerificationActivity" => "verification-activity.yaml".to_string(),
        _ => "verification-action.yaml".to_string(),
    }];
    authored_paths.extend(paths);
    authored_paths.sort();
    authored_paths.dedup();
    Ok(FunctionalDefinition {
        build: BuildDefinition {
            kind: kind.to_string(),
            name,
            direct_references: references,
            authored_paths,
        },
        document,
    })
}

fn project_definition(value: &ControlledValue) -> Result<DefinitionDocument, VerificationError> {
    let root = object(value, "definition")?;
    exact_fields(root, &["apiVersion", "kind", "spec"], "definition")?;
    let api_version = required_text(root, "apiVersion", "definition")?;
    let kind = required_text(root, "kind", "definition")?;
    let specification = root
        .get("spec")
        .ok_or_else(|| VerificationError::invalid("definition spec is required"))?;
    let spec = match kind.as_str() {
        "VerificationProcedure" => DefinitionSpec::Procedure(project_procedure(specification)?),
        "VerificationActivity" => DefinitionSpec::Activity(project_activity(specification)?),
        "VerificationAction" => DefinitionSpec::Action(Box::new(project_action(specification)?)),
        _ => {
            return Err(VerificationError::invalid(
                "functional definition kind is unsupported",
            ))
        }
    };
    Ok(DefinitionDocument {
        api_version,
        kind,
        spec,
    })
}

fn project_procedure(value: &ControlledValue) -> Result<ProcedureSpec, VerificationError> {
    let object = object(value, "Procedure spec")?;
    exact_fields(
        object,
        &[
            "name",
            "label",
            "description",
            "example",
            "evidence_limit",
            "activity",
        ],
        "Procedure spec",
    )?;
    let evidence_limit = object
        .get("evidence_limit")
        .map(|value| {
            let value = self::object(value, "Procedure evidence_limit")?;
            exact_fields(value, &["maximum_bytes"], "Procedure evidence_limit")?;
            Ok(ProcedureEvidenceLimit {
                maximum_bytes: required_u64(value, "maximum_bytes", "Procedure evidence_limit")?,
            })
        })
        .transpose()?;
    Ok(ProcedureSpec {
        name: required_text(object, "name", "Procedure spec")?,
        label: required_text(object, "label", "Procedure spec")?,
        description: optional_text_member(object, "description", "Procedure spec")?,
        example: optional_text_member(object, "example", "Procedure spec")?,
        evidence_limit,
        activity: required_array(object, "activity", "Procedure spec")?
            .iter()
            .map(project_activity_use)
            .collect::<Result<Vec<_>, _>>()?,
    })
}

fn project_activity_use(value: &ControlledValue) -> Result<ActivityUse, VerificationError> {
    let object = object(value, "Activity use")?;
    if object.contains_key("package") {
        exact_fields(object, &["name", "package"], "Activity reference")?;
        Ok(ActivityUse::Referenced(ActivityReference {
            name: required_text(object, "name", "Activity reference")?,
            package: required_text(object, "package", "Activity reference")?,
        }))
    } else {
        Ok(ActivityUse::Embedded(project_activity(value)?))
    }
}

fn project_activity(value: &ControlledValue) -> Result<ActivitySpec, VerificationError> {
    let object = object(value, "Activity")?;
    exact_fields(
        object,
        &["name", "label", "description", "example", "action"],
        "Activity",
    )?;
    Ok(ActivitySpec {
        name: required_text(object, "name", "Activity")?,
        label: required_text(object, "label", "Activity")?,
        description: optional_text_member(object, "description", "Activity")?,
        example: optional_text_member(object, "example", "Activity")?,
        action: required_array(object, "action", "Activity")?
            .iter()
            .map(project_action_use)
            .collect::<Result<Vec<_>, _>>()?,
    })
}

fn project_action_use(value: &ControlledValue) -> Result<ActionUse, VerificationError> {
    let object = object(value, "Action use")?;
    if object.contains_key("package") {
        exact_fields(
            object,
            &["name", "package", "evaluations"],
            "Action reference",
        )?;
        Ok(ActionUse::Referenced(ActionReference {
            name: required_text(object, "name", "Action reference")?,
            package: required_text(object, "package", "Action reference")?,
            evaluations: optional_array(object, "evaluations", "Action reference")?
                .iter()
                .map(project_evaluation_assignment)
                .collect::<Result<Vec<_>, _>>()?,
        }))
    } else {
        Ok(ActionUse::Embedded(Box::new(project_action(value)?)))
    }
}

fn project_action(value: &ControlledValue) -> Result<ActionSpec, VerificationError> {
    let object = object(value, "Action")?;
    exact_fields(
        object,
        &[
            "name",
            "label",
            "claim",
            "description",
            "example",
            "evidence",
            "evaluations",
            "test",
        ],
        "Action",
    )?;
    Ok(ActionSpec {
        name: required_text(object, "name", "Action")?,
        label: required_text(object, "label", "Action")?,
        claim: required_text(object, "claim", "Action")?,
        description: optional_text_member(object, "description", "Action")?,
        example: optional_text_member(object, "example", "Action")?,
        evidence: project_evidence(required_member(object, "evidence", "Action")?)?,
        evaluations: optional_array(object, "evaluations", "Action")?
            .iter()
            .map(project_evaluation)
            .collect::<Result<Vec<_>, _>>()?,
        test: project_test(required_member(object, "test", "Action")?)?,
    })
}

fn project_evidence(value: &ControlledValue) -> Result<EvidenceDefinition, VerificationError> {
    let object = object(value, "Evidence")?;
    exact_fields(
        object,
        &["name", "file", "media_type", "maximum_bytes", "schema"],
        "Evidence",
    )?;
    let schema = object.get("schema").map(project_schema).transpose()?;
    Ok(EvidenceDefinition {
        name: required_text(object, "name", "Evidence")?,
        file: required_text(object, "file", "Evidence")?,
        media_type: optional_text_member(object, "media_type", "Evidence")?,
        maximum_bytes: optional_u64(object, "maximum_bytes", "Evidence")?,
        schema,
    })
}

fn project_schema(value: &ControlledValue) -> Result<SchemaSelection, VerificationError> {
    let object = object(value, "Evidence schema")?;
    exact_fields(object, &["media_type", "file"], "Evidence schema")?;
    Ok(SchemaSelection {
        media_type: required_text(object, "media_type", "Evidence schema")?,
        file: required_text(object, "file", "Evidence schema")?,
    })
}

fn project_evaluation(value: &ControlledValue) -> Result<EvaluationDefinition, VerificationError> {
    let object = object(value, "Evaluation")?;
    exact_fields(
        object,
        &[
            "name",
            "label",
            "description",
            "example",
            "subject",
            "criteria",
            "criteria_assignment",
        ],
        "Evaluation",
    )?;
    Ok(EvaluationDefinition {
        name: required_text(object, "name", "Evaluation")?,
        label: required_text(object, "label", "Evaluation")?,
        description: optional_text_member(object, "description", "Evaluation")?,
        example: optional_text_member(object, "example", "Evaluation")?,
        subject: project_evaluation_subject(required_member(object, "subject", "Evaluation")?)?,
        criteria: controlled_object(
            required_member(object, "criteria", "Evaluation")?,
            "criteria",
        )?
        .clone(),
        criteria_assignment: optional_array(object, "criteria_assignment", "Evaluation")?
            .iter()
            .map(project_criteria_assignment)
            .collect::<Result<Vec<_>, _>>()?,
    })
}

fn project_evaluation_subject(
    value: &ControlledValue,
) -> Result<EvaluationSubject, VerificationError> {
    let object = object(value, "Evaluation subject")?;
    exact_fields(
        object,
        &["name", "label", "data_type"],
        "Evaluation subject",
    )?;
    Ok(EvaluationSubject {
        name: required_text(object, "name", "Evaluation subject")?,
        label: required_text(object, "label", "Evaluation subject")?,
        data_type: required_text(object, "data_type", "Evaluation subject")?,
    })
}

fn project_criteria_assignment(
    value: &ControlledValue,
) -> Result<CriteriaAssignment, VerificationError> {
    let object = object(value, "criteria_assignment")?;
    exact_fields(
        object,
        &["name", "authorized_values"],
        "criteria_assignment",
    )?;
    Ok(CriteriaAssignment {
        name: required_text(object, "name", "criteria_assignment")?,
        authorized_values: required_array(object, "authorized_values", "criteria_assignment")?
            .to_vec(),
    })
}

fn project_evaluation_assignment(
    value: &ControlledValue,
) -> Result<EvaluationAssignment, VerificationError> {
    let object = object(value, "assigned Evaluation")?;
    exact_fields(object, &["name", "criteria"], "assigned Evaluation")?;
    Ok(EvaluationAssignment {
        name: required_text(object, "name", "assigned Evaluation")?,
        criteria: controlled_object(
            required_member(object, "criteria", "assigned Evaluation")?,
            "assigned Evaluation criteria",
        )?
        .clone(),
    })
}

fn project_test(value: &ControlledValue) -> Result<TestDefinition, VerificationError> {
    let object = object(value, "Test")?;
    exact_fields(
        object,
        &[
            "name",
            "label",
            "description",
            "example",
            "file",
            "module",
            "resource",
        ],
        "Test",
    )?;
    Ok(TestDefinition {
        name: required_text(object, "name", "Test")?,
        label: optional_text_member(object, "label", "Test")?,
        description: optional_text_member(object, "description", "Test")?,
        example: optional_text_member(object, "example", "Test")?,
        file: required_text(object, "file", "Test")?,
        module: optional_array(object, "module", "Test")?
            .iter()
            .map(project_test_module)
            .collect::<Result<Vec<_>, _>>()?,
        resource: object
            .get("resource")
            .map(project_test_resource)
            .transpose()?,
    })
}

fn project_test_module(value: &ControlledValue) -> Result<TestModule, VerificationError> {
    let object = object(value, "Test module")?;
    exact_fields(object, &["name", "file"], "Test module")?;
    Ok(TestModule {
        name: required_text(object, "name", "Test module")?,
        file: required_text(object, "file", "Test module")?,
    })
}

fn project_test_resource(value: &ControlledValue) -> Result<TestResource, VerificationError> {
    let object = object(value, "Test resource")?;
    exact_fields(
        object,
        &["maximum_execution_work_units", "maximum_result_bytes"],
        "Test resource",
    )?;
    Ok(TestResource {
        maximum_execution_work_units: optional_u64(
            object,
            "maximum_execution_work_units",
            "Test resource",
        )?,
        maximum_result_bytes: optional_u64(object, "maximum_result_bytes", "Test resource")?,
    })
}

fn object<'a>(
    value: &'a ControlledValue,
    field: &str,
) -> Result<&'a BTreeMap<String, ControlledValue>, VerificationError> {
    controlled_object(value, field)
}

fn controlled_object<'a>(
    value: &'a ControlledValue,
    field: &str,
) -> Result<&'a BTreeMap<String, ControlledValue>, VerificationError> {
    value
        .as_object()
        .ok_or_else(|| VerificationError::invalid(format!("{field} must be an object")))
}

fn exact_fields(
    object: &BTreeMap<String, ControlledValue>,
    allowed: &[&str],
    field: &str,
) -> Result<(), VerificationError> {
    if object.keys().any(|name| !allowed.contains(&name.as_str())) {
        return Err(VerificationError::invalid(format!(
            "{field} contains an unknown field"
        )));
    }
    Ok(())
}

fn required_member<'a>(
    object: &'a BTreeMap<String, ControlledValue>,
    name: &str,
    field: &str,
) -> Result<&'a ControlledValue, VerificationError> {
    object
        .get(name)
        .ok_or_else(|| VerificationError::invalid(format!("{field} {name} is required")))
}

fn required_text(
    object: &BTreeMap<String, ControlledValue>,
    name: &str,
    field: &str,
) -> Result<String, VerificationError> {
    required_member(object, name, field)?
        .as_str()
        .map(str::to_string)
        .ok_or_else(|| VerificationError::invalid(format!("{field} {name} must be text")))
}

fn optional_text_member(
    object: &BTreeMap<String, ControlledValue>,
    name: &str,
    field: &str,
) -> Result<Option<String>, VerificationError> {
    object
        .get(name)
        .map(|value| {
            value
                .as_str()
                .map(str::to_string)
                .ok_or_else(|| VerificationError::invalid(format!("{field} {name} must be text")))
        })
        .transpose()
}

fn required_array<'a>(
    object: &'a BTreeMap<String, ControlledValue>,
    name: &str,
    field: &str,
) -> Result<&'a [ControlledValue], VerificationError> {
    required_member(object, name, field)?
        .as_array()
        .ok_or_else(|| VerificationError::invalid(format!("{field} {name} must be an array")))
}

fn optional_array<'a>(
    object: &'a BTreeMap<String, ControlledValue>,
    name: &str,
    field: &str,
) -> Result<&'a [ControlledValue], VerificationError> {
    match object.get(name) {
        Some(value) => value
            .as_array()
            .ok_or_else(|| VerificationError::invalid(format!("{field} {name} must be an array"))),
        None => Ok(&[]),
    }
}

fn required_u64(
    object: &BTreeMap<String, ControlledValue>,
    name: &str,
    field: &str,
) -> Result<u64, VerificationError> {
    required_member(object, name, field)?
        .as_number()
        .and_then(|value| value.parse::<u64>().ok())
        .ok_or_else(|| {
            VerificationError::invalid(format!("{field} {name} must be an unsigned integer"))
        })
}

fn optional_u64(
    object: &BTreeMap<String, ControlledValue>,
    name: &str,
    field: &str,
) -> Result<Option<u64>, VerificationError> {
    object
        .get(name)
        .map(|value| {
            value
                .as_number()
                .and_then(|value| value.parse::<u64>().ok())
                .ok_or_else(|| {
                    VerificationError::invalid(format!(
                        "{field} {name} must be an unsigned integer"
                    ))
                })
        })
        .transpose()
}

fn validate_procedure(spec: &ProcedureSpec) -> Result<(), VerificationError> {
    validate_name(&spec.name, "Procedure name")?;
    validate_text(&spec.label, 1, 256, "Procedure label")?;
    optional_text(&spec.description, 4096, "Procedure description")?;
    optional_text(&spec.example, 4096, "Procedure example")?;
    if let Some(limit) = &spec.evidence_limit {
        if !(1..=2_147_483_648).contains(&limit.maximum_bytes) {
            return Err(VerificationError::invalid(
                "Procedure evidence_limit.maximum_bytes is outside the Profile ceiling",
            ));
        }
    }
    if spec.activity.is_empty() || spec.activity.len() > 256 {
        return Err(VerificationError::invalid(
            "Procedure must contain between one and 256 Activities",
        ));
    }
    let mut names = BTreeSet::new();
    for use_site in &spec.activity {
        let name = match use_site {
            ActivityUse::Referenced(reference) => {
                validate_name(&reference.name, "Activity reference name")?;
                validate_functional_package_purl(
                    &reference.package,
                    "verification-activity",
                    "Activity reference",
                )?;
                &reference.name
            }
            ActivityUse::Embedded(activity) => {
                validate_activity(activity, "embedded Activity")?;
                &activity.name
            }
        };
        if !names.insert(name) {
            return Err(VerificationError::invalid(
                "Procedure Activity occurrence names must be unique",
            ));
        }
    }
    Ok(())
}

fn validate_activity(spec: &ActivitySpec, field: &str) -> Result<(), VerificationError> {
    validate_name(&spec.name, &format!("{field} name"))?;
    validate_text(&spec.label, 1, 256, &format!("{field} label"))?;
    optional_text(&spec.description, 4096, &format!("{field} description"))?;
    optional_text(&spec.example, 4096, &format!("{field} example"))?;
    if spec.action.is_empty() || spec.action.len() > 4096 {
        return Err(VerificationError::invalid(
            "Activity must contain between one and 4096 Actions",
        ));
    }
    let mut names = BTreeSet::new();
    for use_site in &spec.action {
        let name = match use_site {
            ActionUse::Referenced(reference) => {
                validate_name(&reference.name, "Action reference name")?;
                validate_functional_package_purl(
                    &reference.package,
                    "verification-action",
                    "Action reference",
                )?;
                validate_assignments(&reference.evaluations)?;
                &reference.name
            }
            ActionUse::Embedded(action) => {
                validate_action(
                    action,
                    "embedded Action",
                    &format!("activity/{}/{}", spec.name, action.name),
                    false,
                )?;
                &action.name
            }
        };
        if !names.insert(name) {
            return Err(VerificationError::invalid(
                "Activity Action occurrence names must be unique",
            ));
        }
    }
    Ok(())
}

fn validate_action(
    spec: &ActionSpec,
    field: &str,
    static_base: &str,
    independent: bool,
) -> Result<(), VerificationError> {
    validate_name(&spec.name, &format!("{field} name"))?;
    validate_text(&spec.label, 1, 256, &format!("{field} label"))?;
    validate_text(&spec.claim, 1, 4096, &format!("{field} claim"))?;
    optional_text(&spec.description, 4096, &format!("{field} description"))?;
    optional_text(&spec.example, 4096, &format!("{field} example"))?;
    validate_name(&spec.evidence.name, "Evidence name")?;
    if spec.evidence.file.contains('/') {
        validate_relative_path(&spec.evidence.file, "Evidence file")?;
    } else {
        validate_file_name(&spec.evidence.file, "Evidence file")?;
        let evidence_stem = spec
            .evidence
            .file
            .rsplit_once('.')
            .map(|(stem, _)| stem)
            .ok_or_else(|| {
                VerificationError::invalid("Evidence file requires a registered suffix")
            })?;
        if evidence_stem != spec.evidence.name {
            return Err(VerificationError::invalid(
                "bare Evidence file stem must equal the Evidence name",
            ));
        }
    }
    if let Some(media_type) = &spec.evidence.media_type {
        validate_media_type(media_type, "Evidence media type")?;
        if evidence_media_type(&spec.evidence.file) != Some(media_type.as_str()) {
            return Err(VerificationError::invalid(
                "Evidence authored media_type is incompatible with its registered file suffix",
            ));
        }
    }
    if let Some(maximum) = spec.evidence.maximum_bytes {
        if !(1..=268_435_456).contains(&maximum) {
            return Err(VerificationError::invalid(
                "Evidence maximum_bytes is outside the Profile ceiling",
            ));
        }
    }
    if let Some(schema) = &spec.evidence.schema {
        if schema.media_type != "application/schema+json" {
            return Err(VerificationError::invalid(
                "Evidence schema media_type must be application/schema+json",
            ));
        }
        validate_relative_path(&schema.file, "Evidence schema file")?;
        let controlled = spec
            .evidence
            .media_type
            .as_deref()
            .or_else(|| evidence_media_type(&spec.evidence.file));
        if !matches!(
            controlled,
            Some("application/json" | "application/yaml" | "application/toml")
        ) {
            return Err(VerificationError::invalid(
                "schema-selected Evidence must use controlled JSON, YAML, or TOML",
            ));
        }
    }
    validate_evaluations(&spec.evaluations, independent)?;
    validate_name(&spec.test.name, "Test name")?;
    if spec.test.file.contains('/') {
        validate_relative_path(&spec.test.file, "Test file")?;
        if !spec.test.file.ends_with(".py") {
            return Err(VerificationError::invalid(
                "explicit Test file must have the .py suffix",
            ));
        }
    } else {
        validate_file_name(&spec.test.file, "Test file")?;
        if spec.test.file != format!("{}.py", spec.test.name) {
            return Err(VerificationError::invalid(
                "bare Test file must equal the Test name plus .py",
            ));
        }
    }
    optional_text(&spec.test.label, 256, "Test label")?;
    optional_text(&spec.test.description, 4096, "Test description")?;
    optional_text(&spec.test.example, 4096, "Test example")?;
    if spec.test.module.len() > 64 {
        return Err(VerificationError::invalid(
            "Test declares more than 64 modules",
        ));
    }
    let mut module_names = BTreeSet::new();
    let mut module_files = BTreeSet::new();
    let mut previous = None;
    for module in &spec.test.module {
        validate_module_name(&module.name)?;
        validate_relative_path(&module.file, "Test module file")?;
        if module.file != format!("{static_base}/module/{}.py", module.name) {
            return Err(VerificationError::invalid(
                "Test module file must equal the canonical Action-owned module path",
            ));
        }
        if previous.is_some_and(|value: &str| value >= module.name.as_str()) {
            return Err(VerificationError::invalid(
                "Test modules must be strictly sorted by name",
            ));
        }
        previous = Some(&module.name);
        if !module_names.insert(&module.name) || !module_files.insert(&module.file) {
            return Err(VerificationError::invalid(
                "Test module names and files must be unique",
            ));
        }
    }
    if let Some(resource) = &spec.test.resource {
        if resource
            .maximum_execution_work_units
            .is_some_and(|value| !(1..=100_000_000).contains(&value))
            || resource
                .maximum_result_bytes
                .is_some_and(|value| !(1..=1_048_576).contains(&value))
        {
            return Err(VerificationError::invalid(
                "Test resource value is outside the Profile envelope",
            ));
        }
    }
    Ok(())
}

fn validate_evaluations(
    values: &[EvaluationDefinition],
    allow_assignments: bool,
) -> Result<(), VerificationError> {
    if values.len() > 64 {
        return Err(VerificationError::invalid(
            "Action declares more than 64 evaluations",
        ));
    }
    let mut names = BTreeSet::new();
    for value in values {
        validate_name(&value.name, "Evaluation name")?;
        if !names.insert(&value.name) {
            return Err(VerificationError::invalid(
                "Evaluation names must be unique",
            ));
        }
        validate_text(&value.label, 1, 256, "Evaluation label")?;
        optional_text(&value.description, 4096, "Evaluation description")?;
        optional_text(&value.example, 4096, "Evaluation example")?;
        validate_fact_name(&value.subject.name)?;
        validate_text(&value.subject.label, 1, 256, "Evaluation subject label")?;
        if !matches!(
            value.subject.data_type.as_str(),
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
            return Err(VerificationError::invalid(
                "Evaluation data_type is unsupported",
            ));
        }
        validate_criteria(&value.criteria)?;
        validate_criterion_types(&value.subject.data_type, &value.criteria)?;
        if !allow_assignments && !value.criteria_assignment.is_empty() {
            return Err(VerificationError::invalid(
                "criteria_assignment is permitted only on an independent Action root",
            ));
        }
        let mut assignments = BTreeSet::new();
        let mut previous_assignment = None;
        for assignment in &value.criteria_assignment {
            validate_name(&assignment.name, "criteria_assignment name")?;
            let Some(default) = value.criteria.get(&assignment.name) else {
                return Err(VerificationError::invalid(
                    "criteria_assignment must name an existing criterion",
                ));
            };
            if previous_assignment
                .is_some_and(|previous: &str| previous >= assignment.name.as_str())
                || !(2..=64).contains(&assignment.authorized_values.len())
                || !assignment.authorized_values.contains(default)
                || !assignments.insert(&assignment.name)
                || !authorized_values_are_canonical(
                    &assignment.authorized_values,
                    &value.subject.data_type,
                    &assignment.name,
                )
            {
                return Err(VerificationError::invalid(
                    "criteria_assignment must be ordered and uniquely authorize its default plus a distinct compatible alternate",
                ));
            }
            previous_assignment = Some(&assignment.name);
        }
    }
    Ok(())
}

fn validate_assignments(values: &[EvaluationAssignment]) -> Result<(), VerificationError> {
    if values.len() > 64 {
        return Err(VerificationError::invalid(
            "Action reference assigns more than 64 evaluations",
        ));
    }
    let mut names = BTreeSet::new();
    for value in values {
        validate_name(&value.name, "assigned Evaluation name")?;
        if !names.insert(&value.name) {
            return Err(VerificationError::invalid(
                "assigned Evaluation names must be unique",
            ));
        }
        validate_criteria(&value.criteria)?;
    }
    Ok(())
}

fn validate_criteria(
    criteria: &BTreeMap<String, ControlledValue>,
) -> Result<(), VerificationError> {
    const ALLOWED: &[&str] = &[
        "allowed_values",
        "disallowed_values",
        "equals",
        "maximum",
        "minimum",
        "required",
    ];
    if criteria.is_empty() || criteria.len() > 64 {
        return Err(VerificationError::invalid(
            "criteria must contain between one and 64 operators",
        ));
    }
    for name in criteria.keys() {
        validate_name(name, "criterion operator")?;
        if !ALLOWED.contains(&name.as_str()) {
            return Err(VerificationError::invalid(
                "criterion operator is unsupported",
            ));
        }
    }
    for (name, value) in criteria {
        match name.as_str() {
            "allowed_values" | "disallowed_values" => {
                if !value
                    .as_array()
                    .is_some_and(|values| (1..=64).contains(&values.len()))
                {
                    return Err(VerificationError::invalid(format!(
                        "criterion {name} must be an array containing between one and 64 values"
                    )));
                }
            }
            "required" if value.as_bool().is_none() => {
                return Err(VerificationError::invalid(
                    "criterion required must be boolean",
                ));
            }
            _ => {}
        }
    }
    Ok(())
}

fn validate_criterion_types(
    data_type: &str,
    criteria: &BTreeMap<String, ControlledValue>,
) -> Result<(), VerificationError> {
    for (operator, value) in criteria {
        let compatible = match operator.as_str() {
            "allowed_values" | "disallowed_values" => value.as_array().is_some_and(|values| {
                !values.is_empty()
                    && values
                        .iter()
                        .all(|candidate| value_matches_data_type(candidate, data_type))
            }),
            "required" => value.as_bool().is_some(),
            _ => value_matches_data_type(value, data_type),
        };
        if !compatible {
            return Err(VerificationError::invalid(format!(
                "criterion {operator} is incompatible with subject data_type {data_type}"
            )));
        }
    }
    Ok(())
}

fn authorized_values_are_canonical(
    values: &[ControlledValue],
    data_type: &str,
    operator: &str,
) -> bool {
    let mut previous: Option<&ControlledValue> = None;
    for value in values {
        let compatible = if operator == "required" {
            value.as_bool().is_some()
        } else {
            value_matches_data_type(value, data_type)
        };
        if !compatible || previous.is_some_and(|previous| previous >= value) {
            return false;
        }
        previous = Some(value);
    }
    true
}

fn value_matches_data_type(value: &ControlledValue, data_type: &str) -> bool {
    match data_type {
        "text" | "date" | "datetime" | "duration" => value.as_str().is_some(),
        "integer" => value
            .as_number()
            .is_some_and(|value| value.parse::<i128>().is_ok()),
        "number" => value.as_number().is_some(),
        "boolean" => value.as_bool().is_some(),
        "array" => value.as_array().is_some(),
        "object" => value.as_object().is_some(),
        "null" => matches!(value, ControlledValue::Null),
        _ => false,
    }
}

fn procedure_projection(spec: &ProcedureSpec) -> (Vec<BuildReference>, Vec<String>) {
    let mut references = Vec::new();
    let mut paths = Vec::new();
    for activity in &spec.activity {
        match activity {
            ActivityUse::Referenced(reference) => references.push(BuildReference {
                use_selector: reference.name.clone(),
                package: reference.package.clone(),
            }),
            ActivityUse::Embedded(activity) => {
                for action in &activity.action {
                    match action {
                        ActionUse::Referenced(reference) => references.push(BuildReference {
                            use_selector: format!("{}.{}", activity.name, reference.name),
                            package: reference.package.clone(),
                        }),
                        ActionUse::Embedded(action) => paths.extend(action_paths(
                            action,
                            &format!("activity/{}/{}", activity.name, action.name),
                        )),
                    }
                }
            }
        }
    }
    (references, paths)
}

fn activity_projection(spec: &ActivitySpec) -> (Vec<BuildReference>, Vec<String>) {
    let mut references = Vec::new();
    let mut paths = Vec::new();
    for action in &spec.action {
        match action {
            ActionUse::Referenced(reference) => references.push(BuildReference {
                use_selector: reference.name.clone(),
                package: reference.package.clone(),
            }),
            ActionUse::Embedded(action) => paths.extend(action_paths(
                action,
                &format!("activity/{}/{}", spec.name, action.name),
            )),
        }
    }
    (references, paths)
}

fn action_paths(action: &ActionSpec, base: &str) -> Vec<String> {
    let mut paths = vec![if action.test.file.contains('/') {
        action.test.file.clone()
    } else {
        format!("{base}/{}", action.test.file)
    }];
    paths.extend(action.test.module.iter().map(|module| module.file.clone()));
    if let Some(schema) = &action.evidence.schema {
        paths.push(schema.file.clone());
    }
    paths
}

fn optional_text(
    value: &Option<String>,
    maximum: usize,
    field: &str,
) -> Result<(), VerificationError> {
    if let Some(value) = value {
        validate_text(value, 1, maximum, field)?;
    }
    Ok(())
}

fn validate_name(value: &str, field: &str) -> Result<(), VerificationError> {
    let bytes = value.as_bytes();
    if bytes.is_empty()
        || bytes.len() > 128
        || !(bytes[0].is_ascii_lowercase() || bytes[0].is_ascii_digit())
        || !(bytes[bytes.len() - 1].is_ascii_lowercase() || bytes[bytes.len() - 1].is_ascii_digit())
        || value.contains("--")
        || !bytes
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-')
    {
        return Err(VerificationError::invalid(format!(
            "{field} must match [a-z0-9]+(?:-[a-z0-9]+)*"
        )));
    }
    Ok(())
}

fn validate_functional_package_purl(
    value: &str,
    expected_token: &str,
    field: &str,
) -> Result<(), VerificationError> {
    if value.len() > 2048
        || !value.is_ascii()
        || value != value.to_ascii_lowercase()
        || !value.starts_with("pkg:attestify/")
        || value.contains(['?', '#', '%'])
        || value.matches('@').count() != 1
    {
        return Err(VerificationError::invalid(format!(
            "{field} package is not a canonical Attestify PURL"
        )));
    }
    let coordinate = value.strip_prefix("pkg:attestify/").unwrap_or_default();
    let (coordinate, version) = coordinate
        .rsplit_once('@')
        .ok_or_else(|| VerificationError::invalid(format!("{field} package version is absent")))?;
    let segments = coordinate.split('/').collect::<Vec<_>>();
    if !(4..=6).contains(&segments.len())
        || segments[1] != expected_token
        || segments[0].split('.').count() < 2
        || !segments[0].split('.').all(|label| {
            let bytes = label.as_bytes();
            !bytes.is_empty()
                && bytes.len() <= 63
                && (bytes[0].is_ascii_lowercase() || bytes[0].is_ascii_digit())
                && (bytes[bytes.len() - 1].is_ascii_lowercase()
                    || bytes[bytes.len() - 1].is_ascii_digit())
                && bytes
                    .iter()
                    .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'-')
        })
        || segments[2..]
            .iter()
            .any(|segment| validate_name(segment, "package component").is_err())
        || !semver_subset(version)
    {
        return Err(VerificationError::invalid(format!(
            "{field} package publisher, kind, domain, name, or version is invalid"
        )));
    }
    Ok(())
}

fn validate_media_type(value: &str, field: &str) -> Result<(), VerificationError> {
    let Some((top, subtype)) = value.split_once('/') else {
        return Err(VerificationError::invalid(format!("{field} is invalid")));
    };
    if top.is_empty()
        || subtype.is_empty()
        || !value.is_ascii()
        || value
            .chars()
            .any(|character| character.is_ascii_whitespace() || character.is_control())
    {
        return Err(VerificationError::invalid(format!("{field} is invalid")));
    }
    Ok(())
}

fn validate_relative_path(value: &str, field: &str) -> Result<(), VerificationError> {
    if value.is_empty()
        || value.len() > 1024
        || value.starts_with('/')
        || value.contains('\\')
        || !value.is_ascii()
        || value.split('/').any(|segment| {
            segment.is_empty() || segment == "." || segment == ".." || segment.starts_with('.')
        })
        || !value.bytes().all(|byte| {
            byte.is_ascii_lowercase()
                || byte.is_ascii_digit()
                || matches!(byte, b'-' | b'_' | b'.' | b'/')
        })
    {
        return Err(VerificationError::invalid(format!(
            "{field} is not a safe relative path"
        )));
    }
    Ok(())
}

fn validate_module_name(value: &str) -> Result<(), VerificationError> {
    let bytes = value.as_bytes();
    if bytes.is_empty()
        || bytes.len() > 64
        || !bytes[0].is_ascii_lowercase()
        || value == "json"
        || !bytes
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'_')
    {
        return Err(VerificationError::invalid(
            "Test module name must match [a-z][a-z0-9_]*",
        ));
    }
    Ok(())
}

fn validate_fact_name(value: &str) -> Result<(), VerificationError> {
    let bytes = value.as_bytes();
    if bytes.is_empty()
        || bytes.len() > 128
        || !bytes[0].is_ascii_lowercase()
        || !bytes
            .iter()
            .all(|byte| byte.is_ascii_lowercase() || byte.is_ascii_digit() || *byte == b'_')
    {
        return Err(VerificationError::invalid(
            "Evaluation subject name must match [a-z][a-z0-9_]*",
        ));
    }
    Ok(())
}

fn validate_file_name(value: &str, field: &str) -> Result<(), VerificationError> {
    if value.is_empty()
        || value.len() > 255
        || value == "."
        || value == ".."
        || value.starts_with('.')
        || value.contains(['/', '\\'])
        || value.contains("..")
        || !value.is_ascii()
        || !value.bytes().all(|byte| {
            byte.is_ascii_lowercase() || byte.is_ascii_digit() || matches!(byte, b'-' | b'.')
        })
    {
        return Err(VerificationError::invalid(format!(
            "{field} is not a safe file name"
        )));
    }
    Ok(())
}

fn validate_text(
    value: &str,
    minimum: usize,
    maximum: usize,
    field: &str,
) -> Result<(), VerificationError> {
    if value.len() < minimum || value.len() > maximum || value.chars().any(char::is_control) {
        return Err(VerificationError::invalid(format!(
            "{field} is outside its text bounds"
        )));
    }
    Ok(())
}

fn evidence_media_type(file: &str) -> Option<&'static str> {
    match file.rsplit_once('.')?.1 {
        "json" => Some("application/json"),
        "yaml" | "yml" => Some("application/yaml"),
        "toml" => Some("application/toml"),
        _ => None,
    }
}

fn package_name(value: &str) -> Option<String> {
    value
        .strip_prefix("pkg:attestify/")?
        .rsplit_once('@')?
        .0
        .rsplit('/')
        .next()
        .map(str::to_string)
}

fn semver_subset(value: &str) -> bool {
    if value.is_empty() || value.len() > 128 || value.contains('+') {
        return false;
    }
    let (core, prerelease) = value
        .split_once('-')
        .map_or((value, None), |(left, right)| (left, Some(right)));
    let parts = core.split('.').collect::<Vec<_>>();
    if parts.len() != 3 || !parts.iter().all(|part| numeric_identifier(part)) {
        return false;
    }
    prerelease.is_none_or(|suffix| {
        !suffix.is_empty()
            && suffix.split('.').all(|part| {
                !part.is_empty()
                    && part.bytes().all(|byte| {
                        byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'
                    })
                    && (!part.bytes().all(|byte| byte.is_ascii_digit()) || numeric_identifier(part))
            })
    })
}

fn numeric_identifier(value: &str) -> bool {
    !value.is_empty()
        && value.bytes().all(|byte| byte.is_ascii_digit())
        && (value == "0" || !value.starts_with('0'))
}
