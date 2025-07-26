use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::commands::CommandHasNoArgs;
use crate::{command_argument_struct, command_file};
use serenity::all::User;
use std::collections::HashMap;

command_argument_struct!(PayArgs {
    user: User, PCACV::User,
    amount: i32, PCACV::Number
});

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "pay".to_string(),
        t: TrancerCommandType::Help,
        description: "Give someone else your money".to_string(),
        details: TrancerDetails {
            arguments: Some(TrancerArguments {
               required: 2,
                args: vec![
                    Argument {
                        name: "user".to_string(),
                        t: ArgType::User {
                            infer: true,
                            allow_bots: false,
                        },
                        details: Default::default()
                    },
                    Argument {
                        name: "amount".to_string(),
                        t: ArgType::Currency {
                            allow_negative: false,
                            range: None,
                        },
                        details: Default::default()
                    }
                ]
            }),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            Ok(TrancerResponseType::Content("pong".to_string()))
        }),
    }
}
