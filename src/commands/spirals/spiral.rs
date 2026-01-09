use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::models::spiral::Spiral;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "spiral".to_string(),
        t: TrancerCommandType::Spirals,
        description: "Send a random spiral!".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["s".to_string()]),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            let spiral = Spiral::get_random(&ctx.sy).await?;
            Ok(TrancerResponseType::Content(spiral.link))
        }),
    }
}
