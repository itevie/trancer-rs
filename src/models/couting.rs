use crate::database::Database;
use crate::impl_from_row;
use crate::models::user_data::{UserData, UserDataFields};
use crate::util::config::CONFIG;
use chrono::Utc;
use serenity::all::{ChannelId, Context, GuildId, UserId};

impl_from_row!(ServerCount, ServerCountFields {
    server_id: String,
    channel_id: String,
    current_count: u32,
    last_counter: Option<String>,
    highest_count: u32,
    ignore_failure: bool,
    ignore_failure_weekend: bool,
    allow_consecutive_counts: bool,
});

impl_from_row!(
    ServerCountingRuin,
    ServerCountingRuinFields {
        server_id: String,
        channel_id: String,
        count: u32,
        ruined_at: String,
        ruined_by: Option<String>
    }
);

impl ServerCount {
    pub async fn fetch(
        ctx: &Context,
        server_id: GuildId,
        channel_id: ChannelId,
    ) -> rusqlite::Result<Option<ServerCount>> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        let result = db.get_one(
            "SELECT * FROM server_count WHERE channel_id = ?1 AND server_id = ?2 LIMIT 1",
            &[&channel_id.to_string(), &server_id.to_string()],
            ServerCount::from_row,
        );

        match result {
            Ok(row) => Ok(Some(row)),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub async fn increase(
        ctx: &Context,
        server_id: GuildId,
        channel_id: ChannelId,
        counter: UserId,
    ) -> rusqlite::Result<()> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        db.run(
            "UPDATE server_count SET current_count = current_count + 1, highest_count = MAX(highest_count, current_count + 1), last_counter = ?3 WHERE server_id = ?1 AND channel_id = ?2",
        &[&server_id.to_string(), &channel_id.to_string(), &Some(counter.to_string())]
        )?;

        Ok(())
    }

    pub async fn fetch_ruined(
        ctx: &Context,
        server_id: GuildId,
        channel_id: ChannelId,
    ) -> rusqlite::Result<Vec<ServerCountingRuin>> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        db.get_many(
            "SELECT * FROM server_count_ruins WHERE channel_id = ?1 AND server_id = ?2 LIMIT 1",
            &[&channel_id.to_string(), &server_id.to_string()],
            ServerCountingRuin::from_row,
        )
    }

    /// Returns whether the user was count-banned.
    pub async fn ruined(
        ctx: &Context,
        server_id: GuildId,
        channel_id: ChannelId,
        ruined_by: UserId,
    ) -> rusqlite::Result<bool> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        let current_count = ServerCount::fetch(ctx, server_id, channel_id)
            .await?
            .unwrap();

        db.run(
            "INSERT INTO server_count_ruins (server_id, channel_id, count, ruined_at, ruined_by) VALUES (?1, ?2, ?3, ?4, ?5)",
            &[&server_id.to_string(), &channel_id.to_string(), &current_count.current_count, &Utc::now().to_rfc3339(), &Some(ruined_by.to_string())]
        )?;

        db.run(
            "UPDATE server_count SET current_count = 0, last_counter = NULL WHERE server_id = ?1 AND channel_id = ?2",
            &[&server_id.to_string(), &channel_id.to_string()],
        )?;

        let user_data = UserData::fetch(ctx, ruined_by, server_id).await?;
        let ruined_count = user_data.count_ruined + 1;

        user_data
            .update_key(ctx, UserDataFields::count_ruined, ruined_count)
            .await?;

        if ruined_count >= CONFIG.counting.ruins_to_be_banned {
            user_data
                .update_key(ctx, UserDataFields::count_banned, true)
                .await?;
            return Ok(true);
        }

        Ok(false)
    }
}
