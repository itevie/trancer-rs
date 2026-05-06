use crate::cmd_util::TrancerError;
use crate::models::server_settings::{ServerSettings, ServerSettingsFields};
use crate::util::embeds::create_embed;
use chrono::Local;
use serenity::all::{Channel, ChannelId, Context};
use serenity::builder::CreateMessage;
use tracing::instrument;

#[instrument]
pub async fn run(ctx: Context) -> Result<(), TrancerError> {
    let servers: Vec<ServerSettings> = ServerSettings::fetch_all(&ctx)
        .await?
        .iter()
        .filter(|x| x.remind_bumps == true)
        .cloned()
        .collect();

    for server in servers {
        if server.bump_reminded {
            continue;
        }

        let now = Local::now().timestamp();
        let last_bump = server
            .last_bump
            .clone()
            .unwrap_or("0".to_string())
            .parse::<i64>()?;

        if now - last_bump < 7200 {
            continue;
        }

        server
            .update_key(&ctx, ServerSettingsFields::bump_reminded, true)
            .await?;

        let Some(channel_id) = server.bump_channel else {
            continue;
        };

        let channel = match channel_id.parse::<ChannelId>()?.to_channel(&ctx).await? {
            Channel::Guild(channel) => channel,
            _ => continue,
        };

        channel
            .send_message(
                &ctx,
                CreateMessage::new()
                    .content(if let Some(last_bumper) = server.last_bumper {
                        format!("<@{last_bumper}>")
                    } else {
                        "".to_string()
                    })
                    .embed(
                        create_embed()
                            .title("It's time to bump!")
                            .description("Run `/bump` with DISBOARD to help us grow!"),
                    ),
            )
            .await?;
    }

    Ok(())
}
