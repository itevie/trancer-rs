use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::cmd_util::{CommandTrait, TrancerFlag};
use crate::models::economy::Economy;
use crate::util::lang::currency;
use crate::{command_argument_struct, command_file};
use serenity::all::User;
use std::collections::HashMap;

command_argument_struct!(AddMoney {
    user: User, PCACV::User,
    amount: i32, PCACV::Number
});

command_file! {
    TrancerCommand::<AddMoney> {
        name: "addmoney".to_string(),
        t: TrancerCommandType::Economy,
        description: "Add money to someone's balance".to_string(),
        details: TrancerDetails {
            arguments: Some(TrancerArguments {
               required: 2,
                args: vec![
                    Argument {
                        name: "user".to_string(),
                        t: ArgType::User {
                            infer: true,
                            allow_bots: false,
                        },
                        details: Default::default()
                    },
                    Argument {
                        name: "amount".to_string(),
                        t: ArgType::Number { min: None, max: None },
                        details: Default::default()
                    }
                ]
            }),
            flags: Some(vec![TrancerFlag::BotOwnerOnly]),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let user = Economy::fetch(&ctx.sy, args.user.id).await?;
            user.add_money(&ctx.sy, args.amount as u32, None).await?;

            Ok(TrancerResponseType::Content(
                format!("Gave **{}** {}", args.user.name, currency(args.amount))
            ))
        }),
    }
}
