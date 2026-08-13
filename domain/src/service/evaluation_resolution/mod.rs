//! Deterministic evaluation criteria resolution.

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;

use kernel_oss::error::{Error, Kind};

use crate::value::{controlled_value::ControlledValue, evaluation::EffectiveEvaluation};

/// Applies one authorized criteria override by name without mutating package declarations.
pub fn resolve_evaluation(
    package: &EffectiveEvaluation,
    override_criteria: Option<&BTreeMap<String, ControlledValue>>,
    authorized_values: &BTreeMap<String, Vec<ControlledValue>>,
) -> Result<EffectiveEvaluation, Error> {
    let Some(override_criteria) = override_criteria else {
        return Ok(package.clone());
    };
    for (operator, value) in override_criteria {
        if !authorized_values
            .get(operator)
            .is_some_and(|values| values.contains(value))
        {
            return Err(Error::for_user(
                Kind::PermissionDenied,
                "evaluation criteria override is not authorized by the package",
            ));
        }
    }
    match package.wire() {
        Some(wire) => EffectiveEvaluation::try_with_wire(
            package.name(),
            override_criteria.clone(),
            wire.clone(),
        ),
        None => EffectiveEvaluation::try_new(package.name(), override_criteria.clone()),
    }
}
