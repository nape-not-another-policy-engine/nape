//! Verifies source-name independence and optional destination-name assertion.
//!
//! Requirement validation points:
//! - Accepted FB-05 preserves file-only and explicit filename forms.

use test_framework_oss::is_ok;

use super::select_evidence_requirement;
use crate::value::{
    effective_graph::CanonicalActionSelector,
    evidence::{EvidenceFileName, EvidenceRequirement},
};

/// Requirement validation: no source basename is needed to select the definition filename.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn file_only_requirement_selection_success() {
    let action = is_ok!(CanonicalActionSelector::try_new(
        "release-readiness.database-connection"
    ));
    let requirement = is_ok!(EvidenceRequirement::try_new(
        action.clone(),
        "application-configuration",
        is_ok!(EvidenceFileName::try_new("application-configuration.json")),
        1_024,
    ));

    let selected = is_ok!(select_evidence_requirement(&[requirement], &action, None));
    assert_eq!(selected.file().value(), "application-configuration.json");
}
