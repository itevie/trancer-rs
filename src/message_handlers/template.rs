use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use crate::models::user_data::UserDataFields;
use tracing::instrument;

#[instrument]
pub async fn handle(ctx: &TrancerRunnerContext) -> Result<(), TrancerError> {
    Ok(())
}
