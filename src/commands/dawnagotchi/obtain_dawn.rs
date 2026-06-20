use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::TrancerCommand;
use crate::cmd_util::{content_response, trancer_handler, TrancerDetails};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::models::dawnagotchi::Dawnagotchi;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "obtaindawnagotchi".to_string(),
        t: TrancerCommandType::Dawnagotchi,
        description: "Start your adventure with the adorable Dawnagotchi!".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["obtaindawn".to_string(), "setupdawnagotchi".to_string(), "setupdawn".to_string()]),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            let dawn = Dawnagotchi::fetch(&ctx.sy, ctx.msg.author.id).await;

            if let Ok(_) = dawn {
                return Ok(content_response("Sorry, you already have a Dawnagotchi!"))
            }

            Dawnagotchi::create(&ctx.sy, ctx.msg.author.id).await?;

            Ok(content_response("A Dawn came up to you! And you obtained it! This must be the start of an amazing adventure"))
        }),
    }
}
