use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use crate::models::economy::MoneyAddReasion;
use crate::models::user_data::UserDataFields;
use crate::reply;
use crate::util::config::CONFIG;
use crate::util::lang::currency;
use crate::util::level_calc::{calculate_level, TIME_BETWEEN, XP_ECO_REWARD};
use crate::util::other::random_range;
use chrono::{DateTime, Utc};
use rand::random;
use serenity::builder::CreateMessage;
use serenity::prelude::TypeMapKey;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::error;

#[derive(Clone)]
pub struct XpLastAwards(pub Arc<Mutex<HashMap<String, DateTime<Utc>>>>);

impl TypeMapKey for XpLastAwards {
    type Value = Arc<Mutex<HashMap<String, DateTime<Utc>>>>;
}

pub async fn handle_xp(ctx: &TrancerRunnerContext) -> Result<(), TrancerError> {
    let xp = {
        let data_lock = ctx.sy.data.read().await;
        let xp = data_lock.get::<XpLastAwards>().unwrap();
        xp.clone()
    };

    if CONFIG
        .xp
        .exclude
        .iter()
        .any(|x| ctx.channel.id.to_string().eq(x))
    {
        return Ok(());
    }

    let now = Utc::now();
    let last = xp.lock().unwrap().get(&ctx.user_data.user_id).cloned();

    if let Some(last) = last {
        if now.timestamp() - last.timestamp() < TIME_BETWEEN as i64 {
            return Ok(());
        }
    }

    xp.lock().unwrap().remove(&ctx.user_data.user_id);
    xp.lock()
        .unwrap()
        .insert(ctx.user_data.user_id.clone(), now);

    let pre_level = calculate_level(ctx.user_data.xp);
    let award = random_range(CONFIG.xp.min..CONFIG.xp.max);
    ctx.user_data
        .increment(&ctx.sy, UserDataFields::xp, award as i32)
        .await?;
    let post_level = calculate_level(ctx.user_data.xp + award);

    if pre_level != post_level && ctx.server_settings.level_notifications {
        let mut reward: Vec<String> = vec![];

        if ctx.guild_id.to_string() == CONFIG.server.id {
            let amount = XP_ECO_REWARD * (post_level / 2);
            reward.push(currency(amount));
            ctx.economy
                .add_money(&ctx.sy, amount, Some(MoneyAddReasion::Messaging))
                .await?;
        }

        // TODO: Add level roles

        reply!(ctx, CreateMessage::new().content(
            format!("Well-done! You levelled up from level **{pre_level}** to **{post_level}**! :cyclone:{}",
                if !reward.is_empty() {
                    format!("\n\nYou got: {}", reward.join(", "))
                } else {
                    "".to_string()
                }
            )
        ))?;
    }

    Ok(())
}
