//! Effective evaluation declarations and assignments.

#[cfg(test)]
mod tests;

use std::collections::BTreeMap;

use kernel_oss::error::Error;

use super::{bounded_nonempty, controlled_value::ControlledValue};

/// One effective evaluation criterion set supplied to a Test occurrence.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectiveEvaluation {
    name: String,
    criteria: BTreeMap<String, ControlledValue>,
    wire: Option<ControlledValue>,
}

impl EffectiveEvaluation {
    /// Creates one effective evaluation.
    pub fn try_new(
        name: impl Into<String>,
        criteria: BTreeMap<String, ControlledValue>,
    ) -> Result<Self, Error> {
        Ok(Self {
            name: bounded_nonempty(name, "evaluation name", 128)?,
            criteria,
            wire: None,
        })
    }

    /// Creates one effective evaluation while preserving its complete
    /// package-governed Evaluator wire projection.
    pub fn try_with_wire(
        name: impl Into<String>,
        criteria: BTreeMap<String, ControlledValue>,
        wire: ControlledValue,
    ) -> Result<Self, Error> {
        Ok(Self {
            name: bounded_nonempty(name, "evaluation name", 128)?,
            criteria,
            wire: Some(wire),
        })
    }

    /// Returns the evaluation name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns deterministic effective criteria.
    pub fn criteria(&self) -> &BTreeMap<String, ControlledValue> {
        &self.criteria
    }

    /// Returns the complete preserved Evaluator wire projection when this
    /// evaluation came from a Product definition.
    pub fn wire(&self) -> Option<&ControlledValue> {
        self.wire.as_ref()
    }
}
