use std::fmt::Debug;

use serde::Serialize;
use tokio::sync::RwLock;
use tokio_postgres::{NoTls, types::Json};

use crate::{Result, cli};

static CLIENT: RwLock<Option<tokio_postgres::Client>> = RwLock::const_new(None);

pub struct Connection {
    client: tokio_postgres::Client,
}

pub async fn connect(db_url: &str) -> Result<Connection> {
    let (client, connection) = tokio_postgres::connect(db_url, NoTls).await?;

    tokio::spawn(async move {
        if let Err(err) = connection.await {
            log::error!("{err}");
        }
    });

    Ok(Connection { client })
}

impl Connection {
    pub async fn add_event(
        &self,
        event_type: &str,
        external_id: &str,
        meta: &impl Serialize,
    ) -> Result {
        let meta = serde_json::to_string(meta)?;

        self.client
            .execute(
                "INSERT INTO events (event_type, external_id, meta) VALUES ($1, $2, $3::jsonb)",
                &[&event_type, &external_id, &meta],
            )
            .await?;

        Ok(())
    }
}
