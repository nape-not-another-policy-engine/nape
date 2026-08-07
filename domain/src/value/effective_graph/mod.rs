//! Deterministic effective Procedure occurrence graph values.

#[cfg(test)]
mod tests;

use kernel_oss::error::{Error, Kind};

use super::{
    controlled_value::ControlledValue, evaluation::EffectiveEvaluation,
    package::PackageReleaseIdentity,
};

/// One canonical contextual Action selector: `<activity>.<action>`.
#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct CanonicalActionSelector(String);

impl CanonicalActionSelector {
    /// Creates an exact two-segment selector.
    pub fn try_new(value: impl Into<String>) -> Result<Self, Error> {
        let value = value.into();
        let segments = value.split('.').collect::<Vec<_>>();
        if segments.len() != 2 || !segments.iter().all(|segment| canonical_name(segment)) {
            return Err(Error::for_user(
                Kind::InvalidInput,
                "Action selector must be <activity>.<action> using canonical names",
            ));
        }
        Ok(Self(value))
    }

    /// Returns the exact selector.
    pub fn value(&self) -> &str {
        &self.0
    }
}

fn canonical_name(value: &str) -> bool {
    !value.is_empty()
        && !value.starts_with('-')
        && !value.ends_with('-')
        && value.chars().all(|character| {
            character.is_ascii_lowercase() || character.is_ascii_digit() || character == '-'
        })
}

/// One effective Action occurrence and its package owner.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectiveActionOccurrence {
    selector: CanonicalActionSelector,
    package_owner: PackageReleaseIdentity,
    detail: Option<EffectiveActionDetail>,
}

impl EffectiveActionOccurrence {
    /// Creates one contextual occurrence.
    pub fn new(selector: CanonicalActionSelector, package_owner: PackageReleaseIdentity) -> Self {
        Self {
            selector,
            package_owner,
            detail: None,
        }
    }

    /// Creates one fully resolved executable occurrence.
    pub fn with_detail(
        selector: CanonicalActionSelector,
        package_owner: PackageReleaseIdentity,
        detail: EffectiveActionDetail,
    ) -> Self {
        Self {
            selector,
            package_owner,
            detail: Some(detail),
        }
    }

    /// Returns the contextual selector.
    pub fn selector(&self) -> &CanonicalActionSelector {
        &self.selector
    }

    /// Returns the exact package owning static Action bytes.
    pub fn package_owner(&self) -> &PackageReleaseIdentity {
        &self.package_owner
    }

    /// Returns complete resolved execution and reporting semantics.
    pub fn detail(&self) -> Option<&EffectiveActionDetail> {
        self.detail.as_ref()
    }
}

/// Whether a definition is embedded in its owner or independently packaged.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DefinitionOriginType {
    /// Definition bytes belong to the parent package.
    Embedded,
    /// Definition bytes belong to their own exact package release.
    Package,
}

/// Exact definition ownership used by resolution, execution, and reporting.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DefinitionOwner {
    /// Product definition kind.
    pub kind: String,
    /// Definition-owned name.
    pub name: String,
    /// Exact package release owning the bytes.
    pub package: PackageReleaseIdentity,
    /// Embedded or independently packaged origin.
    pub origin_type: DefinitionOriginType,
}

/// Optional restricted schema selected by an Action Evidence declaration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectiveEvidenceSchema {
    /// Controlled schema media type.
    pub media_type: String,
    /// Package-owned schema path.
    pub file: String,
}

/// One exact Action-owned Python helper module.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectiveTestModule {
    /// Python import name.
    pub name: String,
    /// Exact package-owned path.
    pub file: String,
}

/// Complete resolved Action occurrence semantics.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectiveActionDetail {
    /// Procedure definition-owned name and label.
    pub procedure_name: String,
    pub procedure_label: String,
    /// Contextual Activity occurrence data.
    pub activity_name: String,
    pub activity_label: String,
    pub activity_description: Option<String>,
    pub activity_example: Option<String>,
    /// Contextual Action occurrence and package-owned presentation data.
    pub action_name: String,
    pub action_label: String,
    pub action_description: Option<String>,
    pub action_example: Option<String>,
    pub claim: String,
    /// Exact owner and Activity origin.
    pub action_owner: DefinitionOwner,
    pub activity_owner: DefinitionOwner,
    /// Package-owned Test declaration and static location.
    pub test_name: String,
    pub test_file: String,
    pub package_test_path: String,
    pub test_modules: Vec<EffectiveTestModule>,
    /// Definition-owned Evidence declaration and canonical occurrence path.
    pub evidence_name: String,
    pub evidence_file: String,
    pub evidence_input_path: String,
    pub evidence_media_type: Option<String>,
    pub evidence_schema: Option<EffectiveEvidenceSchema>,
    /// Effective package defaults plus authorized parent assignments.
    pub evaluations: Vec<EffectiveEvaluation>,
    pub criterion_origins: Vec<ControlledValue>,
    /// Effective profile or authored limits and their origins.
    pub maximum_evidence_bytes: u64,
    pub maximum_execution_work_units: u64,
    pub maximum_result_bytes: u64,
    pub evidence_maximum_authored: bool,
    pub execution_work_authored: bool,
    pub result_bytes_authored: bool,
}

/// Complete deterministic Procedure occurrence graph.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectiveVerificationGraph {
    /// Ordered executable Action occurrences.
    pub occurrences: Vec<EffectiveActionOccurrence>,
    /// Effective unique-Evidence aggregate ceiling.
    pub maximum_invocation_evidence_bytes: u64,
    /// Whether the Procedure authored the aggregate ceiling.
    pub invocation_evidence_limit_authored: bool,
    /// Count of contextual Activity occurrences.
    pub activity_count: u64,
}
