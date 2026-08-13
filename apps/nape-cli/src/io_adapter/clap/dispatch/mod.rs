//! Bounded Clap-to-Use-Case command dispatcher.

#[cfg(test)]
mod tests;

use clap::ArgMatches;
use kernel_oss::error::{Error, Kind};
use nape_domain::{
    diagnostic::NapeDiagnostic,
    value::{
        external_resource_handle::ExternalResourceHandle,
        package::{ManifestDigest, PackageReleaseIdentity, PackageReleasePurl},
    },
};
use serde_json::{json, Value};

use crate::{
    composition::NapeApplication,
    gateway_driver::registry_configuration_attestify_oci::{
        acquire_registry_map, project_registry_location, RegistryConfigurationFailure,
        RegistryConfigurationSource,
    },
    io_adapter::clap::{
        evidence, package_build, package_publish, package_resolve_plan, receipt::ReceiptAdapter,
        start, verify,
    },
};

/// Dispatches one already-admitted top-level command.
pub async fn dispatch(matches: &ArgMatches) -> i32 {
    let application = match NapeApplication::initialize() {
        Ok(value) => value,
        Err(error) => return unexpected(error),
    };
    let receipt = ReceiptAdapter::new(application.build_digest());
    match matches.subcommand() {
        Some(("package", arguments)) => match arguments.subcommand() {
            Some(("build", arguments)) => {
                package_build::dispatch(&application, &receipt, arguments)
            }
            Some(("publish", arguments)) => {
                package_publish::dispatch(&application, &receipt, arguments).await
            }
            Some(("resolve", arguments)) => {
                package_resolve_plan::dispatch(&application, &receipt, arguments).await
            }
            _ => unreachable!("Clap admits one package subcommand"),
        },
        Some(("start", arguments)) => start::dispatch(&application, arguments).await,
        Some(("evidence", arguments)) => evidence::dispatch(&application, arguments),
        Some(("verify", _)) => verify::dispatch(&application, &receipt).await,
        _ => unreachable!("Clap admits one closed top-level command"),
    }
}

pub(super) fn resolve_plan_value(
    plan: &nape_domain::usecase::resolve_definition_package_plan::DefinitionPackagePlan,
    map: &attestify_oci_oss::registry::RegistryMap,
) -> Result<Value, Error> {
    let root = plan.closure.root();
    let location = project_registry_location(map, root.identity())?;
    let mut dependency = Vec::new();
    for edge in root.lock().edges() {
        let package = plan
            .closure
            .dependencies()
            .iter()
            .find(|candidate| candidate.identity().purl() == &edge.to)
            .ok_or_else(|| {
                Error::for_system(
                    Kind::ProcessingFailure,
                    "verified Lock Edge target is absent from the closure",
                )
            })?;
        let mapped = project_registry_location(map, package.identity())?;
        let location = digest_location_value(&mapped)?;
        dependency.push(json!({
            "disposition": "planned-not-acquired",
            "location": location,
            "manifestDigest": package.identity().manifest_digest().value(),
            "package": package.identity().purl().value(),
            "use": edge.use_selector,
            "from": edge.from.value(),
        }));
    }
    let location = digest_location_value(&location)?;
    Ok(json!({
        "dependency": dependency,
        "lock": {
            "lockVersion": root.lock().lock_version(),
            "profileVersion": root.lock().profile_version(),
            "validation": "passed",
        },
        "root": {
            "disposition": "verified",
            "integrity": "digest-verified",
            "location": location,
            "manifestDigest": root.identity().manifest_digest().value(),
            "package": root.identity().purl().value(),
            "trust": "not-evaluated",
        },
    }))
}

pub(super) fn location_observation(
    value: &nape_domain::gateway::definition_package_publication::RegistryLocationObservation,
) -> Result<Value, Error> {
    if value.reference_class != "tag" {
        return Err(unsupported_reference_class());
    }
    Ok(json!({
        "profileVersion": value.profile_version,
        "publisher": value.publisher,
        "reference": value.reference,
        "referenceClass": value.reference_class,
        "registry": value.registry,
        "repository": value.repository,
        "scheme": value.scheme,
    }))
}

