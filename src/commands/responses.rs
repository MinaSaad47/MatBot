use std::path::Path;

use ansi_term::Color;
use log::*;
use tokio::fs;
use url::{Url,ParseError};
use rand::{self, Rng};

use serenity::{
    builder::CreateInteractionResponseData,
    http::{Http, AttachmentType},
    model::{
        id::ChannelId, interactions::application_command::ApplicationCommandInteractionDataOption,
        prelude::User,
    },
    utils::Colour,
};

use crate::{config::Config, materials::{MatRow, self}};

type ResponseData = CreateInteractionResponseData;
type CommandOpts = Vec<ApplicationCommandInteractionDataOption>;

const VERSION: &'static str = "0.1.0";

pub fn version() -> ResponseData {
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

pub fn display(cmd_opts: &CommandOpts) -> ResponseData {
    debug!("cmd_opts:\n{:#?}", cmd_opts);
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

    let material_fields = gen_resources_fields(material_type, false);

    ResponseData::default()
        .create_embed(|embed| {
            embed
                .title(material_type)
                .footer(|f| f.text("STUDY WELL !!!"))
                .colour(COLOURS[rand::thread_rng().gen_range(0..COLOURS.len())])
                .fields(material_fields)
        })
        .clone()
}

pub fn add(cmd_opts: &CommandOpts, author: &User) -> ResponseData {
    // TODO: check permissions
    info!("'{}': requested `add method`", author.name);
    debug!("cmd_opts:\n{:#?}", cmd_opts);
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
    let abs_url = match Url::parse(&url) {
        Ok(url) => url.to_string(),
        Err(ParseError::RelativeUrlWithoutBase) => {
            format!("https://{}", url)
        },
        Err(error) => {
            error!("{:#?}", error);
            panic!("{:#?}", error);
        }
    };
    let matrow = MatRow::new(name, &abs_url, &author.name);
    if let Err(error) = matrow.insert_into_database(material_type) {
        error!("{}", error);
        return ResponseData::default().content(error).clone();
    }
    let status = format!("added {:?} to {}", matrow, material_type);
    info!("'{}' {}", author.name, status);
    ResponseData::default().content(status).clone()
}

pub fn delete(cmd_opts: &CommandOpts, author: &User) -> ResponseData {
    // TODO: check permissions
    info!("'{}': requested `delete method`", &author.name);
    debug!("cmd_opts:\n{:#?}", cmd_opts);
    let (material_type, id) = {
        (
            cmd_opts.get(0).as_ref().unwrap().value.as_ref().unwrap().as_str().unwrap(),
            cmd_opts.get(1).as_ref().unwrap().value.as_ref().unwrap().as_i64().unwrap(),
        )
    };

    if let Err(error) = materials::delete_from_database(material_type, id as usize) {
        error!("{}", error);
        return ResponseData::default().content(error).clone();
    }
    let status = format!("resource '{}' removed from `{}`", id, material_type);
    info!("'{}' {}", author.name, status);
    ResponseData::default().content(status).clone()
}

pub async fn publish(http: &impl AsRef<Http>) -> ResponseData {
    info!("a user requested to publish");

    let conf = match Config::from_json_file("settings.json") {
        Ok(conf) => conf,
        Err(error) => {
            error!("{}", error);
            panic!();
        }
    };

    let history = match ChannelId(conf.main_channel_id)
        .messages(http, |get_msg| get_msg.limit(100)).await {
            Ok(history) => history,
            Err(error) => {
                error!("{}", error);
                panic!("{}", error);
            }
    };

    if let Err(error) = ChannelId(conf.main_channel_id)
        .delete_messages(http, history).await {
        error!("{}", error);
        panic!("{}", error);
    }

    let database_file = fs::File::open(&conf.database_path).await;

    for (material, _) in &conf.material_types {
        let fields = gen_resources_fields(material, false);
        if let Err(error) = ChannelId(conf.main_channel_id)
            .send_message(http, |msg| {
                msg.add_embed(|embed| {
                    embed
                        .title(material)
                        .footer(|f| f.text("STUDY WELL !!!"))
                        .colour(COLOURS[rand::thread_rng().gen_range(0..COLOURS.len())])
                        .fields(fields)
                })
            })
            .await
        {
            error!("{}", error);
            panic!("{}", error);
        }
        info!("Published: `{}` Material Type", Color::Green.paint(material));
    }
    if let Err(error) = ChannelId(conf.main_channel_id)
        .send_message(http, |msg| {
            if let Ok(file) = &database_file {
                msg.add_file(AttachmentType::File {
                    file,
                    filename: Path::new(&conf.database_path).file_name().unwrap().to_str().unwrap().to_string()
                });
            } else {
                error!("could not attach database file to the message")
            }
            msg
        })
        .await {
        error!("{}", error);
        panic!("{}", error);
    }
    ResponseData::default()
        .content("Materials updated successfully")
        .clone()
}

fn gen_resources_fields(material_type: &str, required: bool) -> Vec<(String, String, bool)> {
    let mat_row: Vec<(String, String, bool)> = match MatRow::vec_from_database(material_type) {
        Ok(mat_row) => mat_row
            .into_iter()
            .map(|row| {
                    (
                        format!(
                            "**[{}] __{}__** \t\t\t\t\t\tby *{}* at *{}*",
                            row.id, row.name, row.author, row.time_added),
                        format!("<{}>", row.url),
                        required
                    )
            })
            .collect(),
        Err(error) => {
            error!("{}", error);
            panic!("{}", error);
        }
    };
    mat_row
}
