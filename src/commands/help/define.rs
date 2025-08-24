use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::util::define::handle_define_message;
use crate::util::embeds::create_embed;
use crate::util::pagination::{paginate, Field, PaginationDataType, PaginationOptions};
use crate::{command_argument_struct, command_file};
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashMap;

command_argument_struct!(DefineArgs {
    what: String, PCACV::String
});

command_file! {
    TrancerCommand::<DefineArgs> {
        name: "define".to_string(),
        t: TrancerCommandType::Help,
        description: "Get a word's definition!".to_string(),
        details: TrancerDetails {
            arguments: Some(TrancerArguments {
                required: 1,
                args: vec![Argument {
                    name: "what".to_string(),
                    t: ArgType::String { flags: None },
                    details: Default::default(),
                }]
            }),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            handle_define_message(&ctx, args.what).await
        }),
    }
}
