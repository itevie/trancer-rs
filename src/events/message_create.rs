use crate::cmd_util::arg_parser::parse_args;
use crate::cmd_util::{TrancerError, TrancerResponseType, TrancerRunnerContext};
use crate::commands::reply_response_type;
use crate::events::something_happened;
use crate::message_handlers::handle_message_handlers;
use crate::models::economy::Economy;
use crate::models::ratelimit::Ratelimit;
use crate::models::server_settings::ServerSettings;
use crate::models::user_data::UserData;
use crate::util::cached_usernames::set_cached_username;
use crate::util::embeds::create_embed;
use crate::util::lang::{permission_names, warn};
use crate::{commands, reply, something_happened};
use chrono::{DateTime, Utc};
use chrono_humanize::HumanTime;
use serenity::all::{
    Channel, ChannelType, Context, CreateMessage, EventHandler, Message, ReactionType,
};
use tracing::error;

pub async fn message(ctx: Context, msg: Message) {
    set_cached_username(msg.author.id.to_string(), msg.author.name.clone());

    // ----- Guards -----
    if msg.author.id == ctx.cache.current_user().id {
        return;
    }

    // ----- Unwrap Data -----
    let channel = if let Ok(channel) = msg.channel_id.to_channel(&ctx).await {
        match channel {
            Channel::Guild(channel) => {
                if channel.kind == ChannelType::Text {
                    channel
                } else {
                    return;
                }
            }
            _ => return,
        }
    } else {
        return;
    };

    let guild_id = match msg.guild_id {
        Some(id) => id,
        None => return,
    };

    let server_settings = match ServerSettings::fetch(&ctx, guild_id).await {
        Ok(ok) => ok,
        Err(e) => {
            error!(
                "Failed to fetch server_settings during message handler: {:?}",
                e
            );
            return;
        }
    };

    let user_data = match UserData::fetch(&ctx, msg.author.id, guild_id).await {
        Ok(ok) => ok,
        Err(e) => {
            error!(
                "Failed to fetch user_data during message handler: {}",
                e.to_string()
            );
            return;
        }
    };

    let economy = match Economy::fetch(&ctx, msg.author.id).await {
        Ok(ok) => ok,
        Err(e) => {
            error!(
                "Failed to fetch economy during message handler: {}",
                e.to_string()
            );
            return;
        }
    };

    let mut context = TrancerRunnerContext {
        sy: ctx.clone(),
        msg: msg.clone(),
        server_settings,
        channel: channel.clone(),
        user_data,
        economy,
        guild_id,
        command_name: "loading".to_string(),
        original_command: msg.content.to_string(),
    };

    if let Err(err) = handle_message_handlers(&context).await {
        error!("Failed while running message handlers (ignoring): {err}")
    }

    // ----- Command Checking -----
    if !msg
        .content
        .starts_with(context.server_settings.prefix.as_str())
    {
        return;
    }

    let contents = msg.content[context.server_settings.prefix.len()..].trim();
    let commands = commands::init();
    let mut args = parse_args(contents.to_string());

    if args.args.is_empty() {
        return;
    }

    let command_name = args.args[0].clone();
    args.args.remove(0);

    let Some(cmd) = commands.iter().find(|cmd| {
        cmd.name().eq(command_name.as_str())
            || cmd
                .details()
                .aliases
                .unwrap_or_default()
                .contains(&command_name)
    }) else {
        return;
    };
    context.command_name = cmd.name();

    let member = match guild_id.member(&ctx.http, msg.author.id).await {
        Ok(m) => m,
        Err(e) => {
            error!(
                "Failed to fetch member during message handler: {}",
                e.to_string()
            );
            return;
        }
    };

    // ----- Command Guard Checks -----
    if let Some(user_permission) = cmd.details().user_permissions {
        let permissions = match guild_id.to_guild_cached(&ctx.cache).map(|x| x.clone()) {
            Some(some) => some.user_permissions_in(&channel, &member),
            None => match ctx.http.get_guild(guild_id).await {
                Ok(some) => some.user_permissions_in(&channel, &member),
                Err(e) => {
                    something_happened(&context, "Failed to fetch the guild from cache or http, so I couldn't check your permissions",
                                           TrancerError::from(e)).await;
                    return;
                }
            },
        };

        if !permissions.contains(user_permission) {
            let _ = reply!(
                context,
                CreateMessage::new().embed(
                    create_embed()
                        .title("Sorry... you don't have permission to do that.")
                        .description(format!(
                            "You need the following permissions: {}",
                            permission_names(user_permission)
                        ))
                )
            );
            return;
        }
    }

    if let Some(r) = cmd.details().ratelimit {
        let ratelimit = match Ratelimit::fetch(&ctx, msg.author.id, cmd.name()).await {
            Ok(ok) => ok,
            Err(e) => {
                something_happened(&context, "Failed to fetch ratelimit", TrancerError::from(e))
                    .await;
                return;
            }
        };

        let prev = something_happened!(context, DateTime::parse_from_rfc3339(&ratelimit.last_used))
            .timestamp();
        let now = Utc::now().timestamp();

        if now - prev < r as i64 {
            let _ = reply!(
                context,
                CreateMessage::new().embed(create_embed().title(format!(
                    "Hey! You can't do that! Try again in **{}**",
                    HumanTime::from(DateTime::from_timestamp(now, 0).unwrap())
                )))
            );
            return;
        }

        match Ratelimit::update(&ctx, msg.author.id, cmd.name()).await {
            Ok(_) => (),
            Err(e) => {
                something_happened(
                    &context,
                    "Failed to update ratelimit",
                    TrancerError::from(e),
                )
                .await;
            }
        }
    }

    if cmd.details().requires_message_reference && msg.referenced_message.is_none() {
        let _ = reply!(
            context,
            CreateMessage::new()
                .content(warn("You need to reply to a message to use this command."))
        );
    }

    if cmd.details().slow {
        let _ = msg
            .react(&ctx, ReactionType::Unicode("⏳".to_string()))
            .await;
    }

    // ----- Command Running -----
    let response = match cmd.run(context.clone(), args).await {
        Ok(response) => response,
        Err(err) => {
            something_happened(&context, "Error while executing command handler", err).await;
            return;
        }
    };

    reply_response_type(&context, response).await
}
