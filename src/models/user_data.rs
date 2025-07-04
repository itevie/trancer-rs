use crate::database::Database;
use crate::impl_from_row;
use rusqlite::Error::QueryReturnedNoRows;
use rusqlite::ToSql;
use serenity::all::{GuildId, UserId};
use serenity::client::Context;

impl_from_row!(UserData, UserDataFields,
    user_id: String,
    guild_id: String,
    bumps: i32,
    messages_sent: i32,
    vc_time: i32,
    xp: i32,
    site_quote_opt_in: bool,
    ttt_win: i32,
    ttt_lose: i32,
    ttt_tie: i32,
    c4_win: i32,
    c4_lose: i32,
    c4_tie: i32,
    allow_requests: bool,
    allow_triggers: bool,
    count_ruined: i32,
    hypno_status: String,
    relationships: bool,
    count_banned: bool,
    birthday: Option<String>,
    talking_streak: i32,
    last_talking_streak: String,
    highest_talking_streak: i32,
);

impl UserData {
    /// Fetch a UserData for a specific user
    pub async fn fetch(
        ctx: &Context,
        user_id: UserId,
        server_id: GuildId,
    ) -> rusqlite::Result<UserData> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        let result = db.get_one(
            "SELECT * FROM user_data WHERE user_id = ?1 AND guild_id = ?2 LIMIT 1",
            &[&user_id.to_string(), &server_id.to_string()],
            |r| UserData::from_row(r),
        );

        match result {
            Ok(ok) => Ok(ok),
            Err(QueryReturnedNoRows) => UserData::create(ctx, user_id, server_id).await,
            Err(e) => Err(e),
        }
    }

    pub async fn create(
        ctx: &Context,
        user_id: UserId,
        server_id: GuildId,
    ) -> rusqlite::Result<UserData> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        db.get_one(
            "INSERT INTO user_data (user_id, guild_id) VALUES (?1, ?2) RETURNING *",
            &[&user_id.to_string(), &server_id.to_string()],
            |r| UserData::from_row(r),
        )
    }

    /// Fetch all UserData's for any given server
    pub async fn fetch_for_server(
        ctx: &Context,
        server_id: GuildId,
    ) -> rusqlite::Result<Vec<UserData>> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        db.get_many(
            "SELECT * FROM user_data WHERE guild_id = ?1",
            &[&server_id.to_string()],
            |r| UserData::from_row(r),
        )
    }

    pub async fn increment(
        &self,
        ctx: &Context,
        key: UserDataFields,
        value: i32,
    ) -> rusqlite::Result<()> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();
        let sql = format!("UPDATE user_data SET {} = {} + 1 WHERE user_id = ?1 AND guild_id = ?2", key.as_str(), key.as_str());

        db.run(&sql, &[&value, &self.user_id, &self.guild_id])?;
        Ok(())
    }

    pub async fn update_key<T>(
        &self,
        ctx: &Context,
        key: UserDataFields,
        value: T,
    ) -> rusqlite::Result<()>
    where
        T: ToSql + Send + Sync + 'static,
    {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();
        let sql = format!(
            "UPDATE user_data SET {} = ?1 WHERE user_id = ?2 AND guild_id = ?3",
            key.as_str()
        );

        db.run(&sql, &[&value, &self.user_id, &self.guild_id])?;
        Ok(())
    }
}
