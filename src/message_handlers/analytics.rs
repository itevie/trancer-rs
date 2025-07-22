use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use crate::models::user_data::UserDataFields;

pub async fn handle_analytics(ctx: &TrancerRunnerContext) -> Result<(), TrancerError> {
    ctx.user_data
        .increment(&ctx.sy, UserDataFields::messages_sent, 1)
        .await?;
    Ok(())
}
