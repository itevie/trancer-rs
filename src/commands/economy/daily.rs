use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::models::economy::MoneyAddReason;
use crate::util::embeds::create_embed;
use crate::util::random_rewards::{
    englishify_random_reward, generate_random_rewards, give_random_reward, RandomRewardPresets,
};
use crate::util::units;
use serenity::all::CreateMessage;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "daily".to_string(),
        t: TrancerCommandType::Economy,
        description: "Get your daily reward of goodies!".to_string(),
        details: TrancerDetails {
            ratelimit: Some(units::hours(24)),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            let rewards = generate_random_rewards(&ctx.sy, RandomRewardPresets::daily()).await?;
            give_random_reward(&ctx.sy, ctx.msg.author.id, &rewards, MoneyAddReason::Commands).await?;

            Ok(TrancerResponseType::Big(CreateMessage::new().embed(
                create_embed().title("You opened your daily reward...")
                .description(format!("And you got {}", englishify_random_reward(rewards)))
            )))
        }),
    }
}
