use std::io::Write;
use std::thread;
use std::time::Duration;

use args::cli;
use jiff::{civil::DateTime, ToSpan, Zoned};
use figlet_rs::FIGfont;

mod args;

fn main() {
  let _args = cli::get_cli_args();

  thread::scope(|s| {
    s.spawn(|| {
      let standard_font = FIGfont::standard().unwrap();
      // TODO: Have a way to specify a font file
      // let standard_font = FIGfont::from_file("bulbhead.flf").unwrap();
      let mut now: DateTime = Zoned::now().datetime();
      let now_plus_five = now.checked_add(5.seconds()).unwrap();
      let diff = now_plus_five.since(now).unwrap();
      let diff_string = format!("{}:{}:{}", diff.get_hours(), diff.get_minutes(), diff.get_seconds());
      let fig_diff = standard_font.convert(&diff_string).unwrap();
      let lines = fig_diff.height;
      println!("{}", standard_font.convert(&diff_string).unwrap());

      while now <= now_plus_five {
        for line in 0 .. lines+1 {
          print!("\x1b[1A\x1b[0K");
        }

        let diff = now_plus_five.since(now).unwrap();
        let diff_string = format!("{}:{}:{}", diff.get_hours(), diff.get_minutes(), diff.get_seconds());
        let fig_diff = standard_font.convert(&diff_string).unwrap();
        println!("{}", standard_font.convert(&diff_string).unwrap());
        thread::sleep(Duration::from_millis(250));
        now = Zoned::now().datetime();
      }
    });
  })
}


