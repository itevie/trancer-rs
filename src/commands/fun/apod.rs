use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::util::embeds::create_embed;
use crate::{command_argument_struct, command_file};
use serde::Deserialize;
use serenity::all::CreateAttachment;
use serenity::builder::CreateMessage;
use std::collections::HashMap;

command_argument_struct!(ApodArgs {
    date: Option<String>, PCACV::OpString
});

#[derive(Debug, Deserialize)]
pub struct Apod {
    pub title: String,
    pub explanation: String,
    pub url: String,
    pub media_type: String,
}

command_file! {
    TrancerCommand::<ApodArgs> {
        name: "apod".to_string(),
        t: TrancerCommandType::Help,
        description: "Get today's Astronomy Picture Of the Day".to_string(),
        details: TrancerDetails {
            arguments: Some(TrancerArguments {
                required: 0,
                args: vec![
                    Argument {
                        t: ArgType::String { flags: None },
                        name: "date".to_string(),
                        details: Default::default()
                    }
                ]
            }),
            slow: true,
            ..Default::default()
        },

        handler: trancer_handler!(|_ctx, args| {

            let client = reqwest::Client::new();

             let api_key =
                    std::env::var("NASA_API_KEY").unwrap_or_else(|_| "DEMO_KEY".to_string());

            let mut url = format!(
                "https://api.nasa.gov/planetary/apod?api_key={}",
                api_key
            );

            if let Some(date) = &args.date {
                url.push_str(&format!("&date={date}"));
            }

            let response = match client.get(&url).send().await {
                Ok(res) => res,
                Err(err) => {
                    return Err(TrancerError::NonScary(format!("Failed to fetch the APOD!\n> {}", err)))
                }
            };

            let data: Apod = match response.json().await {
                Ok(data) => data,
                Err(err) => {
                    return Err(TrancerError::NonScary(format!("Failed to parse the APOD response!\n> {}", err)))
                }
            };

            if data.media_type != "image" {
                return Err(TrancerError::NonScary("The APOD for this date is not an image!".to_string()))
            }

            let image_bytes = match client.get(&data.url).send().await {
                Ok(res) => match res.bytes().await {
                    Ok(bytes) => bytes,
                    Err(err) => {
                        return Err(TrancerError::NonScary(format!("Failed to read the APOD image!\n> {}", err)))
                    }
                },
                Err(err) => {
                    return Err(TrancerError::NonScary(format!("Failed to download the APOD image!\n> {}", err)))
                }
            };

            let attachment = CreateAttachment::bytes(
                image_bytes.to_vec(),
                "apod.jpg",
            );

            let embed = create_embed()
                .title(data.title)
                .url(data.url)
                .description(data.explanation)
                .image("attachment://apod.jpg");

            Ok(TrancerResponseType::Big(
                CreateMessage::new().embed(embed).add_file(attachment)
            ))
        }),
    }
}
