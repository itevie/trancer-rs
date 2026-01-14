use crate::util::random_rewards::{RandomRewardOptions, RandomRewardPresets};
use std::collections::HashMap;
use std::sync::LazyLock;

pub struct TrancerCrate {
    pub name: String,
    pub loot_pool: RandomRewardOptions,
}

macro_rules! trancer_crate {
    ($name:expr, $loot_pool:expr) => {
        (
            $name,
            TrancerCrate {
                name: $name.to_string(),
                loot_pool: $loot_pool,
            },
        )
    };
}

pub static TRANCER_CRATES: LazyLock<HashMap<&'static str, TrancerCrate>> =
    LazyLock::new(|| HashMap::from([trancer_crate!("common", RandomRewardPresets::daily())]));