fn digest_location_value(
    value: &attestify_oci_oss::registry::RegistryLocation,
) -> Result<Value, Error> {
    if value.reference_class != "manifest-digest" {
        return Err(unsupported_reference_class());
    }
    Ok(json!({
        "profileVersion": value.profile_version,
        "publisher": value.publisher,
        "reference": value.reference,
        "referenceClass": "digest",
        "registry": value.registry,
        "repository": value.repository,
        "scheme": value.scheme,
    }))
}

fn unsupported_reference_class() -> Error {
    Error::for_system(
        Kind::ProcessingFailure,
        "Registry location has an unsupported Receipt reference class",
    )
}

pub(super) fn summary_value(
    value: &nape_domain::value::verification_outcome::VerificationSummary,
) -> Value {
    json!({
        "activity_count": value.activity_count,
        "action_count": value.action_count,
        "actions_completed": value.actions_completed,
        "actions_terminated": value.actions_terminated,
        "actions_blocked": value.actions_blocked,
        "conclusion_true": value.conclusion_true,
        "conclusion_false": value.conclusion_false,
        "conclusion_inconclusive": value.conclusion_inconclusive,
        "diagnostic_count": value.diagnostic_count,
    })
}

pub(super) fn registry_map(
    arguments: &ArgMatches,
) -> Result<attestify_oci_oss::registry::RegistryMap, RegistryConfigurationFailure> {
    match (
        arguments.get_one::<String>("registry-profile"),
        arguments.get_one::<String>("registry-endpoint"),
    ) {
        (Some(path), None) => acquire_registry_map(RegistryConfigurationSource::Profile(path)),
        (None, Some(endpoint)) => {
            acquire_registry_map(RegistryConfigurationSource::Endpoint(endpoint))
        }
        _ => Err(RegistryConfigurationFailure::Unexpected(Error::for_system(
            Kind::ProcessingFailure,
            "Clap did not enforce the Registry configuration group",
        ))),
    }
}

pub(super) fn identity(
    arguments: &ArgMatches,
) -> Result<PackageReleaseIdentity, (&'static str, &'static str, &'static str)> {
    let package = purl(required(arguments, "package")).map_err(|_| {
        (
            "package_purl_invalid",
            "package-purl-invalid-v1",
            "package PURL is invalid",
        )
    })?;
    let digest = manifest_digest(required(arguments, "manifest-digest")).map_err(|_| {
        (
            "package_manifest_digest_invalid",
            "package-manifest-digest-invalid-v1",
            "manifest digest is invalid",
        )
    })?;
    Ok(PackageReleaseIdentity::new(package, digest))
}

pub(super) fn required<'a>(arguments: &'a ArgMatches, name: &str) -> &'a str {
    arguments
        .get_one::<String>(name)
        .map(String::as_str)
        .unwrap_or_else(|| unreachable!("Clap requires {name}"))
}

pub(super) fn purl(value: &str) -> Result<PackageReleasePurl, Error> {
    PackageReleasePurl::try_new(value)
}

pub(super) fn manifest_digest(value: &str) -> Result<ManifestDigest, Error> {
    ManifestDigest::try_new(value)
}

pub(super) fn handle<T>(value: &str) -> Result<T, Error>
where
    T: From<ExternalResourceHandle>,
{
    Ok(ExternalResourceHandle::try_new(value)?.into())
}

pub(super) fn static_failed(
    receipt: &ReceiptAdapter,
    command: &str,
    code: &str,
    reason_template: &str,
    phase: &str,
    detail: &str,
) -> i32 {
    match NapeDiagnostic::try_new(code, reason_template, phase, detail) {
        Ok(value) => receipt.failed(command, &value, None),
        Err(error) => receipt.unexpected(&error),
    }
}

pub(super) fn normal_diagnostic(value: &NapeDiagnostic) -> i32 {
    eprintln!("{}: {}", value.code(), value.detail());
    1
}

pub(super) fn normal_error(value: Error) -> i32 {
    eprintln!("NAPE: {}", value.message());
    1
}

fn unexpected(value: Error) -> i32 {
    eprintln!("NAPE initialization failure: {}", value.message());
    1
}
