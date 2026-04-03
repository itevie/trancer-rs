use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::{content_response, CommandTrait};
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError};
use crate::{command_argument_struct, command_file};
use serenity::model::Permissions;
use std::collections::HashMap;

command_argument_struct!(SlowmodeArgs {
   expr: String, PCACV::String
});

command_file! {
    TrancerCommand::<SlowmodeArgs> {
        name: "matheval".to_string(),
        t: TrancerCommandType::Fun,
        description: "Calculate a math expression".to_string(),
        details: TrancerDetails {
            arguments: Some(TrancerArguments {
                required: 1,
                args: vec![Argument {
                    name: "expr".to_string(),
                    t: ArgType::String { flags: None },
                    details: Default::default()
                }]
            }),
            user_permissions: Some(Permissions::MANAGE_CHANNELS),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            match meval::eval_str(ctx.full_args) {
                Ok(ok) => Ok(content_response(ok.to_string())),
                Err(err) => Ok(content_response(format!("Error: {}", err.to_string())))
            }
        }),
    }
}
