mod analytics;
mod bump_detector;

use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use tracing::instrument;

#[instrument]
pub async fn handle_message_handlers(ctx: &TrancerRunnerContext) -> Result<(), TrancerError> {
    analytics::handle_analytics(ctx).await?;
    bump_detector::detect_bumps(ctx).await?;
    Ok(())
}
