use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use crate::models::economy::MoneyAddReasion;
use crate::models::server_settings::ServerSettingsFields;
use crate::models::user_data::UserDataFields;
use crate::reply;
use crate::util::config::CONFIG;
use crate::util::embeds::create_embed;
use crate::util::random_rewards::{
    englishify_random_reward, generate_random_rewards, give_random_reward, RandomRewardItemOptions,
    RandomRewardOptions,
};
use chrono::Utc;
use serenity::all::{CreateMessage, MessageInteractionMetadata};
use tracing::{error, instrument};

static DISBOARD_ID: &str = "302050872383242240";

#[instrument]
pub async fn detect_bumps(ctx: &TrancerRunnerContext) -> Result<(), TrancerError> {
    if let Some(embed) = ctx.msg.embeds.first() {
        if let Some(description) = embed.description.as_ref() {
            if !description.contains("Bump done!") {
                return Ok(());
            }
        } else {
            return Ok(());
        }
    } else {
        return Ok(());
    }

    let command = if let Some(interaction) = ctx.msg.interaction_metadata.as_ref() {
        match interaction.as_ref() {
            MessageInteractionMetadata::Command(command) => command,
            _ => return Ok(()),
        }
    } else {
        return Ok(());
    };

    ctx.user_data
        .increment(&ctx.sy, UserDataFields::bumps, 1)
        .await?;
    ctx.server_settings
        .update_key(
            &ctx.sy,
            ServerSettingsFields::last_bump,
            Utc::now().to_rfc3339(),
        )
        .await?;
    ctx.server_settings
        .update_key(&ctx.sy, ServerSettingsFields::bump_reminded, false)
        .await?;
    ctx.server_settings
        .update_key(
            &ctx.sy,
            ServerSettingsFields::last_bump,
            command.user.id.to_string(),
        )
        .await?;

    let reward = if ctx.server_settings.server_id == CONFIG.server.id {
        let result = generate_random_rewards(
            &ctx.sy,
            RandomRewardOptions {
                currency: Some((
                    CONFIG.payouts.bumps.currency_min,
                    CONFIG.payouts.bumps.currency_max,
                )),
                items: Some(RandomRewardItemOptions {
                    items: None,
                    count: (1, 3),
                }),
            },
        )
        .await?;

        give_random_reward(&ctx.sy, command.user.id, &result, MoneyAddReasion::Helping).await?;
        Some(englishify_random_reward(result))
    } else {
        None
    };

    let _ = reply!(
        ctx,
        CreateMessage::new()
            .content(format!("<@{}>", command.user.id))
            .embed(
                create_embed()
                    .title(format!(
                        "{}, thanks for bumping our server! 💜",
                        command.user.name
                    ))
                    .description(
                        if let Some(ref reward) = reward {
                            format!("You have been awarded {reward}!\n\n")
                        } else {
                            String::new()
                        } + "I will remind you again in **2 hours**!"
                    )
            )
    );

    Ok(())
}
