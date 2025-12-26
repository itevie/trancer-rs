use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::models::aquired_badge::AquiredBadge;
use crate::util::cached_usernames::get_cached_username;
use crate::util::embeds::create_embed;
use crate::util::pagination::{paginate, PaginationDataType, PaginationOptions};
use crate::{command_argument_struct, command_file};
use std::collections::HashMap;

command_argument_struct!(WhoHasBadgeArgs {
    name: String, PCACV::String
});

command_file! {
    TrancerCommand::<WhoHasBadgeArgs> {
        name: "whohasbadge".to_string(),
        t: TrancerCommandType::Badges,
        description: "See who has a badge".to_string(),
        details: TrancerDetails {
            arguments: Some(TrancerArguments {
                required: 1,
                args: vec![
                    Argument {
                        name: "name".to_string(),
                        t: ArgType::String {
                            flags: None
                        },
                        details: Default::default(),
                    }
                ]
            }),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let badges = AquiredBadge::get_all_by_badge(&ctx.sy, args.name.clone()).await?;

            paginate(PaginationOptions {
                ctx,
                embed: create_embed().title(format!("Who has the badge {}?", args.name)),
                page_size: 10,
                data: PaginationDataType::Description {
                    base_description: None,
                    data: badges.0.iter().map(|x|
                        format!("**{}** has it!", get_cached_username(x.user.clone()))
                    ).collect()
                },

            }).await?;

            Ok(TrancerResponseType::None)
        }),
    }
}

pub fn a<'a, T: Into<&'a str>>(val: T) -> T {
    val
}
