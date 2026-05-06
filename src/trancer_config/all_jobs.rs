use once_cell::sync::Lazy;
use std::collections::HashMap;

#[derive(Debug)]
pub struct WorkJob {
    pub level_required: u32,
    pub description: &'static str,
    pub phrases: &'static [&'static str],
    pub rewards: RandomRewardOptions,
}

#[derive(Debug)]
pub struct RandomRewardOptions {
    pub currency: Option<(u32, u32)>,
    pub items: Option<ItemReward>,
}

#[derive(Debug)]
pub struct ItemReward {
    pub pool: &'static [(&'static str, f64)],
    pub count: (u32, u32),
}

pub static ALL_JOBS: Lazy<HashMap<&'static str, WorkJob>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert(
        "Trash Picker",
        WorkJob {
            level_required: 0,
            description: "A normal person looking the streets for trash",
            phrases: &[
                "You picked up trash and got $r!",
                "You scoured the streets for trash and got $r!",
                "You contemplated your life as a trash picker, anyway, here's your $r.",
            ],
            rewards: RandomRewardOptions {
                currency: Some((1, 20)),
                items: Some(ItemReward {
                    pool: &[
                        ("dirt", 0.7),
                        ("common-fish", 0.6),
                        ("stick", 0.5),
                        ("rock", 0.2),
                        ("lottery-ticket", 0.01),
                    ],
                    count: (0, 2),
                }),
            },
        },
    );

    m.insert(
        "Factory Worker",
        WorkJob {
            level_required: 5,
            description: "A factory worker fixing nerd stuff",
            phrases: &[
                "You fixed some nerd shit in the factory and got $r!",
                "You created a new kind of gear and the factory manager gave you $r!",
                "You found a revolutionary way to save $c, the manager gave you $r!",
            ],
            rewards: RandomRewardOptions {
                currency: Some((5, 30)),
                items: Some(ItemReward {
                    pool: &[
                        ("rock", 0.9),
                        ("dirt", 0.3),
                        ("stone-pickaxe", 0.2),
                        ("emerald-pickaxe", 0.001),
                    ],
                    count: (0, 3),
                }),
            },
        },
    );

    m.insert(
        "Worshipper",
        WorkJob {
            level_required: 25,
            description: "A worshipper for the Trancer gods",
            phrases: &["You prayed to the Trancer gods and they blessed you with $r"],
            rewards: RandomRewardOptions {
                currency: Some((10, 80)),
                items: Some(ItemReward {
                    pool: &[("gold", 0.5), ("angle-fish", 0.3), ("diamond", 0.01)],
                    count: (1, 3),
                }),
            },
        },
    );

    m
});
