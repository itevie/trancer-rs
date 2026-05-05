use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::models::item::Item;
use crate::util::embeds::create_embed;
use crate::util::lang::{currency, item_text};
use crate::util::pagination::{paginate, PaginationDataType, PaginationOptions};

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "shop".to_string(),
        t: TrancerCommandType::Help,
        description: "Get a list of all the items in the shop".to_string(),
        details: TrancerDetails {
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            let mut items = Item::get_all();
            items.sort_by(|a, b| a.name.cmp(&b.name));

            let items_formatted = items.iter().map(|x| {
                let x = x.clone();
                let item_text = item_text(x.clone(), 0);
                let price_indicator = if !x.buyable { "~~" } else { "" };
                let price = format!("{}{}{}", price_indicator, currency(x.price), price_indicator);
                let weight = format!("({:.2}% weight)", x.weight * 100.0);
                let description = x.description.unwrap_or("No Description".to_string());
                let tag = if let Some(tag) = x.tag {
                    format!("[{tag}]")
                } else {
                    "".to_string()
                };

                format!(
                    "{item_text} - Buy {price} {weight}\n- {description} {tag}"
                )
            }).collect::<Vec<String>>();

           paginate(PaginationOptions {
                ctx: ctx.clone(),
                embed: create_embed().title("The Shop"),
                page_size: 10,
                data: PaginationDataType::Description {
                    data: items_formatted,
                    base_description: Some(
                        format!("Buy with `{}buy <item>`", ctx.server_settings.prefix.clone())
                    ),
                }
            }).await?;

            Ok(TrancerResponseType::None)
        }),
    }
}
