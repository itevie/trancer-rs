use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::models::qotd_question::QotdQuestion;
use crate::util::embeds::create_embed;
use crate::util::pagination::{paginate, PaginationDataType, PaginationOptions};
use serenity::builder::CreateEmbedFooter;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "questions".to_string(),
        t: TrancerCommandType::Qotd,
        description: "Get a list of QOTD questions".to_string(),
        details: TrancerDetails {
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            let questions = QotdQuestion::fetch_all(&ctx.sy, ctx.guild_id).await?;
            let asked = questions.iter().filter(|x| x.asked).count();

            paginate(PaginationOptions {
                embed: create_embed()
                    .title("QOTD Questions"),
                ctx: ctx.clone(),
                page_size: 20,
                data: PaginationDataType::Description {
                    data: questions.iter().map(|x| {
                       let check = if x.asked {
                            ":white_check_mark:"
                        }  else {
                            ":x:"
                        };

                        format!("{check} {}", x.question)
                    }).collect::<Vec<String>>(),
                    base_description: Some(format!(
                        "{asked}/{} asked - Use `{}suggestquestion <question>` to suggest", questions.len(),
                        ctx.server_settings.prefix.clone()
                    )),
                },
            }).await?;

            Ok(TrancerResponseType::None)
        }),
    }
}
