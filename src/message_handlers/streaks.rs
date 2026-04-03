use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use crate::models::user_data::UserDataFields;
use crate::reply;
use chrono::{DateTime, Duration, Utc};
use serenity::all::ReactionType;
use serenity::builder::CreateMessage;
use tracing::{error, instrument};

#[instrument]
pub async fn handle(ctx: &TrancerRunnerContext) -> Result<(), TrancerError> {
    let last_talk: DateTime<Utc> = if let Some(t) = ctx.user_data.last_talking_streak.clone() {
        t.parse()?
    } else {
        Utc::now()
    };

    let now = Utc::now();

    let last_day = last_talk.date_naive();
    let today = now.date_naive();

    let diff_days = (today - last_day).num_days();

    if ctx.user_data.talking_streak > ctx.user_data.highest_talking_streak {
        ctx.user_data
            .update_key(
                &ctx.sy,
                UserDataFields::highest_talking_streak,
                ctx.user_data.talking_streak,
            )
            .await?;
    };

    let set_time = async |now: DateTime<Utc>| {
        let result = ctx
            .user_data
            .update_key(
                &ctx.sy,
                UserDataFields::last_talking_streak,
                now.clone().to_rfc3339(),
            )
            .await;
        if let Err(e) = result {
            eprintln!("{}", e.to_string());
        }
    };

    if diff_days > 1 {
        ctx.user_data
            .update_key(&ctx.sy, UserDataFields::talking_streak, 0)
            .await?;
        set_time(now).await;

        reply!(
            ctx,
            CreateMessage::new().content(":x: Uh-oh! You're streak has been reset :(")
        )?;
    } else if diff_days == 0 {
        if let None = ctx.user_data.last_talking_streak {
            set_time(now - Duration::days(1)).await;
        }
    } else {
        if ctx.server_settings.streak_reactions {
            ctx.msg
                .react(&ctx.sy.http, ReactionType::Unicode("🔥".to_string()))
                .await?;
        }

        ctx.user_data
            .update_key(
                &ctx.sy,
                UserDataFields::talking_streak,
                ctx.user_data.talking_streak + 1,
            )
            .await?;
        set_time(now).await;
    }

    Ok(())
}
