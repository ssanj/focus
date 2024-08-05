use std::{path::PathBuf, thread};
use std::time::Duration;
use jiff::{civil::DateTime, ToSpan, Zoned};
use figlet_rs::FIGfont;

use crate::error::{FocusAction, FocusError};
use crate::time::TimerDuration;

pub fn display_timer(figlet_file: Option<PathBuf>, timer_duration: TimerDuration) -> FocusAction {
  thread::scope(|s| {
      let handle = s.spawn(|| {
        let mut now: DateTime = Zoned::now().datetime();
        let now_plus_duration_requested = match timer_duration {
            TimerDuration::Minutes(minutes) => now.checked_add((minutes as i32).minutes()).unwrap(),
            TimerDuration::Seconds(seconds) => now.checked_add((seconds as i32).seconds()).unwrap(),
        };

        let diff = now_plus_duration_requested.since(now).unwrap();

        let default_font = FIGfont::standard().map_err(|e| FocusError::CouldNotLoadDefaultFont(e.to_string()))?;

        let figlet_font_result: Option<Result<FIGfont, FocusError>> =
          figlet_file
            .map(|f| {
              FIGfont::from_file(&f.to_string_lossy()).map_err(|e| FocusError::CouldNotOpenCustomFigletFont(f.to_string_lossy().to_string(), e.to_string()))
            });


        let figlet_font = figlet_font_result.unwrap_or(Ok(default_font))?;

        let diff_string = format!("{:02}:{:02}:{:02}", diff.get_hours(), diff.get_minutes(), diff.get_seconds());

        let fig_diff =
          figlet_font
            .convert(&diff_string)
            .ok_or_else(|| FocusError::CouldNotConvertStringToFiglet(diff_string.clone()))?;

        let lines = fig_diff.height;
        println!("{}", fig_diff);

        // Clear used lines from bottom to top
        // We go from bottom to top, so the next line is ready to be printed
        // See: https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
        while now <= now_plus_duration_requested {
          for _ in 0 .. lines+1 {
            // \x1b[1A - Move up a line
            // \x1b[0K - Erase from cursor until end of line
            print!("\x1b[1A\x1b[0K");
          }

          let diff = now_plus_duration_requested.since(now).unwrap();
          let diff_string = format!("{:02}:{:02}:{:02}", diff.get_hours(), diff.get_minutes(), diff.get_seconds());
          println!("{}", figlet_font.convert(&diff_string).unwrap());
          thread::sleep(Duration::from_millis(250));
          now = Zoned::now().datetime();
        };

        Ok(())
      });

      handle.join().unwrap()
    })
}

pub fn dont_display_timer(timer_duration: TimerDuration) {
  let duration = match timer_duration {
    TimerDuration::Minutes(minutes) => Duration::from_secs((minutes * 60).into()),
    TimerDuration::Seconds(seconds) => Duration::from_secs((seconds).into()),
  };

  thread::sleep(duration)
}
