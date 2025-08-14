use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::trancer_handler;
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::commands::CommandHasNoArgs;
use crate::{command_argument_struct, command_file};
use std::collections::HashMap;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "help".to_string(),
        t: TrancerCommandType::Help,
        description: "This is a test".to_string(),
        details: Default::default(),

        handler: trancer_handler!(|ctx, args| {
            Ok(TrancerResponseType::Content("pong".to_string()))
        }),
    }
}
