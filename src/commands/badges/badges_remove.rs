use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::{content_response, trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError};
use crate::cmd_util::{CommandTrait, TrancerFlag};
use crate::models::aquired_badge::AquiredBadge;
use crate::trancer_config::all_badges::ALL_DEFINED_BADGES;
use crate::util::lang::{success, warn};
use crate::{command_argument_struct, command_file};
use serenity::all::User;
use std::collections::HashMap;

command_argument_struct!(RemoveBadgeArgs {
    user: User, PCACV::User,
    badge: String, PCACV::String
});

command_file! {
    TrancerCommand::<RemoveBadgeArgs> {
        name: "-badge".to_string(),
        t: TrancerCommandType::Badges,
        description: "Remove a badge for a user".to_string(),
        details: TrancerDetails {
            flags: Some(vec![TrancerFlag::BotOwnerOnly]),
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
            let Some(badge) = ALL_DEFINED_BADGES.iter().find(|x| x.id == args.badge) else {
                return Ok(content_response(warn("Invalid badge name!")))
            };

            if !AquiredBadge::has(&ctx.sy, args.user.id, badge.id).await? {
                return Ok(content_response(warn("That user does not have the badge.")))
            }

            AquiredBadge::remove_for(&ctx.sy, args.user.id, badge.id).await?;

            Ok(content_response(success("Removed the badge!")))
        }),
    }
}
