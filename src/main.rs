use args::cli;
use timer::display_timer;
use notification::display_notification;
use sound::play_sound;

mod args;
mod timer;
mod notification;
mod sound;

fn main() {
  let args = cli::get_cli_args();
  let minutes = args.minutes as i32;
  let message = args.message;

  display_timer(minutes);
  display_notification(message);
  play_sound(); // This blocks the thread, run it last
}

