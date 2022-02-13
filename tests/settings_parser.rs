use std::{fs, io::Write};
use matbot::config::Config;

#[test]
#[should_panic]
fn unavailable_json_settings() {
    let conf = Config::from_json_file("unavailable_json_settings.json");
    conf.unwrap();
}

#[test]
fn available_json_settings() {
    let conf = Config::from_json_file("settings.json");
    conf.unwrap();
}

#[test]
fn valid_json_settings() {
    let json_settings = br#"
        {
            "discord_token": "discord token value",
            "database_path": "database path value",
            "main_channel_id": "main channel id value",
            "alternative_channel_id": "alternative channel id value"
        }
    "#;

    let valid_file = "target/tmp/valid_file.json";

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(valid_file).unwrap();

    file.write(json_settings).unwrap();

    let conf = Config::from_json_file(valid_file).unwrap();

    assert_eq!(conf.discord_token, "discord token value");
    assert_eq!(conf.database_path, "database path value");
    assert_eq!(conf.main_channel_id, "main channel id value");
    assert_eq!(conf.alternative_channel_id, "alternative channel id value");
}

#[test]
#[should_panic]
fn unvalid_json_settings() {
    let json_settings = br#"
        {
            "discord_token": "discod oken value",
            "database_path": "databae path value",
            "main_channel_id": "main chnnel id value",
            "alternative_channel_id": "alternative channel id value"
        }
    "#;

    let unvalid_file = "target/tmp/unvalid_file.json";

    let mut file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .open(unvalid_file).unwrap();

    file.write(json_settings).unwrap();

    let conf = Config::from_json_file(unvalid_file).unwrap();

    assert_eq!(conf.discord_token, "discord token value");
    assert_eq!(conf.database_path, "database path value");
    assert_eq!(conf.main_channel_id, "main channel id value");
    assert_eq!(conf.alternative_channel_id, "alternative channel id value");
}

