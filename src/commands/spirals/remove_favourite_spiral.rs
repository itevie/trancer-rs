use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::{content_response, CommandTrait};
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::models::favourite_spiral::FavouriteSpiral;
use crate::models::spiral::Spiral;
use crate::util::lang::warn;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "removefavouritespiral".to_string(),
        t: TrancerCommandType::Spirals,
        description: "Remove a spiral from your favourite spirals list".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["rfs".to_string()]),
            requires_message_reference: true,
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            let spiral = Spiral::get_from_message(&ctx.sy, &ctx.msg).await?;

            if !FavouriteSpiral::exists(&ctx.sy, ctx.msg.author.id, spiral.id).await? {
                return Ok(content_response(warn("This spiral has not been favourited by you!")))
            }

            FavouriteSpiral::remove(&ctx.sy, ctx.msg.author.id, spiral.id).await?;

            Ok(TrancerResponseType::Content("Removed from your favourite spirals!".to_string()))
        }),
    }
}
