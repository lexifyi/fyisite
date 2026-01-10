use std::fs;

use serde::Deserialize;

use crate::Result;

#[derive(Debug, Default, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub garmin: Option<GarminConfig>,
    #[serde(default)]
    pub github: Option<GithubConfig>,
    #[serde(default)]
    pub bluesky: Option<BlueskyConfig>,
}

#[derive(Debug, Deserialize)]
pub struct GarminConfig {
    pub profile_uuid: String,
}

#[derive(Debug, Deserialize)]
pub struct GithubConfig {
    pub user_name: String,
    pub token: String,
}

#[derive(Debug, Deserialize)]
pub struct BlueskyConfig {
    pub did: String,
}

pub fn load() -> Result<Config> {
    let Some(mut config_path) = dirs::config_dir() else {
        return Ok(Config::default());
    };

    config_path.push("lexifyi");
    config_path.push("fyisite.toml");

    if !fs::exists(&config_path)? {
        return Ok(Config::default());
    }

    let toml = fs::read_to_string(config_path)?;
    let config = toml::from_str(&toml)?;

    Ok(config)
}
