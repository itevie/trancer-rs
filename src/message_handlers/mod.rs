mod analytics;
mod bump_detector;

use crate::cmd_util::{TrancerError, TrancerRunnerContext};

pub async fn handle_message_handlers(ctx: &TrancerRunnerContext) -> Result<(), TrancerError> {
    analytics::handle_analytics(ctx).await?;
    Ok(())
}
