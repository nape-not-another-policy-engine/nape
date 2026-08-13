//! `nape package publish` adapter.

use clap::ArgMatches;
use nape_domain::{
    diagnostic::NapeOutcome, usecase::publish_definition_package::PublishDefinitionPackageRequest,
};
use serde_json::json;

use crate::{
    composition::NapeApplication,
    gateway_driver::registry_configuration_attestify_oci::RegistryConfigurationFailure,
    io_adapter::clap::{
        dispatch::{handle, location_observation, registry_map},
        receipt::ReceiptAdapter,
    },
};

pub(super) async fn dispatch(
    application: &NapeApplication,
    receipt: &ReceiptAdapter,
    arguments: &ArgMatches,
) -> i32 {
    let map = match registry_map(arguments) {
        Ok(value) => value,
        Err(RegistryConfigurationFailure::Rejected(value)) => {
            return receipt.failed("package-publish", &value, None)
        }
        Err(RegistryConfigurationFailure::Unexpected(error)) => return receipt.unexpected(&error),
    };
    let local_package = match handle(arguments.get_one::<String>("local-package").map_or_else(
        || unreachable!("Clap requires local-package"),
        String::as_str,
    )) {
        Ok(value) => value,
        Err(error) => return receipt.unexpected(&error),
    };
    let request = PublishDefinitionPackageRequest::new(local_package);
    match application.publish(request, map).await {
        Ok(NapeOutcome::Completed(value)) => {
            let root = value.package.root().identity();
            let location = match location_observation(&value.location) {
                Ok(value) => value,
                Err(error) => return receipt.unexpected(&error),
            };
            receipt.success(
                "package-publish",
                json!({
                    "disposition": value.disposition,
                    "integrity": "digest-verified",
                    "location": location,
                    "manifestDigest": root.manifest_digest().value(),
                    "package": root.purl().value(),
                    "trust": "not-evaluated",
                }),
            )
        }
        Ok(NapeOutcome::Rejected(value)) => receipt.failed("package-publish", &value, None),
        Err(error) => receipt.unexpected(&error),
    }
}

#[cfg(test)]
mod tests;
