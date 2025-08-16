use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, ArgumentDetails, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::commands::CommandHasNoArgs;
use crate::models::user_data::UserDataFields;
use crate::trancer_config::all_pronouns::ALL_PRONOUNS;
use crate::{command_argument_struct, command_file};
use std::collections::HashMap;

command_argument_struct!(SetPronounArgs {
    pronoun: String, PCACV::String
});

command_file! {
    TrancerCommand::<SetPronounArgs> {
        name: "setpronoun".to_string(),
        t: TrancerCommandType::Help,
        description: "Change the pronoun Trancer uses for you".to_string(),
        details: TrancerDetails {
            arguments: Some(TrancerArguments {
              required: 1,
                args: vec![Argument {
                    t: ArgType::String { flags: None },
                    name: "pronoun".to_string(),
                    details: ArgumentDetails {
                        one_of: Some(ALL_PRONOUNS.keys().map(|k| k.to_string()).collect::<Vec<String>>()),
                        ..Default::default()
                    }
                }]
            }),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            ctx.user_data.update_key(&ctx.sy, UserDataFields::pronoun_set, args.pronoun).await?;
            Ok(TrancerResponseType::Content("Updated your pronoun!".to_string()))
        }),
    }
}
