//! `nape package build` adapter.

use std::path::Path;

use clap::ArgMatches;
use nape_domain::{
    diagnostic::NapeOutcome, usecase::build_definition_package::BuildDefinitionPackageRequest,
};
use serde_json::json;

use crate::{
    composition::NapeApplication,
    gateway_driver::application_identity_local,
    io_adapter::clap::{
        dispatch::{handle, purl, required, static_failed},
        receipt::ReceiptAdapter,
    },
};

pub(super) fn dispatch(
    application: &NapeApplication,
    receipt: &ReceiptAdapter,
    arguments: &ArgMatches,
) -> i32 {
    let package = match purl(required(arguments, "package")) {
        Ok(value) => value,
        Err(_) => {
            return static_failed(
                receipt,
                "package-build",
                "package_purl_invalid",
                "package-purl-invalid-v1",
                "package-reference-validation",
                "package PURL is invalid",
            )
        }
    };
    let dependency_packages = match arguments
        .get_many::<String>("dependency-package")
        .into_iter()
        .flatten()
        .map(|value| handle(value))
        .collect::<Result<Vec<_>, _>>()
    {
        Ok(value) => value,
        Err(_) => {
            return static_failed(
                receipt,
                "package-build",
                "dependency_package_invalid",
                "dependency-package-invalid-v1",
                "dependency-envelope",
                "dependency package handle is invalid",
            )
        }
    };
    let request = BuildDefinitionPackageRequest::builder()
        .source(match handle(required(arguments, "source")) {
            Ok(value) => value,
            Err(error) => return receipt.unexpected(&error),
        })
        .package(package)
        .dependency_packages(dependency_packages)
        .output(match handle(required(arguments, "output")) {
            Ok(value) => value,
            Err(error) => return receipt.unexpected(&error),
        })
        .try_build();
    let outcome = request.and_then(|request| application.build(request));
    match outcome {
        Ok(NapeOutcome::Completed(value)) => {
            let output = match application_identity_local::existing_file_uri(Path::new(
                value.output.value(),
            )) {
                Ok(value) => value,
                Err(error) => return receipt.unexpected(&error),
            };
            receipt.success(
                "package-build",
                json!({
                    "integrity": "digest-verified",
                    "manifestDigest": value.identity.manifest_digest().value(),
                    "output": output,
                    "package": value.identity.purl().value(),
                    "trust": "not-evaluated",
                }),
            )
        }
        Ok(NapeOutcome::Rejected(value)) => receipt.failed("package-build", &value, None),
        Err(error) => receipt.unexpected(&error),
    }
}

#[cfg(test)]
mod tests;
