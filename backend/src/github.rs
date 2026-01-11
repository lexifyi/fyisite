use std::fmt::Display;

use chrono::{DateTime, Utc};
use serde::Deserialize;

use crate::Result;

pub struct Client {
    reqwest: reqwest::Client,
    user_name: String,
    authorization: String,
}

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

impl Client {
    pub fn new(user_name: impl Into<String>, auth_token: impl Display) -> Result<Self> {
        Ok(Self {
            reqwest: reqwest::Client::builder()
                .user_agent("lexi.fyi backend")
                .build()?,
            user_name: user_name.into(),
            authorization: format!("Bearer {auth_token}"),
        })
    }

    pub async fn get_events(&self) -> Result<Vec<Event>> {
        let path = format!("https://api.github.com/users/{}/events", &self.user_name);
        let req = self
            .reqwest
            .get(path)
            .header("Authorization", &self.authorization);
        let res = req.send().await?;
        let body = res.text().await?;
        let events: Vec<Event> = serde_json::from_str(&body)?;

        Ok(events)
    }
}
