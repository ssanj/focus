use std::thread;
use std::time::Duration;
use jiff::{civil::DateTime, ToSpan, Zoned};
use figlet_rs::FIGfont;


pub fn display_timer(minutes: i32) {
  thread::scope(|s| {
      s.spawn(|| {
        let mut now: DateTime = Zoned::now().datetime();
        let now_plus_minutes_requested = now.checked_add(minutes.minutes()).unwrap();
        let diff = now_plus_minutes_requested.since(now).unwrap();

        let standard_font = FIGfont::standard().unwrap();
        let diff_string = format!("{:02}:{:02}:{:02}", diff.get_hours(), diff.get_minutes(), diff.get_seconds());
        let fig_diff = standard_font.convert(&diff_string).unwrap();
        let lines = fig_diff.height;
        println!("{}", standard_font.convert(&diff_string).unwrap());

        // Clear used lines from bottom to top
        // We go from bottom to top, so the next line is ready to be printed
        // See: https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
        while now <= now_plus_minutes_requested {
          for _ in 0 .. lines+1 {
            // \x1b[1A - Move up a line
            // \x1b[0K - Erase from cursor until end of line
            print!("\x1b[1A\x1b[0K");
          }

          let diff = now_plus_minutes_requested.since(now).unwrap();
          let diff_string = format!("{:02}:{:02}:{:02}", diff.get_hours(), diff.get_minutes(), diff.get_seconds());
          println!("{}", standard_font.convert(&diff_string).unwrap());
          thread::sleep(Duration::from_millis(250));
          now = Zoned::now().datetime();
        }
      });
    });
}
