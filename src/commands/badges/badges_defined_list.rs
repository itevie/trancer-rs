use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::commands::CommandHasNoArgs;
use crate::trancer_config::all_badges::ALL_DEFINED_BADGES;
use crate::util::embeds::create_embed;
use crate::util::pagination::{paginate, Field, PaginationDataType, PaginationOptions};
use crate::{command_argument_struct, command_file};
use std::collections::HashMap;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "badgelist".to_string(),
        t: TrancerCommandType::Help,
        description: "This is a test".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["bl".to_string()]),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            // TODO: Make it show how many people have it
            paginate(PaginationOptions {
                embed: create_embed().title("List of badges".to_string()),
                ctx,
                page_size: 10,
                data: PaginationDataType::Field(ALL_DEFINED_BADGES.iter().map(|x| Field {
                    name: format!("{} {} ({})", x.emoji, x.id, x.name),
                    value: x.description.to_string(),
                    inline: false,
                }).collect()),
            }).await?;

            Ok(TrancerResponseType::None)
        }),
    }
}
