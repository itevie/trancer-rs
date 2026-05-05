use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::models::favourite_spiral::FavouriteSpiral;
use crate::models::spiral::Spiral;
use serenity::all::{
    ButtonStyle, CreateInteractionResponse, CreateInteractionResponseMessage, CreateMessage,
};
use serenity::builder::{CreateActionRow, CreateButton};
use serenity::collector::ComponentInteractionCollector;
use std::time::Duration;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "spiral".to_string(),
        t: TrancerCommandType::Spirals,
        description: "Send a random spiral!".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["s".to_string()]),
            ..Default::default()
        },
        handler: trancer_handler!(|ctx, _args| {
            let spiral = Spiral::get_random(&ctx.sy).await?;

            let msg = ctx.msg.channel_id.send_message(&ctx.sy.http, CreateMessage::new().content(&spiral.link)
                .components(
                    vec![CreateActionRow::Buttons(vec![
                         CreateButton::new("favourite")
                            .emoji('⭐')
                            .style(ButtonStyle::Primary),

                        CreateButton::new("info")
                            .emoji('ℹ')
                            .style(ButtonStyle::Secondary),
                    ])]
                )
            ).await?;

            while let Some(interaction) = ComponentInteractionCollector::new(&ctx.sy)
                .message_id(msg.id)
                .timeout(Duration::from_secs(60))
                .await
            {
                let custom_id = &interaction.data.custom_id;

                if custom_id == "favourite" {
                    if FavouriteSpiral::exists(&ctx.sy, interaction.user.id, spiral.id).await? {
                        interaction.create_response(
                            &ctx.sy.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new()
                                    .content("You have already favourited this spiral!".to_string())
                                    .ephemeral(true),
                            )
                        ).await?;
                        continue;
                    }

                    FavouriteSpiral::add(&ctx.sy, interaction.user.id, spiral.id).await?;
                     interaction.create_response(
                            &ctx.sy.http,
                            CreateInteractionResponse::Message(
                                CreateInteractionResponseMessage::new()
                                    .content("Added to favourites!".to_string())
                                    .ephemeral(true),
                            )
                        ).await?;
                }

                if custom_id == "info" {

                }
            }

            Ok(TrancerResponseType::None)
        }),
    }
}
