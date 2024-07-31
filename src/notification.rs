use notify_rust::{Notification, Timeout};

pub fn display_notification(message: Option<String>) {
    Notification::new()
      .summary("Focus")
      .subtitle(&message.unwrap_or_else(|| "Time's Up!".to_owned()))
      .timeout(Timeout::Milliseconds(3000)) //milliseconds
      .show()
      .unwrap();
}
