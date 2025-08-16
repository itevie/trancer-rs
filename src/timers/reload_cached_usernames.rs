use crate::cmd_util::TrancerError;
use crate::util::cached_usernames::load_from_sy_cache;
use serenity::all::Context;
use tracing::instrument;

#[instrument]
pub async fn run(ctx: Context) -> Result<(), TrancerError> {
    load_from_sy_cache(&ctx).await;
    Ok(())
}
