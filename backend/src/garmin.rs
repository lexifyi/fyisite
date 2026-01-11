use std::{sync::mpsc, time::Duration};

use headless_chrome::Browser;
use serde::Deserialize;

use crate::Result;

pub struct Scraper {
    browser: Browser,
    user_uuid: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResponseJson {
    pub activity_list: Vec<ActivityJson>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityJson {
    pub activity_id: u64,
    pub activity_type: ActivityType,
    pub duration: f64,
    #[serde(default, rename = "averageHR")]
    pub average_hr: f64,
    #[serde(default)]
    pub distance: f64,
    #[serde(default)]
    pub bmr_calories: f64,
    #[serde(default)]
    pub calories: f64,
    #[serde(default)]
    pub speed: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityType {
    pub type_id: u32,
    pub type_key: String,
    pub parent_type_id: u32,
}

impl Scraper {
    pub fn new(user_uuid: impl Into<String>) -> Result<Self> {
        Ok(Self {
            browser: Browser::default()?,
            user_uuid: user_uuid.into(),
        })
    }

    pub async fn get_activities(&self) -> Result<ResponseJson> {
        let browser = self.browser.clone();
        let user_uuid = &self.user_uuid;
        let html_url = format!("https://connect.garmin.com/app/profile/{user_uuid}");
        let api_url = format!(
            "https://connect.garmin.com/gc-api/activitylist-service/activities/{user_uuid}"
        );

        tokio::task::spawn_blocking(move || {
            let tab = browser.new_tab()?;
            let (tx, rx) = mpsc::channel();

            tab.register_response_handling(
                "girl in your walls reading your json",
                Box::new(move |ev, fetch_body| {
                    if !ev.response.url.starts_with(&api_url) {
                        return;
                    }

                    let output = fetch_body()
                        .and_then(|b| serde_json::from_str(&b.body).map_err(anyhow::Error::from));
                    let _ = tx.send(output);
                }),
            )?;

            tab.navigate_to(&html_url)?;
            rx.recv_timeout(Duration::from_secs(15))?
        })
        .await
        .unwrap()
    }
}
