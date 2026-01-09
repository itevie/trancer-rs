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
        name: "addfavouritespiral".to_string(),
        t: TrancerCommandType::Spirals,
        description: "Add a spiral to your favourite spirals list!".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["afs".to_string()]),
            requires_message_reference: true,
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            let spiral = Spiral::get_from_message(&ctx.sy, &ctx.msg).await?;

            if FavouriteSpiral::exists(&ctx.sy, ctx.msg.author.id, spiral.id).await? {
                return Ok(content_response(warn("You have already favourite this spiral!")))
            }

            FavouriteSpiral::add(&ctx.sy, ctx.msg.author.id, spiral.id).await?;

            Ok(TrancerResponseType::Content("Added to your favourite spirals!".to_string()))
        }),
    }
}
