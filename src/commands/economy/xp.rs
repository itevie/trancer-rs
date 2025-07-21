use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::models::user_data::UserData;
use crate::util::lang::{make_percentage, pronoun};
use crate::util::level_calc;
use crate::util::level_calc::{MAX_XP, MIN_XP, TIME_BETWEEN};
use crate::{command_argument_struct, command_file};
use serenity::all::User;
use std::collections::HashMap;

command_argument_struct!(XpArgs {
    user: User, PCACV::User
});

command_file! {
    TrancerCommand::<XpArgs> {
        name: "xp".to_string(),
        t: TrancerCommandType::Help,
        description: "Get yours or another person's XP".to_string(),
        details: TrancerDetails {
            arguments: Some(TrancerArguments {
                required: 1,
                args: vec![Argument {
                    name: "user".to_string(),
                    t: ArgType::User { allow_bots: true, infer: true },
                    details: Default::default()
                }]
            }),
            slow: true,
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let user_data = UserData::fetch(&ctx.sy, args.user.id, ctx.msg.guild_id.unwrap()).await?;
            let xp = user_data.xp as f64;

            let level = level_calc::calculate_level(xp as u32) as f64;
            let next_level = level + 1.;
            let current_level_xp = level_calc::get_xp_for_level(level as u32) as f64;
            let next_level_xp = level_calc::get_xp_for_level(level as u32 + 1) as f64;
            let needed_xp = next_level_xp - current_level_xp;

            let progress = if needed_xp == 0.0 {
                100.0
            } else {
                ((xp - current_level_xp) / needed_xp) * 100.0
            };
            let amount_progress = next_level_xp - xp;

            let most = amount_progress / (TIME_BETWEEN as f64 * MIN_XP as f64);
            let least = amount_progress / (MAX_XP as f64 * (TIME_BETWEEN as f64 / 60_000.0));
            let average = amount_progress
                / ((MAX_XP as f64 / 2.0).floor() * ((TIME_BETWEEN as f64 / 2.0) / 60_000.0));

            Ok(TrancerResponseType::Content(
                format!(
                    "**{}** have **{xp} XP** (level {level}), you need **{amount_progress}** more xp until **level {next_level}**\n\n\
                    {level} {} {}\n\n\
                    If you sent a message every minute, it would take:\n\
                    > Least: **{least}** minutes\n\
                    > Average: **{average}** minutes\n\
                    > Most: **{most}** minutes",
                    pronoun(&ctx.msg.author, &args.user, "You", "They"),
                    make_percentage(progress as f64, 20),
                    level + 1.0,
                ),
            ))
        }),
    }
}
