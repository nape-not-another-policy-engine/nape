//! `nape evidence` adapter.

use clap::ArgMatches;
use nape_domain::{
    diagnostic::NapeOutcome,
    usecase::collect_verification_evidence::CollectVerificationEvidenceRequest,
    value::{effective_graph::CanonicalActionSelector, evidence::EvidenceFileName},
};

use crate::{
    composition::NapeApplication,
    io_adapter::clap::dispatch::{handle, normal_diagnostic, normal_error, required},
};

pub(super) fn dispatch(application: &NapeApplication, arguments: &ArgMatches) -> i32 {
    let mut builder = CollectVerificationEvidenceRequest::builder()
        .action(
            match CanonicalActionSelector::try_new(required(arguments, "action")) {
                Ok(value) => value,
                Err(error) => return normal_error(error),
            },
        )
        .evidence(match handle(required(arguments, "file")) {
            Ok(value) => value,
            Err(error) => return normal_error(error),
        });
    if let Some(value) = arguments.get_one::<String>("file-name") {
        match EvidenceFileName::try_new(value) {
            Ok(value) => builder = builder.file_name(value),
            Err(error) => return normal_error(error),
        }
    }
    let request = match builder.try_build() {
        Ok(value) => value,
        Err(error) => return normal_error(error),
    };
    match application.evidence(request) {
        Ok(NapeOutcome::Completed(_)) => 0,
        Ok(NapeOutcome::Rejected(value)) => normal_diagnostic(&value),
        Err(error) => normal_error(error),
    }
}

#[cfg(test)]
mod tests;
