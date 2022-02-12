#[allow(unused_imports)]
use std::fs;
use serde::{Serialize, Deserialize};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    discord_token: String,
    database_path: String,
    main_channel_id: String,
    alternative_channel_id: String
}
