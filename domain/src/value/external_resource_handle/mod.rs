//! Opaque handles for Application-owned external resources.

#[cfg(test)]
mod tests;

use kernel_oss::error::Error;

use super::bounded_nonempty;

/// An opaque, bounded resource locator interpreted only by its driver.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ExternalResourceHandle(String);

impl ExternalResourceHandle {
    /// Creates a bounded handle without interpreting it as Domain identity.
    pub fn try_new(value: impl Into<String>) -> Result<Self, Error> {
        bounded_nonempty(value, "external resource handle", 4_096).map(Self)
    }

    /// Returns the driver-owned opaque locator.
    pub fn value(&self) -> &str {
        &self.0
    }
}
