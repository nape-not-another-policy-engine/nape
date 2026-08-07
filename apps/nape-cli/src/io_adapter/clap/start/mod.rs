//! `nape start` adapter.

use clap::ArgMatches;
use nape_domain::{
    diagnostic::NapeOutcome,
    usecase::start_verification::{StartVerificationRequest, VerificationProcedureSource},
    value::{invocation_metadata::InvocationMetadataEntry, package::PackageReleaseIdentity},
};

use crate::{
    composition::NapeApplication,
    gateway_driver::registry_configuration_attestify_oci::RegistryConfigurationFailure,
    io_adapter::clap::dispatch::{
        handle, manifest_digest, normal_diagnostic, normal_error, purl, registry_map, required,
    },
};

pub(super) async fn dispatch(application: &NapeApplication, arguments: &ArgMatches) -> i32 {
    let (source, registry_map) = if let Some(package) = arguments.get_one::<String>("package") {
        let digest = required(arguments, "manifest-digest");
        let map = match registry_map(arguments) {
            Ok(value) => value,
            Err(RegistryConfigurationFailure::Rejected(value)) => return normal_diagnostic(&value),
            Err(RegistryConfigurationFailure::Unexpected(error)) => return normal_error(error),
        };
        (
            VerificationProcedureSource::Oci(PackageReleaseIdentity::new(
                match purl(package) {
                    Ok(value) => value,
                    Err(error) => return normal_error(error),
                },
                match manifest_digest(digest) {
                    Ok(value) => value,
                    Err(error) => return normal_error(error),
                },
            )),
            Some(map),
        )
    } else {
        (
            VerificationProcedureSource::Local(
                match handle(required(arguments, "local-package")) {
                    Ok(value) => value,
                    Err(error) => return normal_error(error),
                },
            ),
            None,
        )
    };
    let metadata = arguments
        .get_many::<String>("meta")
        .into_iter()
        .flatten()
        .cloned()
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|pair| InvocationMetadataEntry::try_new(pair[0].clone(), pair[1].clone()))
        .collect::<Result<Vec<_>, _>>();
    let subject = match handle(required(arguments, "subject-file")) {
        Ok(value) => value,
        Err(error) => return normal_error(error),
    };
    let request = metadata.and_then(|metadata| {
        StartVerificationRequest::builder()
            .source(source)
            .subject(subject)
            .metadata(metadata)
            .try_build()
    });
    match request {
        Ok(request) => match application.start(request, registry_map).await {
            Ok(NapeOutcome::Completed(_)) => 0,
            Ok(NapeOutcome::Rejected(value)) => normal_diagnostic(&value),
            Err(error) => normal_error(error),
        },
        Err(error) => normal_error(error),
    }
}

#[cfg(test)]
mod tests;
