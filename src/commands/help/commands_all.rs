use crate::cmd_util::trancer_handler;
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::commands::CommandHasNoArgs;
use crate::util::embeds::create_embed;
use crate::util::pagination::{paginate, PaginationDataType, PaginationOptions};
use crate::{command_file, commands};

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "allcommands".to_string(),
        t: TrancerCommandType::Help,
        description: "Get a list of all the commands".to_string(),
        details: Default::default(),

        handler: trancer_handler!(|ctx, args| {
            let commands = commands::init();

            paginate(PaginationOptions {
                ctx: ctx.clone(),
                embed: create_embed().title(format!("All The Commands ({})", commands.len())),
                page_size: 10,
                data: PaginationDataType::Description {
                    data: commands.iter().map(|x| format!("**{}** [*{:?}*]: {}", x.name(), x.t(), x.description())).collect(),
                    base_description: None,
                },

            }).await?;

            Ok(TrancerResponseType::None)
        }),
    }
}
