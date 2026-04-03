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
        name: "streaks".to_string(),
        t: TrancerCommandType::Help,
        description: "See who has the highest streaks".to_string(),
        details: Default::default(),

        handler: trancer_handler!(|ctx, _args| {
            let now = Utc::now();

            let mut user_datas: Vec<UserData> = UserData::fetch_for_server(&ctx.sy, ctx.guild_id).await?
                .iter().filter(|x| x.talking_streak > 0).map(|x| x.clone()).collect();

            user_datas.retain(|user_data| {
                let Some(iso) = user_data.last_talking_streak.clone() else {
                    return false;
                };

                if let Ok(dt) = iso.parse::<DateTime<Utc>>() {
                    now - dt <= Duration::hours(36)
                } else {
                    false
                }
            });

            user_datas.sort_by(|a, b| b.talking_streak.cmp(&a.talking_streak));

            paginate(PaginationOptions {
                ctx,
                embed: create_embed().title("Talking Streaks"),
                page_size: 20,
                data: PaginationDataType::Description {
                    data: user_datas
                        .iter()
                        .map(|x| format!(
                            "**{}**: {} days",
                            get_cached_username(x.user_id.clone()),
                            x.talking_streak
                        ))
                        .collect::<Vec<String>>(),
                    base_description: None,
                },
            }).await?;

            Ok(TrancerResponseType::None)
        }),
    }
}
