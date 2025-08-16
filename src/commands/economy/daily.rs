use crate::cmd_util::trancer_handler;
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::models::economy::MoneyAddReasion;
use crate::util::config::CONFIG;
use crate::util::embeds::create_embed;
use crate::util::random_rewards::{
    englishify_random_reward, generate_random_rewards, give_random_reward, RandomRewardItemOptions,
    RandomRewardOptions,
};
use serenity::all::CreateMessage;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "daily".to_string(),
        t: TrancerCommandType::Help,
        description: "Get your daily reward of goodies!".to_string(),
        details: Default::default(),

        handler: trancer_handler!(|ctx, args| {
            let rewards = generate_random_rewards(&ctx.sy, RandomRewardOptions {
                currency: Some((CONFIG.payouts.daily.currency_min, CONFIG.payouts.daily.currency_max)),
                items: Some(RandomRewardItemOptions {
                    items: None,
                    count: (1, 7)
                })
            }).await?;
            give_random_reward(&ctx.sy, ctx.msg.author.id, &rewards, MoneyAddReasion::Commands).await?;

            Ok(TrancerResponseType::Big(CreateMessage::new().embed(
                create_embed().title("You opened your daily reward...")
                .description(format!("And you got {}", englishify_random_reward(rewards)))
            )))
        }),
    }
}
