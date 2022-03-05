use serenity::{
    builder::{CreateApplicationCommand, CreateApplicationCommandOption},
    model::interactions::application_command::ApplicationCommandOptionType,
};

type AppCommand = CreateApplicationCommand;
type CommandOpt = CreateApplicationCommandOption;

pub fn version_app_cmd() -> AppCommand {
    let mut cmd = AppCommand::default();
    cmd.name("version").description("Request MatBot Version.");
    cmd
}

pub fn display_app_cmd(materials: &Vec<String>) -> AppCommand {
    let mut cmd = AppCommand::default();
    cmd.name("display")
        .description("get material resources of a certain type")
        .set_options(vec![materials_cmd_opt(materials, true)]);
    cmd
}

pub fn update_app_cmd(materials: &Vec<String>) -> AppCommand {
    let mut cmd = AppCommand::default();
    cmd.name("update")
        .description("update existing material type")
        .create_option(|opt| {
            opt.name("method")
                .description("a method to apply to a type")
                .kind(ApplicationCommandOptionType::SubCommandGroup)
                .create_sub_option(|sub_cmd| {
                    sub_cmd
                        .name("add")
                        .description("add a resource to the type")
                        .kind(ApplicationCommandOptionType::SubCommand)
                        .add_sub_option(materials_cmd_opt(materials, true))
                        .create_sub_option(|name_value| {
                            name_value
                                .name("name")
                                .required(true)
                                .description("Enter a new resource name")
                                .kind(ApplicationCommandOptionType::String)
                        })
                        .create_sub_option(|url_value| {
                            url_value
                                .name("url")
                                .required(true)
                                .description("Enter a new reource url")
                                .kind(ApplicationCommandOptionType::String)
                        })
                })
                .create_sub_option(|sub_cmd| {
                    sub_cmd
                        .name("delete")
                        .description("delete a resource from type")
                        .kind(ApplicationCommandOptionType::SubCommand)
                        .add_sub_option(materials_cmd_opt(materials, true))
                })
        });
    cmd
}

pub fn publish_app_cmd() -> AppCommand {
    let mut cmd = AppCommand::default();
    cmd.name("publish")
        .description("publish to the configured channels id if so");
    cmd
}

fn materials_cmd_opt(materials: &Vec<String>, required: bool) -> CommandOpt {
    let mut opt = CommandOpt::default();
    opt.name("material")
        .description("choose a material type")
        .kind(ApplicationCommandOptionType::String)
        .required(required);
    for material in materials {
        opt.add_string_choice(material, material);
    }
    opt
}
