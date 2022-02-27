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
            "discord token": "discord token value",
            "database path": "database path value",
            "main channel id": 47,
            "alternative channel id": 86,
            "material types": [
                ["OS", "kernel.org"]
            ],
            "application id": 64
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
    assert_eq!(conf.main_channel_id, 47);
    assert_eq!(conf.alternative_channel_id, 86);
    assert_eq!(conf.material_types[0].0, "OS");
    assert_eq!(conf.material_types[0].1, "kernel.org");
    assert_eq!(conf.app_id, 64);
}

#[test]
#[should_panic]
fn unvalid_json_settings() {
    let json_settings = br#"
        {
            "discord_token": "discod oken value",
            "database_path": "databae path value",
            "main_channel_id": 1,
            "alternative_channel_id": 1
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
    assert_eq!(conf.main_channel_id, 0);
    assert_eq!(conf.alternative_channel_id, 0);
}

