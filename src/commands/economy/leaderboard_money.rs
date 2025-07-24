use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::commands::CommandHasNoArgs;
use crate::models::economy::Economy;
use crate::util::embeds::create_embed;
use crate::util::leaderboard::leaderboard;
use crate::{command_argument_struct, command_file};
use std::collections::HashMap;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "moneyleaderboard".to_string(),
        t: TrancerCommandType::Help,
        description: "See the economy leaderboard".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["baltop".to_string(), "elb".to_string()]),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let data = Economy::fetch_all(&ctx.sy).await?.iter().map(|x| (x.balance.clone(), x.user_id.clone())).collect::<Vec<(i32, String)>>();
            leaderboard(ctx, create_embed(), data).await?;

            Ok(TrancerResponseType::None)
        }),
    }
}
