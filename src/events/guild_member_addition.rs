use crate::models::server_settings::ServerSettings;
use crate::util::config::CONFIG;
use crate::util::embeds::create_embed;
use crate::util::other::give_role;
use serenity::all::{
    Channel, ChannelId, ChannelType, Context, CreateEmbedFooter, CreateMessage, Member, RoleId,
};
use tracing::{error, instrument};

#[instrument]
pub async fn guild_member_addition(ctx: Context, new_member: Member) {
    // TODO: Add member count analytics
    let Ok(server_settings) = ServerSettings::fetch(&ctx, new_member.guild_id).await else {
        return;
    };

    if ctx.cache.current_user().id.to_string() == CONFIG.dev_bot.developer_bot_id
        && CONFIG.dev_bot.ignore_most_events
    {
        return;
    }

    // TODO: Check autoban

    if let Some(unverified_role) = server_settings.unverified_role_id {
        let Ok(unverified_role) = unverified_role.parse::<RoleId>() else {
            // TODO: Add error to some kind of database for the server owner
            return;
        };
        let Ok(result) = new_member.guild_id.role(&ctx, unverified_role).await else {
            // TODO: Add error to some kind of database for the server owner
            return;
        };
        if let Err(_err) = give_role(&ctx, &new_member, &result).await {
            // TODO: Add error to some kind of database for the server owner
        }
    }

    // TODO: Invite logger

    // Short circuit for non-twilight
    if new_member.guild_id.to_string() != CONFIG.server.id {
        if let Some(welcome_channel_id) = server_settings.welcome_channel_id {
            let Ok(channel_id) = welcome_channel_id.parse::<ChannelId>() else {
                // TODO: Add error to some kind of database for the server owner
                return;
            };
            let Ok(channel) = ctx.http.get_channel(channel_id).await else {
                // TODO: Add error to some kind of database for the server owner
                return;
            };
            let Channel::Guild(channel) = channel else {
                // TODO: Add error to some kind of database for the server owner
                return;
            };
            if channel.kind != ChannelType::Text {
                // TODO: Add error to some kind of database for the server owner
                return;
            }

            // TODO: Replace variables in the string
            if let Err(error) = channel
                .send_message(
                    &ctx,
                    CreateMessage::new().content(server_settings.welcome_message),
                )
                .await
            {
                // TODO: Add error to some kind of database for the server owner
            }
        }

        return;
    }

    let channel = match ctx
        .http
        .get_channel(CONFIG.channels.welcomes.parse::<ChannelId>().unwrap())
        .await
    {
        Ok(Channel::Guild(channel)) => channel,
        Err(err) => {
            error!("{}", err);
            return;
        }
        _ => {
            error!("Invalid channel for welcomes");
            return;
        }
    };

    if let Err(error) = channel.send_message(&ctx, CreateMessage::new()
        .embed(
            create_embed()
                .title("New member! :cyclone:")
                .description(
                    format!("Welcome **{}** to our server!\n\nRead here: <#1283861103964717126> to get access to our server!\n\nWe hope you enjoy your stay! :cyclone:", new_member.user.name)
                )
                .footer(CreateEmbedFooter::new(format!("We now have {} members", match match new_member.guild_id.to_partial_guild_with_counts(&ctx).await {
                    Ok(ok) => ok,
                    Err(err) => {
                        error!("{}", err);
                        return;
                    }
                }.approximate_member_count {
                    Some(ok) => ok,
                    None => {
                        error!("Failed to aproximate member count");
                        return;
                    }
                })))
        )
        .content(format!("<@{}>", new_member.user.id))
    ).await {
        error!("{}", error);
    }

    // TODO: Send temp message in how-to-verify
    // TODO: Delete that message
    // TODO: Check for who invited this member and award the inviter
}
