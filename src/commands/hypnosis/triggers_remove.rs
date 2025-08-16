use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, StringArgTypeFlag, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::models::user_imposition::UserImposition;
use crate::util::lang::warn;
use crate::{command_argument_struct, command_file};
use std::collections::HashMap;

command_argument_struct!(AddTriggerArgs {
    what: String, PCACV::String
});

command_file! {
    TrancerCommand::<AddTriggerArgs> {
        name: "removetrigger".to_string(),
        t: TrancerCommandType::Help,
        description: "Remove a trigger!".to_string(),
        details: TrancerDetails {
            arguments: Some(TrancerArguments {
                required: 1,
                args: vec![Argument {
                    name: "what".to_string(),
                    t: ArgType::String {
                        flags: Some(vec![StringArgTypeFlag::TakeContent])
                    },
                    details: Default::default()
                }],
            }),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            if !UserImposition::has(&ctx.sy, ctx.msg.author.id, args.what.clone()).await? {
                return Ok(TrancerResponseType::Content(warn("You do not have that trigger!")));
            }

            UserImposition::remove(&ctx.sy, ctx.msg.author.id, args.what.clone()).await?;

            Ok(TrancerResponseType::Content("Removed that trigger!".to_string()))
        }),
    }
}
