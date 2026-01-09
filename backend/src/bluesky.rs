use chrono::{DateTime, Utc};
use futures_util::StreamExt as _;
use headless_chrome::Browser;
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
struct JetstreamCommit {
    operation: CommitOperation,
    rkey: String,
    #[serde(default)]
    record: Option<PostRecord>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
enum CommitOperation {
    Create,
    Delete,
    Update,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PostRecord {
    created_at: DateTime<Utc>,
    text: String,
}

async fn read_posts(did: &str, output_tx: mpsc::Sender<JetstreamCommit>) -> Result {
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

        output_tx.send(commit);
    }

    Ok(())
}
