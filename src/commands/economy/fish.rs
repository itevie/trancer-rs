use crate::cmd_util::trancer_handler;
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::models::economy::MoneyAddReason;
use crate::util::embeds::create_embed;
use crate::util::random_rewards::{
    englishify_random_reward, generate_random_rewards, give_random_reward, RandomRewardPresets,
};
use serenity::all::CreateMessage;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "fish".to_string(),
        t: TrancerCommandType::Economy,
        description: "Fish for some fishes in the open sea!".to_string(),
        details: Default::default(),

        handler: trancer_handler!(|ctx, _args| {
            let rewards = generate_random_rewards(&ctx.sy, RandomRewardPresets::fish()).await?;
            give_random_reward(&ctx.sy, ctx.msg.author.id, &rewards, MoneyAddReason::Commands).await?;

            Ok(TrancerResponseType::Big(CreateMessage::new().embed(
                create_embed().title("You went fishing! 🎣")
                .description(format!("You caught {}", englishify_random_reward(rewards)))
            )))
        }),
    }
}
