use log::*;

use serenity::{
    async_trait,
    model::{
        channel::Message,
        gateway::Ready,
        interactions::{
            application_command::{
                ApplicationCommand,
                ApplicationCommandOptionType,
            },
            Interaction,
            InteractionResponseType,
        },
    },
    prelude::*,
};

#[allow(unused_imports)]
use crate::{
    config::Config,
    commands::utils::*,
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
        let commands =
            ApplicationCommand::set_global_application_commands(&ctx.http,
                                                                |cmds| {
                cmds.create_application_command(|cmd| {
                    cmd.name("version").description("Display MatBot version")
                })
                .create_application_command(|cmd| {
                    cmd.name("display")
                        .description("get material resources of a certain type")
                        .create_option(|opt| {
                            opt
                                .name("material")
                                .description("display material resources")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true);
                            for (material, _) in &conf.material_types {
                                info!("regisering '{}' choice", material);
                                opt.add_string_choice(material, material);
                            }
                            opt
                        })
                })
                .create_application_command(|cmd| {
                    cmd.name("add")
                        .description("add material resources")
                        .create_option(|opt| {
                            opt
                                .name("material")
                                .description("choice a material to add to")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true);
                            for (material, _) in &conf.material_types {
                                opt.add_string_choice(&material, &material);
                            }
                            opt
                        })
                        .create_option(|opt| {
                            opt
                                .name("name")
                                .description("the new material resource name")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                        })
                        .create_option(|opt| {
                            opt
                                .name("url")
                                .description("the url to this resource")
                                .kind(ApplicationCommandOptionType::String)
                                .required(true)
                        })
                })
            }).await;

        debug!("set_global_application_commands():\n{:?}", commands);
    }
}


