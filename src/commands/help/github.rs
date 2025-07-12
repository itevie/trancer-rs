use crate::cmd_util::trancer_handler;
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "github".to_string(),
        t: TrancerCommandType::Help,
        description: "Get the bot's GitHub link".to_string(),
        details: Default::default(),

        handler: trancer_handler!(|ctx, args| {
            Ok(TrancerResponseType::Content(format!(
                "Trancer (new - Rust): https://github.com/itevie/trancer-rs{}",
                "\nTrancer (old - TypeScript): https://github.com/itevie/trancer"
            )))
        }),
    }
}
