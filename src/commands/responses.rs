use ansi_term::Color;
use log::*;
use url::{Url,ParseError};
use rand::{self, Rng};

use serenity::{
    builder::CreateInteractionResponseData,
    http::Http,
    model::{
        id::ChannelId, interactions::application_command::ApplicationCommandInteractionDataOption,
        prelude::User,
    },
    utils::Colour,
};

use crate::{config::Config, materials::MatRow};

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

pub fn update(cmd_opts: &CommandOpts, author: &User) -> ResponseData {
    debug!("cmd_opts:\n{:#?}", cmd_opts);
    // TODO: check user permissions
    let sub_cmd = cmd_opts.get(0).unwrap().options.get(0).unwrap();
    match sub_cmd.name.as_str() {
        "add" => add(&sub_cmd.options, &author.name),
        "delete" => delete(&sub_cmd.options),
        _ => unreachable!(),
    }
}

fn add(cmd_opts: &CommandOpts, author: &str) -> ResponseData {
    info!("a user('{}') requested update 'add method'", author);
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
    let matrow = MatRow::new(name, &abs_url, author);
    if let Err(error) = matrow.insert_into_database(material_type) {
        error!("{}", error);
        return ResponseData::default().content(error).clone();
    }
    let status = format!("added {:?} to {}", matrow, material_type);
    info!("'{}' {}", author, status);
    ResponseData::default().content(status).clone()
}

fn delete(cmd_opts: &CommandOpts) -> ResponseData {
    info!("a user requested update 'delete method'");
    debug!("cmd_opts:\n{:#?}", cmd_opts);
    unimplemented!()
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
        info!("Published: `{}` Material Type", Color::Green.paint(material))
    }
    ResponseData::default()
        .content("Materials updated successfully")
        .clone()
}

fn gen_resources_fields(material_type: &str, required: bool) -> Vec<(String, String, bool)> {
    let mut index = 0;
    let mat_row: Vec<(String, String, bool)> = match MatRow::vec_from_database(material_type) {
        Ok(mat_row) => mat_row
            .into_iter()
            .map(|row| {
                    index = index + 1;
                    (
                        format!(
                            "**[{}] __{}__** \t\t\t\t\t\tby *{}* at *{}*",
                            index, row.name, row.author, row.time_added),
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
