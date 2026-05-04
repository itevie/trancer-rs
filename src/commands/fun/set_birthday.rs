use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::{content_response, CommandTrait};
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError};
use crate::models::user_data::UserDataFields;
use crate::{command_argument_struct, command_file};
use chrono::NaiveDate;
use std::collections::HashMap;

command_argument_struct!(GetQuoteArgs {
    date: String, PCACV::String
});

command_file! {
    TrancerCommand::<GetQuoteArgs> {
        name: "setbirthday".to_string(),
        t: TrancerCommandType::Fun,
        description: "Set your birthday in Trancer".to_string(),
        details: TrancerDetails {
            aliases: None,
            arguments: Some(TrancerArguments {
               required: 1,
                args: vec![Argument {
                    t: ArgType::String { flags: None },
                    name: "date".to_string(),
                    details: Default::default()
                }]
            }),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let date = match parse_date(&args.date) {
                Ok(ok) => ok,
                Err(err) => return Ok(content_response("You need to give a date like YYYY/MM/DD (2026/03/25), or MM/DD (03/25)\nThe thing that's wrong: ".to_owned() + &*err))
            };

            ctx.user_data.update_key(&ctx.sy, UserDataFields::birthday, date.clone()).await?;

            Ok(content_response(format!("Updated your birthday to {date}")))
        }),
    }
}

fn parse_date(input: &str) -> Result<String, String> {
    // Try full date (YYYY-M-D)
    if let Ok(date) = NaiveDate::parse_from_str(input, "%Y/%-m/%-d") {
        return Ok(date.format("%Y/%m/%d").to_string());
    }

    // Try MM-DD
    let parts: Vec<&str> = input.split('/').collect();
    if parts.len() == 2 {
        let month: u32 = parts[0]
            .parse()
            .map_err(|_| format!("Invalid month in '{}'", input))?;
        let day: u32 = parts[1]
            .parse()
            .map_err(|_| format!("Invalid day in '{}'", input))?;

        // Validate using a dummy year (leap-safe)
        if NaiveDate::from_ymd_opt(2024, month, day).is_some() {
            return Ok(format!("????-{:02}/{:02}", month, day));
        }
    }

    Err(format!("Invalid date format: {}", input))
}
