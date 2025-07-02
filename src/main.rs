mod command;
mod commands;

use std::env;
use dotenvy::dotenv;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use crate::command::TrancerResponseType;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.author.id == ctx.cache.current_user().id {
            return;
        }

        let commands = commands::init();
        let Some(cmd) = commands.iter().find(|cmd| cmd.name == msg.content) else {
          return;
        };

        let response = (cmd.handler)(&ctx, &msg).await;

        match response {
            TrancerResponseType::Content(string) => {msg.reply(&ctx, string).await.unwrap();},
            TrancerResponseType::None => ()
        };
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();

    let token = env::var("BOT_TOKEN")
        .expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
