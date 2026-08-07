//! System UTC timestamp driver.

#[cfg(test)]
mod tests;

use std::time::{SystemTime, UNIX_EPOCH};

use kernel_oss::{
    error::{Error, Kind},
    gateway::{current_utc_timestamp::CurrentUTCTimestampGW, VoidGateway},
    values::datetime::utc_timestamp::UTCTimestamp,
};

/// Returns the current UTC instant with millisecond precision.
pub struct SystemUtcTimestampDriver;

impl VoidGateway for SystemUtcTimestampDriver {
    type Response = UTCTimestamp;

    fn execute(&self) -> Result<Self::Response, Error> {
        let milliseconds = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| Error::for_system(Kind::GatewayError, "system clock predates UNIX epoch"))?
            .as_millis();
        let bounded = u64::try_from(milliseconds).map_err(|_| {
            Error::for_system(
                Kind::GatewayError,
                "system clock exceeds the UTC timestamp range",
            )
        })?;
        UTCTimestamp::builder().use_ms(bounded).build()
    }
}

impl CurrentUTCTimestampGW for SystemUtcTimestampDriver {}
