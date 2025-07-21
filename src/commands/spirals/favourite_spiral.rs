use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::trancer_handler;
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::commands::CommandHasNoArgs;
use crate::models::favourite_spiral::FavouriteSpiral;
use crate::models::spiral::Spiral;
use crate::util::lang::warn;
use crate::{command_argument_struct, command_file};
use std::collections::HashMap;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "favouritespiral".to_string(),
        t: TrancerCommandType::Spirals,
        description: "Send a random favourite spiral!".to_string(),
        details: Default::default(),

        handler: trancer_handler!(|ctx, args| {
            let Some(spiral) = FavouriteSpiral::get_random_for(&ctx.sy, ctx.msg.author.id).await? else {
                return Ok(TrancerResponseType::Content(warn("You don't have any favourite spirals! Favourite some by replying to a sent spiral with the addfavourite command!")))
            };

            Ok(TrancerResponseType::Content(spiral.link))
        }),
    }
}
