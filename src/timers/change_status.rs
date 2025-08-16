use crate::cmd_util::TrancerError;
use rand::Rng;
use serenity::all::{ActivityData, ActivityType};
use serenity::client::Context;
use tracing::instrument;

static STATUSES: &[(ActivityType, &str)] = &[
    (ActivityType::Playing, "type .help for help!"),
    (ActivityType::Playing, "with your mind"),
    (ActivityType::Playing, "join my server with .invite"),
    (ActivityType::Playing, "with spirals"),
    (ActivityType::Playing, "with pendulums"),
    (ActivityType::Playing, "*patpat if green*"),
    (ActivityType::Playing, "Among Us"),
    (ActivityType::Playing, "with my Dawnagotchi"),
    (ActivityType::Playing, "I'm Trancer!"),
    (ActivityType::Watching, "you"),
    (ActivityType::Watching, "you sleep"),
    (ActivityType::Watching, "for people to .autoban"),
];

#[instrument]
pub async fn run(ctx: Context) -> Result<(), TrancerError> {
    let mut rand = rand::thread_rng();
    let Some(item) = STATUSES.get(rand.gen_range(0..STATUSES.len())) else {
        return Err(TrancerError::Generic(
            "Failed to get a random status to set. .get was None".to_string(),
        ));
    };

    ctx.shard.set_activity(Some(ActivityData {
        name: item.1.to_string(),
        kind: item.0,
        state: None,
        url: None,
    }));

    Ok(())
}
