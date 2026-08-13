//! Argument-free `nape verify` adapter.

use std::path::Path;

use nape_domain::diagnostic::NapeOutcome;
use serde_json::json;

use crate::{
    composition::NapeApplication,
    gateway_driver::application_identity_local,
    io_adapter::clap::{
        dispatch::{normal_diagnostic, summary_value},
        receipt::ReceiptAdapter,
    },
};

pub(super) async fn dispatch(application: &NapeApplication, receipt: &ReceiptAdapter) -> i32 {
    match application.verify().await {
        Ok(NapeOutcome::Completed(value)) => {
            let output = match application_identity_local::existing_file_uri(Path::new(
                value.commitment.output.value(),
            )) {
                Ok(value) => value,
                Err(error) => return receipt.unexpected(&error),
            };
            let acquisition = if value.acquisition == "oci-pull" {
                json!({
                    "binding": "invocation",
                    "integrity": "digest-verified",
                    "source": "oci-pull",
                    "trust": "not-evaluated",
                })
            } else {
                json!({
                    "binding": "local-build-result",
                    "integrity": "digest-verified",
                    "source": "local-build",
                    "trust": "not-evaluated",
                })
            };
            receipt.success(
                "verify",
                json!({
                    "output": output,
                    "procedure": {
                        "acquisition": acquisition,
                        "manifestDigest": value.procedure.manifest_digest().value(),
                        "package": value.procedure.purl().value(),
                    },
                    "report": {
                        "id": value.report_id,
                        "kind": "VerificationReport",
                        "summary": summary_value(&value.summary),
                    },
                }),
            )
        }
        Ok(NapeOutcome::Rejected(value)) => match application.current_verify_receipt_context() {
            Ok((invocation, source)) => {
                receipt.failed("verify", &value, Some((&invocation, &source)))
            }
            Err(_) => normal_diagnostic(&value),
        },
        Err(error) => receipt.unexpected(&error),
    }
}

#[cfg(test)]
mod tests;
