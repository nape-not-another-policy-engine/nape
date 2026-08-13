//! Verifies transport-neutral definition candidates.
//!
//! Requirement validation points:
//! - Verification definition kinds are closed.

use test_framework_oss::is_ok;

use super::{DefinitionCandidate, DefinitionKind};
use crate::value::controlled_value::ControlledValue;

/// Requirement validation: an exact Procedure candidate is admitted.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn procedure_candidate_success() {
    let candidate = is_ok!(DefinitionCandidate::try_new(
        DefinitionKind::VerificationProcedure,
        "release-readiness",
        ControlledValue::Null,
    ));

    assert_eq!(candidate.name(), "release-readiness");
    assert_eq!(candidate.document(), &ControlledValue::Null);
}
