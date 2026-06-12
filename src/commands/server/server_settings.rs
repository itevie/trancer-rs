use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{CommandTrait, TrancerError};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::util::embeds::create_embed;
use crate::util::pagination::{paginate, PaginationDataType, PaginationOptions};
use serde_json::Value;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "serversettings".to_string(),
        t: TrancerCommandType::Qotd,
        description: "List all your server's settings".to_string(),
        details: TrancerDetails {
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            let json = serde_json::to_value(&ctx.server_settings).unwrap();
            if let Value::Object(map) = json {
                let result = map
                    .iter()
                    .map(|(k, v)| format!("**{}**: {}", k, v))
                    .collect::<Vec<_>>();

                  paginate(PaginationOptions {
                    embed: create_embed()
                        .title("QOTD Questions"),
                    ctx: ctx.clone(),
                    page_size: 20,
                    data: PaginationDataType::Description {
                        data: result,
                        base_description: None,
                    },
                }).await?;
            } else {
                return Err(TrancerError::Generic("Failed to get object from server_settings struct".to_string()));
            }

            Ok(TrancerResponseType::None)
        }),
    }
}
