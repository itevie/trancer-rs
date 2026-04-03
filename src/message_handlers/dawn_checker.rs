use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use crate::models::dawnagotchi::Dawnagotchi;
use crate::models::economy::Economy;
use crate::reply;
use crate::util::config::CONFIG;
use crate::util::lang::currency;
use serenity::builder::CreateMessage;
use tracing::{error, instrument};

#[instrument]
pub async fn handle(ctx: &TrancerRunnerContext) -> Result<(), TrancerError> {
    let Ok(dawn) = Dawnagotchi::fetch(&ctx.sy, ctx.msg.author.id).await else {
        return Ok(());
    };

    let requirements = dawn.get_requirements();

    let mut missing_requirements: Vec<String> = vec![];

    if requirements.drink == 0 {
        missing_requirements.push("Your Dawn got too thirsty...".to_string());
    }

    if requirements.feed == 0 {
        missing_requirements.push("Your Dawn got too hungry...".to_string());
    }

    if requirements.play == 0 {
        missing_requirements.push("Your Dawn got too lonely without attention...".to_string());
    }

    if !missing_requirements.is_empty() {
        dawn.remove(&ctx.sy).await?;

        let eco = Economy::fetch(&ctx.sy, ctx.msg.author.id).await?;
        eco.remove_money(&ctx.sy, CONFIG.payouts.dawn.not_caring_punishment, false)
            .await?;

        reply!(
            ctx,
            CreateMessage::new().content(format!(
                "Uh oh... Your Dawn has left you... \n\n{}\nYou have lost {} for not caring",
                missing_requirements.join("\n"),
                currency(CONFIG.payouts.dawn.not_caring_punishment)
            ))
        )?;
    }

    Ok(())
}
