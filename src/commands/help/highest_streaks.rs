use crate::cmd_util::trancer_handler;
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::models::user_data::UserData;
use crate::util::cached_usernames::get_cached_username;
use crate::util::embeds::create_embed;
use crate::util::pagination::{paginate, PaginationDataType, PaginationOptions};
use chrono::{DateTime, Duration, Utc};

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "higheststreaks".to_string(),
        t: TrancerCommandType::Help,
        description: "See who the highest talking streaks everyone has had".to_string(),
        details: Default::default(),

        handler: trancer_handler!(|ctx, _args| {
            let now = Utc::now();

            let mut user_datas: Vec<UserData> = UserData::fetch_for_server(&ctx.sy, ctx.guild_id).await?
                .iter().filter(|x| x.highest_talking_streak > 0).map(|x| x.clone()).collect();

            user_datas.sort_by(|a, b| b.highest_talking_streak.cmp(&a.highest_talking_streak));

            paginate(PaginationOptions {
                ctx,
                embed: create_embed().title("Highest Talking Streaks"),
                page_size: 20,
                data: PaginationDataType::Description {
                    data: user_datas
                        .iter()
                        .map(|x| format!(
                            "**{}**: {} days",
                            get_cached_username(x.user_id.clone()),
                            x.highest_talking_streak
                        ))
                        .collect::<Vec<String>>(),
                    base_description: None,
                },
            }).await?;

            Ok(TrancerResponseType::None)
        }),
    }
}
