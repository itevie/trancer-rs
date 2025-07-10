use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::util::lang::success;
use crate::{command_argument_struct, command_file};
use serenity::builder::EditChannel;
use serenity::model::Permissions;
use std::collections::HashMap;

command_argument_struct!(SlowmodeArgs {
   amount: i32, PCACV::Number
});

command_file! {
    TrancerCommand::<SlowmodeArgs> {
        name: "slowmode".to_string(),
        t: TrancerCommandType::Help,
        description: "Change the slowmode in a channel".to_string(),
        details: TrancerDetails {
            arguments: Some(TrancerArguments {
                required: 1,
                args: vec![Argument {
                    name: "amount".to_string(),
                    t: ArgType::Number { min: Some(0), max: Some(21600) },
                    details: Default::default()
                }]
            }),
            user_permissions: Some(Permissions::MANAGE_CHANNELS),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            ctx.channel.id.edit(&ctx.sy, EditChannel::new().rate_limit_per_user(args.amount as u16)).await?;
            Ok(TrancerResponseType::Content(success(
                format!("Set the slowmode in this channel to **{}** seconds!", args.amount)
            )))
        }),
    }
}
