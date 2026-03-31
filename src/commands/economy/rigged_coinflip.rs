use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::{content_response, CommandTrait};
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::commands::error;
use crate::models::economy::{Economy, MoneyAddReason};
use crate::util::embeds::create_embed;
use crate::util::lang::currency;
use crate::{command_argument_struct, command_file, confirm_action, reply};
use rand::random;
use serenity::all::{ButtonStyle, CreateActionRow, CreateButton, EditMessage};
use serenity::all::{CreateMessage, Message};
use std::collections::HashMap;
use std::time::Duration;

command_argument_struct!(RiggedCoinflipArgs {
    amount: i32, PCACV::Number
});

command_file! {
    TrancerCommand::<RiggedCoinflipArgs> {
        name: "riggedcoinflip".to_string(),
        t: TrancerCommandType::Economy,
        description: "Flip a ***RIGGED*** coin (40% chance of winning)".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["rcf".to_string()]),
            arguments: Some(TrancerArguments {
               required: 1,
                args: vec![
                    Argument {
                        t: ArgType::Currency { range: Some(10..i32::MAX), allow_negative: false},
                        name: "amount".to_string(),
                        details: Default::default(),
                    }
                ]
            }),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let is_big =
              args.amount >= 1000 ||
              ctx.economy.balance <= 100 ||
              args.amount >= ctx.economy.balance / 2;

            let mut msg: Option<Message> = None;

            if is_big {
                let result = confirm_action!(ctx, CreateMessage::new().content(format!("You are coin-flipping a lot of money ({})!, are you sure?", currency(args.amount))));
                if result.0 == false {
                    return Ok(TrancerResponseType::None);
                }

                if Economy::fetch(&ctx.sy, ctx.msg.author.id).await?.balance < args.amount {
                    return Ok(content_response(format!("You no longer have {} because your balance went lower while waiting for confirmation.", currency(args.amount))));
                }

                msg = Some(result.1);
            }

            let win = random::<f64>() < 0.4;

            if win {
                ctx.economy.add_money(&ctx.sy, args.amount as u32, Some(MoneyAddReason::Gambling)).await?;
            } else {
                ctx.economy.remove_money(&ctx.sy, args.amount as u32, true).await?;
            }

            if !win && args.amount > 1000 {
                reply!(ctx, CreateMessage::new().content("https://tenor.com/view/not-stonks-profit-down-sad-frown-arms-crossed-gif-15684535"))?;
            }

            let embed = create_embed().title("Coinflip Outcome").description(
                if win {
                    format!(":green_circle: The coin landed in your favour! Your earnt {}!", currency(args.amount))
                } else {
                    format!(":red_circle: The coin did not land in your favour, you lost {} :(", currency(args.amount))
                }
            );

            if let Some(mut msg) = msg {
                msg.edit(&ctx.sy.http, EditMessage::new().embed(embed.clone()).content("")).await?;
            } else {
                reply!(ctx, CreateMessage::new().embed(embed.clone()))?;
            }

            Ok(TrancerResponseType::None)
        }),
    }
}
