use crate::cmd_util::TrancerError;
use crate::models::item::Item;
use rand::prelude::SliceRandom;
use rand::{random, Rng};
use serenity::client::Context;
use std::collections::HashMap;

pub struct RandomRewardItemOptions {
    /// Leave Some(None) for completely random items, otherwise Some(Some(Vec<id, weighht>))
    items: Option<Vec<(u32, f64)>>,

    /// (min, max)
    count: (u32, u32),
}

pub struct RandomRewardOptions {
    /// Currency to give (min, max)
    currency: Option<(u32, u32)>,

    items: Option<RandomRewardItemOptions>,
}

pub struct RandomRewardResult {
    currency: u32,
    /// Vec<(id, amount)>
    items: HashMap<u32, u32>,
}

pub async fn generate_random_rewards(
    ctx: &Context,
    options: RandomRewardOptions,
) -> Result<RandomRewardResult, TrancerError> {
    let all_items = Item::get_all();
    let mut result = RandomRewardResult {
        currency: 0,
        items: HashMap::new(),
    };

    if let Some(currency) = options.currency {
        let mut rng = rand::thread_rng();
        result.currency = rng.gen_range(currency.0..currency.1);
    }

    if let Some(items) = options.items {
        let mut actual_items: Vec<(Item, f64)> = match items.items {
            Some(items) => items
                .into_iter()
                .map(|item| {
                    (
                        (all_items.iter().find(|x| x.id == item.0)).unwrap().clone(),
                        item.1,
                    )
                })
                .collect(),
            None => all_items
                .into_iter()
                .map(|x| (x.clone(), x.weight))
                .collect(),
        };
        let mut rng = rand::thread_rng();
        actual_items.shuffle(&mut rng);

        let total_weight = actual_items.iter().map(|(_, weight)| *weight).sum::<f64>();
        let amount = biased_random(items.count.0, items.count.1);
        let mut selected_items: HashMap<u32, u32> = HashMap::new();

        for _ in 0..amount {
            let random_value = rng.gen_range(0.0..total_weight);

            let mut cumulative_weight = 0.0;
            for (item, weight) in &actual_items {
                cumulative_weight += weight;
                if random_value <= cumulative_weight {
                    *selected_items.entry(item.id).or_insert(0) += 1;
                    break;
                }
            }
        }

        result.items = selected_items;
    }

    Ok(result)
}

fn biased_random(min: u32, max: u32) -> u32 {
    let uniform: f64 = random();
    let biased = uniform * uniform;
    let range = (max - min + 1) as f64;
    min + (biased * range).floor() as u32
}
