use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::models::mission::TrancerMission;
use crate::trancer_config::all_missions::get_defined_missions;
use crate::util::embeds::create_embed;
use crate::util::random_rewards::englishify_random_reward;
use serenity::all::CreateMessage;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "missions".to_string(),
        t: TrancerCommandType::Help,
        description: "Get a list of your missions today!".to_string(),
        details: TrancerDetails {
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            let user_missions = TrancerMission::get_for(&ctx.sy, ctx.user_id).await?;
            let missions = get_defined_missions();

            let mut fields = Vec::new();

            for user_mission in &user_missions {
                let mission = missions.get(user_mission.name.as_str()).unwrap();

                let progress = (mission.check)(ctx.clone(), user_mission.clone()).await?;

                fields.push((
                    format!("{} ({progress}%)", mission.description.to_string()) ,
                    englishify_random_reward(user_mission.json_rewards()?),
                    true,
                ));
            }

            Ok(TrancerResponseType::Big(
                CreateMessage::new()
                    .embed(create_embed().title("Here are your missions!").fields(
                       fields
                    ))
            ))
        })
    }
}
