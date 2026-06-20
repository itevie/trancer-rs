use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::TrancerCommand;
use crate::cmd_util::{content_response, trancer_handler, TrancerDetails};
use crate::cmd_util::{CommandTrait, TrancerError};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::models::dawnagotchi::Dawnagotchi;
use crate::util::lang::currency;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "revivedawn".to_string(),
        t: TrancerCommandType::Dawnagotchi,
        description: "Revive your Dawnagotchi you evil scum.".to_string(),
        details: TrancerDetails {
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            let dawn = Dawnagotchi::fetch_for_revival(&ctx.sy, ctx.msg.author.id).await?;

            if dawn.alive {
                return Ok(content_response("Your Dawn is alive! :cyclone:"));
            }

            if ctx.economy.balance < 2500 {
                return Err(TrancerError::NonScary(
                    format!("You do not have {} to revive your Dawn!", currency(2500))
                ));
            }

            dawn.revive(&ctx.sy).await?;
            ctx.economy.remove_money(&ctx.sy, 2500, false).await?;

            Ok(content_response(
                format!("Your Dawn was revived for {} you evil scum!", currency(2500))
            ))
        }),
    }
}
