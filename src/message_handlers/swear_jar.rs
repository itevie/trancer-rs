use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use crate::models::swear_jar::SwearJar;
use std::collections::HashMap;
use tracing::instrument;

pub static SWEARS: &[&str] = &[
    "dipshit",
    "fuck",
    "fucking",
    "shit",
    "bullshit",
    "bitch",
    "bastard",
    "cunt",
    "twat",
    "asshole",
    "wanker",
    "bollocks",
    "prick",
    "dick",
    "penis",
    "vagina",
    "dumbass",
    "retard",
    "moron",
    "idiot",
    "ass",
    "twit",
    "bastard",
    "tranny",
    "bollocks",
    "bullshit",
    "cock",
    "cocksucker",
    "crap",
    "faggot",
    "fag",
    "pussy",
    "slut",
    "wanker",
    "twink",
    "sissy",
    "cum",
    "sex",
    ":alien:",
    "french",
    "france",
    "bottom",
    "dawn",
    "jud",
];

#[instrument]
pub async fn handle(ctx: &TrancerRunnerContext) -> Result<(), TrancerError> {
    let mut counts = HashMap::new();

    for &swear in SWEARS {
        let c = ctx.msg.content.to_lowercase().matches(swear).count();
        if c > 0 {
            counts.insert(swear, c as u32);
        }
    }

    for (word, count) in counts {
        SwearJar::create(
            &ctx.sy,
            ctx.msg.author.id,
            ctx.guild_id,
            word.to_string(),
            count,
        )
        .await?;
    }

    Ok(())
}
