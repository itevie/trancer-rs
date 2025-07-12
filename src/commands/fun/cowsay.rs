use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, StringArgTypeFlag, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::{command_argument_struct, command_file};
use std::collections::HashMap;

command_argument_struct!(CowsayArgs {
    text: String, PCACV::String
});

command_file! {
    TrancerCommand::<CowsayArgs> {
        name: "cowsay".to_string(),
        t: TrancerCommandType::Help,
        description: "Make a cow or something else say something".to_string(),
        details: TrancerDetails {
            arguments: Some(TrancerArguments {
                required: 1,
                args: vec![Argument {
                    name: "text".to_string(),
                    t: ArgType::String {
                        flags: Some(vec![StringArgTypeFlag::TakeContent])
                    },
                    details: Default::default()
                }]
            }),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let result = std::process::Command::new("cowsay").arg(args.text.clone()).output()?;
            Ok(TrancerResponseType::Content(format!("```{}```", String::from_utf8_lossy(&result.stdout))))
        }),
    }
}
