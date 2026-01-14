use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::models::user_data::UserData;
use crate::util::lang::{make_percentage, pronoun};
use crate::util::level_calc;
use crate::util::level_calc::CalcEverything;
use crate::{command_argument_struct, command_file};
use serenity::all::User;
use std::collections::HashMap;

command_argument_struct!(XpArgs {
    user: User, PCACV::User
});

command_file! {
    TrancerCommand::<XpArgs> {
        name: "xp".to_string(),
        t: TrancerCommandType::Economy,
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
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let user_data = UserData::fetch(&ctx.sy, args.user.id, ctx.msg.guild_id.unwrap()).await?;
            let xp = user_data.xp;

            let CalcEverything {level, next_level, amount_progress, least, average, most, progress, ..} = level_calc::calc_everything(xp);

            Ok(TrancerResponseType::Content(
                format!(
                    "**{}** have **{xp} XP** (level {level}), you need **{amount_progress}** more xp until **level {next_level}**\n\n\
                    {level} {} {}\n\n\
                    If you sent a message every minute, it would take:\n\
                    > Least: **{least}** minutes\n\
                    > Average: **{average}** minutes\n\
                    > Most: **{most}** minutes",
                    pronoun(&ctx.msg.author, &args.user, "You", "They"),
                    make_percentage(progress, 20),
                    level + 1,
                ),
            ))
        }),
    }
}
