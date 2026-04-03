use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use crate::models::couting::ServerCount;
use crate::models::user_data::UserData;
use crate::reply;
use chrono::{Datelike, Local};
use serenity::all::{CreateMessage, ReactionType};
use tracing::{error, instrument};

#[instrument]
pub async fn handle(ctx: &TrancerRunnerContext) -> Result<(), TrancerError> {
    if ctx.channel.id.to_string() == "1257426818479161394".to_string() {
        return Ok(());
    }

    let Some(server_count) = ServerCount::fetch(&ctx.sy, ctx.guild_id, ctx.channel.id).await?
    else {
        return Ok(());
    };

    let Ok(result) = meval::eval_str(ctx.msg.content.clone()) else {
        return Ok(());
    };

    if result.fract() != 0.0 {
        return Ok(());
    }

    let user_data = UserData::fetch(&ctx.sy, ctx.msg.author.id, ctx.guild_id).await?;

    if user_data.count_banned {
        reply!(
            ctx,
            CreateMessage::new().content(":x: This has not count as you're too bad at counting!")
        )?;
        return Ok(());
    }

    if !server_count.allow_consecutive_counts
        && server_count.last_counter.unwrap_or("0".to_string()) == user_data.user_id
    {
        reply!(
            ctx,
            CreateMessage::new()
                .content("You cannot count twice in a row - wait for someone else!")
        )?;
        return Ok(());
    }

    let number = result as u32;

    if number != server_count.current_count + 1 {
        let now = Local::now();
        let weekday = now.weekday().num_days_from_sunday();

        ctx.msg
            .react(&ctx.sy.http, ReactionType::Unicode("❌".to_string()))
            .await?;

        if server_count.ignore_failure
            || (server_count.ignore_failure_weekend && (weekday == 0 || weekday == 6))
        {
            return Ok(());
        }

        ServerCount::ruined(&ctx.sy, ctx.guild_id, ctx.channel.id, ctx.msg.author.id).await?;
        return Ok(());
    }

    ServerCount::increase(&ctx.sy, ctx.guild_id, ctx.channel.id, ctx.msg.author.id).await?;

    ctx.msg
        .react(
            &ctx.sy.http,
            ReactionType::Unicode(if number < server_count.highest_count {
                "✅".to_string()
            } else {
                "☑️".to_string()
            }),
        )
        .await?;

    Ok(())
}
