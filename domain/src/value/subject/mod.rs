//! Closed Verification V2 subject input values.

#[cfg(test)]
mod tests;

use kernel_oss::error::Error;

use super::{bounded_nonempty, external_resource_handle::ExternalResourceHandle};

/// Driver-owned handle to a closed subject input document.
pub type VerificationSubjectHandle = ExternalResourceHandle;

/// One admitted subject observation.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct VerificationSubject {
    arn: String,
    label: Option<String>,
    description: Option<String>,
}

impl VerificationSubject {
    /// Creates one closed subject.
    pub fn try_new(
        arn: impl Into<String>,
        label: Option<String>,
        description: Option<String>,
    ) -> Result<Self, Error> {
        Ok(Self {
            arn: bounded_nonempty(arn, "subject ARN", 2_048)?,
            label,
            description,
        })
    }

    /// Returns the external subject identity.
    pub fn arn(&self) -> &str {
        &self.arn
    }

    /// Returns the optional label.
    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }

    /// Returns the optional description.
    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }
}
