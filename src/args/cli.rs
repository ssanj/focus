use clap::Parser;

/// A pomodoro timer in the cli
#[derive(Parser, Debug, Clone)]
#[command(author, version, about)]
pub struct Args {
   /// Verbose debug logging
   #[arg(long)]
   pub verbose: bool
}

pub fn get_cli_args() -> Args {
  Args::parse()
}
