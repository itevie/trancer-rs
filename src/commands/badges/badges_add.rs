use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{content_response, trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::commands::CommandHasNoArgs;
use crate::models::aquired_badge::AquiredBadge;
use crate::trancer_config::all_badges::ALL_DEFINED_BADGES;
use crate::util::lang::{success, warn};
use crate::{command_argument_struct, command_file};
use serenity::all::User;
use std::collections::HashMap;

command_argument_struct!(AddBadgeArgs {
    user: User, PCACV::User,
    badge: String, PCACV::String
});

command_file! {
    TrancerCommand::<AddBadgeArgs> {
        name: "+badge".to_string(),
        t: TrancerCommandType::Help,
        description: "This is a test".to_string(),
        details: TrancerDetails {
            arguments: Some(TrancerArguments {
                required: 2,
                args: vec![
                    Argument {
                        name: "user".to_string(),
                        t: ArgType::User {
                            allow_bots: true,
                            infer: false,
                        },
                        details: Default::default()
                    },
                    Argument {
                        name: "badge".to_string(),
                        t: ArgType::String {
                            flags: None,
                        },
                        details: Default::default()
                    }
                ]
            }),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            println!("{:#?}", ALL_DEFINED_BADGES.iter().map(|x| x.id.to_string()).collect::<Vec<String>>().join(", "));
            let Some(badge) = ALL_DEFINED_BADGES.iter().find(|x| x.id == args.badge) else {
                return Ok(content_response(warn("Invalid badge name!")))
            };

            if AquiredBadge::has(&ctx.sy, args.user.id, badge.id).await? {
                return Ok(content_response(warn("That user already has the badge.")))
            }

            AquiredBadge::add_for(&ctx.sy, args.user.id, badge.id).await?;

            Ok(content_response(success("Added the badge!")))
        }),
    }
}
