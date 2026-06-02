use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use crate::util::config::CONFIG;
use crate::util::level_calc;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, LazyLock};

#[derive(Clone)]
pub struct DefinedBadgesOptions {
    pub give_roles: Vec<&'static str>,
}

#[derive(Clone)]
pub struct DefinedBadge {
    pub id: &'static str,
    pub name: &'static str,
    pub description: &'static str,
    pub emoji: &'static str,
    pub check: Arc<
        Box<
            dyn Fn(
                    TrancerRunnerContext,
                )
                    -> Pin<Box<dyn Future<Output = Result<bool, TrancerError>> + Send>>
                + Send
                + Sync,
        >,
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
            check: Arc::new(Box::new(|$ctx| Box::pin(async move { $body }))),
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
            |ctx| Ok(ctx.user_data.messages_sent > 10000)
        ),
        badge!(
            "7talkstreak",
            "7 Day Talking Streak",
            "Talk in Trancy Twilight 7 days in a row",
            ":fire:",
            None,
            |ctx| Ok(ctx.user_data.talking_streak >= 7)
        ),
        badge!(
            "14talkstreak",
            "14 Day Talking Streak",
            "Talk in Trancy Twilight 14 days in a row",
            ":fire:",
            None,
            |ctx| Ok(ctx.user_data.talking_streak >= 14)
        ),
        badge!(
            "21talkstreak",
            "21 Day Talking Streak",
            "Talk in Trancy Twilight 21 days in a row",
            ":fire:",
            None,
            |ctx| Ok(ctx.user_data.talking_streak >= 21)
        ),
        badge!(
            "178talkstreak",
            "Half A Year Talking Streak",
            "Talk in Trancy Twilight 178 days in a row",
            ":fire:",
            None,
            |ctx| Ok(ctx.user_data.talking_streak >= 178)
        ),
        badge!(
            "1yeartalkstreak",
            "1 Year Talking Streak",
            "Talk in Trancy Twilight for 356 days in a row",
            ":fire:",
            None,
            |ctx| Ok(ctx.user_data.talking_streak >= 356)
        ),
        badge!(
            "booster",
            "Booster",
            "Boost Trancy Twilight at least once",
            ":pink_heart:",
            None,
            |_ctx| Ok(false)
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
            |_ctx| Ok(false)
        ),
        badge!(
            "500vcminutes",
            "500 VC Minutes",
            "Been in VC for 500 minutes (about 8 hours)",
            ":telephone_receiver:",
            None,
            |ctx| Ok(ctx.user_data.vc_time > 500)
        ),
        badge!(
            "5kmoney",
            "Money Maker",
            "Reached 500 currency at some point",
            ":cyclone:",
            None,
            |ctx| Ok(ctx.economy.balance >= 500)
        ),
        badge!(
            "bumper",
            "Bumper",
            "Bumped Trancy Twilight 15 times",
            ":right_facing_fist:",
            None,
            |ctx| Ok(ctx.user_data.bumps >= 15)
        ),
        badge!(
            "og",
            "Founder",
            "Joined the server before 100 members",
            ":snowflake:",
            None,
            |_ctx| Ok(false)
        ),
        badge!(
            "birthday",
            "Twilight's Birthday",
            "Be in Trancy Twilight when it hit 1 year old",
            ":birthday:",
            Some(DefinedBadgesOptions {
                give_roles: vec![&*CONFIG.roles.birthday]
            }),
            |_ctx| Ok(false)
        ),
        // TODO: Fix this once cards are added
        badge!(
            "mythiccard",
            "Mythic Card",
            "Got a mythical card at some point",
            ":flower_playing_cards:",
            None,
            |_ctx| Ok(false)
        ),
        // TODO: Fix this once (if) relationships are added
        badge!(
            "cult",
            "Cult Leader",
            "Get 5 people to worship you with the tree feature",
            ":pray:",
            None,
            |_ctx| Ok(false)
        ),
        // The following are handled magically
        badge!(
            "eco#1",
            "Economy #1",
            "At economy position #1",
            ":first_place:",
            None,
            |_ctx| Ok(false)
        ),
        badge!(
            "eco#2",
            "Economy #2",
            "At economy position #2",
            ":second_place:",
            None,
            |_ctx| Ok(false)
        ),
        badge!(
            "eco#3",
            "Economy #3",
            "At economy position #3",
            ":third_place:",
            None,
            |_ctx| Ok(false)
        ),
        badge!(
            "can-request",
            "Can Request",
            "Reached level 5 on Trancer",
            ":fish:",
            Some(DefinedBadgesOptions {
                give_roles: vec![&*CONFIG.roles.can_request]
            }),
            |ctx| Ok(level_calc::calculate_level(ctx.user_data.xp) >= 5)
        ),
        // TODO: Fix once Dawnagotchi is added
        badge!(
            "babysitter",
            "Dawn Babysitter",
            "Have a Dawnagotchi for more than a month",
            "<:uppies:1278754282413490259>",
            None,
            |_ctx| Ok(false)
        ),
        badge!(
            "french",
            "French",
            "⚠️ This user is French - be weary ⚠️",
            "🇨🇵",
            None,
            |_ctx| Ok(false)
        ),
        badge!(
            "pride2026",
            "Pride 2026",
            "Pride Month 2026",
            "🏳️‍🌈",
            None,
            |_ctx| Ok(true)
        ),
        badge!(
            "newtrancer",
            "New Trancer",
            "Used the new trancer in the first month",
            "👀",
            None,
            |_ctx| Ok(true)
        ),
    ]
});
