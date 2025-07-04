use crate::cmd_util::{trancer_handler, TrancerCommand, TrancerDetails, TrancerResponseType};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::command_file;

command_file!(
    TrancerCommand {
        name: "command".to_string(),
        description: "Get information on a command".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["cmd".to_string()]),

            arguments: Some(TrancerArguments {
                required: 1,
                args: vec![
                    Argument {
                        name: "name".to_string(),
                        t: ArgType::String { flags: None },
                        details: Default::default(),
                    }
                ]
            }),

            ..Default::default()
        },

        handler: trancer_handler!(|ctx, msg| {
            Ok(TrancerResponseType::None)
        })
    }
);