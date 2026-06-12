use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use crate::models::economy::MoneyAddReason;
use crate::models::level_role::LevelRole;
use crate::models::user_data::UserDataFields;
use crate::reply;
use crate::util::config::CONFIG;
use crate::util::lang::{currency, englishify_list};
use crate::util::level_calc::calculate_level;
use crate::util::other::{give_role, random_range};
use chrono::{DateTime, Utc};
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

pub async fn handle(ctx: &TrancerRunnerContext) -> Result<(), TrancerError> {
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

    println!("{} {:?}", now.to_rfc3339(), last.map(|x| x.to_rfc3339()));

    // Time between = "120000"
    if let Some(last) = last {
        if now.timestamp() - last.timestamp() < CONFIG.xp.time_between as i64 {
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
            let amount = CONFIG.xp.eco_reward * (post_level / 2);
            reward.push(currency(amount));
            ctx.economy
                .add_money(&ctx.sy, amount, Some(MoneyAddReason::Messaging))
                .await?;
            println!("Add {}", amount);
        }

        if let Some(level_role) =
            LevelRole::fetch_by_level(&ctx.sy, ctx.guild_id, post_level).await?
        {
            if let Some(role_id) = level_role.role_id {
                let roles = ctx.guild_id.roles(&ctx.sy.http).await?;
                let role_id = role_id.parse::<u64>()?;
                // TODO: Handle error properly

                if let Some(role) = roles.iter().find(|r| r.0.get() == role_id) {
                    give_role(&ctx.sy, &ctx.msg.member(&ctx.sy.http).await?, role.1).await?;
                    reward.push(format!("role {}", role.1.name));
                }
            }
        }

        reply!(ctx, CreateMessage::new().content(
            format!("Well-done! You levelled up from level **{pre_level}** to **{post_level}**! :cyclone:{}",
                if !reward.is_empty() {
                    format!("\n\nYou got: {}", englishify_list(reward.clone(), false))
                } else {
                    "".to_string()
                }
            )
        ))?;
    }

    Ok(())
}
