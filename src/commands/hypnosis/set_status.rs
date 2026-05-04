use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::TrancerCommand;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{CommandTrait, TrancerError, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;

use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::models::user_data::UserDataFields;
use regex::Regex;
use serenity::all::EditMember;

fn type_map(input: &str) -> Option<&'static str> {
    match input {
        "red" | "r" => Some("red"),
        "yellow" | "orange" | "y" | "o" => Some("orange"),
        "green" | "g" => Some("green"),
        _ => None,
    }
}

fn db_map(input: &str) -> Option<&'static str> {
    match input {
        "red" | "r" => Some("red"),
        "yellow" | "orange" | "y" | "o" => Some("yellow"),
        "green" | "g" => Some("green"),
        _ => None,
    }
}

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "setstatus".to_string(),
        t: TrancerCommandType::Hypnosis,
        description: "Set your traffic light status".to_string(),
        details: TrancerDetails {
            arguments: Some(TrancerArguments {
                required: 1,
                args: vec![Argument {
                    name: "status".to_string(),
                    details: Default::default(),
                    t: ArgType::String { flags: None }
                }]
            }),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            let args: Vec<&str> = ctx.full_args.split_whitespace().collect();

            let input = args[0];
            let no_nickname = args.get(1).map(|v| *v == "true").unwrap_or(false);

            let status = match type_map(input) {
                Some(s) => s,
                None => {
                    return Err(TrancerError::NonScary("That's not a correct status! Try: green, yellow or red!".to_string()));
                }
            };

            let db_status = db_map(input).unwrap();

            let _ = ctx.user_data
                .update_key(&ctx.sy, UserDataFields::hypno_status, db_status)
                .await;

            if !no_nickname {
                let guild_id = ctx.guild_id;
                let user_id = ctx.user_id;

                let mut member = match guild_id.member(&ctx.sy.http, user_id).await {
                    Ok(m) => m,
                    Err(_) => {
                        return Err(TrancerError::NonScary("Oopsie... I failed to fetch your member ".to_string()));
                    }
                };

                let mut current_nick = member.nick.clone()
                    .unwrap_or_else(|| member.user.name.clone());

                let re = Regex::new(r"\(\p{Emoji}\)").unwrap();
                current_nick = re.replace(&current_nick, "").to_string();

                let emoji = match ctx.server_settings.status_theme.as_str() {
                    "circles" => match status {
                        "red" => "🔴",
                        "orange" => "🟠",
                        "green" => "🟢",
                        _ => "🔴",
                    },
                    "squares" => match status {
                        "red" => "🟥",
                        "orange" => "🟧",
                        "green" => "🟩",
                        _ => "🟥",
                    },
                    "fruit" => match status {
                        "red" => "🍎",
                        "orange" => "🍊",
                        "green" => "🍏",
                        _ => "🍎",
                    },
                    "hearts" => match status {
                        "red" => "❤️",
                        "orange" => "🧡",
                        "green" => "💚",
                        _ => "❤️",
                    },
                    "books" => match status {
                        "red" => "📕",
                        "orange" => "📙",
                        "green" => "📗",
                        _ => "📕",
                    },
                    "flowers" => match status {
                        "red" => "🌹",
                        "orange" => "🌻",
                        "green" => "🥬",
                        _ => "🌹",
                    },
                    _ => "🔴",
                };

                let nickname = format!("{} ({})", current_nick.trim(), emoji);

                if nickname.encode_utf16().count() >= 32 {
                    return Err(TrancerError::NonScary(
                        "I can't change your nickname as your nickname would be too long if I added the status indicator!\nBut your status has indeed been updated! :cyclone:".to_string()
                    ));
                }

                let guild = match ctx.sy.http.get_guild(ctx.guild_id).await {
                    Ok(g) => g,
                    Err(_) => {
                        return Err(TrancerError::NonScary(
                            "I failed... I couldn't fetch the server!".to_string()
                        ));
                    }
                };

                let bot_id = ctx.sy.cache.current_user().id;

                let bot_member = match ctx.guild_id.member(&ctx.sy.http, bot_id).await {
                    Ok(m) => m,
                    Err(_) => {
                        return Err(TrancerError::NonScary(
                            "I somehow failed to fetch my own member :(".to_string()
                        ));
                    }
                };

                let bot_permissions = bot_member.roles.iter()
                    .filter_map(|role_id| guild.roles.get(role_id))
                    .fold(serenity::model::permissions::Permissions::empty(), |acc, role| {
                        acc | role.permissions
                    });

                if !bot_permissions.change_nickname() {
                    return Err(TrancerError::NonScary(
                        "I can't change your nickname as my role doesnt have the Manage Nicknames permission!".to_string()
                    ));
                }

                let member_highest = member.roles.iter()
                    .filter_map(|role_id| guild.roles.get(role_id))
                    .map(|r| r.position)
                    .max()
                    .unwrap_or(0);

                let bot_highest = bot_member.roles.iter()
                    .filter_map(|role_id| guild.roles.get(role_id))
                    .map(|r| r.position)
                    .max()
                    .unwrap_or(0);

                let is_owner = user_id == guild.owner_id;

                if is_owner {
                    return Err(TrancerError::NonScary(
                        "I can't change your nickname as you are the owner!".to_string()
                    ));
                }

                if member_highest >= bot_highest {
                    return Err(TrancerError::NonScary(
                        "I can't change your nickname as your role is higher than mine! Try dragging the Trancer role higher".to_string()
                    ));
                }

                if let Err(e) = member
                    .edit(&ctx.sy.http, EditMember::new().nickname(nickname))
                    .await
                {
                    return Err(TrancerError::NonScary(
                        format!("Failed to change nickname: {}", e)
                    ));
                }
            }

            Ok(TrancerResponseType::Content(
                format!("Updated your status to **{}**! :cyclone:", status)
            ))
        }),
    }
}
