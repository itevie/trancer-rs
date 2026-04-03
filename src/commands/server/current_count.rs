use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{content_response, trancer_handler};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::models::couting::ServerCount;
use crate::util::embeds::create_embed;
use serenity::all::CreateMessage;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "currentcount".to_string(),
        t: TrancerCommandType::Counting,
        description: "Get details on the current count".to_string(),
        details: Default::default(),

        handler: trancer_handler!(|ctx, _args| {
            let Some(current_count) = ServerCount::fetch(&ctx.sy, ctx.guild_id, ctx.channel.id).await? else {
                return Ok(content_response("This channel does not have a count set up!"));
            };

            Ok(
                TrancerResponseType::Big(
                    CreateMessage::new()
                    .embed(
                        create_embed()
                        .title(format!("Count for {}", ctx.channel.name))
                        .description(format!("Current Count: {}\nHighest Count: {}\n\nUse `{}countruins` to see who ruined", current_count.current_count, current_count.highest_count, ctx.server_settings.prefix))
                    ))
            )
        }),
    }
}
