use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::TrancerCommand;
use crate::cmd_util::{content_response, CommandTrait, TrancerError};
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use regex::Regex;
use serenity::all::EditMember;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "removestatus".to_string(),
        t: TrancerCommandType::Hypnosis,
        description: "Remove the status from your nickname!".to_string(),
        details: TrancerDetails {
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            let mut member = match ctx.guild_id.member(&ctx.sy.http, ctx.user_id).await {
                Ok(m) => m,
                Err(_) => {
                    return Err(TrancerError::NonScary("Oopsie... I failed to fetch your member ".to_string()));
                }
            };

            let nick = member.nick.clone()
                .ok_or(TrancerError::NonScary("You don't have a nick applied!".to_string()))?;


            let re = Regex::new(r"\(\p{Emoji}\)").unwrap();
            let new_nick = re.replace(&nick, "").to_string();

            if nick == new_nick {
                return Err(TrancerError::NonScary("You don't have a status emoji in your nickname! Nothing to remove :cyclone:".to_string()));
            }

             if let Err(e) = member
                .edit(&ctx.sy.http, EditMember::new().nickname(new_nick))
                .await
            {
                return Err(TrancerError::NonScary(
                    format!("Failed to change nickname: {}", e)
                ));
            }

            Ok(content_response("Removed! :cyclone:"))
        }),
    }
}
