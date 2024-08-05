use notify_rust::{Notification, Timeout};

use crate::error::{FocusAction, FocusError};

const APP_NAME: &str = "Focus";
const DEFAULT_MESSAGE: &str = "Time's Up!";

pub fn display_notification(message: Option<String>) -> FocusAction {
  Notification::new()
    .summary(APP_NAME)
    .subtitle(&message.unwrap_or_else(|| DEFAULT_MESSAGE.to_owned()))
    .timeout(Timeout::Milliseconds(3000)) //milliseconds
    .show()
    .map_err(|e| FocusError::CouldNotCreateNotification(e.to_string()))
    .map(|_| ())
}
