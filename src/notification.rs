use notify_rust::{Notification, Timeout};

pub fn display_notification() {
    Notification::new()
      .summary("Focus")
      .subtitle("Time's Up!")
      .timeout(Timeout::Milliseconds(3000)) //milliseconds
      .show()
      .unwrap();
}
