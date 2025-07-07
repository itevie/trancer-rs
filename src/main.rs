#![allow(dead_code)]

mod cmd_util;
mod commands;
mod database;
mod models;
mod util;
mod config;

use crate::cmd_util::arg_parser::parse_args;
use crate::cmd_util::{TrancerResponseType, TrancerRunnerContext, TrancerError};
use crate::database::Database;
use crate::models::server_settings::ServerSettings;
use crate::models::user_data::UserData;
use dotenvy::dotenv;
use serenity::all::{Channel, ChannelType, CreateMessage};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::any::Any;
use std::env;
use chrono::format::Item;
use crate::models::command_creation::CommandCreation;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == ctx.cache.current_user().id {
            return;
        }

        // Check if it's sent in a guild text channel
        let channel = if let Ok(channel) = msg.channel_id.to_channel(&ctx).await {
            match channel {
                Channel::Guild(channel) => {
                    if channel.kind == ChannelType::Text {
                        channel
                    } else {
                        return;
                    }
                }
                _ => return,
            }
        } else {
            return;
        };

        let server_settings = match ServerSettings::fetch(&ctx, msg.guild_id.unwrap()).await {
            Ok(ok) => ok,
            Err(e) => {
                eprintln!("{:#?}", e);
                return;
            }
        };

        if !msg.content.starts_with(server_settings.prefix.as_str()) {
            return;
        }

        let contents = msg.content[server_settings.prefix.len()..].trim();
        let commands = commands::init();
        let mut args = parse_args(contents.to_string());

        if args.args.is_empty() {
            return;
        }

        let command_name = args.args[0].clone();
        args.args.remove(0);

        let Some(cmd) = commands
            .iter()
            .find(|cmd| cmd.name().eq(command_name.as_str()))
        else {
            return;
        };

        let user_data = match UserData::fetch(&ctx, msg.author.id, msg.guild_id.unwrap()).await {
            Ok(ok) => ok,
            Err(e) => {
                eprintln!("{:#?}", e);
                return;
            }
        };

        let context = TrancerRunnerContext {
            sy: ctx.clone(),
            msg: msg.clone(),
            server_settings,
            channel,
            user_data,
        };

        let response = match cmd.run(context.clone(), args).await {
            Ok(response) => response,
            Err(err) => {
                msg.reply(&ctx, err.to_string()).await.unwrap();
                return;
            }
        };

        match response {
            TrancerResponseType::Content(string) => {
                // Ignore the error, because if it errors then it doesn't matter
                // probably likely a timeout issue.
                let _ = reply!(context, CreateMessage::new().content(string.as_str()));
            }
            TrancerResponseType::Big(big) => {
                let _ = reply!(context, big.clone());
            }
            TrancerResponseType::None => (),
        };
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        let commands = commands::init();
        CommandCreation::insert_commands(&ctx, commands.iter().map(|x| x.name().clone()).collect()).await.unwrap();
        models::item::Item::insert_all(&ctx).await.unwrap();

        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("BOT_TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;

        let db = Database::new();
        data.insert::<Database>(db);
    }

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
