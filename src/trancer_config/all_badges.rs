use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use crate::util::level_calc;
use std::future::Future;
use std::pin::Pin;
use std::sync::LazyLock;

pub struct DefinedBadgesOptions {
    pub give_roles: Vec<&'static str>,
}

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
    pub extra: Option<DefinedBadgesOptions>,
}

macro_rules! badge {
    ($id:expr, $name:expr, $description:expr, $emoji:expr, $extra:expr, |$ctx:ident| $body:expr) => {
        DefinedBadge {
            id: $id,
            name: $name,
            description: $description,
            emoji: $emoji,
            check: Box::new(|$ctx| Box::pin(async move { $body })),
            extra: $extra,
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
            None,
            |ctx| Ok(ctx.user_data.messages_sent > 1000)
        ),
        badge!(
            "yapper2",
            "Mega Yapper",
            "Sent 10,000 messages",
            ":loud_sound:",
            None,
            |ctx| Ok(ctx.user_data.messages_sent > 1000)
        ),
        badge!(
            "7talkstreak",
            "7 Day Talking Streak",
            "Talk in Trancy Twilight 7 days in a row",
            ":fire:",
            None,
            |ctx| Ok(ctx.user_data.talking_streak > 7)
        ),
        badge!(
            "14talkstreak",
            "14 Day Talking Streak",
            "Talk in Trancy Twilight 14 days in a row",
            ":fire:",
            None,
            |ctx| Ok(ctx.user_data.talking_streak > 14)
        ),
        badge!(
            "21talkstreak",
            "21 Day Talking Streak",
            "Talk in Trancy Twilight 21 days in a row",
            ":fire:",
            None,
            |ctx| Ok(ctx.user_data.talking_streak > 21)
        ),
        badge!(
            "booster",
            "Booster",
            "Boost Trancy Twilight at least once",
            ":pink_heart:",
            None,
            |ctx| Ok(false)
        ),
        badge!(
            "level15",
            "Level 15",
            "Get to level 15",
            ":chart_with_upwards_trend:",
            None,
            |ctx| Ok(level_calc::calculate_level(ctx.user_data.xp) >= 15)
        ),
        badge!(
            "level30",
            "Level 30",
            "Get to level 30",
            ":fire:",
            None,
            |ctx| Ok(level_calc::calculate_level(ctx.user_data.xp) >= 30)
        ),
        badge!(
            "botfuckerupper",
            "Bot Fucker Upper",
            "Broke the bot (like found a glitch)",
            ":sob:",
            None,
            |ctx| Ok(false)
        ),
        badge!(
            "500vcminutes",
            "500 VC Minutes",
            "Been in VC for 500 minutes (about 8 hours)",
            ":telephone_receiver:",
            None,
            |ctx| Ok(ctx.user_data.vc_time > 500)
        ),
        // TODO: This requires that ctx has the users eco
        badge!(
            "5kmoney",
            "Money Maker",
            "Reached 500 currency at some point",
            ":cyclone:",
            None,
            |ctx| Ok(false)
        ),
        badge!(
            "bumper",
            "Bumper",
            "Bumped Trancy Twilight 15 times",
            ":right_facing_fist:",
            None,
            |ctx| Ok(ctx.user_data.bumps >= 15)
        ),
    ]
});
