mod absolute_cinema;
mod analytics;
mod bump_detector;
mod counting;
mod dawn_checker;
mod react_bot;
mod streaks;
mod swear_jar;
mod template;
mod trancer_english_commands;
pub mod xp;

use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use tracing::instrument;

#[instrument]
pub async fn handle_message_handlers(ctx: &TrancerRunnerContext) -> Result<(), TrancerError> {
    counting::handle(ctx).await?;
    streaks::handle(ctx).await?;
    analytics::handle(ctx).await?;
    bump_detector::handle(ctx).await?;
    xp::handle(ctx).await?;
    react_bot::handle(ctx).await?;
    trancer_english_commands::handle(ctx).await?;
    swear_jar::handle(ctx).await?;
    dawn_checker::handle(ctx).await?;
    absolute_cinema::handle(ctx).await?;
    Ok(())
}
