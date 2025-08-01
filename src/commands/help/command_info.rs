use crate::cmd_util::arg_parser::CommandArgumentStruct;
use crate::cmd_util::arg_parser::PCACV;
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::{
    trancer_handler, ArgumentError, CommandTrait, TrancerCommand, TrancerDetails, TrancerError,
    TrancerResponseType,
};
use crate::util::embeds::create_embed;
use crate::{command_argument_struct, command_file};
use serenity::all::CreateMessage;
use std::collections::HashMap;

command_argument_struct!(ComamndInfoArgs {
   name: String, PCACV::String
});

command_file!(TrancerCommand::<ComamndInfoArgs> {
    name: "command".to_string(),
    t: TrancerCommandType::Help,
    description: "Get information on a command".to_string(),
    details: TrancerDetails {
        aliases: Some(vec!["cmd".to_string()]),

        arguments: Some(TrancerArguments {
            required: 1,
            args: vec![Argument {
                name: "name".to_string(),
                t: ArgType::String { flags: None },
                details: Default::default(),
            }]
        }),

        ..Default::default()
    },

    handler: trancer_handler!(|ctx, args| {
        todo!("Implement this");
        Ok(TrancerResponseType::Big(
            CreateMessage::new().content("hi").embed(create_embed()),
        ))
    })
});
