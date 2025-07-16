use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use std::future::Future;
use std::pin::Pin;
use std::sync::LazyLock;

pub struct DefinedBadge {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub emoji: &'static str,
    pub check: Box<
        dyn Fn(
                TrancerRunnerContext,
            ) -> Pin<Box<dyn Future<Output = Result<bool, TrancerError>> + Send>>
            + Send
            + Sync,
    >,
}

macro_rules! badge {
    ($id:expr, $name:expr, $description:expr, $emoji:expr, |$ctx:ident| $body:expr) => {
        DefinedBadge {
            id: $id,
            name: $name,
            description: $description,
            emoji: $emoji,
            check: Box::new(|$ctx| Box::pin(async move { $body })),
        }
    };
}

pub static ALL_DEFINED_BADGES: LazyLock<Vec<DefinedBadge>> = LazyLock::new(|| {
    vec![
        badge!(
            "yapper",
            "Yapper",
            "Sent 1000 messages",
            ":speaking_head:",
            |ctx| Ok(ctx.user_data.messages_sent > 1000)
        ),
        badge!(
            "yapper2",
            "Mega Yapper",
            "Sent 10,000 messages",
            ":loud_sound:",
            |ctx| Ok(ctx.user_data.messages_sent > 1000)
        ),
        badge!(
            "7talkstreak",
            "7 Day Talking Streak",
            "Talk in Trancy Twilight 7 days in a row",
            ":fire:",
            |ctx| Ok(ctx.user_data.talking_streak > 7)
        ),
        badge!(
            "14talkstreak",
            "14 Day Talking Streak",
            "Talk in Trancy Twilight 14 days in a row",
            ":fire:",
            |ctx| Ok(ctx.user_data.talking_streak > 14)
        ),
        badge!(
            "21talkstreak",
            "21 Day Talking Streak",
            "Talk in Trancy Twilight 21 days in a row",
            ":fire:",
            |ctx| Ok(ctx.user_data.talking_streak > 21)
        ),
        badge!(
            "booster",
            "Booster",
            "Boost Trancy Twilight at least once",
            ":pink_heart:",
            |ctx| Ok(false)
        ),
    ]
});
