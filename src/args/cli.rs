use std::path::PathBuf;

use clap::Parser;

/// A Pomodoro timer in the cli
#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct Args {
   /// Verbose debug logging
   #[arg(long)]
   pub verbose: bool,

   /// Number of minutes to run the timer for
   #[arg(long, short)]
   pub minutes: u8,

   /// Notification message to display, when the timer is up.
   #[arg(long, short='s')]
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

pub fn get_cli_args() -> Args {
  Args::parse()
}
