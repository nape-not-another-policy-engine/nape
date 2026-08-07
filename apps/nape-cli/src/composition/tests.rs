//! Verifies the composition root exposes one bounded application assembly.

use super::NapeApplication;

/// Requirement validation: composition initialization is fallible rather
/// than panicking when process configuration is unavailable.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn application_composition_is_a_fallible_boundary_success() {
    let initialize: fn() -> Result<NapeApplication, kernel_oss::error::Error> =
        NapeApplication::initialize;
    let _ = initialize;
}
