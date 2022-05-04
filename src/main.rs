use log::*;
use env_logger;
use ansi_term::Color;
use serenity::{
    framework::standard::{
        StandardFramework,
        macros::group
    },
    prelude::*,
};

use  matbot::{
    config::CONF,
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

    let client = Client::builder(&CONF.discord_token, GatewayIntents::empty())
        .event_handler(Handler)
        .application_id(CONF.app_id)
        .framework(framework).await;

    let mut client = match client {
        Ok(client) => client,
        Err(error) =>  {
            error!("{}", error);
            panic!("{}", error);
        }
    };
    info!("{}", Color::Yellow.paint("starting the bot ..."));
    if let Err(why) = client.start().await {
        error!("An error occurred while running the client: {:#?}", why);
    }
}
