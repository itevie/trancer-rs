use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{content_response, trancer_handler};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::util::embeds::create_embed;
use crate::util::lang::list;
use serenity::all::CreateMessage;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "mcauth".to_string(),
        t: TrancerCommandType::Help,
        description: "Temporary".to_string(),
        details: Default::default(),

        handler: trancer_handler!(|ctx, _args| {
            return Ok(content_response("Please use `t!mcauth` instead."))
        }),
    }
}
