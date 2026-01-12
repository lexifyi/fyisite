use crate::{Result, config, db};

pub async fn run() -> Result {
    let config = config::load()?;
    let db = db::connect(&config.postgres).await?;

    db.add_event("test", "1", &(1, true, "hello")).await?;

    Ok(())
}
