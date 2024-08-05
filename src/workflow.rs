use crate::args::cli;
use crate::error::FocusAction;
use crate::time::TimerDuration;
use crate::timer::{display_timer, dont_display_timer};
use crate::notification::display_notification;
use crate::sound::play_sound;

pub fn perform() {
  match perform_action() {
    Ok(_) => (),
    Err(error) => eprintln!("{}", error)
  }
}

pub fn perform_action() -> FocusAction {
  let args = cli::get_cli_args();
  let message = args.message;
  let no_sound = args.no_sound;
  let no_countdown = args.no_countdown;
  let no_notification = args.no_notification;
  let figlet_file = args.figlet_file;
  let sound_file = args.sound_file;
  let time_duration = TimerDuration::try_new(args.time)?;


  if no_countdown {
    dont_display_timer(time_duration)
  } else {
    display_timer(figlet_file, time_duration)?
  }

  if !no_notification {
    display_notification(message)?;
  }

  if !no_sound{
    play_sound(sound_file)? // This blocks the thread, run it last
  }

  Ok(())
}
