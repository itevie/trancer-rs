use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{content_response, trancer_handler, TrancerDetails};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::dawnagotchi::get_dawnagotchi::send_dawn_message;
use crate::commands::CommandHasNoArgs;
use crate::models::dawnagotchi::Dawnagotchi;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "water".to_string(),
        t: TrancerCommandType::Dawnagotchi,
        description: "Water your Dawnagotchi!".to_string(),
        details: TrancerDetails {
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            let dawn = match Dawnagotchi::fetch(&ctx.sy, ctx.msg.author.id).await {
                Ok(ok) => ok,
                Err(_) => return Ok(content_response("You do not have a Dawnagotchi!"))
            };

            dawn.water(&ctx.sy).await?;
            send_dawn_message(&ctx).await?;

            Ok(TrancerResponseType::None)
        }),
    }
}
