use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{content_response, generic, trancer_handler, TrancerDetails};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::util::lang::warn;
use crate::util::other::give_role;
use serenity::all::{ChannelId, CreateMessage, Permissions, ReactionType, RoleId};

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "verify".to_string(),
        t: TrancerCommandType::Admin,
        description: "Verify someone into the server.".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["v".to_string()]),
            user_permissions: Some(Permissions::MANAGE_MESSAGES),
            bot_permissions: Some(Permissions::MANAGE_ROLES),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            if ctx.server_settings.verification_role_id.is_none() && ctx.server_settings.unverified_role_id.is_none() {
                return Ok(content_response(warn("This server does not have a verify or a unverified role!")));
            }

            let referenced_message = ctx.clone().msg.referenced_message.unwrap();
            let member = ctx.guild_id.member(&ctx.sy, referenced_message.author.id).await.map_err(|x| generic(
                format!("Failed to fetch member for verification: {}", x),
            ))?;

            if let Some(v) = ctx.server_settings.verification_role_id {
                let role = ctx.sy.http.get_guild_role(ctx.guild_id, v.parse::<RoleId>()?).await
                    .map_err(|x| generic(format!("Failed to fetch the verified role: {x}")))?;
                give_role(&ctx.sy, &member, &role).await?;
            }

            if let Some(uv) = ctx.server_settings.unverified_role_id {
                let role = ctx.sy.http.get_guild_role(ctx.guild_id, uv.parse::<RoleId>()?).await
                    .map_err(|x| generic(format!("Failed to fetch the unverified role: {x}")))?;
                member.remove_role(&ctx.sy, role).await?;
            }

            if let (Some(msg), Some(ch)) = (ctx.server_settings.verified_string, ctx.server_settings.verified_channel_id) {
                let channel = match ctx.sy.http.get_channel(ch.parse::<ChannelId>()?).await?.guild() {
                    Some(ch) => ch,
                    None => return Err(generic("The channel to send the verified message in is not a guild channel"))
                };

                // TODO: Make it replace the variables
                channel.send_message(&ctx.sy, CreateMessage::new().content(msg)).await?;
            }

            referenced_message.react(&ctx.sy, ReactionType::Unicode("✅".to_string())).await?;
            let _ = ctx.msg.delete(&ctx.sy).await;
            Ok(TrancerResponseType::None)
        }),
    }
}
