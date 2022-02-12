use disbot::disbot::config::Config;

fn main() {
    let config = r#"
        {
            "discord_token": "discord token value",
            "database_path": "database path value",
            "main_channel_id": "main channel id value",
            "alternative_channel_id": "alternative channel id value"
        }
    "#;

    let res = serde_json::from_str::<Config>(config);

    if res.is_ok() {
        println!("Deserialized: {:?}", res.unwrap());
    } else {
        panic!("Error Deserializing");
    }

}
