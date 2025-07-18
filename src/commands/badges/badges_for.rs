use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::{only_user_args, OnlyUserArgs};
use crate::models::aquired_badge::AquiredBadge;
use crate::util::embeds::create_embed;
use crate::util::lang::pronu;
use crate::util::pagination::{paginate, PaginationDataType, PaginationOptions};

command_file! {
    TrancerCommand::<OnlyUserArgs> {
        name: "badges".to_string(),
        t: TrancerCommandType::Help,
        description: "Get a list of yours or someone else's badges".to_string(),
        details: TrancerDetails {
            arguments: Some(only_user_args(true, true)),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let binding = AquiredBadge::get_all_for(&ctx.sy, args.user.id).await?;
            let badges = binding.as_defined();

            paginate(PaginationOptions {
                embed: create_embed().title(
                    format!("{} Badges", pronu(&ctx.msg.author, &args.user))
                ),
                ctx,
                page_size: 20,
                data: PaginationDataType::Description {
                    data: badges.iter().map(|x| format!("{}: {}", x.emoji, x.description)).collect(),
                    base_description: None
                }
            }).await?;

            Ok(TrancerResponseType::None)
        }),
    }
}
