use crate::cmd_util::TrancerError;
use serenity::client::Context;
use std::time::Duration;
use tokio::time::interval;
use tracing::{error, instrument};

mod birthday_checker;
mod bump_reminder;
mod change_status;
mod persistent_messages;
mod qotd;
mod reload_cached_usernames;

macro_rules! timer {
    ($amount:expr, $func:expr, $ctx:expr, $name:expr) => {
        let ctx2 = $ctx.clone();
        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs($amount));

            loop {
                ticker.tick().await;

                println!("Running Timer: {}", $name);
                match ($func)(ctx2.clone()).await {
                    Ok(_) => (),
                    Err(err) => handle_error(err),
                }
            }
        })
    };
}

pub fn start_all(ctx: Context) {
    timer!(60 * 10, change_status::run, ctx.clone(), "Change Status");
    timer!(
        30,
        reload_cached_usernames::run,
        ctx.clone(),
        "Reload Cached Usernames"
    );
    timer!(
        60 * 30,
        persistent_messages::run,
        ctx.clone(),
        "Persistent Messages"
    );
    timer!(
        60 * 120,
        birthday_checker::run,
        ctx.clone(),
        "Birthday Checker"
    );
    timer!(60 * 60 * 12, qotd::run, ctx.clone(), "QOTD");
    timer!(60 * 10, bump_reminder::run, ctx.clone(), "Bump Reminder");
}

#[instrument]
fn handle_error(error: TrancerError) {
    error!("A timer has failed: {}", error.to_string());
}
