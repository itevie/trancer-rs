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

use crate::database::Database;
use crate::message_handlers::xp::XpLastAwards;
use crate::util::cached_usernames::init_cached_usernames_database;
use crate::util::config::load_config;
use dotenvy::dotenv;
use serenity::prelude::*;
use std::env;
use std::sync::Arc;
use tracing::info;

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
