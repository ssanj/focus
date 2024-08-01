use args::cli;
use timer::{display_timer, dont_display_timer};
use notification::display_notification;
use sound::play_sound;

mod args;
mod timer;
mod notification;
mod sound;

fn main() {
  // TODO: Move this to a Workflow
  let args = cli::get_cli_args();
  let minutes = args.minutes.into();
  let message = args.message;
  let no_sound = args.no_sound;
  let no_countdown = args.no_countdown;
  let no_notification = args.no_notification;

  if no_countdown {
    dont_display_timer(minutes)
  } else {
    display_timer(minutes)
  }

  if !no_notification {
    display_notification(message);
  }

  if !no_sound{
    play_sound(); // This blocks the thread, run it last
  }
}

