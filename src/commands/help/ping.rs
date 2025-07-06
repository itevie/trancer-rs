use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::trancer_handler;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{TrancerCommand, TrancerError, TrancerResponseType};
use crate::{command_argument_struct, command_file};
use std::collections::HashMap;
use crate::cmd_util::types::TrancerCommandType;

command_argument_struct!(PingArgs {});

command_file! {
    TrancerCommand::<PingArgs> {
        name: "ping".to_string(),
        t: TrancerCommandType::Help,
        description: "This is a test".to_string(),
        details: Default::default(),

        handler: trancer_handler!(|ctx, args| {
            Ok(TrancerResponseType::Content("pong".to_string()))
        }),
    }
}
