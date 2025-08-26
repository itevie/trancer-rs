use crate::cmd_util::{TrancerError, TrancerResponseType, TrancerRunnerContext};
use crate::commands::reply_response_type;
use crate::util::define::handle_define_message;
use crate::util::other::random_bool;
use tracing::instrument;

#[instrument]
pub async fn handle(ctx: &TrancerRunnerContext) -> Result<(), TrancerError> {
    let own = ctx.sy.http.get_current_user().await?;
    let ping = format!("<@{}>", own.id);
    let alt_ping = String::from("@grok");

    let matches = |test: &String, command: &str| -> Option<String> {
        let pat = if test
            .to_lowercase()
            .starts_with(&format!("{ping} {command}"))
        {
            &format!("{ping} {command}")
        } else if test
            .to_lowercase()
            .starts_with(&format!("{alt_ping} {command}"))
        {
            &format!("{alt_ping} {command}")
        } else {
            return None;
        };

        Some(test.replace(pat, ""))
    };

    if let Some(what) = matches(&ctx.msg.content, "what is") {
        reply_response_type(ctx, handle_define_message(ctx, what).await?).await
    }

    if matches(&ctx.msg.content, "is this true").is_some() {
        reply_response_type(
            ctx,
            TrancerResponseType::Content(if random_bool() { "Yes" } else { "No" }.to_string()),
        )
        .await
    }

    Ok(())
}
