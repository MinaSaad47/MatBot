use log::*;

use rand::{self, Rng};

use serenity::{
    builder::CreateInteractionResponseData,
    model::interactions::application_command::ApplicationCommandInteractionDataOption,
    utils::Colour,
};

use crate::{config::Config, materials::MatRow};

type ResponseData = CreateInteractionResponseData;
type CommandOpts = Vec<ApplicationCommandInteractionDataOption>;

const VERSION: &'static str = "0.1.0";

pub fn version_res_msg() -> ResponseData {
    info!("a user requested the version");
    ResponseData::default()
        .content(format!("MatBot Version: {}", VERSION))
        .clone()
}

const COLOURS: [Colour; 7] = [
    Colour::DARK_GREEN,
    Colour::DARK_RED,
    Colour::DARK_BLUE,
    Colour::DARK_PURPLE,
    Colour::DARK_MAGENTA,
    Colour::DARK_GOLD,
    Colour::DARK_GREY,
];

pub fn display_res_msg(cmd_opts: &CommandOpts) -> ResponseData {
    let material_type = cmd_opts
        .get(0)
        .as_ref()
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();

    info!("a user requested '{}' material type", material_type);

    let conf = match Config::from_json_file("settings.json") {
        Ok(conf) => conf,
        Err(error) => {
            error!("{}", error);
            panic!("{}", error);
        }
    };

    let mat_row: Vec<(String, String, bool)> =
        match MatRow::vec_from_database(&conf.database_path, material_type) {
            Ok(mat_row) => mat_row
                .into_iter()
                .map(|row| (row.name, format!("- [{}]({})", row.url, row.url), false))
                .collect(),
            Err(error) => {
                error!("{}", error);
                panic!("{}", error);
            }
        };

    ResponseData::default()
        .create_embed(|embed| {
            embed
                .title(material_type)
                .footer(|f| f.text("STUDY WELL !!!"))
                .colour(COLOURS[rand::thread_rng().gen_range(0..COLOURS.len())])
                .fields(mat_row)
        })
        .clone()
}
