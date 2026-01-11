use chrono::{DateTime, Utc};
use futures_util::StreamExt as _;
use serde::Deserialize;
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message;

use crate::Result;

#[derive(Debug, Deserialize)]
struct JetstreamMessage {
    time_us: u64,
    #[serde(default)]
    commit: Option<JetstreamCommit>,
}

#[derive(Debug, Deserialize)]
pub struct JetstreamCommit {
    pub operation: CommitOperation,
    pub rkey: String,
    #[serde(default)]
    pub record: Option<PostRecord>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum CommitOperation {
    Create,
    Delete,
    Update,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PostRecord {
    pub created_at: DateTime<Utc>,
    pub text: String,
}

pub async fn subscribe_to_posts(did: &str, output_tx: mpsc::Sender<JetstreamCommit>) -> Result {
    let (mut stream, _) = tokio_tungstenite::connect_async(
        format!("wss://jetstream2.us-east.bsky.network/subscribe?wantedCollections=app.bsky.feed.post&wantedDids={did}"),
    )
    .await?;

    while let Some(msg) = stream.next().await {
        let msg = msg?;

        let Message::Text(text) = msg else {
            continue;
        };

        let JetstreamMessage {
            commit: Some(commit),
            ..
        } = serde_json::from_str(&text)?
        else {
            continue;
        };

        if output_tx.send(commit).await.is_err() {
            break;
        }
    }

    Ok(())
}
