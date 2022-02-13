#[allow(unused_imports)]
use std::fs;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub discord_token: String,
    pub database_path: String,
    pub main_channel_id: String,
    pub alternative_channel_id: String
}

impl Config {
    pub fn from_json_file(file_path: &str) -> Result<Self, &str> {
        let res = fs::read_to_string(file_path);
        if res.is_err() {
            return Err("could not open 'settings.json'")
        }
        let json_content = res.unwrap();
        let res = serde_json::from_str::<Config>(&json_content);
        if res.is_err() {
            return Err("error parsing 'settings.json'");
        }
        Ok(res.unwrap())
    }
}
