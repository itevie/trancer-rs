use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerError};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::commands::CommandHasNoArgs;
use crate::models::swear_jar::SwearJar;
use crate::util::embeds::create_embed;
use crate::util::leaderboard::{leaderboard, LeaderboardFormatter, LeaderboardOptions};
use crate::{command_argument_struct, command_file};
use std::collections::HashMap;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "totalswears".to_string(),
        t: TrancerCommandType::Economy,
        description: "See who has said which swear the most".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["totalswearlb".to_string(), "tslb".to_string()]),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            let data = SwearJar::get_all(&ctx.sy)
                .await?
                .iter()
                .filter(|x| x.server_id == ctx.server_settings.server_id)
                .fold(HashMap::<String, i32>::new(), |mut acc, x| {
                    *acc.entry(x.user_id.clone()).or_insert(0) += x.uses as i32;
                    acc
                })
                .into_iter()
                .map(|(user_id, uses)| (uses, user_id))
                .collect::<Vec<(i32, String)>>();

            leaderboard(LeaderboardOptions {
                ctx,
                embed: create_embed().title("Who has sworn the most?"),
                data,
                formatter: LeaderboardFormatter::Suffix("times".to_string())
            }).await?;

            Ok(TrancerResponseType::None)
        }),
    }
}
