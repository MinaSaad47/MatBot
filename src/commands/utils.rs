use log::*;

use rand::{self, Rng};

use serenity::{
    builder::CreateInteractionResponseData,
    model::{
        interactions::application_command::ApplicationCommandInteractionDataOption, prelude::User,
    },
    utils::Colour,
};

use crate::materials::MatRow;

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
    debug!("cmd_opts:\n{:?}", cmd_opts);
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

    let mat_row: Vec<(String, String, bool)> = match MatRow::vec_from_database(material_type) {
        Ok(mat_row) => mat_row
            .into_iter()
            .map(|row| {
                (
                    format!(
                        "**__{}__** \t\t\t\t\t\tby *{}* at *{}*",
                        row.name, row.author, row.time_added
                    ),
                    format!("- <{}>", row.url),
                    false,
                )
            })
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

pub fn update_res_msg(cmd_opts: &CommandOpts, author: &User) -> ResponseData {
    debug!("cmd_opts:\n{:?}", cmd_opts);
    // TODO: check user permissions
    let sub_cmd = cmd_opts.get(0).unwrap().options.get(0).unwrap();
    match sub_cmd.name.as_str() {
        "add" => add_res_msg(&sub_cmd.options, &author.name),
        "delete" => delete_res_msg(&sub_cmd.options),
        _ => unreachable!(),
    }
}

fn add_res_msg(cmd_opts: &CommandOpts, author: &str) -> ResponseData {
    info!("a user('{}') requested update 'add method'", author);
    debug!("cmd_opts:\n{:?}", cmd_opts);
    let material_type = cmd_opts
        .get(0)
        .as_ref()
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();
    let name = cmd_opts
        .get(1)
        .as_ref()
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();
    let url = cmd_opts
        .get(2)
        .as_ref()
        .unwrap()
        .value
        .as_ref()
        .unwrap()
        .as_str()
        .unwrap();
    let matrow = MatRow::new(name, url, author);
    if let Err(error) = matrow.insert_into_database(material_type) {
        error!("{}", error);
        return ResponseData::default().content(error).clone();
    }
    let status = format!("added {:?} to {}", matrow, material_type);
    info!("'{}' {}", author, status);
    ResponseData::default().content(status).clone()
}

fn delete_res_msg(cmd_opts: &CommandOpts) -> ResponseData {
    info!("a user requested update 'delete method'");
    debug!("cmd_opts:\n{:?}", cmd_opts);
    unimplemented!()
}
