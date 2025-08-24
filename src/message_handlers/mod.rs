mod analytics;
mod bump_detector;
mod react_bot;
mod template;
mod trancer_english_commands;
pub mod xp;

use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use tracing::instrument;

#[instrument]
pub async fn handle_message_handlers(ctx: &TrancerRunnerContext) -> Result<(), TrancerError> {
    analytics::handle(ctx).await?;
    bump_detector::handle(ctx).await?;
    xp::handle(ctx).await?;
    react_bot::handle(ctx).await?;
    trancer_english_commands::handle(ctx).await?;
    Ok(())
}
