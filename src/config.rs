use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    #[serde(rename(deserialize = "discord token"))]
    pub discord_token: String,
    #[serde(rename(deserialize = "application id"))]
    pub app_id: u64,
    #[serde(rename(deserialize = "database path"))]
    pub database_path: String,
    #[serde(rename(deserialize = "main channel id"))]
    pub main_channel_id: u64,
    #[serde(rename(deserialize = "alternative channel id"))]
    pub alternative_channel_id: u64,
    #[serde(rename(deserialize = "material types"))]
    pub material_types: Vec<(String, String)>,
}

impl Config {
    pub fn from_json_file(file_path: &str) -> Result<Self, &str> {
        let res = fs::read_to_string(file_path);
        if res.is_err() {
            return Err("could not open 'settings.json'");
        }
        let json_content = res.unwrap();
        let res = serde_json::from_str::<Config>(&json_content);
        if res.is_err() {
            return Err("error parsing 'settings.json'");
        }
        Ok(res.unwrap())
    }
}
