use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, ArgumentDetails, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::commands::CommandHasNoArgs;
use crate::models::aquired_item::AquiredItem;
use crate::models::item::{get_item, get_item_name};
use crate::trancer_config::all_items::ALL_ITEMS_DEF;
use crate::trancer_config::all_recipes::CRAFTING_RECIPES;
use crate::util::lang::{englishify_list, item_text};
use crate::{command_argument_struct, command_file};
use std::collections::HashMap;

command_argument_struct!(CraftArgs {
   item: String, PCACV::String,
    amount: Option<i32>, PCACV::OpNumber
});

command_file! {
    TrancerCommand::<CraftArgs> {
        name: "craft".to_string(),
        t: TrancerCommandType::Help,
        description: "Craft an item".to_string(),
        details: TrancerDetails {
            arguments: Some(TrancerArguments {
               required: 1,
                args: vec![
                    Argument {
                        // TODO: Add item type
                        t: ArgType::String { flags: None },
                        name: "item".to_string(),
                        details: ArgumentDetails {
                            one_of: Some(CRAFTING_RECIPES.keys().map(|x| x.to_string()).collect()),
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
            let aquired = AquiredItem::fetch_all_for(&ctx.sy, ctx.msg.author.id).await?;
            let recipe = CRAFTING_RECIPES.get(&args.item).unwrap();

            for (name, amount) in recipe {
                let item = get_item_name(*name)?;
                if aquired.iter().find(|x| x.item_id == item.id).is_none() ||
                aquired.iter().find(|x| x.item_id == item.id).unwrap().amount < (*amount * wanted_amount) {
                    return Ok(TrancerResponseType::Content(format!("You do not have {}", item_text(item, (*amount * wanted_amount)))))
                }
            }

            for (name, amount) in recipe {
                let item = get_item_name(*name)?;
                AquiredItem::remove_item_from(&ctx.sy, ctx.msg.author.id, item.id, *amount).await?;
            }

            let item = get_item_name(args.item)?;
            AquiredItem::give_item_to(&ctx.sy, ctx.msg.author.id, item.id, wanted_amount).await?;


            Ok(TrancerResponseType::Content(format!(
                "You crafted {} for {}",
                item_text(item, wanted_amount),
                englishify_list(vec![
                    recipe.iter().map(|(name, amount)|
                    item_text(get_item_name(*name).unwrap(), *amount)).collect()
                ], false)
            )))
        }),
    }
}
