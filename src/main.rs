use serenity::{
    framework::standard::{
        StandardFramework,
        CommandResult,
        macros::{
            command,
            group
        },
    },
    prelude::*,
};

use  matbot::{
    config::Config,
    materials::MatRow,
    event_handler::Handler
};

#[group]
struct General;

#[tokio::main]
async fn main() {
    let conf = Config::from_json_file("settings.json");

    let conf = match conf {
        Ok(conf) => conf,
        Err(error) => panic!("{}", error)
    };

    println!("config:\n{:?}", conf);

    let table = MatRow::vec_from_database(&conf.database_path,
                                          &conf.material_types[0].0);
    let table = match table {
        Ok(table) => table,
        Err(error) => panic!("{}", error)
    };

    println!("table [{:?}]:\n{:?}",
             conf.material_types[0], table);

    let framework = StandardFramework::new()
        .configure(|c| c.prefix(":"))
        .group(&GENERAL_GROUP);

    let mut client = Client::builder(&conf.discord_token)
        .event_handler(Handler)
        .framework(framework).await;

    let mut client = match client {
        Ok(client) => client,
        Err(error) => panic!("{}", error)
    };

    if let Err(why) = client.start().await {
        eprintln!("An error occurred while running the client: {:?}", why);
    }
}
