//! Stable bounded Verification subject acquisition seam.

#[cfg(test)]
mod tests;

use kernel_oss::gateway::Gateway;

use crate::{
    diagnostic::NapeOutcome,
    value::subject::{VerificationSubject, VerificationSubjectHandle},
};

/// Reads and validates one closed subject document.
pub trait VerificationSubjectAcquisitionGW:
    Gateway<Request = VerificationSubjectHandle, Response = NapeOutcome<VerificationSubject>>
{
}
