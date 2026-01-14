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
use crate::models::item::ALL_ITEMS;
use crate::trancer_config::all_items::ALL_ITEMS_DEF;
use crate::trancer_config::all_recipes::CRAFTING_RECIPES;
use crate::util::cached_usernames::init_cached_usernames_database;
use crate::util::config::{load_config, CONFIG};
use crate::util::random_rewards::RandomRewardPresets;
use dotenvy::dotenv;
use serenity::prelude::*;
use std::sync::Arc;
use std::{env, fs};
use tracing::info;

struct Handler;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    init_cached_usernames_database();
    let config = load_config().unwrap();
    info!("Starting bot...");

    dotenv().ok();

    // Generate balance
    let mut string = "===== Item Details =====\n".to_string();
    string.push_str("Format: name[tag]: price, weight, buyable\n");
    string.push_str("Sell price is always price/2\n\n");

    for i in ALL_ITEMS_DEF {
        string.push_str(&format!(
            "{}[{:?}]: {}, {}, {}\n",
            i.name, i.tag, i.price, i.weight, i.buyable
        ))
    }

    string.push_str("\n===== Crafting Recipies =====\n\n");

    for recipie in CRAFTING_RECIPES.iter() {
        string.push_str(&format!(
            "{}: {}\n",
            recipie.0,
            recipie
                .1
                .iter()
                .map(|x| format!("{}: {}, ", x.0, x.1))
                .collect::<String>()
        ))
    }

    string.push_str("\n===== Rewards from commands =====\n\n");

    string.push_str(&format!(
        "Daily:\n{}\n",
        RandomRewardPresets::daily().to_string()
    ));

    string.push_str("\n===== XP Config =====\n\n");
    string.push_str("after = last lavel + after = every level's xp after\n");
    string.push_str("level up reward = eco_reward * level\n");
    string.push_str(&format!("{:?}", CONFIG.xp));

    fs::write(
        "/home/isabella/Documents/projects/rust/trancer-rs/balance.txt",
        string,
    );

    let token = env::var("BOT_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        let db = Database::new(config.general.data_dir);

        db.run("DELETE FROM economy", &[]);
        db.run(
            "DELETE FROM aquired_items
                    WHERE NOT EXISTS (
                        SELECT 1
                        FROM items
                        WHERE items.id = aquired_items.item_id
                          AND items.tag = 'collectable'
                    );",
            &[],
        );

        data.insert::<Database>(db);
        data.insert::<XpLastAwards>(Arc::default());
    }

    if let Err(why) = client.start().await {
        eprintln!("Client error: {:?}", why);
    }
}
