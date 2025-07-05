mod cmd_util;
mod commands;
mod database;
mod models;

use crate::cmd_util::arg_parser::parse_args;
use crate::cmd_util::TrancerResponseType;
use crate::database::Database;
use dotenvy::dotenv;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use std::env;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == ctx.cache.current_user().id {
            return;
        }

        let prefix = "!";

        if !msg.content.starts_with(prefix) {
            return;
        }

        let contents = msg.content[prefix.len()..].trim();
        let commands = commands::init();
        let mut args = parse_args(contents.to_string()).unwrap();

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

        let response = match cmd.run(ctx.clone(), msg.clone(), args).await {
            Ok(response) => response,
            Err(err) => {
                msg.reply(&ctx, err.to_string()).await.unwrap();
                return;
            }
        };

        match response {
            TrancerResponseType::Content(string) => {
                msg.reply(&ctx, string).await.unwrap();
            }
            TrancerResponseType::None => (),
        };
    }

    async fn ready(&self, _: Context, ready: Ready) {
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
