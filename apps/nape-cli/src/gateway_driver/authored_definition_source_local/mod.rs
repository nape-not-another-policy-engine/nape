//! Stable local authored-definition source driver.

#[cfg(test)]
mod tests;

use std::path::Path;

use kernel_oss::{error::Error, gateway::Gateway};
use nape_domain::{
    diagnostic::NapeOutcome,
    gateway::authored_definition_source::{
        AuthoredDefinitionSourceGW, AuthoredDefinitionSourceRequest, SealedAuthoredDefinition,
    },
};

use super::{
    definition_package_profile_attestify_oci::{
        definition_kind_for_package, definition_package_profile,
    },
    package_support::translate_package_error,
};

/// Seals one local authored source tree before semantic admission.
pub struct LocalAuthoredDefinitionSourceDriver;

impl Gateway for LocalAuthoredDefinitionSourceDriver {
    type Request = AuthoredDefinitionSourceRequest;
    type Response = NapeOutcome<SealedAuthoredDefinition>;

    fn execute(&self, request: Self::Request) -> Result<Self::Response, Error> {
        let profile = definition_package_profile()?;
        let kind = definition_kind_for_package(request.package().value())?;
        match attestify_oci::seal_package_source(
            profile,
            kind,
            Path::new(request.source().value()),
            request.package().value(),
        ) {
            Ok(value) => Ok(NapeOutcome::Completed(SealedAuthoredDefinition::new(
                value.semantic_root(),
                value.semantic_root_media_type(),
                value.files().clone(),
            ))),
            Err(error) => Ok(NapeOutcome::Rejected(translate_package_error(error)?)),
        }
    }
}

impl AuthoredDefinitionSourceGW for LocalAuthoredDefinitionSourceDriver {}
