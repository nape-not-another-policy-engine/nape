//! Complete current Verification acquisition from NAPE's private state.

#[cfg(test)]
mod tests;

use kernel_oss::{error::Error, gateway::Gateway};
use nape_domain::{
    diagnostic::NapeOutcome,
    gateway::current_verification_acquisition::{
        CurrentVerificationAcquisitionGW, CurrentVerificationAcquisitionRequest,
    },
    value::current_verification::CurrentVerification,
};

use super::current_run_state::CurrentRunStateStore;

/// Loads the one selected current run and re-verifies its frozen package
/// custody through `attestify-oci`.
pub struct LocalCurrentVerificationAcquisitionDriver {
    state: CurrentRunStateStore,
}

impl LocalCurrentVerificationAcquisitionDriver {
    /// Creates the driver around explicit private state ownership.
    pub fn new(state: CurrentRunStateStore) -> Self {
        Self { state }
    }
}

impl Gateway for LocalCurrentVerificationAcquisitionDriver {
    type Request = CurrentVerificationAcquisitionRequest;
    type Response = NapeOutcome<CurrentVerification>;

    fn execute(&self, _request: Self::Request) -> Result<Self::Response, Error> {
        self.state.load().map(NapeOutcome::Completed)
    }
}

impl CurrentVerificationAcquisitionGW for LocalCurrentVerificationAcquisitionDriver {}
