//! Restricted deterministic JSON Schema validation gateway.

#[cfg(test)]
mod tests;

use kernel_oss::{error::Error, gateway::Gateway};

use crate::{diagnostic::NapeOutcome, value::controlled_value::ControlledValue};

/// The one initial restricted schema profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RestrictedSchemaProfile {
    /// Attestify restricted JSON Schema 2020-12 profile 1.
    AttestifyJsonSchema202012Profile1,
}

/// One exact schema/instance validation request.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct RestrictedSchemaValidationRequest {
    profile: RestrictedSchemaProfile,
    schema: ControlledValue,
    instance: ControlledValue,
}

impl RestrictedSchemaValidationRequest {
    /// Creates an immutable validation request.
    pub fn new(
        profile: RestrictedSchemaProfile,
        schema: ControlledValue,
        instance: ControlledValue,
    ) -> Self {
        Self {
            profile,
            schema,
            instance,
        }
    }

    /// Returns the exact selected profile.
    pub fn profile(&self) -> RestrictedSchemaProfile {
        self.profile
    }

    /// Returns the controlled schema value.
    pub fn schema(&self) -> &ControlledValue {
        &self.schema
    }

    /// Returns the controlled instance value.
    pub fn instance(&self) -> &ControlledValue {
        &self.instance
    }
}

/// A successful immutable validation observation.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct SchemaValidationPassed;

/// Validates an instance without defaulting, coercion, normalization, or mutation.
pub trait RestrictedSchemaValidationGW:
    Gateway<Request = RestrictedSchemaValidationRequest, Response = NapeOutcome<SchemaValidationPassed>>
{
}

/// Builds the shared Kernel error used only for unexpected schema-driver defects.
pub fn unexpected_schema_driver_error(message: impl Into<String>) -> Error {
    Error::for_system(kernel_oss::error::Kind::GatewayError, message)
}
