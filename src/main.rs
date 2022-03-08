use log::*;
use env_logger;
use serenity::{
    framework::standard::{
        StandardFramework,
        macros::group
    },
    prelude::*,
};

use  matbot::{
    config::Config,
    materials,
    event_handler::Handler,
};

#[group]
struct General;

#[tokio::main]
async fn main() {
    env_logger::init_from_env(
        env_logger::Env::default().filter_or(env_logger::DEFAULT_FILTER_ENV, "matbot=info"));

    info!("Logging Enabled");

    if let Err(error) = materials::build_database() {
        error!("{}", error);
        panic!("{}", error);
    } else {
        info!("material type tables updated");
    }

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(":"))
        .group(&GENERAL_GROUP);

    let conf = Config::from_json_file("settings.json").unwrap();

    let client = Client::builder(&conf.discord_token)
        .event_handler(Handler)
        .application_id(conf.app_id)
        .framework(framework).await;

    let mut client = match client {
        Ok(client) => client,
        Err(error) =>  {
            error!("{}", error);
            panic!("{}", error);
        }
    };

    if let Err(why) = client.start().await {
        error!("An error occurred while running the client: {:#?}", why);
    }
}
