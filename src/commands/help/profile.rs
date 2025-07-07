use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, ArgumentError, TrancerDetails};
use crate::cmd_util::{TrancerCommand, TrancerError, TrancerResponseType};
use crate::models::economy::Economy;
use crate::models::user_data::UserData;
use crate::util::embeds::create_embed;
use crate::util::lang::{currency, list, pronoun};
use crate::{command_argument_struct, command_file};
use serenity::all::{CreateMessage, User};
use std::collections::HashMap;
use crate::util::level_calc;

command_argument_struct!(ProfileArgs {
    user: User, PCACV::User
});

command_file! {
    TrancerCommand::<ProfileArgs> {
        name: "profile".to_string(),
        t: TrancerCommandType::Help,
        description: "View yours or someone else's profile".to_string(),
        details: TrancerDetails {
            arguments: Some(TrancerArguments {
                required: 1,
                args: vec![
                    Argument {
                        name: "user".to_string(),
                        t: ArgType::User {
                            allow_bots: true,
                            infer: true,
                        },
                        details: Default::default()
                    }
                ]
            }),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let user_data = UserData::fetch(&ctx.sy, args.user.id, ctx.msg.guild_id.unwrap()).await?;
            let economy = Economy::fetch(&ctx.sy, args.user.id).await?;

            let mut embed = create_embed().title(
                format!("{} profile", pronoun(&ctx.msg.author, &args.user, "Your", "$name's"))
            )
            .description(list(vec![
                ("Username", args.user.name.clone()),
                ("ID", args.user.id.to_string()),
                ("Birthday", user_data.birthday.unwrap_or("Not Set".into())),
                ("Hypno Status", user_data.hypno_status.to_string()),
                ("Talking Streak", user_data.talking_streak.to_string() + " days"),
                ("Highest Talking Streak", user_data.highest_talking_streak.to_string() + " days"),
                ("Level", format!("{} ({} xp)", level_calc::calculate_level(user_data.xp), user_data.xp)),
                ("Messages", user_data.messages_sent.to_string()),
                ("VC Time", user_data.vc_time.to_string() + " minutes"),
                ("Bumps", user_data.bumps.to_string()),
                ("Balance", currency(economy.balance)),
                ("Ruined the count", format!("{} times {}", user_data.count_ruined, if user_data.count_banned {
                    "(count banned)"
                 } else { "" }))

            ]));

            if let Some(ref avatar) = args.user.avatar_url() {
                embed = embed.thumbnail(avatar);
            }

            Ok(TrancerResponseType::Big(
                CreateMessage::new().embed(embed)
            ))
        }),
    }
}
