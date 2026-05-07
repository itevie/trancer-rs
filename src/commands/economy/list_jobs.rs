use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::trancer_config::all_jobs::ALL_JOBS;
use crate::util::embeds::create_embed;
use crate::util::lang::list;
use crate::util::level_calc::calculate_level;
use crate::util::pagination::{paginate, Field, PaginationDataType, PaginationOptions};

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "jobs".to_string(),
        t: TrancerCommandType::Help,
        description: "List all of the jobs available".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["j*bs".to_string()]),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            let level = calculate_level(ctx.economy.work_xp as u32);

           paginate(PaginationOptions {
                ctx: ctx.clone(),
                embed: create_embed().title("All J*bs").description(
                    format!("Type `{}job <job name>` to select. Your Work level is **{}**", ctx.server_settings.prefix.clone(), level)
                ),
                page_size: 5,
                data: PaginationDataType::Field(
                    ALL_JOBS.iter().map(|job| {
                        Field {
                            name: job.0.to_string(),
                            value: job.1.description.to_string() + "\n" + &*list(vec![
                                ("Level Required", format!("{} ({})", job.1.level_required, if level >= job.1.level_required {
                                    "✅"
                                } else {
                                    ":x:"
                                })),
                            ]),
                            inline: false
                        }
                    }).collect()
                ),
            }).await?;

            Ok(TrancerResponseType::None)
        }),
    }
}
