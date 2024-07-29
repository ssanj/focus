use std::io::Write;
use std::thread;
use std::time::Duration;

use args::cli;
use jiff::{civil::DateTime, ToSpan, Zoned};

mod args;

fn main() {
  let _args = cli::get_cli_args();

  thread::scope(|s| {
    s.spawn(|| {
      let mut now: DateTime = Zoned::now().datetime();
      let now_plus_five = now.checked_add(5.seconds()).unwrap();

      while now <= now_plus_five {
        let diff = now_plus_five.since(now).unwrap();
        print!("\r{}:{}:{}", diff.get_hours(), diff.get_minutes(), diff.get_seconds());
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(250));
        now = Zoned::now().datetime();
      }
    });
  })
}

