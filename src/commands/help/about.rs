use crate::cmd_util::trancer_handler;
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::util::embeds::create_embed;
use crate::util::lang::list;
use serenity::all::CreateMessage;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "about".to_string(),
        t: TrancerCommandType::Help,
        description: "Get some basic details about the bot!".to_string(),
        details: Default::default(),

        handler: trancer_handler!(|ctx, args| {
            // TODO: Finish these details
            Ok(TrancerResponseType::Big(CreateMessage::new().embed(
                create_embed().title("About Trancer!")
                    .description(format!("I am a hypnosis Discord bot with many features! Check out `{}help` to lean how to use me!", ctx.server_settings.prefix))
                    .field("Details", list(vec![("test", "test")]), false)
                    .field("Credits", list(vec![("test", "test")]), false)
            )))
        }),
    }
}
