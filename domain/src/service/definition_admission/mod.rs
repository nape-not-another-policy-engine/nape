//! Product-semantic admission of controlled definition roots.

mod functional;
#[cfg(test)]
mod tests;

pub use functional::{
    admit_build_definition, ActionSpec, ActionUse, ActivitySpec, ActivityUse, DefinitionSpec,
    EvaluationAssignment, EvaluationDefinition, FunctionalDefinition, ProcedureSpec,
    RUNNER_PROFILE,
};

use kernel_oss::error::{Error, Kind};

use crate::value::{
    controlled_value::ControlledValue,
    definition::{DefinitionCandidate, DefinitionKind},
};

/// Admits the common `apiVersion`, `kind`, and `spec.name` definition envelope.
pub fn admit_definition(document: ControlledValue) -> Result<DefinitionCandidate, Error> {
    let api_version = text_member(&document, "apiVersion")?;
    if api_version != "2.0.0" {
        return Err(Error::for_user(
            Kind::InvalidInput,
            "Verification definition apiVersion must be 2.0.0",
        ));
    }
    let kind = DefinitionKind::try_from_product(text_member(&document, "kind")?)?;
    let specification = document.get("spec").ok_or_else(|| {
        Error::for_user(
            Kind::InvalidInput,
            "Verification definition spec is required",
        )
    })?;
    let name = text_member(specification, "name")?.to_string();
    DefinitionCandidate::try_new(kind, name, document)
}

fn text_member<'a>(value: &'a ControlledValue, name: &str) -> Result<&'a str, Error> {
    match value.get(name) {
        Some(ControlledValue::String(value)) => Ok(value),
        _ => Err(Error::for_user(
            Kind::InvalidInput,
            format!("Verification definition {name} must be text"),
        )),
    }
}
