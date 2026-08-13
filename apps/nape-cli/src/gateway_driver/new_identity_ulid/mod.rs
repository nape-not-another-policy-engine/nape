//! System-backed ULID identity driver.

#[cfg(test)]
mod tests;

use std::{
    fs::File,
    io::Read,
    time::{SystemTime, UNIX_EPOCH},
};

use kernel_oss::{
    error::{Error, Kind},
    gateway::{new_identity::NewIdentityGW, VoidGateway},
    ulid::ULID,
};

/// Produces one time-ordered ULID from the current clock and OS randomness.
pub struct UlidIdentityDriver;

impl VoidGateway for UlidIdentityDriver {
    type Response = ULID;

    fn execute(&self) -> Result<Self::Response, Error> {
        let milliseconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| Error::for_system(Kind::GatewayError, "system clock predates UNIX epoch"))?
            .as_millis();
        if milliseconds >= (1_u128 << 48) {
            return Err(Error::for_system(
                Kind::GatewayError,
                "system time exceeds the ULID time range",
            ));
        }
        let mut random = [0_u8; 10];
        File::open("/dev/urandom")
            .and_then(|mut source| source.read_exact(&mut random))
            .map_err(|_| Error::for_system(Kind::GatewayError, "OS randomness is unavailable"))?;
        let randomness = random
            .into_iter()
            .fold(0_u128, |value, byte| (value << 8) | u128::from(byte));
        Ok(ULID::from_parts(milliseconds as u64, randomness))
    }
}

impl NewIdentityGW for UlidIdentityDriver {}
