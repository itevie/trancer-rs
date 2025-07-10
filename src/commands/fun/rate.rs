use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::commands::CommandHasNoArgs;
use crate::util::lang::pronoun;
use crate::util::other::random_number_from_string;
use crate::{command_argument_struct, command_file};
use serenity::all::User;
use std::collections::HashMap;

command_argument_struct!(RateArgs {
    user: User, PCACV::User,
    what: String, PCACV::String
});

command_file! {
    TrancerCommand::<RateArgs> {
        name: "rate".to_string(),
        t: TrancerCommandType::Help,
        description: "Give someone a rating for anything!".to_string(),
        details: TrancerDetails {
            arguments: Some(TrancerArguments {
               required: 2,
                args: vec![
                    Argument {
                        name: "user".to_string(),
                        t: ArgType::User {
                            allow_bots: true,
                            infer: true
                        },
                        details: Default::default()
                    },
                    Argument {
                        name: "what".to_string(),
                        t: ArgType::String {
                            flags: None,
                        },
                        details: Default::default()
                    }
                ]
            }),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let amount = random_number_from_string(&format!("{}-{}", args.user.name, args.what), -5, 100);
            Ok(TrancerResponseType::Content(
                format!("According to my calculation... **{}** is... **{}% {}**", args.user.name , amount, args.what)
            ))
        }),
    }
}
