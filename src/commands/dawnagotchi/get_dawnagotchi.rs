use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails, TrancerError, TrancerRunnerContext};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::models::dawnagotchi::Dawnagotchi;
use crate::util::embeds::create_embed;
use crate::util::lang::warn;
use serenity::all::{
    ButtonStyle, CreateActionRow, CreateAttachment, CreateButton, CreateEmbed,
    CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage, EditMessage,
    Message,
};
use serenity::futures::StreamExt;
use std::time::Duration;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "dawnagotchi".to_string(),
        t: TrancerCommandType::Dawnagotchi,
        description: "See your Dawnagotchi!".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["dawn".to_string(), "getdawn".to_string(), "dawndet".to_string()]),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            let mut msg = send_dawn_message(&ctx).await?;

             let mut collector = msg
                .await_component_interactions(&ctx.sy)
                .timeout(Duration::from_secs(5 * 60))
                .stream();

            while let Some(ref i) = collector.next().await {
                if i.user.id != ctx.msg.author.id {
                    let _ = i
                        .create_response(
                            &ctx.sy,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new()
                                    .content(warn("This is not for you!"))
                                    .ephemeral(true),
                            ),
                        )
                        .await;
                    continue;
                }

                let dawn = Dawnagotchi::fetch(&ctx.sy, ctx.msg.author.id).await?;

                let result: Result<(), TrancerError>;

                match i.data.custom_id.as_str() {
                    "dawn-feed" => result = dawn.feed(&ctx.sy).await,
                    "dawn-water" => result = dawn.water(&ctx.sy).await,
                    "dawn-play" => result = dawn.play(&ctx.sy).await,
                    _ => unreachable!()
                }

                if let Err(err) = result {
                    i.create_response(
                        &ctx.sy.http,
                        CreateInteractionResponse::Message(
                            CreateInteractionResponseMessage::new()
                                .content(err.to_string()),
                        ),
                    )
                     .await?;
                }

                i.defer(&ctx.sy.http).await?;

                edit_dawn_message(&ctx, &mut msg).await?;
            }

            Ok(TrancerResponseType::None)
        }),
    }
}

pub async fn make_dawn_message_components(
    ctx: &TrancerRunnerContext,
) -> Result<(CreateEmbed, CreateActionRow, CreateAttachment), TrancerError> {
    let dawn = match Dawnagotchi::fetch(&ctx.sy, ctx.msg.author.id).await {
        Ok(ok) => ok,
        Err(_) => {
            return Err(TrancerError::NonScary(
                "You do not have a Dawnagotchi!".to_string(),
            ))
        }
    };

    let requirements = dawn.get_requirements();

    let embed = create_embed().description(format!(
        "Feed: {}, Drink: {}, Play: {}",
        requirements.feed, requirements.drink, requirements.play
    ));

    let buttons = CreateActionRow::Buttons(vec![
        CreateButton::new("dawn-feed")
            .style(ButtonStyle::Primary)
            .label("Feed")
            .disabled(requirements.feed >= 100),
        CreateButton::new("dawn-water")
            .style(ButtonStyle::Primary)
            .label("Water")
            .disabled(requirements.drink >= 100),
        CreateButton::new("dawn-play")
            .style(ButtonStyle::Primary)
            .label("Play")
            .disabled(requirements.play >= 100),
    ]);

    let image_bytes = dawn.make_dawn_image();
    let attachment = CreateAttachment::bytes(image_bytes, "dawn.png");

    let image_embed = embed.image("attachment://dawn.png");

    Ok((image_embed, buttons, attachment))
}

pub async fn edit_dawn_message(
    ctx: &TrancerRunnerContext,
    to_edit: &mut Message,
) -> Result<(), TrancerError> {
    let parts = make_dawn_message_components(&ctx).await?;

    to_edit
        .edit(
            &ctx.sy.http,
            EditMessage::new()
                .embed(parts.0.clone().image("attachment://dawn.png"))
                .new_attachment(parts.2),
        )
        .await?;

    Ok(())
}

pub async fn send_dawn_message(ctx: &TrancerRunnerContext) -> Result<Message, TrancerError> {
    let parts = make_dawn_message_components(&ctx).await?;

    let msg = ctx
        .msg
        .channel_id
        .send_message(
            &ctx.sy,
            CreateMessage::new()
                .embed(parts.0)
                .reference_message(&ctx.msg)
                .components(vec![parts.1])
                .add_file(parts.2),
        )
        .await?;

    Ok(msg)
}
