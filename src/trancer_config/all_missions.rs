use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use crate::models::mission::TrancerMission;
use crate::util::random_rewards::RandomRewardOptions;
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
    let base_currency = 30;

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
                currency: Some((base_currency * 2, base_currency * 2)),
                items: None,
            },
        ),
        (
            MissionDifficulty::Hard,
            RandomRewardOptions {
                currency: Some((base_currency * 3, base_currency * 4)),
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
                    let old = mission.json_details()?.user_data.xp;
                    let new = ctx.user_data.xp;

                    let gained = new.saturating_sub(old);

                    let percentage = ((gained as f64 / 50.0) * 100.0).clamp(0.0, 100.0);

                    Ok(percentage as u32)
                }),
            },
        ),
        // (
        //     "20 fish",
        //     Mission {
        //         description: "Get any 20 fish",
        //         difficulty: MissionDifficulty::Normal,
        //         reward: None,
        //         check: Arc::new(Box::new(|ctx, mission| Box::pin(async move { Ok(100) }))),
        //     },
        // ),
        // (
        //     "20 minerals",
        //     Mission {
        //         description: "Get any 20 minerals",
        //         difficulty: MissionDifficulty::Normal,
        //         reward: None,
        //         check: Arc::new(Box::new(|ctx, mission| Box::pin(async move { Ok(100) }))),
        //     },
        // ),
        // (
        //     "win tictactoe",
        //     Mission {
        //         description: "Win a game of tic-tac-toe",
        //         difficulty: MissionDifficulty::Hard,
        //         reward: None,
        //         check: Arc::new(Box::new(|ctx, mission| Box::pin(async move { Ok(100) }))),
        //     },
        // ),
        // (
        //     "get a card",
        //     Mission {
        //         description: "Obtain 1 card",
        //         difficulty: MissionDifficulty::Easy,
        //         reward: None,
        //         check: Arc::new(Box::new(|ctx, mission| Box::pin(async move { Ok(100) }))),
        //     },
        // ),
        // (
        //     "get 3 card",
        //     Mission {
        //         description: "Obtain 3 cards",
        //         difficulty: MissionDifficulty::Hard,
        //         reward: None,
        //         check: Arc::new(Box::new(|ctx, mission| Box::pin(async move { Ok(100) }))),
        //     },
        // ),
    ])
}
