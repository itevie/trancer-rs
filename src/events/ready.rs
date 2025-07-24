use crate::models::command_creation::CommandCreation;
use crate::models::item::ALL_ITEMS;
use crate::timers::start_all;
use crate::{commands, models, Handler};
use serenity::all::{Context, EventHandler, Ready};
use serenity::async_trait;
use tracing::info;

pub async fn ready(ctx: Context, ready: Ready) {
    let commands = commands::init();
    CommandCreation::insert_commands(&ctx, commands.iter().map(|x| x.name().clone()).collect())
        .await
        .unwrap();
    models::item::Item::insert_all(&ctx).await.unwrap();
    ALL_ITEMS
        .set(models::item::Item::get_all_db(&ctx).await.unwrap())
        .unwrap();

    start_all(ctx.clone());

    info!("{} has connected and is ready", ready.user.name);
}
