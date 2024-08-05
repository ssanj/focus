use std::path::PathBuf;

use clap::{Args as ClapArgs, Parser};

/// A Pomodoro timer on the cli
#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct Args {
   /// Verbose debug logging
   #[arg(long)]
   pub verbose: bool,

   #[command(flatten)]
   pub time: Time,

   /// Notification message to display, when the timer is up.
   #[arg(long, short='g')]
   pub message: Option<String>,

   /// Turn off melody at the end of the timer
   #[arg(long)]
   pub no_sound: bool,

   /// Turn off countdown
   #[arg(long)]
   pub no_countdown: bool,

   /// Turn off notification
   #[arg(long)]
   pub no_notification: bool,

   /// Supply an alternate Figlet font.
   /// See: http://www.figlet.org/fontdb.cgi
   #[arg(long)]
   pub figlet_file: Option<PathBuf>,

   /// A sound file to use. Accepted formats are .mp3, .wave, .flac and .vorbis
   #[arg(long)]
   pub sound_file: Option<PathBuf>,
}


#[derive(ClapArgs, Debug, Clone)]
#[group(required = true, multiple = false)]
pub struct Time {

   /// Number of minutes to run the timer for
  #[arg(short)]
  pub minutes: Option<u8>,

  /// Number of seconds to run the timer for
  #[arg(short)]
  pub seconds: Option<u8>,
}

pub fn get_cli_args() -> Args {
  Args::parse()
}
