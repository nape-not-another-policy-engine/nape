//! Governed NAPE diagnostics and bounded domain outcomes.

#[cfg(test)]
mod tests;

use kernel_oss::error::{Error, Kind};

const MAX_DIAGNOSTIC_DETAIL_BYTES: usize = 1_024;

/// One safe, machine-addressable NAPE diagnostic.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NapeDiagnostic {
    code: String,
    reason_template: String,
    phase: String,
    detail: String,
}

impl NapeDiagnostic {
    /// Builds a diagnostic after enforcing the closed identifier grammar and
    /// safe-detail ceiling. Registry membership is established by the
    /// Application translation owner before this Domain constructor is used.
    pub fn try_new(
        code: impl Into<String>,
        reason_template: impl Into<String>,
        phase: impl Into<String>,
        detail: impl Into<String>,
    ) -> Result<Self, Error> {
        let code = code.into();
        let reason_template = reason_template.into();
        let phase = phase.into();
        let detail = detail.into();
        if !closed_identifier(&code, '_')
            || !closed_identifier(&reason_template, '-')
            || !closed_identifier(&phase, '-')
        {
            return Err(Error::for_system(
                Kind::ProcessingFailure,
                "diagnostic identifiers are outside the governed grammar",
            ));
        }
        if detail.trim().is_empty() {
            return Err(Error::for_system(
                Kind::ProcessingFailure,
                "diagnostic detail must not be empty",
            ));
        }
        if detail.len() > MAX_DIAGNOSTIC_DETAIL_BYTES {
            return Err(Error::for_system(
                Kind::ExceedsMax,
                "diagnostic detail exceeds 1024 UTF-8 bytes",
            ));
        }
        Ok(Self {
            code,
            reason_template,
            phase,
            detail,
        })
    }

    /// Returns the governed diagnostic code.
    pub fn code(&self) -> &str {
        &self.code
    }

    /// Returns the governed reason-template identifier.
    pub fn reason_template(&self) -> &str {
        &self.reason_template
    }

    /// Returns the governed failure phase.
    pub fn phase(&self) -> &str {
        &self.phase
    }

    /// Returns bounded, credential-safe detail.
    pub fn detail(&self) -> &str {
        &self.detail
    }
}

fn closed_identifier(value: &str, separator: char) -> bool {
    !value.is_empty()
        && !value.starts_with(separator)
        && !value.ends_with(separator)
        && value.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == separator
        })
}

/// A completed bounded value or a governed Product rejection.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NapeOutcome<T> {
    /// The operation completed with a validated value.
    Completed(T),
    /// The operation reached a governed rejection boundary.
    Rejected(NapeDiagnostic),
}
