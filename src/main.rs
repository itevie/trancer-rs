#![allow(dead_code)]

mod cmd_util;
mod commands;
mod database;
mod events;
mod message_handlers;
mod models;
pub mod timers;
mod trancer_config;
mod util;

use crate::cmd_util::arg_parser::parse_args;
use crate::cmd_util::{TrancerError, TrancerResponseType, TrancerRunnerContext};
use crate::database::Database;
use crate::message_handlers::xp::XpLastAwards;
use crate::models::command_creation::CommandCreation;
use crate::models::item::ALL_ITEMS;
use crate::models::ratelimit::Ratelimit;
use crate::models::server_settings::ServerSettings;
use crate::models::user_data::UserData;
use crate::util::cached_usernames::init_cached_usernames_database;
use crate::util::config::{load_config, TrancerXpConfig};
use crate::util::embeds::create_embed;
use crate::util::lang::{permission_names, warn};
use crate::util::random_rewards::{
    generate_random_rewards, RandomRewardItemOptions, RandomRewardOptions,
};
use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use config::Config;
use dotenvy::dotenv;
use lazy_static::lazy_static;
use serde::Deserialize;
use serenity::all::{Channel, ChannelType, CreateMessage, ReactionType};
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;
use std::sync::Arc;
use tokio::io::AsyncWriteExt;
use tracing::{error, info, instrument};

struct Handler;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    init_cached_usernames_database();
    let config = load_config().unwrap();
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
        let db = Database::new(config.general.data_dir);
        data.insert::<Database>(db);
        data.insert::<XpLastAwards>(Arc::default());
    }

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
