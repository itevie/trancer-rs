use crate::cmd_util::TrancerError;
use serenity::client::Context;
use std::time::Duration;
use tokio::time::interval;
use tracing::{error, instrument};

mod change_status;
mod reload_cached_usernames;

macro_rules! timer {
    ($amount:expr, $func:expr, $ctx:expr) => {
        let ctx2 = $ctx.clone();
        tokio::spawn(async move {
            let mut ticker = interval(Duration::from_secs($amount));

            loop {
                match ($func)(ctx2.clone()).await {
                    Ok(_) => (),
                    Err(err) => handle_error(err),
                }

                ticker.tick().await;
            }
        })
    };
}

pub fn start_all(ctx: Context) {
    timer!(60 * 10, change_status::run, ctx.clone());
    timer!(30, reload_cached_usernames::run, ctx.clone());
}

#[instrument]
fn handle_error(error: TrancerError) {
    error!("A timer has failed: {}", error.to_string());
}
