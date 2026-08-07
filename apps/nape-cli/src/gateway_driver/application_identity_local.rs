//! Application-owned executable identity and canonical local URI projection.

use std::path::Path;

use kernel_oss::error::{Error, Kind};
use sha2::{Digest, Sha256};

/// Hashes the exact running executable bytes selected by the composition root.
pub fn executable_digest(path: &Path) -> Result<String, Error> {
    let bytes = super::local_file::read_regular_stable(path, 512 * 1_024 * 1_024)?;
    Ok(format!("sha256:{}", hex::encode(Sha256::digest(bytes))))
}

/// Projects one existing local path into a canonical absolute file URI.
pub fn existing_file_uri(path: &Path) -> Result<String, Error> {
    let path = std::fs::canonicalize(path).map_err(|_| {
        Error::for_user(
            Kind::InvalidInput,
            "receipt output location does not exist or cannot be canonicalized",
        )
    })?;
    let text = path.to_str().ok_or_else(|| {
        Error::for_system(Kind::GatewayError, "receipt output location is not UTF-8")
    })?;
    Ok(format!("file://{text}"))
}
