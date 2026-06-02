use crate::cmd_util::TrancerError;
use crate::models::qotd_question::QotdQuestion;
use crate::models::server_settings::ServerSettings;
use crate::models::state_config::{StateConfig, StateConfigFields};
use crate::util::cached_usernames::get_cached_username;
use crate::util::config::CONFIG;
use crate::util::embeds::create_embed;
use chrono::{Local, Timelike};
use rand::prelude::SliceRandom;
use rand::rngs::StdRng;
use rand::SeedableRng;
use serenity::all::{Channel, ChannelId, ChannelType, Context, CreateEmbedFooter, CreateMessage};
use tracing::instrument;

#[instrument]
pub async fn run(ctx: Context) -> Result<(), TrancerError> {
    let Some(channel_id) = CONFIG.qotd.channel.clone() else {
        return Err(TrancerError::Generic("Qotd channel was null".to_string()));
    };

    let now = Local::now();
    let today = now.date_naive();

    let mut config = StateConfig::fetch(&ctx).await;

    println!("{:?} {}", config.last_qotd, today);
    if let Some(last) = &config.last_qotd {
        if let Ok(last_dt) = last.parse::<chrono::DateTime<Local>>() {
            if last_dt.date_naive() == today {
                return Ok(());
            }
        }
    }

    if now.hour() != CONFIG.qotd.hour {
        return Ok(());
    }

    let all_questions = QotdQuestion::fetch_all(&ctx, CONFIG.server.id.parse()?)
        .await?
        .into_iter()
        .filter(|x| !x.asked)
        .collect::<Vec<_>>();

    if all_questions.is_empty() {
        return Err(TrancerError::Generic("Questions was empty".to_string()));
    }

    let mut rng = StdRng::from_entropy();
    let question = all_questions.choose(&mut rng).unwrap();
    let remaining = all_questions.len().saturating_sub(1);

    let embed = create_embed()
        .title(":cyclone: Question of the day :cyclone:")
        .description(question.question.clone())
        .footer(CreateEmbedFooter::new(format!(
            "Author: {}, ID: {}, Remaining {}",
            get_cached_username(question.suggestor.clone().to_string()),
            question.id,
            remaining
        )));

    let channel = match channel_id.parse::<ChannelId>()?.to_channel(&ctx.http).await {
        Ok(Channel::Guild(c)) if c.kind == ChannelType::Text => c,
        Ok(Channel::Guild(c)) if c.kind == ChannelType::News => c,
        _ => {
            return Err(TrancerError::Generic(
                "Invalid channel type was found".to_string(),
            ))
        }
    };

    config
        .update_key(&ctx, StateConfigFields::last_qotd, Some(now.to_rfc3339()))
        .await?;
    println!("{:?}", StateConfig::fetch(&ctx).await);

    channel
        .send_message(
            &ctx.http,
            CreateMessage::new()
                .content(CONFIG.qotd.content.clone())
                .embed(embed),
        )
        .await?;

    question.set_asked(&ctx, true).await?;

    Ok(())
}
