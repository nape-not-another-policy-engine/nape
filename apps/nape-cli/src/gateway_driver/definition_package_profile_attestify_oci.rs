//! NAPE-owned Verification package meaning supplied to Attestify OCI OSS.

use attestify_oci::{
    ExactPackageDependencyRule, ExactPackageKindProfile, ExactPackageProfile, PackageError,
};
use kernel_oss::error::{Error, Kind};

const ARTIFACT_TYPE: &str = "application/vnd.attestify.definition-package.v1";
const CONFIG_MEDIA_TYPE: &str = "application/vnd.attestify.definition-package.config.v1+json";
const LAYER_MEDIA_TYPE: &str = "application/vnd.attestify.package.v1.tar";
const PROFILE_VERSION: &str = "attestify-oci-repository-profile/1";
const PURL_TYPE: &str = "attestify";

const PROCEDURE_KIND: &str = "VerificationProcedure";
const ACTIVITY_KIND: &str = "VerificationActivity";
const ACTION_KIND: &str = "VerificationAction";

/// Builds the exact Verification package profile owned by NAPE.
pub(crate) fn definition_package_profile() -> Result<ExactPackageProfile, Error> {
    let kinds = vec![
        ExactPackageKindProfile::try_new(
            PROCEDURE_KIND,
            "verification-procedure",
            "verification-procedure.yaml",
            "application/yaml",
        ),
        ExactPackageKindProfile::try_new(
            ACTIVITY_KIND,
            "verification-activity",
            "verification-activity.yaml",
            "application/yaml",
        ),
        ExactPackageKindProfile::try_new(
            ACTION_KIND,
            "verification-action",
            "verification-action.yaml",
            "application/yaml",
        ),
    ]
    .into_iter()
    .collect::<Result<Vec<_>, _>>()
    .map_err(profile_error)?;
    let dependencies = vec![
        ExactPackageDependencyRule::try_new(PROCEDURE_KIND, ACTIVITY_KIND),
        ExactPackageDependencyRule::try_new(PROCEDURE_KIND, ACTION_KIND),
        ExactPackageDependencyRule::try_new(ACTIVITY_KIND, ACTION_KIND),
    ]
    .into_iter()
    .collect::<Result<Vec<_>, _>>()
    .map_err(profile_error)?;
    ExactPackageProfile::try_new(
        ARTIFACT_TYPE,
        CONFIG_MEDIA_TYPE,
        LAYER_MEDIA_TYPE,
        PROFILE_VERSION,
        PURL_TYPE,
        kinds,
        dependencies,
    )
    .map_err(profile_error)
}

/// Selects the exact Verification kind encoded by one admitted package PURL.
pub(crate) fn definition_kind_for_package(package: &str) -> Result<&'static str, Error> {
    let coordinate = package
        .strip_prefix("pkg:attestify/")
        .and_then(|value| value.split_once('@').map(|(coordinate, _)| coordinate))
        .ok_or_else(invalid_definition_package)?;
    let mut segments = coordinate.split('/');
    let _publisher = segments.next().ok_or_else(invalid_definition_package)?;
    match segments.next() {
        Some("verification-procedure") => Ok(PROCEDURE_KIND),
        Some("verification-activity") => Ok(ACTIVITY_KIND),
        Some("verification-action") => Ok(ACTION_KIND),
        _ => Err(invalid_definition_package()),
    }
}

fn profile_error(_: PackageError) -> Error {
    Error::for_system(
        Kind::ProcessingFailure,
        "NAPE Verification package profile is invalid",
    )
}

fn invalid_definition_package() -> Error {
    Error::for_user(
        Kind::InvalidInput,
        "package PURL does not identify a supported Verification definition kind",
    )
}

#[cfg(test)]
mod tests {
    use super::{definition_kind_for_package, definition_package_profile};

    #[test]
    fn exact_profile_is_constructible_success() {
        definition_package_profile().expect("Verification package profile should be valid");
    }

    #[test]
    fn exact_three_definition_kinds_are_selected_success() {
        assert_eq!(
            definition_kind_for_package(
                "pkg:attestify/acme.example/verification-procedure/readiness/check@1.0.0"
            )
            .expect("procedure kind should be supported"),
            "VerificationProcedure"
        );
        assert_eq!(
            definition_kind_for_package(
                "pkg:attestify/acme.example/verification-activity/readiness/check@1.0.0"
            )
            .expect("activity kind should be supported"),
            "VerificationActivity"
        );
        assert_eq!(
            definition_kind_for_package(
                "pkg:attestify/acme.example/verification-action/readiness/check@1.0.0"
            )
            .expect("action kind should be supported"),
            "VerificationAction"
        );
        assert!(definition_kind_for_package(
            "pkg:attestify/acme.example/risk-evaluation/readiness/check@1.0.0"
        )
        .is_err());
    }
}
