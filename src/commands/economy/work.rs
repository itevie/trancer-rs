use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails, TrancerError};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::models::economy::{EconomyFields, MoneyAddReason};
use crate::models::item::Item;
use crate::trancer_config::all_jobs::ALL_JOBS;
use crate::util::embeds::create_embed;
use crate::util::lang::{currency, item_text};
use crate::util::level_calc::calculate_level;
use crate::util::other::random_range;
use crate::util::random_rewards::{
    generate_random_rewards, give_random_reward, RandomRewardItemOptions, RandomRewardOptions,
};
use rand::prelude::{SliceRandom, StdRng};
use rand::SeedableRng;
use serenity::all::CreateMessage;
use serenity::builder::CreateEmbedFooter;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "work".to_string(),
        t: TrancerCommandType::Economy,
        description: "Work for those special spirals!".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["w".to_string()]),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            let Some(user_job) = ctx.economy.job.clone() else {
                return Err(TrancerError::NonScary(
                    format!("You do not have a job! View jobs with `{}jobs`", ctx.server_settings.prefix)
                ))
            };

            let job = ALL_JOBS.get(&user_job.as_str()).clone().unwrap();
            let rewards = generate_random_rewards(&ctx.sy, RandomRewardOptions {
                currency: job.rewards.currency,
                items: if let Some(ref items) = job.rewards.items {
                    Some(RandomRewardItemOptions {
                        items: Some(items.pool.iter().map(|x| (Item::get_by_name(x.0).id, x.1)).collect()),
                        count: items.count
                    })
                } else {
                    None
                },
            }).await?;

            give_random_reward(&ctx.sy, ctx.msg.author.id, &rewards, MoneyAddReason::Commands).await?;
            ctx.economy.update_key(&ctx.sy, EconomyFields::work_xp, ctx.economy.work_xp + random_range(1..10)).await?;

            let mut rng = StdRng::from_entropy();
            let value = job.phrases.choose(&mut rng).unwrap()
                .to_string()
                .replace("$r", &format!("**{}**", currency(rewards.currency)))
                .replace(
                    "$c",
                    &rewards.items
                        .iter()
                        .map(|x| item_text(Item::get_by_id(*x.0), *x.1))
                        .collect::<Vec<_>>()
                        .join(", ")
                );

            Ok(TrancerResponseType::Big(CreateMessage::new().embed(
                create_embed().title(format!("You worked as a {}!", user_job))
                .description(value)
                .footer(CreateEmbedFooter::new(
                    format!("You are level {} in Work", calculate_level(ctx.economy.work_xp as u32))
                ))
            )))
        }),
    }
}
