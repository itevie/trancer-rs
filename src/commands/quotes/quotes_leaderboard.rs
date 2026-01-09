use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::models::quote::Quote;
use crate::util::embeds::create_embed;
use crate::util::leaderboard::{
    lb_accumulate, leaderboard, LeaderboardFormatter, LeaderboardOptions,
};

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "quotesleaderboard".to_string(),
        t: TrancerCommandType::Quotes,
        description: "See who has been quoted the most".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["qlb".to_string(), "quoteslb".to_string()]),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            let quotes = Quote::all(&ctx.sy).await?;

            leaderboard(LeaderboardOptions {
                ctx,
                embed: create_embed(),
                data: lb_accumulate(quotes.0.iter().map(|x| x.author_id.clone()).collect()),
                formatter: LeaderboardFormatter::Suffix("quotes".to_string())
            }).await?;

            Ok(TrancerResponseType::None)
        }),
    }
}
