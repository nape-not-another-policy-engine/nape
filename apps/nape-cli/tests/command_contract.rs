#![cfg(unix)]

pub mod support;

/// Requirement validation: current authored examples build deterministically.
#[test]
fn current_examples_are_exact_command_inputs_success() {
    support::checked_current_v2_examples_build_directly_and_deterministically();
}
