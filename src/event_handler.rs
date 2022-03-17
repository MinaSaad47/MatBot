use log::*;

use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::Ready,
        interactions::{
        application_command::ApplicationCommand,
        Interaction,
        InteractionResponseType,
        },
    },
    prelude::*,
};

#[allow(unused_imports)]
use crate::{
    config::Config,
    commands::{
        requests,
        responses,
    },
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    #[allow(unused_variables)]
    async fn message(&self, ctx: Context, msg: Message) {

    }

    #[allow(unused_variables)]
    async fn interaction_create(&self, ctx: Context, int: Interaction) {
        if let Interaction::ApplicationCommand(command) = int {
            let resdata = match command.data.name.as_str() {
                "version" => responses::version(),
                "display" => responses::display(&command.data.options),
                "update" => responses::update(&command.data.options, &command.user),
                "publish" => responses::publish(&ctx.http).await,
                _ => unreachable!()
            };
            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(move |res| {
                            *res = resdata;
                            res
                        })
                })
                .await
            {
                error!("Cannot respond to slash command: {}", why);
            }
        }
    }

    #[allow(unused_variables)]
    async fn ready(&self, ctx: Context, rdy: Ready) {
        info!("{} is connected", rdy.user.name);

        let conf = Config::from_json_file("settings.json").unwrap();
        let material_types = conf.material_types.iter()
            .map(|(material, _)| {
                material.clone()
            }).collect();
        let commands =
            ApplicationCommand::set_global_application_commands(&ctx.http,
                                                                |cmds| {
                cmds.set_application_commands(vec![
                    requests::version(),
                    requests::display(&material_types),
                    requests::update(&material_types),
                    requests::publish(),
                ])
            }).await;

        debug!("set_global_application_commands():\n{:#?}", &commands);
    }
}


