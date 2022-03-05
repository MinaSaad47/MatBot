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
        res::*,
        cmds::*,
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
                "version" => version_res_msg(),
                "display" => display_res_msg(&command.data.options),
                "update" => update_res_msg(&command.data.options, &command.user),
                "publish" => publish_res_msg(&ctx.http).await,
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
                info!("found '{}' material type", material);
                material.clone()
            }).collect();
        let commands =
            ApplicationCommand::set_global_application_commands(&ctx.http,
                                                                |cmds| {
                cmds.set_application_commands(vec![
                    version_app_cmd(),
                    display_app_cmd(&material_types),
                    update_app_cmd(&material_types),
                    publish_app_cmd(),
                ])
            }).await;

        debug!("set_global_application_commands():\n{:#?}", &commands);
    }
}


