use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::trancer_handler;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{
    ArgumentError, TrancerCommand, TrancerDetails, TrancerError, TrancerResponseType,
};
use crate::models::user_data::UserData;
use crate::models::user_imposition::{ImpositionTag, UserImposition};
use crate::util::embeds::create_embed;
use crate::util::lang::pronoun;
use crate::util::pagination::{paginate, PaginationDataType, PaginationOptions};
use crate::{command_argument_struct, command_file};
use rand::random;
use serenity::all::User;
use std::collections::HashMap;

command_argument_struct!(ViewTriggersArgs {
   user: User, PCACV::User
});

command_file!(TrancerCommand::<ViewTriggersArgs> {
    name: "triggers".to_string(),
    description: "View yours or someone else's triggers".to_string(),
    details: TrancerDetails {
        aliases: None,
        arguments: Some(TrancerArguments {
            required: 1,
            args: vec![Argument {
                name: "user".to_string(),
                t: ArgType::User {
                    allow_bots: false,
                    infer: true
                },
                details: Default::default()
            }]
        }),
        ..Default::default()
    },

    handler: trancer_handler!(|ctx, args| {
        let triggers = UserImposition::fetch_all_for(&ctx.sy, args.user.id).await?;

        paginate(PaginationOptions {
            ctx: ctx.clone(),
            embed: create_embed().title(format!("{} triggers", pronoun(&ctx.msg.author, &args.user, "Your", "$name's"))),
            page_size: 20,
            data: PaginationDataType::Description {
                data: triggers.iter().map(|x| x.what.clone()).collect(),
                base_description: None,
            },
        })
        .await;

        Ok(TrancerResponseType::None)
    })
});
