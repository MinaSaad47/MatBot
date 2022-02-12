use  disbot::config::Config;

fn main() {
    let conf = Config::from_json_file("settings.json");

    let conf = match conf {
        Ok(conf) => conf,
        Err(error) => panic!("{}", error)
    };

    println!("config: {:?}", conf);
}
