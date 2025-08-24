use crate::cmd_util::TrancerError;
use crate::models::aquired_item::AquiredItem;
use crate::models::economy::{Economy, MoneyAddReason};
use crate::models::item::{Item, ALL_ITEMS};
use crate::util::lang::{currency, englishify_list, item_text};
use rand::prelude::SliceRandom;
use rand::{random, Rng};
use serenity::all::UserId;
use serenity::client::Context;
use std::collections::HashMap;

pub struct RandomRewardItemOptions {
    /// Leave Some(None) for completely random items, otherwise Some(Some(Vec<id, weighht>))
    pub(crate) items: Option<Vec<(u32, f64)>>,

    /// (min, max)
    pub(crate) count: (u32, u32),
}

pub struct RandomRewardOptions {
    /// Currency to give (min, max)
    pub(crate) currency: Option<(u32, u32)>,

    pub(crate) items: Option<RandomRewardItemOptions>,
}

#[derive(Debug)]
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

pub fn englishify_random_reward(reward: RandomRewardResult) -> String {
    let mut winnings: Vec<String> = vec![];

    if reward.currency != 0 {
        winnings.push(currency(reward.currency));
    }

    for (_item, amount) in reward.items {
        let item = ALL_ITEMS
            .get()
            .unwrap()
            .iter()
            .find(|x| x.id == _item)
            .unwrap();
        winnings.push(item_text(item.clone(), amount))
    }

    englishify_list(winnings, false)
}

pub async fn give_random_reward(
    ctx: &Context,
    user: UserId,
    reward: &RandomRewardResult,
    money_reason: MoneyAddReason,
) -> Result<(), TrancerError> {
    let eco = Economy::fetch(ctx, user).await?;
    eco.add_money(ctx, reward.currency, Some(money_reason))
        .await?;

    for (item_id, amount) in &reward.items {
        AquiredItem::give_item_to(ctx, user, *item_id, *amount).await?
    }

    Ok(())
}

fn biased_random(min: u32, max: u32) -> u32 {
    let uniform: f64 = random();
    let biased = uniform * uniform;
    let range = (max - min + 1) as f64;
    min + (biased * range).floor() as u32
}
