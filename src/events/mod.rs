use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use crate::Handler;
use serenity::all::{Context, CreateMessage, EventHandler, Member, Message, Ready};
use serenity::async_trait;
use tracing::error;

mod guild_member_addition;
mod message_create;
mod ready;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, new_message: Message) {
        message_create::message(ctx, new_message).await;
    }

    async fn ready(&self, ctx: Context, data_about_bot: Ready) {
        ready::ready(ctx, data_about_bot).await;
    }

    async fn guild_member_addition(&self, ctx: Context, new_member: Member) {
        guild_member_addition::guild_member_addition(ctx, new_member).await;
    }
}

pub async fn something_happened(ctx: &TrancerRunnerContext, m: impl Into<String>, e: TrancerError) {
    let dev_error = format!(
        "{}: {}\n> Command: {} ({})",
        m.into(),
        e.to_string(),
        ctx.original_command,
        ctx.command_name
    );
    error!(dev_error);

    let m = format!(
        ":red_circle: Sorry! I couldn't run the command as something bad happened!\n:information_source: Please report this to the bot owner\n> {}",
        dev_error
    );

    let result = ctx.msg.reply(&ctx.sy, &m).await;
    if result.is_err() {
        let _ = ctx
            .msg
            .channel_id
            .send_message(
                &ctx.sy,
                CreateMessage::new().content(format!("**{}**: {}", ctx.msg.author.name, m)),
            )
            .await;
    }
}

#[macro_export]
macro_rules! something_happened {
    ($ctx:ident, $what:expr) => {
        match $what {
            Ok(ok) => ok,
            Err(e) => {
                something_happened(
                    &$ctx,
                    "Failed to run something via something_happened macro",
                    TrancerError::from(e),
                );
                return;
            }
        }
    };
}
