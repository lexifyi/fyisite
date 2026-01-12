use clap::Parser;

use crate::{Result, daemon};

/// Backend management tool for lexi.fyi.
#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Parser)]
pub enum Command {
    /// Run the backend daemon
    Daemon,
}

pub async fn run() -> Result {
    let args = Args::parse();

    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .init();

    match args.command {
        Command::Daemon => {
            daemon::run().await?;
        }
    }

    Ok(())
}
