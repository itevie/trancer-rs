use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use crate::util::cached_usernames::get_cached_username;
use crate::util::pagination::{paginate, PaginationDataType, PaginationOptions};
use serenity::all::CreateEmbed;
use std::collections::HashMap;

pub fn lb_accumulate(data: Vec<String>) -> Vec<(i32, String)> {
    let mut hashmap = HashMap::new();

    for i in data {
        if let std::collections::hash_map::Entry::Vacant(e) = hashmap.entry(i.clone()) {
            e.insert(1);
            continue;
        } else {
            let mut binding = hashmap.get_mut(&i).unwrap();
            *binding += 1;
        }
    }

    hashmap.iter().map(|x| (*x.1, x.0.clone())).collect()
}

pub async fn leaderboard(
    ctx: TrancerRunnerContext,
    embed: CreateEmbed,
    data: Vec<(i32, String)>,
) -> Result<(), TrancerError> {
    let mut sorted = data
        .iter()
        .filter(|x| x.0 > 0)
        .collect::<Vec<&(i32, String)>>();
    sorted.sort_by(|a, b| b.0.cmp(&a.0));

    paginate(PaginationOptions {
        ctx,
        embed,
        page_size: 10,
        data: PaginationDataType::Description {
            base_description: None,
            data: sorted
                .iter()
                .enumerate()
                .map(|(i, x)| format!("{}. {}: {}", i + 1, get_cached_username(x.1.clone()), x.0))
                .collect::<Vec<String>>(),
        },
    })
    .await
}
