use crate::cmd_util::TrancerError;
use crate::models::server_settings::ServerSettings;
use crate::models::user_data::{UserData, UserDataFields};
use crate::util::cached_usernames::get_cached_username;
use crate::util::lang::{replace_curly_string, CurlyStringParts};
use chrono::{DateTime, Local};
use serenity::all::{Channel, ChannelId, ChannelType, Context, CreateMessage, GuildId};
use tracing::instrument;

#[instrument]
pub async fn run(ctx: Context) -> Result<(), TrancerError> {
    for server_settings in ServerSettings::fetch_all(&ctx).await? {
        let Some(channel_id) = server_settings.birthday_channel_id else {
            continue;
        };

        let parsed_channel_id = channel_id.parse::<ChannelId>()?;
        let guild_id = server_settings.server_id.parse::<GuildId>()?;

        let channel = ctx
            .cache
            .guild(guild_id)
            .and_then(|g| g.channels.get(&parsed_channel_id).cloned());

        let guild_channel = match channel {
            Some(c) if c.kind == ChannelType::Text => c,
            _ => match parsed_channel_id.to_channel(&ctx.http).await {
                Ok(Channel::Guild(c)) if c.kind == ChannelType::Text => c,
                Ok(Channel::Guild(c)) if c.kind == ChannelType::News => c,
                _ => continue,
            },
        };

        for user_data in UserData::fetch_for_server(&ctx, server_settings.server_id.parse()?)
            .await?
            .iter()
            .filter(|x| x.birthday.is_some())
        {
            if !user_data.is_birthday_today() {
                continue;
            }

            if let Some(ref last_announced) = user_data.birthday_last_announced {
                if is_today(&last_announced) {
                    continue;
                }
            }

            let s = replace_curly_string(
                server_settings.birthday_announcement_text.clone(),
                CurlyStringParts {
                    user: None,
                    user_id: Some(user_data.user_id.clone()),
                    user_username: Some(get_cached_username(user_data.user_id.clone())),
                },
            );

            guild_channel
                .send_message(&ctx.http, CreateMessage::new().content(s))
                .await?;

            user_data
                .update_key(
                    &ctx,
                    UserDataFields::birthday_last_announced,
                    Some(Local::now().to_rfc3339()),
                )
                .await?;
        }
    }

    Ok(())
}

fn is_today(date_str: &str) -> bool {
    let Ok(parsed) = DateTime::parse_from_rfc3339(date_str) else {
        return false;
    };

    let parsed_local = parsed.with_timezone(&Local);
    let today = Local::now().date_naive();

    parsed_local.date_naive() == today
}
