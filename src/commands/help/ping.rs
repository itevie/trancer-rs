use crate::models::user_data::{UserData, UserDataFields};
use crate::cmd_util::trancer_handler;
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;

command_file! {
    TrancerCommand {
        name: "ping".to_string(),
        description: "This is a test".to_string(),
        details: Default::default(),

        handler: trancer_handler!(|ctx, msg| {
            let user = UserData::fetch(&ctx, msg.author.id, msg.guild_id.unwrap()).await?;

            msg.reply(
                ctx.clone(),
                format!("Your birthday before: {:?}", user.birthday),
            )
            .await?;
            user.update_key(&ctx, UserDataFields::birthday, &"2007/02/28")
                .await?;

            Ok(TrancerResponseType::None)
        }),
    }
}
