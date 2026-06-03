use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use crate::models::aquired_item::AquiredItem;
use crate::models::item::Item;
use crate::models::mission::TrancerMission;
use crate::models::user_data::UserData;
use crate::util::config::CONFIG;
use crate::util::random_rewards::RandomRewardOptions;
use serenity::all::GuildId;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum MissionDifficulty {
    Easy,
    Normal,
    Hard,
}

#[derive(Clone)]
pub struct Mission {
    pub description: &'static str,
    pub difficulty: MissionDifficulty,
    pub reward: Option<RandomRewardOptions>,
    pub check: Arc<
        Box<
            dyn Fn(
                    TrancerRunnerContext,
                    TrancerMission,
                )
                    -> Pin<Box<dyn Future<Output = Result<u32, TrancerError>> + Send>>
                + Send
                + Sync,
        >,
    >,
}

pub fn base_random_rewards() -> HashMap<MissionDifficulty, RandomRewardOptions> {
    let base_currency = 50;

    HashMap::from([
        (
            MissionDifficulty::Easy,
            RandomRewardOptions {
                currency: Some((base_currency, base_currency)),
                items: None,
            },
        ),
        (
            MissionDifficulty::Normal,
            RandomRewardOptions {
                currency: Some((base_currency * 2, base_currency * 3)),
                items: None,
            },
        ),
        (
            MissionDifficulty::Hard,
            RandomRewardOptions {
                currency: Some((base_currency * 4, base_currency * 6)),
                items: None,
            },
        ),
    ])
}

fn always_true(_m: &Mission) -> Arc<Pin<Box<impl Future<Output = i32>>>> {
    Arc::new(Box::pin(async move { 100 }))
}

macro_rules! mission_check {
    ($ctx:ident, $m:ident, $block:expr) => {
        Arc::new(Box::new(|$ctx, $m| Box::pin(async move { $block })))
    };
}

pub type MissionName = &'static str;

pub fn get_defined_missions() -> HashMap<MissionName, Mission> {
    HashMap::from([
        (
            "50 xp",
            Mission {
                description: "Get 50 XP",
                difficulty: MissionDifficulty::Easy,
                reward: None,
                check: mission_check!(ctx, mission, {
                    Ok(calc_percentage(
                        mission.json_details()?.user_data.xp as i32,
                        ctx.user_data.xp as i32,
                        50,
                    ))
                }),
            },
        ),
        (
            "100 xp",
            Mission {
                description: "Get 100 XP",
                difficulty: MissionDifficulty::Normal,
                reward: None,
                check: mission_check!(ctx, mission, {
                    Ok(calc_percentage(
                        mission.json_details()?.user_data.xp as i32,
                        ctx.user_data.xp as i32,
                        100,
                    ))
                }),
            },
        ),
        (
            "500 money",
            Mission {
                description: "Get 500 Spirals",
                difficulty: MissionDifficulty::Hard,
                reward: None,
                check: mission_check!(ctx, mission, {
                    Ok(calc_percentage(
                        mission.json_details()?.eco.balance,
                        ctx.economy.balance,
                        500,
                    ))
                }),
            },
        ),
        (
            "25 messages",
            Mission {
                description: "Send 25 messages",
                difficulty: MissionDifficulty::Easy,
                reward: None,
                check: mission_check!(ctx, mission, {
                    Ok(calc_percentage(
                        mission.json_details()?.user_data.messages_sent as i32,
                        ctx.user_data.messages_sent as i32,
                        25,
                    ))
                }),
            },
        ),
        (
            "50 messages",
            Mission {
                description: "Send 25 messages",
                difficulty: MissionDifficulty::Normal,
                reward: None,
                check: mission_check!(ctx, mission, {
                    Ok(calc_percentage(
                        mission.json_details()?.user_data.messages_sent as i32,
                        ctx.user_data.messages_sent as i32,
                        50,
                    ))
                }),
            },
        ),
        (
            "100 messages",
            Mission {
                description: "Send 100 messages",
                difficulty: MissionDifficulty::Hard,
                reward: None,
                check: mission_check!(ctx, mission, {
                    Ok(calc_percentage(
                        mission.json_details()?.user_data.messages_sent as i32,
                        ctx.user_data.messages_sent as i32,
                        100,
                    ))
                }),
            },
        ),
        (
            "1000 money",
            Mission {
                description: "Get 1000 Spirals",
                difficulty: MissionDifficulty::Hard,
                reward: None,
                check: mission_check!(ctx, mission, {
                    Ok(calc_percentage(
                        mission.json_details()?.eco.balance,
                        ctx.economy.balance,
                        1000,
                    ))
                }),
            },
        ),
        (
            "20 fish",
            Mission {
                description: "Get 20 fish",
                difficulty: MissionDifficulty::Normal,
                reward: None,
                check: mission_check!(ctx, mission, {
                    check_item_tagged(
                        mission.json_details()?.items,
                        AquiredItem::fetch_all_for(&ctx.sy, ctx.user_id).await?,
                        "fish".to_string(),
                        20,
                    )
                }),
            },
        ),
        (
            "50 fish",
            Mission {
                description: "Get 50 fish",
                difficulty: MissionDifficulty::Hard,
                reward: None,
                check: mission_check!(ctx, mission, {
                    check_item_tagged(
                        mission.json_details()?.items,
                        AquiredItem::fetch_all_for(&ctx.sy, ctx.user_id).await?,
                        "fish".to_string(),
                        50,
                    )
                }),
            },
        ),
        (
            "20 minerals",
            Mission {
                description: "Get 20 minerals",
                difficulty: MissionDifficulty::Normal,
                reward: None,
                check: mission_check!(ctx, mission, {
                    check_item_tagged(
                        mission.json_details()?.items,
                        AquiredItem::fetch_all_for(&ctx.sy, ctx.user_id).await?,
                        "mineral".to_string(),
                        20,
                    )
                }),
            },
        ),
        (
            "50 mineral",
            Mission {
                description: "Get 50 minerals",
                difficulty: MissionDifficulty::Hard,
                reward: None,
                check: mission_check!(ctx, mission, {
                    check_item_tagged(
                        mission.json_details()?.items,
                        AquiredItem::fetch_all_for(&ctx.sy, ctx.user_id).await?,
                        "mineral".to_string(),
                        50,
                    )
                }),
            },
        ),
    ])
}

fn calc_percentage(old: i32, new: i32, needed: i32) -> u32 {
    if needed <= 0 {
        return 100;
    }

    let gained = new.saturating_sub(old);
    let percentage = ((gained as f64 / needed as f64) * 100.0).clamp(0.0, 100.0);
    percentage as u32
}

fn check_item_tagged(
    old: Vec<AquiredItem>,
    new: Vec<AquiredItem>,
    tag: String,
    amount: u32,
) -> Result<u32, TrancerError> {
    let old_amount: u32 = old
        .iter()
        .filter(|x| Item::get_by_id(x.item_id.clone()).tag.as_ref() == Some(&tag))
        .map(|x| x.amount)
        .sum();

    let new_amount: u32 = new
        .iter()
        .filter(|x| Item::get_by_id(x.item_id.clone()).tag.as_ref() == Some(&tag))
        .map(|x| x.amount)
        .sum();

    Ok(calc_percentage(
        old_amount as i32,
        new_amount as i32,
        amount as i32,
    ))
}
