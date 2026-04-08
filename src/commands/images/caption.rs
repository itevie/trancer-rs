use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::TrancerCommand;
use crate::cmd_util::{trancer_handler, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::util::image_gen::ffmpeg::add_caption_to_gif;
use serenity::all::{CreateAttachment, CreateMessage};

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "caption".to_string(),
        t: TrancerCommandType::Help,
        description: "Get some basic details about the bot!".to_string(),
        details: Default::default(),

        handler: trancer_handler!(|_ctx, _args| {
            let result = add_caption_to_gif("/home/isabella/Documents/projects/rust/trancer-rs/src/images/dawn/base_dawn.png", "hello", "png")?;

            let attachment = CreateAttachment::bytes(result.0, result.1);

            Ok(TrancerResponseType::Big(CreateMessage::new().add_file(attachment)))
        }),
    }
}
