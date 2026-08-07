//! `nape package resolve` adapter.

use clap::ArgMatches;
use nape_domain::{
    diagnostic::NapeOutcome,
    usecase::resolve_definition_package_plan::ResolveDefinitionPackagePlanRequest,
};

use crate::{
    composition::NapeApplication,
    gateway_driver::registry_configuration_attestify_oci::RegistryConfigurationFailure,
    io_adapter::clap::{
        dispatch::{identity, registry_map, resolve_plan_value, static_failed},
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
            return receipt.failed("package-resolve-plan", &value, None)
        }
        Err(RegistryConfigurationFailure::Unexpected(error)) => return receipt.unexpected(&error),
    };
    let identity = match identity(arguments) {
        Ok(value) => value,
        Err(value) => {
            return static_failed(
                receipt,
                "package-resolve-plan",
                value.0,
                value.1,
                "package-reference-validation",
                value.2,
            )
        }
    };
    let request = ResolveDefinitionPackagePlanRequest::new(identity);
    match application.resolve(request, map.clone()).await {
        Ok(NapeOutcome::Completed(value)) => match resolve_plan_value(&value, &map) {
            Ok(value) => receipt.success("package-resolve-plan", value),
            Err(error) => receipt.unexpected(&error),
        },
        Ok(NapeOutcome::Rejected(value)) => receipt.failed("package-resolve-plan", &value, None),
        Err(error) => receipt.unexpected(&error),
    }
}

#[cfg(test)]
mod tests;
