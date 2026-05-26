use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::models::economy::{EconomyFields, MoneyAddReason};
use crate::util::embeds::create_embed;
use crate::util::level_calc::calculate_level;
use crate::util::other::random_range;
use crate::util::random_rewards::{
    englishify_random_reward, generate_random_rewards, give_random_reward, RandomRewardPresets,
};
use crate::util::units;
use serenity::all::{CreateEmbedFooter, CreateMessage};
use std::time::Duration;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "fish".to_string(),
        t: TrancerCommandType::Economy,
        description: "Fish for some fishes in the open sea!".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["f".to_string()]),
            ratelimit: Some(units::mins(15)),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            let rewards = generate_random_rewards(&ctx.sy, RandomRewardPresets::fish()).await?;
            give_random_reward(&ctx.sy, ctx.msg.author.id, &rewards, MoneyAddReason::Commands).await?;
            ctx.economy.update_key(&ctx.sy, EconomyFields::work_xp, ctx.economy.work_xp + random_range(1..10)).await?;

            Ok(TrancerResponseType::Big(CreateMessage::new().embed(
                create_embed().title("You went fishing! 🎣")
                .description(format!("You caught {}", englishify_random_reward(rewards)))
                .footer(CreateEmbedFooter::new(
                    format!("You are level {} in Fishing", calculate_level(ctx.economy.fish_xp as u32))
                ))
            )))
        }),
    }
}
