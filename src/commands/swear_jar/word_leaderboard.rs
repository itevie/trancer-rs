use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerError};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::models::swear_jar::SwearJar;
use crate::util::embeds::create_embed;
use crate::util::leaderboard::{leaderboard, LeaderboardFormatter, LeaderboardOptions};
use crate::{command_argument_struct, command_file};
use std::collections::HashMap;

command_argument_struct!( UserSwearsWordArgs {
    word: String, PCACV::String
});

command_file! {
    TrancerCommand::<UserSwearsWordArgs> {
        name: "swearleaderboard".to_string(),
        t: TrancerCommandType::SwearJar,
        description: "See who has said which swear the most".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["swearlb".to_string()]),
            arguments: Some(TrancerArguments {
                required: 1,
                args: vec![Argument {
                    name: "word".to_string(),
                    details: Default::default(),
                    t: ArgType::String {
                        flags: None
                    },
                }],
            }),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let data = SwearJar::get_all(&ctx.sy)
                .await?
                .iter().filter(|x| x.server_id == ctx.server_settings.server_id && x.word == args.word)
                .map(|x| (x.uses as i32, x.user_id.clone()))
                .collect::<Vec<(i32, String)>>();

            leaderboard(LeaderboardOptions {
                ctx,
                embed: create_embed().title(format!("Who said {} the most?", args.word)),
                data,
                formatter: LeaderboardFormatter::Suffix("times".to_string())
            }).await?;

            Ok(TrancerResponseType::None)
        }),
    }
}
