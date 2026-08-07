//! Bounded values owned by NAPE Verification V2 Domain contracts.

pub mod controlled_value;
pub mod current_verification;
pub mod definition;
pub mod effective_graph;
pub mod evaluation;
pub mod evidence;
pub mod external_resource_handle;
pub mod invocation_metadata;
pub mod package;
pub mod subject;
pub mod verification_outcome;

use kernel_oss::error::{Error, Kind};

pub(crate) fn bounded_nonempty(
    value: impl Into<String>,
    field: &str,
    maximum_bytes: usize,
) -> Result<String, Error> {
    let value = value.into();
    if value.trim().is_empty() {
        return Err(Error::for_user(
            Kind::InvalidInput,
            format!("{field} must not be empty"),
        ));
    }
    if value.len() > maximum_bytes {
        return Err(Error::for_user(
            Kind::ExceedsMax,
            format!("{field} exceeds {maximum_bytes} UTF-8 bytes"),
        ));
    }
    Ok(value)
}
