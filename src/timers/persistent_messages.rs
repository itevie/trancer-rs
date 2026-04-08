use crate::cmd_util::TrancerError;
use crate::commands;
use crate::models::economy::Economy;
use crate::models::persistent_messages::{PersistentMessages, PersistentMessagesFields};
use crate::models::server_settings::ServerSettings;
use crate::models::user_data::UserData;
use crate::util::config::CONFIG;
use crate::util::embeds::create_embed;
use crate::util::leaderboard::{leaderboard_string, LeaderboardFormatter};
use rand::prelude::{SliceRandom, StdRng};
use rand::{thread_rng, SeedableRng};
use serenity::all::{
    ChannelId, Context, CreateMessage, EditMessage, GuildChannel, GuildId, Message,
};
use tracing::instrument;

#[instrument]
pub async fn run(ctx: Context) -> Result<(), TrancerError> {
    let server_id: GuildId = CONFIG.server.id.parse()?;
    let server_settings = ServerSettings::fetch(&ctx, server_id).await?;

    if let Some(economy_channel) = CONFIG.persistent_messages.economy_channel.clone() {
        let mut message =
            get_message(&ctx, "economy".to_string(), economy_channel, server_id).await?;

        let eco_data = Economy::fetch_all(&ctx)
            .await?
            .iter()
            .map(|x| (x.balance, x.user_id.clone()))
            .collect::<Vec<(i32, String)>>();
        let mut eco_l_str = leaderboard_string(eco_data, LeaderboardFormatter::Eco);
        eco_l_str.truncate(10);

        let streak_data = UserData::fetch_for_server(&ctx, server_id)
            .await?
            .iter()
            .filter(|x| x.talking_streak > 0)
            .map(|x| x.clone())
            .map(|x| (x.talking_streak as i32, x.user_id.clone()))
            .collect::<Vec<(i32, String)>>();
        let mut streak_l_str = leaderboard_string(
            streak_data,
            LeaderboardFormatter::Suffix("days".to_string()),
        );
        streak_l_str.truncate(10);

        message
            .edit(
                &ctx.http,
                EditMessage::new()
                    .embed(create_embed().title("Top 10...").fields([
                        ("Economy", eco_l_str.join("\n"), true),
                        ("Streaks", streak_l_str.join("\n"), true),
                    ]))
                    .content(""),
            )
            .await?;
    }

    if let Some(random_command_channel) = CONFIG.persistent_messages.random_command_channel.clone()
    {
        let mut message = get_message(
            &ctx,
            "random_command".to_string(),
            random_command_channel,
            server_id,
        )
        .await?;

        let cmds = commands::init();

        let mut rng = StdRng::from_entropy();
        if let Some(random_item) = cmds.choose(&mut rng) {
            message
                .edit(
                    &ctx.http,
                    EditMessage::new().content("").embed(
                        create_embed().title("Random Command!").description(format!(
                            "Try the `{}{}` command!\n\n{}",
                            server_settings.prefix,
                            random_item.name(),
                            random_item.description()
                        )),
                    ),
                )
                .await?;
        }
    }

    Ok(())
}

async fn get_message(
    ctx: &Context,
    name: String,
    channel_id: String,
    server_id: GuildId,
) -> Result<Message, TrancerError> {
    let channel_id = ChannelId::new(channel_id.parse()?);
    let eco_message = setup_persistent_message(&ctx, name.clone(), channel_id, server_id).await?;

    match channel_id
        .message(&ctx.http, eco_message.message_id.parse::<u64>()?)
        .await
    {
        Ok(ok) => Ok(ok),
        Err(_) => {
            let eco_message = create_persistent_message(&ctx, name, channel_id, server_id).await?;
            Ok(channel_id
                .message(&ctx.http, eco_message.message_id.parse::<u64>()?)
                .await?)
        }
    }
}

async fn setup_persistent_message(
    ctx: &Context,
    name: String,
    channel_id: ChannelId,
    server_id: GuildId,
) -> Result<PersistentMessages, TrancerError> {
    let channel = channel_id.to_channel(&ctx.http).await?;

    let guild_channel: GuildChannel = match channel {
        serenity::model::channel::Channel::Guild(gc) => gc,
        _ => return Err(TrancerError::Generic("Not a guild channel".to_string())),
    };

    match PersistentMessages::fetch(&ctx, name.clone(), server_id).await {
        Ok(ok) => Ok(ok),
        Err(_) => create_persistent_message(ctx, name, channel_id, server_id).await,
    }
}

async fn create_persistent_message(
    ctx: &Context,
    name: String,
    channel_id: ChannelId,
    server_id: GuildId,
) -> Result<PersistentMessages, TrancerError> {
    let channel = channel_id.to_channel(&ctx.http).await?;

    let guild_channel: GuildChannel = match channel {
        serenity::model::channel::Channel::Guild(gc) => gc,
        _ => return Err(TrancerError::Generic("Not a guild channel".to_string())),
    };

    let msg = guild_channel
        .send_message(&ctx.http, CreateMessage::new().content("Loading..."))
        .await?;
    let thing = match PersistentMessages::fetch(&ctx, name.clone(), server_id).await {
        Ok(ok) => ok,
        Err(_) => {
            PersistentMessages::create(&ctx, name.clone(), server_id, channel_id, msg.id).await?
        }
    };
    thing
        .update_key(
            ctx,
            PersistentMessagesFields::message_id,
            msg.id.to_string(),
        )
        .await?;

    Ok(PersistentMessages::fetch(&ctx, name.clone(), server_id).await?)
}
