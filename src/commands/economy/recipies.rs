use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::models::item::Item;
use crate::trancer_config::all_recipes::CRAFTING_RECIPES;
use crate::util::embeds::create_embed;
use crate::util::lang::{englishify_list, item_text};
use crate::util::pagination::{paginate, PaginationDataType, PaginationOptions};

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "recipes".to_string(),
        t: TrancerCommandType::Help,
        description: "Get a list of all the recipes you can craft".to_string(),
        details: TrancerDetails {
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            let recipies = CRAFTING_RECIPES.clone();
            let items = Item::get_all();

            let mut data: Vec<String> = vec![];

            for recipie in recipies {
                data.push(
                    format!(
                        "{}\n- {}",
                        item_text(items.iter().find(|x| x.name == recipie.0).unwrap().clone(), 0),
                        englishify_list(
                            recipie.1
                                .iter()
                                .map(|x|
                                    item_text(
                                        items
                                        .iter()
                                        .find(|y| y.name == x.0.clone())
                                        .unwrap()
                                        .clone(), 0)
                            ).collect(),
                            false
                        )
                    )
                );
            }

           paginate(PaginationOptions {
                ctx: ctx.clone(),
                embed: create_embed().title("This is a Crafting Table"),
                page_size: 10,
                data: PaginationDataType::Description {
                    data,
                    base_description: Some(
                        format!("Get with `{}craft <item>`", ctx.server_settings.prefix.clone())
                    ),
                }
            }).await?;

            Ok(TrancerResponseType::None)
        }),
    }
}
