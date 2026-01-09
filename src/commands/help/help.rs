use crate::cmd_util::trancer_handler;
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::commands::CommandHasNoArgs;
use crate::util::embeds::create_embed;
use crate::util::pagination::{paginate, Field, PaginationDataType, PaginationOptions};
use crate::{command_file, commands};
use serenity::all::CreateMessage;
use std::collections::HashMap;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "help".to_string(),
        t: TrancerCommandType::Help,
        description: "This is a test".to_string(),
        details: Default::default(),

        handler: trancer_handler!(|ctx, args| {
            let commands = commands::init();

            let mut map: HashMap<TrancerCommandType, Vec<String>> = HashMap::new();
            for cmd in &commands {
                map.entry(cmd.t().clone())
                    .or_default()
                    .push(cmd.name());
            }

            let mut fields: Vec<Field> = map
                .iter()
                .map(|entry| {
                    let mut names = entry.1.clone();
                    names.sort();

                    Field {
                        name: format!(
                            "{} {} ({} commands)",
                            entry.0.emoji(),
                            entry.0,
                            entry.1.len()
                        ),
                        value: names.join(", "),
                        inline: true,
                    }
                })
                .collect();

            fields.sort_by(|a, b| a.name.cmp(&b.name));

            paginate(PaginationOptions {
                ctx,
                embed: create_embed().title("All of the commands").description(format!("There are {} commands!", commands.len())),
                page_size: 10,
                data: PaginationDataType::Field(fields)
            }).await?;

            Ok(TrancerResponseType::None)
        }),
    }
}
