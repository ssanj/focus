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
   pub minutes: u8
}

pub fn get_cli_args() -> Args {
  Args::parse()
}
