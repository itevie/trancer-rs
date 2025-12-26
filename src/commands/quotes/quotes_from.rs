use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError};
use crate::models::quote::{Quote, QuoteListPaginationType};
use crate::{command_argument_struct, command_file};
use serenity::all::User;
use std::collections::HashMap;

command_argument_struct!(QuotesFromArgs {
    user: User, PCACV::User
});

command_file! {
    TrancerCommand::<QuotesFromArgs> {
        name: "quotesfrom".to_string(),
        t: TrancerCommandType::Quotes,
        description: "Get a list of quotes from a user".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["quotesfor".to_string(), "qs".to_string()]),
            arguments: Some(TrancerArguments {
              required: 1,
                args: vec![
                    Argument {
                        t: ArgType::User { allow_bots: true, infer: true },
                        name: "user".to_string(),
                        details: Default::default()
                    }
                ]
            }),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            Quote::get_from(&ctx.sy, args.user.id)
                .await?
                .paginate(
                    ctx.clone(),
                    QuoteListPaginationType::From(args.user.clone()
                )
            ).await
        }),
    }
}
