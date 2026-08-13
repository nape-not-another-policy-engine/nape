#![cfg(unix)]

pub mod support;

/// Requirement validation: staged Verify invokes the real Evaluator process.
#[test]
fn real_evaluator_process_is_used_by_verify_success() {
    support::verify_runs_one_occurrence_and_commits_or_leaves_no_output();
}
