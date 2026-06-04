use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::{content_response, CommandTrait};
use crate::cmd_util::{trancer_handler, TrancerDetails, TrancerError};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::commands::CommandHasNoArgs;
use crate::models::aquired_item::AquiredItem;
use crate::models::economy::{EconomyFields, MoneyAddReason};
use crate::models::item::Item;
use crate::trancer_config::all_jobs::ALL_JOBS;
use crate::util::embeds::create_embed;
use crate::util::lang::{currency, englishify_list, item_text};
use crate::util::level_calc::calculate_level;
use crate::util::other::random_range;
use crate::util::random_rewards::{
    generate_random_rewards, give_random_reward, RandomRewardItemOptions, RandomRewardOptions,
};
use crate::util::units;
use crate::{command_file, reply};
use rand::prelude::{SliceRandom, StdRng};
use rand::{Rng, SeedableRng};
use serenity::all::{ButtonStyle, CreateActionRow, CreateButton, CreateMessage, EditMessage};
use serenity::builder::CreateEmbedFooter;
use serenity::futures::StreamExt;
use std::collections::HashMap;
use std::time::Duration;
use tracing::error;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "mine".to_string(),
        t: TrancerCommandType::Economy,
        description: "Yearn for the mines! Requires a pickaxe".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["m".to_string()]),
            ratelimit: Some(units::mins(15)),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            let items = AquiredItem::fetch_all_for(&ctx.sy, ctx.user_id)
                .await?;
            let pickaxe = items
                .iter().filter(|x| Item::get_by_id(x.item_id).tag.as_ref() == Some(&"pickaxe".to_string()))
                .collect::<Vec<&AquiredItem>>();

            if pickaxe.len() == 0 {
                return Err(TrancerError::NonScary(
                    "You need a pickaxe to go mining! Try craft one! :spiral:".to_string()
                ));
            }

            let emerald_pickaxe = Item::get_by_name("emerald-pickaxe");
            let luck_multiplier = if pickaxe.iter().find(|x| x.item_id == emerald_pickaxe.id).is_some() {
                0.1
            } else {
                0.0
            };

            let mineral_items = Item::get_by_tag("mineral".to_string());
            let minerals = mineral_items
                .iter().filter(|x| x.name != "rock".to_string())
                .collect::<Vec<&Item>>();

            let rock = Item::get_by_name("rock");
            let map: Vec<Vec<&Item>> = (0..5)
                .map(|_| {
                    (0..5)
                        .map(|_| generate_mineral(&minerals, &rock, luck_multiplier))
                        .collect()
                })
                .collect();

            let mut row_marker = vec![":one:", ":two:", ":three:", ":four:", ":five:"];

            let unknown_space = ":question:";
            let embed = create_embed()
                .title("Select where to mine!")
                .description(
                    format!("{}\n", unknown_space.repeat(5)).repeat(5)
                        + &row_marker.join("")
                )
                .footer(
                    CreateEmbedFooter::new(
                    format!("Luck multiplier: {}%", luck_multiplier * 100.0)
                    )
                );

            let action_row = CreateActionRow::Buttons(
                vec![1, 2, 3, 4, 5].iter().map(|x|
                    CreateButton::new(x.to_string())
                        .label(x.to_string())
                        .style(ButtonStyle::Primary)
                ).collect()
            );

            let mut msg = reply!(ctx, CreateMessage::new().embed(embed.clone()).components(vec![action_row.clone()]))?;

            let mut collector = msg
                .await_component_interactions(&ctx.sy)
                .timeout(Duration::from_secs(5 * 60))
                .stream();

            let Some(result) = collector.next().await else {
                 return Err(TrancerError::Generic(
                    "The result of the collector was null".to_string()
                ));
            };
            result.defer(&ctx.sy).await?;

            let selected_idx = result.data.custom_id.parse::<usize>().unwrap() - 1;
            row_marker[selected_idx] = ":arrow_double_up:";
            let selected_minerals: Vec<&Item> = map
                .iter()
                .map(|row| row[selected_idx])
                .collect();

            let mut counts: HashMap<u32, u32> = HashMap::new();

            for item in selected_minerals {
                *counts.entry(item.id).or_insert(0) += 1;
                AquiredItem::give_item_to(&ctx.sy, ctx.user_id, item.id, 1).await?;
            }

            ctx.economy.update_key(&ctx.sy, EconomyFields::mine_xp, ctx.economy.mine_xp + 5).await?;

            let item_texts = counts
                .iter()
                .map(|x| item_text(Item::get_by_id(*x.0), *x.1))
                .collect::<Vec<String>>();

            let total_value: u64 = counts
                .iter()
                .map(|(id, amount)| {
                    let item = Item::get_by_id(*id);
                    item.price as u64 * (*amount as u64)
                })
                .sum();
            let items_text = englishify_list(item_texts, false);
            let map_text = map
                .iter()
                .map(|row| {
                    row.iter()
                        .map(|item| item.emoji.clone().unwrap_or(":question:".to_string()))
                        .collect::<String>()
                })
                .collect::<Vec<_>>()
                .join("\n");
            let marker_text = row_marker.join("");

            let description = format!(
                "{} worth {}\n{}\n{}",
                items_text,
                currency(total_value as i64),
                map_text,
                marker_text
            );

            let new_embed = create_embed()
                .title("You yearned for the mines")
                .description(description)
                .footer(
                    CreateEmbedFooter::new(format!(
                        "Luck multiplier: {}%",
                        luck_multiplier * 100.0
                    ))
                );

            msg.edit(&ctx.sy, EditMessage::new().embed(new_embed).components(vec![])).await?;

            Ok(TrancerResponseType::None)
        }),
    }
}

fn generate_mineral<'a>(minerals: &'a [&Item], rock: &'a Item, luck_multiplier: f64) -> &'a Item {
    let mut rng = StdRng::from_entropy();

    minerals
        .iter()
        .find(|mineral| rng.gen::<f64>() - luck_multiplier < mineral.weight)
        .copied()
        .unwrap_or(rock)
}
