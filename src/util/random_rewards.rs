use crate::cmd_util::TrancerError;
use crate::models::item::Item;
use rand::prelude::SliceRandom;
use rand::{random, Rng};
use serenity::client::Context;

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
    items: Vec<(u32, u32)>,
}

pub async fn generate_random_rewards(
    ctx: &Context,
    options: RandomRewardOptions,
) -> Result<(), TrancerError> {
    let all_items = Item::get_all(ctx).await?;
    let mut result = RandomRewardResult {
        currency: 0,
        items: vec![],
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
        let given_items: Vec<(u32, u32)> = vec![];
    }

    Ok(())
}

fn biased_random(min: u32, max: u32) -> u32 {
    let uniform: f64 = random();
    let biased = uniform * uniform;
    let range = (max - min + 1) as f64;
    min + (biased * range).floor() as u32
}
