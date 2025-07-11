use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::trancer_handler;
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::commands::CommandHasNoArgs;
use crate::models::user_data::UserData;
use crate::util::embeds::create_embed;
use crate::util::pagination::{paginate, PaginationDataType, PaginationOptions};
use crate::{command_argument_struct, command_file};
use chrono_humanize::HumanTime;
use std::collections::HashMap;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "birthdays".to_string(),
        t: TrancerCommandType::Help,
        description: "List the upcoming birthdays!".to_string(),
        details: Default::default(),

        handler: trancer_handler!(|ctx, args| {
            let mut user_datas = UserData::fetch_for_server(&ctx.sy, ctx.msg.guild_id.unwrap()).await?
                .iter()
                .map(|x| (x.user_id.clone(), x.next_birthday()))
                .filter(|x| match x {
                    (_, Ok(ok)) => ok.is_some(),
                    (_, Err(_)) => true
                })
                .map(|x| match x {
                    (id, Ok(ok)) => Ok((id, ok.unwrap())),
                    (_, Err(err)) => Err(err)
                })
                .collect::<Result<Vec<_>, _>>()?;
            user_datas.sort_by(|a, b| b.1.cmp(&a.1));

            paginate(PaginationOptions {
                ctx,
                embed: create_embed().title("Upcoming Birthdays :birthday:"),
                page_size: 20,
                data: PaginationDataType::Description {
                    data: user_datas.iter().map(|x| format!("**{}**: {} ({})", "Todo", HumanTime::from(x.1), x.1.format("%Y-%m-%d"))).collect(),
                    base_description: None,
                }

            }).await?;

            Ok(TrancerResponseType::None)
        }),
    }
}
