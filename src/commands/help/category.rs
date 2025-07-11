use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::commands::CommandHasNoArgs;
use crate::{command_argument_struct, command_file, commands};
use std::collections::HashMap;

command_argument_struct!(CategoryArgs {
    category: String, PCACV::String
});

command_file! {
    TrancerCommand::<CategoryArgs> {
        name: "category".to_string(),
        t: TrancerCommandType::Help,
        description: "Get all the commands in a category".to_string(),
        details: TrancerDetails {
            arguments: Some(TrancerArguments {
                required: 1,
                args: vec![Argument {
                    name: "category".to_string(),
                    t: ArgType::String {
                        flags: None
                    },
                    details: Default::default()
                }]
            }),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let cmds =  commands::init().into_iter().filter(|x| x.t().to_string() == args.category).collect::<Vec<_>>();

            if cmds.is_empty() {
                return Ok(TrancerResponseType::Content(format!(
                    "It appears there are no commands in that category! Here are the list of categories:\n\n{}",
                    TrancerCommandType::all().join(", ")
                )))
            }

            Ok(TrancerResponseType::Content(format!(
                "Here are all the commands in the **{}** category:\n\n{}",
                args.category,
                cmds.iter().map(|x| x.name()).collect::<Vec<_>>().join(", ")
            )))
        }),
    }
}
