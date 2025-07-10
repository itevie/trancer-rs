use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::{trancer_handler, TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::{CommandHasNoArgs, CommandTrait};
use crate::models::command_creation::CommandCreation;
use crate::util::embeds::create_embed;
use crate::util::lang::date_time;
use crate::util::pagination::{paginate, PaginationDataType, PaginationOptions};
use chrono::DateTime;

command_file!(TrancerCommand::<CommandHasNoArgs> {
    name: "newcommands".to_string(),
    description: "Get a list of the newly added commands".to_string(),
    t: TrancerCommandType::Help,
    details: Default::default(),

    handler: trancer_handler!(|ctx, args| {
        let data = CommandCreation::get_all(&ctx.sy).await?;

        paginate(PaginationOptions {
            ctx: ctx.clone(),
            embed: create_embed().title("Newly Added Commands"),
            page_size: 20,
            data: PaginationDataType::Description {
                data: data
                    .iter()
                    .map(|x| {
                        DateTime::parse_from_rfc3339(&x.created_at)
                            .map(|dt| format!("**{}**: {}", x.name, date_time(dt)))
                    })
                    .rev()
                    .collect::<Result<Vec<_>, _>>()?,
                base_description: None,
            },
        })
        .await?;

        Ok(TrancerResponseType::None)
    })
});
