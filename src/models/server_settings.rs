use crate::database::Database;
use crate::impl_from_row;
use rusqlite::Error::QueryReturnedNoRows;
use rusqlite::ToSql;
use serenity::all::GuildId;
use serenity::client::Context;

impl_from_row!(ServerSettings, ServerSettingsFields {
    server_id: String,
    prefix: String,

    last_bump: u64,
    bump_reminded: bool,
    last_bumper: Option<String>,

    sub_role_id: Option<String>,
    tist_role_id: Option<String>,
    switch_role_id: Option<String>,

    unverified_role_id: Option<String>,
    verification_role_id: Option<String>,
    verified_string: Option<String>,
    verified_channel_id: Option<String>,

    welcome_channel_id: Option<String>,
    welcome_message: String,
    leave_channel_id: Option<String>,
    leave_message: String,

    quotes_channel_id: Option<String>,
    invite_logger_channel_id: Option<String>,
    remind_bumps: bool,
    bump_channel: Option<String>,
    level_notifications: bool,

    auto_ban_keywords: String,
    auto_ban_enabled: bool,
    auto_ban_count: u64,

    report_channel: Option<String>,
    report_trusted: bool,
    report_ban_log_channel: Option<String>,

    status_theme: String,

    allow_nsfw_file_directory_sources: bool,

    confessions_channel_id: Option<String>,

    analytics: bool,
    random_replies: bool,
    react_bot: bool,

    birthday_channel_id: Option<String>,
    birthday_announcement_text: String
});

impl ServerSettings {
    pub async fn fetch(ctx: &Context, server_id: GuildId) -> rusqlite::Result<Self> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        let result = db.get_one(
            "SELECT * FROM server_settings WHERE server_id = ?1 LIMIT 1",
            &[&server_id.to_string()],
            |r| ServerSettings::from_row(r),
        );

        match result {
            Ok(result) => Ok(result),
            Err(QueryReturnedNoRows) => ServerSettings::create(ctx, server_id).await,
            Err(e) => Err(e),
        }
    }

    pub async fn create(ctx: &Context, server_id: GuildId) -> rusqlite::Result<Self> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        db.get_one(
            "INSERT INTO server_settings (guild_id) VALUES (?1) RETURNING *",
            &[&server_id.to_string()],
            |r| ServerSettings::from_row(r),
        )
    }

    pub async fn update_key<T>(
        &self,
        ctx: &Context,
        key: ServerSettingsFields,
        value: T,
    ) -> rusqlite::Result<()>
    where
        T: ToSql + Send + Sync + 'static,
    {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();
        let sql = format!(
            "UPDATE server_settings SET {} = ?1 WHERE server_id = ?2",
            key.as_str()
        );

        db.run(&sql, &[&value, &self.server_id])?;
        Ok(())
    }
}
