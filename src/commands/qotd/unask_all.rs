use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{CommandTrait, TrancerError, TrancerFlag};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::commands::CommandHasNoArgs;
use crate::models::qotd_question::QotdQuestion;
use crate::util::embeds::create_embed;
use crate::util::pagination::{PaginationDataType, PaginationOptions};
use crate::{command_file, confirm_action, reply};
use serenity::all::{ButtonStyle, CreateActionRow, CreateButton, CreateMessage, EditMessage};
use std::time::Duration;
use tracing::error;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "unaskall".to_string(),
        t: TrancerCommandType::Qotd,
        description: "Unask all questions".to_string(),
        details: TrancerDetails {
            flags: Some(vec![TrancerFlag::AdminOnly]),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            let mut confirm = confirm_action!(ctx, CreateMessage::new().embed(create_embed().title("Are you sure?").description("All questions will be marked as not asked.")));

            match confirm.0 {
                false => {
                    confirm.1.edit(&ctx.sy, EditMessage::new().embeds(vec![]).content("Cancelled.")).await?;
                },
                true => {
                    QotdQuestion::unask_questions(&ctx.sy, ctx.guild_id).await?;
                    confirm.1.edit(&ctx.sy, EditMessage::new().embeds(vec![]).content("Set all questions as not asked! :cyclone:")).await?;
                }
            }

            Ok(TrancerResponseType::None)
        }),
    }
}
