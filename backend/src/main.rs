mod bluesky;
mod cli;
mod config;
mod garmin;
mod github;

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

#[tokio::main]
async fn main() -> Result {
    env_logger::Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .init();

    let config = config::load()?;

    println!("{config:#?}");

    Ok(())
}
