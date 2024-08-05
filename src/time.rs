use crate::args::cli::Time;
use crate::error::{FocusError, FocusType};

pub enum TimerDuration {
  Minutes(u8),
  Seconds(u8)
}

impl TimerDuration {
  pub fn try_new(time: Time) -> FocusType<Self> {
    match (time.seconds, time.minutes) {
      (Some(seconds), None) => Ok(TimerDuration::Seconds(seconds)),
      (None, Some(minutes)) => Ok(TimerDuration::Minutes(minutes)),
      (Some(_), Some(_)) => Err(FocusError::InvalidTimeCombinationMultipleTimesSupplied),
      (None, None) => Err(FocusError::InvalidTimeCombinationNoTimeSupplied),
    }
  }
}
