use std::io::Write;
use std::thread;
use std::time::Duration;

use args::cli;
use jiff::{civil::DateTime, ToSpan, Zoned};
use figlet_rs::FIGfont;
use notify_rust::{Notification, Timeout};

use std::fs::File;
use std::io::BufReader;
use rodio::{Decoder, OutputStream, source::Source};


mod args;

fn main() {
  let args = cli::get_cli_args();

  let minutes = args.minutes as i32;

  thread::scope(|s| {
    s.spawn(|| {
      let standard_font = FIGfont::standard().unwrap();
      let mut now: DateTime = Zoned::now().datetime();
      let now_plus_minutes_requested = now.checked_add(minutes.minutes()).unwrap();
      let diff = now_plus_minutes_requested.since(now).unwrap();
      let diff_string = format!("{}:{}:{}", diff.get_hours(), diff.get_minutes(), diff.get_seconds());
      let fig_diff = standard_font.convert(&diff_string).unwrap();
      let lines = fig_diff.height;
      println!("{}", standard_font.convert(&diff_string).unwrap());

      // Clear used lines from bottom to top
      // See: https://gist.github.com/fnky/458719343aabd01cfb17a3a4f7296797
      while now <= now_plus_minutes_requested {
        for _ in 0 .. lines+1 {
          // \x1b[1A - Move up a line
          // \x1b[0K - Erase from cursor until end of line
          print!("\x1b[1A\x1b[0K");
        }

        let diff = now_plus_minutes_requested.since(now).unwrap();
        let diff_string = format!("{}:{}:{}", diff.get_hours(), diff.get_minutes(), diff.get_seconds());
        println!("{}", standard_font.convert(&diff_string).unwrap());
        thread::sleep(Duration::from_millis(250));
        now = Zoned::now().datetime();
      }

    });
  });

  display_notification();
  play_sound();
}


fn display_notification() {
    Notification::new()
        .summary("Focus")
        .subtitle("Time's Up!")
        .timeout(Timeout::Milliseconds(3000)) //milliseconds
        .show()
        .unwrap();
}


fn play_sound() {

// https://soundbible.com/1630-Computer-Magic.html

  // Get an output stream handle to the default physical sound device
  let (_stream, stream_handle) = OutputStream::try_default().unwrap();
  // Load a sound from a file, using a path relative to Cargo.toml
  let file = BufReader::new(File::open("/Users/sanj/Downloads/Computer_Magic-Microsift-1901299923.mp3").unwrap());
  // Decode that sound file into a source
  let source = Decoder::new(file).unwrap();
  // Play the sound directly on the device
  let _ = stream_handle.play_raw(source.convert_samples());

  // The sound plays in a separate audio thread,
  // so we need to keep the main thread alive while it's playing.
  std::thread::sleep(std::time::Duration::from_secs(5));
}


