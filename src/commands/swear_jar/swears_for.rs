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
use serenity::all::User;
use std::collections::HashMap;

command_argument_struct!( UserSwearsWordArgs {
    user: User, PCACV::User
});

command_file! {
    TrancerCommand::<UserSwearsWordArgs> {
        name: "swearsfrom".to_string(),
        t: TrancerCommandType::Economy,
        description: "See all the swears someone has used".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["myswears".to_string()]),
            arguments: Some(TrancerArguments {
                required: 1,
                args: vec![Argument {
                    name: "user".to_string(),
                    details: Default::default(),
                    t: ArgType::User {
                        allow_bots: true,
                        infer: false,
                    },
                }],
            }),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let data = SwearJar::get_all(&ctx.sy)
                .await?
                .iter().filter(|x| x.server_id == ctx.server_settings.server_id && x.user_id == args.user.id.to_string())
                .map(|x| (x.uses as i32, x.word.clone()))
                .collect::<Vec<(i32, String)>>();

            leaderboard(LeaderboardOptions {
                ctx,
                embed: create_embed().title(format!("What swears has {} said?", args.user.name)),
                data,
                formatter: LeaderboardFormatter::Suffix("times".to_string())
            }).await?;

            Ok(TrancerResponseType::None)
        }),
    }
}
