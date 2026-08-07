//! Verifies one-file evidence requirements.
//!
//! Requirement validation points:
//! - Staged Evidence accepts one caller source and preserves the definition filename.

use test_framework_oss::is_ok;

use super::{EvidenceFileName, EvidenceRequirement};
use crate::value::effective_graph::CanonicalActionSelector;

/// Requirement validation: a definition-owned evidence filename is retained.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn evidence_requirement_success() {
    let requirement = is_ok!(EvidenceRequirement::try_new(
        is_ok!(CanonicalActionSelector::try_new(
            "release-readiness.database-connection"
        )),
        "application-configuration",
        is_ok!(EvidenceFileName::try_new("application-configuration.json")),
        268_435_456,
    ));

    assert_eq!(requirement.file().value(), "application-configuration.json");
}
