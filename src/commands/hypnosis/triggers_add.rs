use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, StringArgTypeFlag, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::commands::CommandHasNoArgs;
use crate::models::user_imposition::UserImposition;
use crate::{command_argument_struct, command_file, reply};
use serenity::all::{
    ButtonStyle, CreateActionRow, CreateButton, CreateInteractionResponse,
    CreateInteractionResponseMessage, EditMessage,
};
use serenity::builder::CreateMessage;
use serenity::futures::StreamExt;
use std::collections::HashMap;
use std::time::Duration;
use tracing::error;

command_argument_struct!(AddTriggerArgs {
    what: String, PCACV::String
});

command_file! {
    TrancerCommand::<AddTriggerArgs> {
        name: "addtrigger".to_string(),
        t: TrancerCommandType::Help,
        description: "Add a trigger!".to_string(),
        details: TrancerDetails {
            arguments: Some(TrancerArguments {
                required: 1,
                args: vec![Argument {
                    name: "what".to_string(),
                    t: ArgType::String {
                        flags: Some(vec![StringArgTypeFlag::TakeContent])
                    },
                    details: Default::default()
                }],
            }),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            if UserImposition::has(&ctx.sy, ctx.msg.author.id, args.what.clone()).await? {
                return Ok(TrancerResponseType::Content("You already have that trigger!".to_string()));
            }

            let mut imposition = UserImposition::create(&ctx.sy, ctx.msg.author.id, args.what.clone()).await?;

            let mut tags: Vec<&str> = vec!["green", "yellow", "red", "by others", "bombard"];

            let make_buttons = |imposition: UserImposition| {
                CreateActionRow::Buttons(
                    tags.iter().map(|x| CreateButton::new(x.to_string()).label(x.to_string()).style(if imposition.tags.contains(x) { ButtonStyle::Success } else { ButtonStyle::Danger })).collect()
                )
            };

            let mut msg = reply!(ctx, CreateMessage::new().content(format!(
                "{}\n\n*Use the buttons below to specify when this trigger can be used*", args.what.clone()
            )).components(vec![make_buttons().clone()]))?;

            let mut collector = msg
                .await_component_interactions(&ctx.sy)
                .timeout(Duration::from_secs(5 * 60))
                .stream();

            while let Some(ref i) = collector.next().await {
                if i.user.id != ctx.msg.author.id {
                    let _ = i.create_response(&ctx.sy, CreateInteractionResponse::Message(
                       CreateInteractionResponseMessage::new().content("This is not for you!").ephemeral(true)
                    )).await;
                    continue;
                }

                i.defer(&ctx.sy).await?;

                let mut new = imposition.tags.split(";").collect::<Vec<&str>>();
                if new.contains(&i.data.custom_id.as_str()) {
                    new.retain(|x| x != &i.data.custom_id.as_str());
                } else {
                    new.push(i.data.custom_id.as_str());
                }

                imposition = UserImposition::set_tags(&ctx.sy, ctx.msg.author.id, new.join(";")).await?;
                msg.edit(&ctx.sy, EditMessage::new().components(vec![make_buttons().clone()])).await?;
            }

            Ok(TrancerResponseType::None)
        }),
    }
}
