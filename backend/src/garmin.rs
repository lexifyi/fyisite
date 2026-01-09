use std::{sync::mpsc, thread::sleep, time::Duration};

use headless_chrome::Browser;
use serde::Deserialize;

use crate::Result;

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

pub fn get_activities(browser: Browser, user_id: &str) -> Result<ResponseJson> {
    let tab = browser.new_tab()?;
    let (tx, rx) = mpsc::channel();

    let api_url =
        format!("https://connect.garmin.com/gc-api/activitylist-service/activities/{user_id}");

    tab.register_response_handling(
        "json guy",
        Box::new(move |ev, fetch_body| {
            if !ev.response.url.starts_with(&api_url) {
                return;
            }

            let _ = tx.send(
                fetch_body()
                    .and_then(|b| serde_json::from_str(&b.body).map_err(anyhow::Error::from)),
            );
        }),
    )?;

    tab.navigate_to(&format!("https://connect.garmin.com/app/profile/{user_id}"))?;

    rx.recv_timeout(Duration::from_secs(15))?
}
