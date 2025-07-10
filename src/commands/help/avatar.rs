use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::commands::CommandHasNoArgs;
use crate::{command_argument_struct, command_file};
use serenity::all::User;
use std::collections::HashMap;
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::util::lang::pronoun;

command_argument_struct!(AvatarArgs {
    user: User, PCACV::User
});

command_file! {
    TrancerCommand::<AvatarArgs> {
        name: "avatar".to_string(),
        t: TrancerCommandType::Help,
        description: "Get your or another person's avatar".to_string(),
        details: TrancerDetails {
            arguments: Some(TrancerArguments {
                required: 1,
                args: vec![Argument {
                    name: "user".to_string(),
                    t: ArgType::User { allow_bots: true, infer: true},
                    details: Default::default()
                }]
            }),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let pronoun = pronoun(&ctx.msg.author, &args.user, "Your", "Their");

            Ok(TrancerResponseType::Content(if let Some(avatar) = args.user.avatar_url() {
                avatar
            } else {
                format!("Sorry, I could not fetch {} avatar!", pronoun)
            }))
        }),
    }
}
