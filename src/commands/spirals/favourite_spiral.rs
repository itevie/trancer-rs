use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{content_response, trancer_handler, TrancerDetails};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::models::favourite_spiral::FavouriteSpiral;
use crate::models::spiral::Spiral;
use crate::util::lang::warn;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "favouritespiral".to_string(),
        t: TrancerCommandType::Spirals,
        description: "Send a random spiral!".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["fs".to_string()]),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            let Some(spiral) = FavouriteSpiral::get_random_for(&ctx.sy, ctx.msg.author.id).await? else {
              return Ok(content_response(warn(format!("You do not have any favourite spirals!\n\nReply to a spiral the bot has sent with `{}afs`", ctx.server_settings.prefix))))
            };

            Ok(TrancerResponseType::Content(spiral.link))
        }),
    }
}
