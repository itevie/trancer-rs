use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{content_response, trancer_handler};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::models::couting::ServerCount;
use crate::util::cached_usernames::get_cached_username;
use crate::util::embeds::create_embed;
use crate::util::pagination::{paginate, PaginationDataType, PaginationOptions};

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "countruins".to_string(),
        t: TrancerCommandType::Counting,
        description: "See who has ruined the count".to_string(),
        details: Default::default(),

        handler: trancer_handler!(|ctx, _args| {
            let Some(_) = ServerCount::fetch(&ctx.sy, ctx.guild_id, ctx.channel.id).await? else {
                return Ok(content_response("This channel does not have a count set up!"));
            };

            let count_ruins = ServerCount::fetch_ruined(&ctx.sy, ctx.guild_id, ctx.channel.id).await?;

            paginate(PaginationOptions {
                ctx,
                embed: create_embed()
                    .title("Who has ruined the count?"),
                page_size: 20,
                data: PaginationDataType::Description {
                    data: count_ruins
                        .iter()
                        .map(|x| format!("{}: {}", x.ruined_at, get_cached_username(
                            x.ruined_by
                            .clone()
                            .unwrap_or("?".to_string()))
                        ))
                        .collect(),base_description: None,
                }
            }).await?;

            Ok(TrancerResponseType::None)
        }),
    }
}
