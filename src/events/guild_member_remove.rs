use crate::cmd_util::TrancerError;
use crate::util::config::CONFIG;
use serenity::all::{Channel, ChannelId, Context, CreateMessage, GuildId, User};
use tracing::instrument;

#[instrument]
pub async fn guild_member_remove(
    ctx: Context,
    guild_id: GuildId,
    member: User,
) -> Result<(), TrancerError> {
    // let server_settings = ServerSettings::fetch(&ctx, guild_id).await?;

    let guild = guild_id.to_partial_guild_with_counts(&ctx).await?;

    // TODO: Add to analytics

    if guild_id.to_string() == CONFIG.server.id {
        let channel = match CONFIG
            .channels
            .welcomes
            .parse::<ChannelId>()?
            .to_channel(&ctx)
            .await?
        {
            Channel::Guild(channel) => channel,
            _ => {
                return Err(TrancerError::Generic(
                    "Channel is not a guild text channel".to_string(),
                ))
            }
        };

        channel
            .send_message(
                &ctx,
                CreateMessage::new().content(format!(
                    "**{}** left our server :( We now have {} members",
                    member.name,
                    guild.approximate_member_count.unwrap_or(0)
                )),
            )
            .await?;
    } else {
        // TODO: Add other servers
    }

    Ok(())
}
