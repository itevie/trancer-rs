#![allow(dead_code)]

mod cmd_util;
mod commands;
mod database;
mod models;
mod trancer_config;
mod util;

use crate::cmd_util::arg_parser::parse_args;
use crate::cmd_util::{TrancerError, TrancerResponseType, TrancerRunnerContext};
use crate::database::Database;
use crate::models::command_creation::CommandCreation;
use crate::models::ratelimit::Ratelimit;
use crate::models::server_settings::ServerSettings;
use crate::models::user_data::UserData;
use crate::util::embeds::create_embed;
use crate::util::lang::permission_names;
use crate::util::level_calc;
use crate::util::level_calc::{MAX_XP, MIN_XP, TIME_BETWEEN};
use chrono::format::Item;
use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use config::Config;
use dotenvy::dotenv;
use lazy_static::lazy_static;
use serde::Deserialize;
use serenity::all::{Channel, ChannelType, CreateMessage};
use serenity::builder::EditChannel;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::any::Any;
use std::env;
use tokio::io::AsyncWriteExt;
use tracing::{error, info, instrument};

async fn something_happened(ctx: &TrancerRunnerContext, m: impl Into<String>, e: TrancerError) {
    let dev_error = format!(
        "{}: {}\n> Command: {} ({})",
        m.into(),
        e.to_string(),
        ctx.original_command,
        ctx.command_name
    );
    error!(dev_error);

    let m = format!(
        ":red_circle: Sorry! I couldn't run the command as something bad happened!\n:information_source: Please report this to the bot owner\n> {}",
        dev_error
    );

    let result = ctx.msg.reply(&ctx.sy, &m).await;
    if result.is_err() {
        let _ = ctx
            .msg
            .channel_id
            .send_message(
                &ctx.sy,
                CreateMessage::new().content(format!("**{}**: {}", ctx.msg.author.name, m)),
            )
            .await;
    }
}

macro_rules! something_happened {
    ($ctx:ident, $what:expr) => {
        match $what {
            Ok(ok) => ok,
            Err(e) => {
                something_happened(
                    &$ctx,
                    "Failed to run something via something_happened macro",
                    TrancerError::from(e),
                );
                return;
            }
        }
    };
}

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

        let guild_id = match msg.guild_id {
            Some(id) => id,
            None => return,
        };

        let server_settings = match ServerSettings::fetch(&ctx, guild_id).await {
            Ok(ok) => ok,
            Err(e) => {
                error!(
                    "Failed to fetch server_settings during message handler: {:?}",
                    e
                );
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

        let user_data = match UserData::fetch(&ctx, msg.author.id, guild_id).await {
            Ok(ok) => ok,
            Err(e) => {
                error!(
                    "Failed to fetch user_data during message handler: {}",
                    e.to_string()
                );
                return;
            }
        };

        let context = TrancerRunnerContext {
            sy: ctx.clone(),
            msg: msg.clone(),
            server_settings,
            channel: channel.clone(),
            user_data,
            command_name: cmd.name(),
            original_command: msg.content.to_string(),
        };

        let member = match guild_id.member(&ctx.http, msg.author.id).await {
            Ok(m) => m,
            Err(e) => {
                error!(
                    "Failed to fetch member during message handler: {}",
                    e.to_string()
                );
                return;
            }
        };

        if let Some(user_permission) = cmd.details().user_permissions {
            let permissions = match guild_id.to_guild_cached(&ctx.cache).map(|x| x.clone()) {
                Some(some) => some.user_permissions_in(&channel, &member),
                None => match ctx.http.get_guild(guild_id).await {
                    Ok(some) => some.user_permissions_in(&channel, &member),
                    Err(e) => {
                        something_happened(&context, "Failed to fetch the guild from cache or http, so I couldn't check your permissions",
                        TrancerError::from(e)).await;
                        return;
                    }
                },
            };

            if permissions.contains(user_permission) {
                let _ = reply!(
                    context,
                    CreateMessage::new().embed(
                        create_embed()
                            .title("Sorry... you don't have permission to do that.")
                            .description(format!(
                                "You need the following permissions: {}",
                                permission_names(user_permission)
                            ))
                    )
                );
                return;
            }
        }

        if let Some(r) = cmd.details().ratelimit {
            let ratelimit = match Ratelimit::fetch(&ctx, msg.author.id, cmd.name()).await {
                Ok(ok) => ok,
                Err(e) => {
                    something_happened(
                        &context,
                        "Failed to fetch ratelimit",
                        TrancerError::from(e),
                    )
                    .await;
                    return;
                }
            };

            let prev =
                something_happened!(context, DateTime::parse_from_rfc3339(&*ratelimit.last_used))
                    .timestamp();
            let now = Utc::now().timestamp();

            if now - prev < r as i64 {
                let _ = reply!(
                    context,
                    CreateMessage::new().embed(create_embed().title(format!(
                        "Hey! You can't do that! Try again in **{}**",
                        HumanTime::from(now - prev)
                    )))
                );
                return;
            }

            match Ratelimit::update(&ctx, msg.author.id, cmd.name()).await {
                Ok(_) => (),
                Err(e) => {
                    something_happened(
                        &context,
                        "Failed to update ratelimit",
                        TrancerError::from(e),
                    )
                    .await;
                }
            }
        }

        let response = match cmd.run(context.clone(), args).await {
            Ok(response) => response,
            Err(err) => {
                something_happened(&context, "Error while executing command handler", err).await;
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
        CommandCreation::insert_commands(&ctx, commands.iter().map(|x| x.name().clone()).collect())
            .await
            .unwrap();
        models::item::Item::insert_all(&ctx).await.unwrap();

        info!("{} has connected and is ready", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    load_config().unwrap();
    info!("Starting bot...");

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
        eprintln!("Client error: {:?}", why);
    }
}

#[derive(Debug, Deserialize)]
pub struct TrancerConfig {
    pub server: TrancerServerConfig,
}

#[derive(Debug, Deserialize)]
pub struct TrancerServerConfig {
    pub id: String,
    pub invite_link: String,
}

lazy_static! {
    pub static ref CONFIG: TrancerConfig = {
        let config = Config::builder()
            .add_source(config::File::with_name("config_dev").required(false))
            .add_source(config::File::with_name("config").required(false))
            .build()
            .unwrap()
            .try_deserialize::<TrancerConfig>()
            .unwrap();

        config
    };
}

#[instrument]
fn load_config() -> Result<TrancerConfig, config::ConfigError> {
    info!("Loading config");
    let config = Config::builder()
        .add_source(config::File::with_name("config_dev").required(false))
        .add_source(config::File::with_name("config").required(false))
        .build()?
        .try_deserialize::<TrancerConfig>()?;

    Ok(config)
}
