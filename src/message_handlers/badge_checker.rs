use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use crate::models::aquired_badge::AquiredBadge;
use crate::models::user_data::UserDataFields;
use crate::reply;
use crate::trancer_config::all_badges::{DefinedBadge, ALL_DEFINED_BADGES};
use crate::util::embeds::create_embed;
use crate::util::lang::list;
use crate::util::other::give_role;
use serenity::all::{CreateMessage, RoleId};
use tracing::error;
use tracing::instrument;

#[instrument]
pub async fn handle(ctx: &TrancerRunnerContext) -> Result<(), TrancerError> {
    let badges = ALL_DEFINED_BADGES.iter();

    let user_badges = AquiredBadge::get_all_for(&ctx.sy, ctx.user_id).await?;

    let potential_badges: Vec<_> = badges
        .filter(|x| !user_badges.0.iter().any(|y| y.badge_name == x.id))
        .collect();

    let mut badges: Vec<DefinedBadge> = vec![];
    let mut rewards: Vec<String> = vec![];

    for item in potential_badges.clone() {
        let Ok(result) = (item.check)(ctx.clone()).await else {
            continue;
        };

        if result == false {
            continue;
        }

        AquiredBadge::add_for(&ctx.sy, ctx.user_id, item.id).await?;
        badges.push(item.clone());

        if let Some(options) = item.extra.clone() {
            let roles = ctx.guild_id.roles(&ctx.sy).await?;

            for role in options.give_roles {
                let Some(role) = roles.get(&role.parse::<RoleId>()?) else {
                    continue;
                };

                give_role(&ctx.sy, &ctx.msg.member(&ctx.sy).await?, role).await?;
                rewards.push(format!("The <@&{}> role", role));
            }
        }
    }

    if badges.len() != 0 {
        let _ = reply!(
            ctx,
            CreateMessage::new().embed(
                create_embed()
                    .title("You got the following badges!")
                    .description(format!(
                        "{}{}",
                        badges
                            .iter()
                            .map(|x| format!("{} {}", x.emoji, x.name))
                            .collect::<Vec<_>>()
                            .join(", "),
                        if rewards.len() != 0 {
                            format!("\n\nRewards: {}", rewards.join(", "))
                        } else {
                            "".to_string()
                        }
                    ))
            )
        );
    }

    Ok(())
}
