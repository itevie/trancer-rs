use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::models::quote::Quote;
use crate::{command_argument_struct, command_file};
use serenity::all::CreateMessage;
use std::collections::HashMap;

command_argument_struct!(GetQuoteArgs {
    id: i32, PCACV::Number
});

command_file! {
    TrancerCommand::<GetQuoteArgs> {
        name: "getquote".to_string(),
        t: TrancerCommandType::Help,
        description: "Get a quote by ID".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["gq".to_string()]),
            arguments: Some(TrancerArguments {
               required: 1,
                args: vec![Argument {
                    t: ArgType::Number { min: Some(1), max: None },
                    name: "id".to_string(),
                    details: Default::default()
                }]
            }),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let Some(quote) = Quote::get(&ctx.sy, args.id as u32).await? else {
                return Ok(TrancerResponseType::Content("Could not find a quote with that ID!".to_string()))
            };

            Ok(TrancerResponseType::Big(
                CreateMessage::new().embed(quote.to_embed(&ctx.sy).await?))
            )
        }),
    }
}
