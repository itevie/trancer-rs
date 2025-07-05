use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::trancer_handler;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{
    ArgumentError, TrancerCommand, TrancerDetails, TrancerError, TrancerResponseType,
};
use crate::models::user_imposition::{ImpositionTag, UserImposition};
use crate::{command_argument_struct, command_file};
use serenity::all::User;
use std::collections::HashMap;
use rand::random;
use crate::models::user_data::UserData;
use crate::util::lang::pronoun;

command_argument_struct!(SendTriggerArgs {
   user: User, PCACV::User
});

command_file!(TrancerCommand::<SendTriggerArgs> {
    name: "sendtrigger".to_string(),
    description: "Send yours or someone else's trigger".to_string(),
    details: TrancerDetails {
        aliases: Some(vec![
            "i".to_string(),
            "impo".to_string(),
            "t".to_string(),
            "trigger".to_string()
        ]),
        arguments: Some(TrancerArguments {
            required: 1,
            args: vec![Argument {
                name: "user".to_string(),
                t: ArgType::User {
                    allow_bots: false,
                    infer: true
                },
                details: Default::default()
            }]
        }),
        ..Default::default()
    },

    handler: trancer_handler!(|ctx, args| {
        let user_data = UserData::fetch(&ctx.sy, args.user.id, ctx.msg.guild_id.unwrap()).await?;
        let mut tags: Vec<ImpositionTag> = vec![ImpositionTag::from(user_data.hypno_status.clone())];

        if args.user.id != ctx.msg.author.id {
            tags.push(ImpositionTag::ByOthers);
        }

        let triggers =
            UserImposition::fetch_all_for_by_tag(&ctx.sy, args.user.id, tags.as_slice()).await?;

        if triggers.is_empty() {
            return Ok(TrancerResponseType::Content(format!(
                "I couldn't find a trigger for **{}**!\nNote: {} status is: **{:?}**",
                pronoun(&ctx.msg.author, &args.user , "you", "$name"),
                pronoun(&ctx.msg.author, &args.user, "you", "their"),
                user_data.hypno_status
            )))
        };

        let number = random::<usize>() % triggers.len();
        Ok(TrancerResponseType::Content(triggers.get(number).unwrap().what.clone()))
    })
});
