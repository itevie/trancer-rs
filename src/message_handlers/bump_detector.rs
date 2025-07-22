use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use crate::models::server_settings::ServerSettingsFields;
use crate::models::user_data::UserDataFields;
use crate::util::config::CONFIG;
use crate::util::random_rewards::{
    generate_random_rewards, RandomRewardItemOptions, RandomRewardOptions,
};
use chrono::Utc;
use serenity::all::{MessageCommandInteractionMetadata, MessageInteractionMetadata};

static DISBOARD_ID: &'static str = "302050872383242240";

pub async fn detect_bumps(ctx: &TrancerRunnerContext) -> Result<(), TrancerError> {
    if let Some(embed) = ctx.msg.embeds.get(0) {
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
        Some(
            generate_random_rewards(
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
            .await?,
        )
    } else {
        None
    };

    // TODO: Finish this

    Ok(())
}
