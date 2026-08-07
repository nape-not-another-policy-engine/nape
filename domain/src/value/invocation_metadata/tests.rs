//! Verifies V1-compatible invocation metadata normalization.
//!
//! Requirement validation points:
//! - VerificationReport V2 invocation metadata amendment.

use kernel_oss::error::Kind;
use test_framework_oss::is_ok;

use super::{InvocationMetadata, InvocationMetadataEntry};

/// Requirement validation: normalized duplicate metadata uses the last value.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn metadata_last_normalized_duplicate_wins_success() {
    let metadata = InvocationMetadata::from_entries(vec![
        is_ok!(InvocationMetadataEntry::try_new("Build-ID", " first ")),
        is_ok!(InvocationMetadataEntry::try_new("build-id", "second")),
    ]);

    assert_eq!(
        metadata.values().get("build-id").map(String::as_str),
        Some("second")
    );
}

/// Requirement validation: Engine-owned Report keys cannot be supplied by callers.
/// Requirement validation: exercises one bounded logical path.
#[test]
fn engine_owned_metadata_key_error() {
    let error =
        InvocationMetadataEntry::try_new("utc-start", "1").expect_err("reserved key must fail");
    assert_eq!(error.kind(), Kind::InvalidInput);
}
