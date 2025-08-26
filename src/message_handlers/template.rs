use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use tracing::instrument;

#[instrument]
pub async fn handle(ctx: &TrancerRunnerContext) -> Result<(), TrancerError> {
    Ok(())
}
