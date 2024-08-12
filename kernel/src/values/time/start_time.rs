use std::time::{SystemTime, UNIX_EPOCH};
use crate::error::{Error, Kind};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StartTime {
    pub time: u128,
}

impl StartTime {

    /// # Overview
    /// Creates a new [`StartTime`] struct with the current time in milliseconds.
    ///
    /// # Returns
    ///
    /// * A [`StartTime`] struct.
    pub fn now() -> Self {
        Self { time: get_current_time_in_milliseconds() }
    }

    /// # Overview
    /// Creates a new StartTime struct from a u128 time value.
    ///
    /// # Arguments
    ///
    /// * `time` - A u128 value representing the time in milliseconds.
    ///
    /// # Returns
    ///
    /// * A [`StartTime`] struct.
    /// * If the argument is less than 0, the time value will be set to 0.  No other validation is performed.
    pub fn from(time: u128) -> Self {
        Self { time }
    }

    ///  # Overview
    /// Creates a new StartTime struct from a u128 time value, and validates that the time is greater than 0.
    ///
    /// # Arguments
    ///
    /// * `time` - A u128 value representing the time in milliseconds.
    ///
    /// # Returns
    ///
    /// * A [`Result<StartTime, Error>`] containing the [`StartTime`] struct if the time is greater than 0, or an [`Error`] if the time is less than or equal to 0.
    /// * The Error will be of [`Kind::InvalidInput`] for  [`Audience::User`].
    ///
    pub fn try_from(time: u128) -> Result<Self, Error> {
        if time == 0 {
            return Err(Error::for_user(Kind::InvalidInput,
                                         "The start time cannot be 0.  Please provide a valid utc time.".to_string()));
        }
        Ok(Self { time })
    }

    /// # Overview
    /// Converts the [`StartTime`] struct to a string.
    ///
    pub fn to_string(&self) -> String {
        self.time.to_string()
    }

}

fn get_current_time_in_milliseconds() -> u128 {
    let now = SystemTime::now();
    let since_the_epoch = now.duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
    since_the_epoch.as_millis()
}