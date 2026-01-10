use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::Result;

#[derive(Debug, Deserialize)]
pub struct Event {
    pub id: String,
    #[serde(rename = "type")]
    pub event_type: String,
    pub public: bool,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub repo: Option<Repo>,
    pub payload: EventPayload,
}

#[derive(Debug, Deserialize)]
pub struct Repo {
    name: String,
    url: String,
}

#[derive(Debug, Deserialize)]
pub struct EventPayload {
    #[serde(default, rename = "ref")]
    pub ref_name: Option<String>,
    #[serde(default)]
    pub full_ref_name: Option<String>,
    #[serde(default)]
    pub head: Option<String>,
    #[serde(default)]
    pub before: Option<String>,
}

pub async fn get_events(user_name: &str, authorization: &str) -> Result<Vec<Event>> {
    let client = reqwest::Client::builder()
        .user_agent("lexi.fyi backend")
        .build()?;

    let res = client
        .get(format!("https://api.github.com/users/{user_name}/events"))
        .header("Authorization", authorization)
        .send()
        .await?;

    let text = res.text().await?;
    let events: Vec<Event> = serde_json::from_str(&text)?;

    Ok(events)
}
