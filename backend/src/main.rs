use std::{thread::sleep, time::Duration};

use headless_chrome::Browser;
use serde::Deserialize;

type Result<T = (), E = anyhow::Error> = std::result::Result<T, E>;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ResponseJson {
    activity_list: Vec<ActivityJson>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ActivityJson {
    activity_id: u64,
    activity_type: ActivityType,
    duration: f64,
    #[serde(default, rename = "averageHR")]
    average_hr: f64,
    #[serde(default)]
    distance: f64,
    #[serde(default)]
    bmr_calories: f64,
    #[serde(default)]
    calories: f64,
    #[serde(default)]
    speed: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct ActivityType {
    type_id: u32,
    type_key: String,
    parent_type_id: u32,
}

fn main() -> Result {
    let browser = Browser::default()?;
    let tab = browser.new_tab()?;

    tab.register_response_handling(
        "json guy",
        Box::new(|ev, fetch_body| {
            if ev.response.url.starts_with(
                "https://connect.garmin.com/gc-api/activitylist-service/activities/[redacted]",
            ) && let Ok(json) = fetch_body()
            {
                println!("{:#?}", serde_json::from_str::<ResponseJson>(&json.body));
            }
        }),
    )?;

    tab.navigate_to("https://connect.garmin.com/app/profile/[redacted]")?;

    sleep(Duration::from_hours(1));

    Ok(())
}
