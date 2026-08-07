//! Transport-neutral definition candidates and admitted definition identity.

#[cfg(test)]
mod tests;

use kernel_oss::error::{Error, Kind};

use super::{bounded_nonempty, controlled_value::ControlledValue};

/// One supported Verification definition kind.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DefinitionKind {
    /// Verification Procedure semantic root.
    VerificationProcedure,
    /// Independently managed Verification Activity semantic root.
    VerificationActivity,
    /// Independently managed Verification Action semantic root.
    VerificationAction,
}

impl DefinitionKind {
    /// Parses the exact Product kind spelling.
    pub fn try_from_product(value: &str) -> Result<Self, Error> {
        match value {
            "VerificationProcedure" => Ok(Self::VerificationProcedure),
            "VerificationActivity" => Ok(Self::VerificationActivity),
            "VerificationAction" => Ok(Self::VerificationAction),
            _ => Err(Error::for_user(
                Kind::InvalidInput,
                "unsupported Verification definition kind",
            )),
        }
    }
}

/// One projected definition awaiting semantic admission.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefinitionCandidate {
    kind: DefinitionKind,
    name: String,
    document: ControlledValue,
}

/// One direct definition-package dependency selected at an authored use site.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuildReference {
    /// Contextual selector owned by the parent definition.
    pub use_selector: String,
    /// Exact referenced package PURL.
    pub package: String,
}

/// Domain-admitted package-build projection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BuildDefinition {
    /// Exact Product definition kind.
    pub kind: String,
    /// Definition-owned root name.
    pub name: String,
    /// Ordered direct package references.
    pub direct_references: Vec<BuildReference>,
    /// Exact package-owned authored path inventory.
    pub authored_paths: Vec<String>,
}

impl DefinitionCandidate {
    /// Creates a candidate from a controlled projection.
    pub fn try_new(
        kind: DefinitionKind,
        name: impl Into<String>,
        document: ControlledValue,
    ) -> Result<Self, Error> {
        Ok(Self {
            kind,
            name: bounded_nonempty(name, "definition name", 128)?,
            document,
        })
    }

    /// Returns the semantic-root kind.
    pub fn kind(&self) -> DefinitionKind {
        self.kind
    }

    /// Returns the definition-owned name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the controlled root document.
    pub fn document(&self) -> &ControlledValue {
        &self.document
    }
}
