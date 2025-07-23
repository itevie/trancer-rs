use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{TrancerCommand, TrancerError, TrancerResponseType};
use crate::command_file;
use crate::commands::{only_user_args, OnlyUserArgs};
use crate::models::aquired_item::AquiredItem;
use crate::models::item::get_item;
use crate::util::embeds::create_embed;
use crate::util::lang::{item_text, pronu};
use crate::util::pagination::{paginate, PaginationDataType, PaginationOptions};

command_file! {
    TrancerCommand::<OnlyUserArgs> {
        name: "inventory".to_string(),
        t: TrancerCommandType::Economy,
        description: "View your or someone else's inventory".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["i".to_string(), "inv".to_string()]),
            arguments: Some(only_user_args(true, true)),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let items = AquiredItem::fetch_all_for(&ctx.sy, args.user.id).await?
                .into_iter().filter(|x| x.amount > 0).collect::<Vec<AquiredItem>>();

            paginate(PaginationOptions {
                embed: create_embed().title(pronu(&ctx.msg.author, &args.user)),
                ctx,
                page_size: 20,
                data: PaginationDataType::Description {
                    data: items.iter().map(|x| Ok::<String, TrancerError>(format!("{}: {}", item_text(get_item(x.item_id)?, 0), x.amount))).collect::<Result<_, _>>()?,
                    base_description: None,
                }
            }).await?;

            Ok(TrancerResponseType::None)
        }),
    }
}
