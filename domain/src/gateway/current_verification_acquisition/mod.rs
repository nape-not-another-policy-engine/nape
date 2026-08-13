//! NAPE-managed current Verification state acquisition seam.

#[cfg(test)]
mod tests;

use kernel_oss::gateway::Gateway;

use crate::{diagnostic::NapeOutcome, value::current_verification::CurrentVerification};

/// Closed zero-field request for the selected current run.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct CurrentVerificationAcquisitionRequest;

/// Loads and revalidates the complete selected current-run state.
pub trait CurrentVerificationAcquisitionGW:
    Gateway<
    Request = CurrentVerificationAcquisitionRequest,
    Response = NapeOutcome<CurrentVerification>,
>
{
}
