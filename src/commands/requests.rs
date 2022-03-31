use serenity::{
    builder::{CreateApplicationCommand, CreateApplicationCommandOption},
    model::interactions::application_command::ApplicationCommandOptionType,
};

type AppCommand = CreateApplicationCommand;
type CommandOpt = CreateApplicationCommandOption;

pub fn version() -> AppCommand {
    let mut cmd = AppCommand::default();
    cmd.name("version").description("Request MatBot Version.");
    cmd
}

pub fn display(materials: &[String]) -> AppCommand {
    let mut cmd = AppCommand::default();
    cmd.name("display")
        .description("get material resources of a certain type")
        .set_options(vec![materials_opt(materials, true)]);
    cmd
}

pub fn add(materials: &[String]) -> AppCommand {
    AppCommand::default()
        .name("add")
        .description("add a resource to the type")
        .add_option(materials_opt(materials, true))
        .create_option(|name_value| {
            name_value
                .name("name")
                .required(true)
                .description("Enter a new resource name")
                .kind(ApplicationCommandOptionType::String)
        })
        .create_option(|url_value| {
            url_value
                .name("url")
                .required(true)
                .description("Enter a new reource url")
                .kind(ApplicationCommandOptionType::String)
        })
        .to_owned()
}

pub fn delete(materials: &[String]) -> AppCommand {
    AppCommand::default()
        .name("delete")
        .description("delete resource from material type")
        .add_option(materials_opt(materials, true))
        .create_option(|opt| {
            opt.name("index")
                .description("Enter a resource index to delete")
                .kind(ApplicationCommandOptionType::Integer)
                .required(true)
        })
        .to_owned()
}

pub fn publish() -> AppCommand {
    let mut cmd = AppCommand::default();
    cmd.name("publish")
        .description("publish to the configured channels id if so");
    cmd
}

fn materials_opt(materials: &[String], required: bool) -> CommandOpt {
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
