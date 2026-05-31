use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use crate::models::user_data::UserDataFields;
use crate::reply;
use tracing::error;

use chrono::{DateTime, Duration, Utc};
use serenity::all::ReactionType;
use serenity::builder::CreateMessage;
use tracing::instrument;

#[instrument]
pub async fn handle(ctx: &TrancerRunnerContext) -> Result<(), TrancerError> {
    let now = Utc::now();
    let today = now.date_naive();

    let last_talk: DateTime<Utc> = ctx
        .user_data
        .last_talking_streak
        .as_ref()
        .map(|t| t.parse::<DateTime<Utc>>())
        .transpose()?
        .unwrap_or_else(|| now - Duration::days(1));

    let last_day = last_talk.date_naive();
    let diff_days = (today - last_day).num_days().max(0);

    let update_last_talk = |time: DateTime<Utc>| async move {
        ctx.user_data
            .update_key(
                &ctx.sy,
                UserDataFields::last_talking_streak,
                time.to_rfc3339(),
            )
            .await?;
        Ok::<(), TrancerError>(())
    };

    // RESET STREAK
    if diff_days > 1 {
        if ctx.user_data.talking_streak >= 5 && ctx.server_settings.streak_end_reactions {
            let _ = reply!(
                ctx,
                CreateMessage::new().content(format!(
                    ":x: Uh-oh! Your streak has been reset :(\nIt was at **{}**",
                    ctx.user_data.talking_streak
                ))
            );
        }

        ctx.user_data
            .update_key(&ctx.sy, UserDataFields::talking_streak, 1)
            .await?;

        update_last_talk(now).await?;
        return Ok(());
    }

    // SAME DAY -> DO NOTHING
    if diff_days == 0 {
        return Ok(());
    }

    // NEW DAY

    let new_streak = ctx.user_data.talking_streak + 1;

    ctx.user_data
        .update_key(&ctx.sy, UserDataFields::talking_streak, new_streak)
        .await?;

    if new_streak > ctx.user_data.highest_talking_streak {
        ctx.user_data
            .update_key(&ctx.sy, UserDataFields::highest_talking_streak, new_streak)
            .await?;
    }

    update_last_talk(now).await?;

    if ctx.server_settings.streak_reactions {
        ctx.msg
            .react(&ctx.sy.http, ReactionType::Unicode("🔥".to_string()))
            .await?;
    }

    Ok(())
}
