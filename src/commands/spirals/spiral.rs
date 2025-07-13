use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::trancer_handler;
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::commands::CommandHasNoArgs;
use crate::models::spiral::Spiral;
use crate::{command_argument_struct, command_file};
use std::collections::HashMap;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "spiral".to_string(),
        t: TrancerCommandType::Spirals,
        description: "Send a random spiral!".to_string(),
        details: Default::default(),

        handler: trancer_handler!(|ctx, args| {
            let spiral = Spiral::get_random(&ctx.sy).await?;
            Ok(TrancerResponseType::Content(spiral.link))
        }),
    }
}
