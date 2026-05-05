use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{Argument, ArgumentDetails, StringArgTypeFlag, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{content_response, trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError};
use crate::commands::ArgType;
use crate::models::economy::EconomyFields;
use crate::trancer_config::all_jobs::ALL_JOBS;
use crate::util::level_calc::calculate_level;
use crate::{command_argument_struct, command_file};
use std::collections::HashMap;

command_argument_struct!(SelectJobsArgs {
    job: String, PCACV::String
});

command_file! {
    TrancerCommand::<SelectJobsArgs> {
        name: "job".to_string(),
        t: TrancerCommandType::Economy,
        description: "This is a test".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["selectjob".to_string(), "j*b".to_string(), "selectj*b".to_string()]),

            arguments: Some(TrancerArguments {
                required: 1,
                args: vec![
                    Argument {
                        name: "job".to_string(),
                        details: ArgumentDetails {
                            one_of: Some(ALL_JOBS.keys().map(|x| x.to_string()).collect()),
                            ..Default::default()
                        },
                        t: ArgType::String { flags: Some(vec![StringArgTypeFlag::TakeContent]) },
                    }
                ]
            }),

            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            if ctx.economy.job.is_some() && ctx.economy.job.clone().unwrap() == args.job {
                return Err(TrancerError::NonScary("That is already your job!".to_string()));
            }

            let job = ALL_JOBS.get(&args.job.as_str()).unwrap();

            let level = calculate_level(ctx.economy.work_xp as u32);
            if job.level_required > level {
                return Err(TrancerError::NonScary(
                    format!("You need to be level **{}** in Work, but you are only level {}", job.level_required, level)
                ))
            }

            ctx.economy.update_key(&ctx.sy, EconomyFields::job, args.job.clone()).await?;

            return Ok(content_response(
                format!("You set your job to **{}**! 🎉", args.job)
            ));
        }),
    }
}
