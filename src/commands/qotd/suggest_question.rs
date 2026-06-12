use crate::cmd_util::arg_parser::PCACV;
use crate::cmd_util::args::{ArgType, Argument, StringArgTypeFlag, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::{content_response, ArgumentError, CommandTrait, TrancerError, TrancerFlag};
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::commands::CommandArgumentStruct;
use crate::commands::CommandHasNoArgs;
use crate::models::qotd_question::QotdQuestion;
use crate::util::embeds::create_embed;
use crate::util::pagination::{PaginationDataType, PaginationOptions};
use crate::{command_argument_struct, command_file, confirm_action, reply};
use serenity::all::{ButtonStyle, CreateActionRow, CreateButton, CreateMessage, EditMessage};
use std::collections::HashMap;
use std::time::Duration;
use tracing::error;

command_argument_struct!(SuggestQuestionArgs {
    question: String, PCACV::String
});

command_file! {
    TrancerCommand::<SuggestQuestionArgs> {
        name: "suggestquestion".to_string(),
        t: TrancerCommandType::Qotd,
        description: "Suggest question for the QOTD".to_string(),
        details: TrancerDetails {
            flags: Some(vec![TrancerFlag::AdminOnly]),
            arguments: Some(TrancerArguments {
                required: 1,
                args: vec![
                    Argument {
                        name: "question".to_string(),
                        details: Default::default(),
                        t: ArgType::String { flags: Some(vec![StringArgTypeFlag::TakeContent]) },
                    }
                ]
            }),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            if args.question.len() < 5 {
                return Err(TrancerError::NonScary("The question must be longer than 5 charcters!".to_string()));
            }

            QotdQuestion::add_question(&ctx.sy, args.question.clone(), ctx.guild_id, ctx.user_id).await?;

            Ok(content_response(
                format!("Added question \"{}\"! Thanks! :cyclone:", args.question.clone()),
            ))
        }),
    }
}
