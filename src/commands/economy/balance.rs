use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::models::economy::Economy;
use crate::util::lang::{currency, pronu};
use crate::{command_argument_struct, command_file};
use serenity::all::User;
use std::collections::HashMap;

command_argument_struct!(EcoBalanceArgs {
    user: User, PCACV::User
});

command_file! {
    TrancerCommand::<EcoBalanceArgs> {
        name: "balance".to_string(),
        t: TrancerCommandType::Economy,
        description: "Get yours or another person's economy balance".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["bal".to_string()]),
            arguments: Some(TrancerArguments {
                required: 1,
                args: vec![Argument {
                    name: "user".to_string(),
                    t: ArgType::User {
                        allow_bots: true,
                        infer: true,
                    },
                    details: Default::default()
                }]
            }),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let eco = Economy::fetch(&ctx.sy, args.user.id).await?;

            Ok(TrancerResponseType::Content(format!(
                "{} balance is {}",
                pronu(&ctx.msg.author, &args.user),
                currency(eco.balance)
            )))
        }),
    }
}
