mod bluesky;
mod cli;
mod config;
mod daemon;
mod db;
mod garmin;
mod github;

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result {
    cli::run().await
}
