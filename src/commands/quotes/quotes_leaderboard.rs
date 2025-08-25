use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::commands::CommandHasNoArgs;
use crate::models::quote::Quote;
use crate::util::embeds::create_embed;
use crate::util::leaderboard::{lb_accumulate, leaderboard};
use crate::util::pagination::{paginate, PaginationDataType, PaginationOptions};
use crate::{command_argument_struct, command_file};
use std::collections::HashMap;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "quotesleaderboard".to_string(),
        t: TrancerCommandType::Help,
        description: "See who has been quoted the most".to_string(),
        details: TrancerDetails {
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let quotes = Quote::all(&ctx.sy).await?;

            leaderboard(ctx, create_embed().title("Who has been quoted the most"),
            lb_accumulate(quotes.0.iter().map(|x| x.author_id.clone()).collect())).await?;

            Ok(TrancerResponseType::None)
        }),
    }
}
