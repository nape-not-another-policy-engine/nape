//! Verifies fixed continuation semantics.
//!
//! Requirement validation points:
//! - Contained outcomes continue; terminal boundary loss blocks remaining Actions.

use super::{decide, ContinuationDecision, ExecutionState};

/// Requirement validation: a contained inconclusive Action does not terminate the Procedure.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn contained_action_continues_success() {
    assert_eq!(
        decide(ExecutionState::Contained),
        ContinuationDecision::Continue
    );
}

/// Requirement validation: a terminal boundary failure stops further execution.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn terminal_action_stops_error() {
    assert_eq!(decide(ExecutionState::Terminal), ContinuationDecision::Stop);
}
