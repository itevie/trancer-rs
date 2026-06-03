use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, ArgumentDetails, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::{content_response, CommandTrait};
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError};
use crate::models::aquired_item::AquiredItem;
use crate::models::item::Item;
use crate::util::lang::currency;
use crate::{command_argument_struct, command_file};
use std::collections::HashMap;

command_argument_struct!(BuyArgs {
   item: String, PCACV::String,
    amount: Option<i32>, PCACV::OpNumber
});

command_file! {
    TrancerCommand::<BuyArgs> {
        name: "buy".to_string(),
        t: TrancerCommandType::Economy,
        description: "Buy an item from the shop!".to_string(),
        details: TrancerDetails {
            arguments: Some(TrancerArguments {
               required: 1,
                args: vec![
                    Argument {
                        // TODO: Add item type
                        t: ArgType::String { flags: None },
                        name: "item".to_string(),
                        details: ArgumentDetails {
                            ..Default::default()
                        }
                    },
                    Argument {
                        t: ArgType::Number { min: Some(1), max: None, },
                        name: "amount".to_string(),
                        details: Default::default(),
                    }
                ]
            }),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let wanted_amount = args.amount.unwrap_or(1) as u32;
            let item = Item::get_by_name(args.item.as_str());
            let aquired = AquiredItem::fetch_all_for_by_id(&ctx.sy, ctx.msg.author.id, item.id).await?;

            // Check limits
            if !item.buyable {
                return Err(TrancerError::NonScary(
                    format!("{} is not buyable!", item.text(0))
                ))
            }

            if let Some(max) = item.max {
                if aquired.amount >= max || (aquired.amount + wanted_amount) > max {
                    return Err(TrancerError::NonScary(
                        format!("You can only have {} of {}!", max, item.text(0))
                    ))
                }
            }

            let total_price = item.price * wanted_amount;

            if ctx.economy.balance < total_price as i32 {
                return Err(TrancerError::NonScary(
                    format!("You do not have {} to buy {}!", currency(total_price), item.text(wanted_amount))
                ));
            }

            ctx.economy.remove_money(&ctx.sy, wanted_amount * item.price, false).await?;
            AquiredItem::give_item_to(&ctx.sy, ctx.msg.author.id, item.id, wanted_amount).await?;

            Ok(content_response(
                format!("You bought {} for {}!", item.text(wanted_amount), currency(total_price))
            ))
        }),
    }
}
