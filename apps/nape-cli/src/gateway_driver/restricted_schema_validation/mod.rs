//! Restricted deterministic JSON Schema 2020-12 driver.

#[cfg(test)]
mod tests;

use std::collections::{BTreeMap, BTreeSet};

use kernel_oss::{error::Error, gateway::Gateway};
use nape_domain::{
    diagnostic::{NapeDiagnostic, NapeOutcome},
    gateway::restricted_schema_validation::{
        RestrictedSchemaValidationGW, RestrictedSchemaValidationRequest, SchemaValidationPassed,
    },
};
use serde_json::Value;

use super::controlled_document_projection::to_json;

/// The production restricted-schema validation driver.
pub struct RestrictedSchemaValidationDriver;

impl Gateway for RestrictedSchemaValidationDriver {
    type Request = RestrictedSchemaValidationRequest;
    type Response = NapeOutcome<SchemaValidationPassed>;

    fn execute(&self, request: Self::Request) -> Result<Self::Response, Error> {
        let schema = to_json(request.schema())?;
        let instance = to_json(request.instance())?;
        let validation = validate_profile(&schema).and_then(|()| {
            let validator = jsonschema::JSONSchema::options()
                .with_draft(jsonschema::Draft::Draft202012)
                .compile(&schema)
                .map_err(|_| "schema is not valid JSON Schema 2020-12".to_string())?;
            validator.validate(&instance).map_err(|mut errors| {
                errors
                    .next()
                    .map(|error| {
                        format!(
                            "Evidence failed schema validation at {}",
                            error.instance_path
                        )
                    })
                    .unwrap_or_else(|| "Evidence failed schema validation".to_string())
            })
        });
        Ok(match validation {
            Ok(()) => NapeOutcome::Completed(SchemaValidationPassed),
            Err(detail) => NapeOutcome::Rejected(NapeDiagnostic::try_new(
                "evidence_schema_validation_failed",
                "evidence-schema-validation-failed-v1",
                "evidence-admission",
                detail,
            )?),
        })
    }
}

impl RestrictedSchemaValidationGW for RestrictedSchemaValidationDriver {}

const KEYWORDS: &[&str] = &[
    "$schema",
    "$defs",
    "$ref",
    "type",
    "properties",
    "required",
    "additionalProperties",
    "unevaluatedProperties",
    "items",
    "prefixItems",
    "contains",
    "minContains",
    "maxContains",
    "minItems",
    "maxItems",
    "uniqueItems",
    "minProperties",
    "maxProperties",
    "propertyNames",
    "dependentRequired",
    "enum",
    "const",
    "minimum",
    "maximum",
    "exclusiveMinimum",
    "exclusiveMaximum",
    "multipleOf",
    "minLength",
    "maxLength",
    "allOf",
    "anyOf",
    "oneOf",
    "not",
    "if",
    "then",
    "else",
    "title",
    "description",
    "examples",
];

fn validate_profile(schema: &Value) -> Result<(), String> {
    let definitions = schema
        .as_object()
        .and_then(|object| object.get("$defs"))
        .and_then(Value::as_object);
    let mut graph = BTreeMap::<String, BTreeSet<String>>::new();
    graph.insert("#".to_string(), BTreeSet::new());
    if let Some(definitions) = definitions {
        for name in definitions.keys() {
            graph.insert(format!("#/$defs/{name}"), BTreeSet::new());
        }
    }
    visit_schema(schema, "#", &mut graph)?;
    let mut active = BTreeSet::new();
    let mut completed = BTreeSet::new();
    for node in graph.keys() {
        prove_acyclic(node, &graph, &mut active, &mut completed)?;
    }
    Ok(())
}

fn visit_schema(
    value: &Value,
    owner: &str,
    graph: &mut BTreeMap<String, BTreeSet<String>>,
) -> Result<(), String> {
    if value.is_boolean() {
        return Ok(());
    }
    let object = value
        .as_object()
        .ok_or_else(|| "schema position must contain an object or boolean".to_string())?;
    for key in object.keys() {
        if !KEYWORDS.contains(&key.as_str()) {
            return Err(format!("schema keyword is unsupported or excluded: {key}"));
        }
    }
    if object
        .get("$schema")
        .is_some_and(|value| value.as_str() != Some("https://json-schema.org/draft/2020-12/schema"))
    {
        return Err("Evidence schema dialect is unsupported".to_string());
    }
    if let Some(reference) = object.get("$ref") {
        let reference = reference
            .as_str()
            .ok_or_else(|| "$ref must be text".to_string())?;
        if reference == "#" || !graph.contains_key(reference) {
            return Err("$ref must resolve to one same-document $defs member".to_string());
        }
        graph
            .get_mut(owner)
            .ok_or_else(|| "schema owner is not declared".to_string())?
            .insert(reference.to_string());
    }
    for (key, child) in object {
        match key.as_str() {
            "$defs" => {
                let values = child
                    .as_object()
                    .ok_or_else(|| "$defs must be an object".to_string())?;
                for (name, definition) in values {
                    visit_schema(definition, &format!("#/$defs/{name}"), graph)?;
                }
            }
            "properties" => {
                let values = child
                    .as_object()
                    .ok_or_else(|| "properties must be an object".to_string())?;
                for definition in values.values() {
                    visit_schema(definition, owner, graph)?;
                }
            }
            "items" | "contains" | "propertyNames" | "not" | "if" | "then" | "else" => {
                visit_schema(child, owner, graph)?;
            }
            "prefixItems" | "allOf" | "anyOf" | "oneOf" => {
                let values = child
                    .as_array()
                    .ok_or_else(|| format!("{key} must be an array"))?;
                for definition in values {
                    visit_schema(definition, owner, graph)?;
                }
            }
            _ => {}
        }
    }
    Ok(())
}

fn prove_acyclic(
    node: &str,
    graph: &BTreeMap<String, BTreeSet<String>>,
    active: &mut BTreeSet<String>,
    completed: &mut BTreeSet<String>,
) -> Result<(), String> {
    if active.contains(node) {
        return Err("recursive Evidence schema $ref cycle".to_string());
    }
    if completed.contains(node) {
        return Ok(());
    }
    active.insert(node.to_string());
    for target in graph
        .get(node)
        .ok_or_else(|| "schema reference target is not declared".to_string())?
    {
        prove_acyclic(target, graph, active, completed)?;
    }
    active.remove(node);
    completed.insert(node.to_string());
    Ok(())
}
