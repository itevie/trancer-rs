use crate::cmd_util::TrancerError;
use crate::database::Database;
use crate::{enum_with_sql, impl_from_row};
use chrono::{DateTime, Datelike, NaiveDate, TimeZone, Utc};
use rusqlite::Error::QueryReturnedNoRows;
use rusqlite::ToSql;
use serenity::all::{GuildId, UserId};
use serenity::client::Context;
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::{Arc, LazyLock, RwLock};

enum_with_sql!(HypnoStatus {
    Green = "green",
    Yellow = "yellow",
    Red = "red"
});

impl Display for HypnoStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                HypnoStatus::Green => "green 🟢",
                HypnoStatus::Yellow => "yellow 🟡",
                HypnoStatus::Red => "red 🔴",
            }
        )
    }
}

impl_from_row!(UserData, UserDataFields {
    user_id: String,
    guild_id: String,
    bumps: u32,
    messages_sent: u32,
    vc_time: u32,
    xp: u32,
    site_quote_opt_in: bool,
    ttt_win: u32,
    ttt_lose: u32,
    ttt_tie: u32,
    c4_win: u32,
    c4_lose: u32,
    c4_tie: u32,
    allow_requests: bool,
    allow_triggers: bool,
    count_ruined: u32,
    hypno_status: HypnoStatus,
    relationships: bool,
    count_banned: bool,
    birthday: Option<String>,
    talking_streak: u32,
    last_talking_streak: Option<String>,
    highest_talking_streak: u32,
    pronoun_set: String,
});

pub static PRONOUN_SET_CACHE: LazyLock<Arc<RwLock<HashMap<String, String>>>> =
    LazyLock::new(Default::default);

impl UserData {
    pub fn birthday_date(&self) -> Result<Option<DateTime<Utc>>, TrancerError> {
        // Early return None if birthday is None
        let Some(birthday) = &self.birthday else {
            return Ok(None);
        };

        let current_year = Utc::now().year().to_string();
        let replaced = birthday.replace("????", &current_year);
        let date_str = replaced.replace('/', "-");

        let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d")?;
        let datetime = Utc.from_utc_datetime(&date.and_hms_opt(0, 0, 0).unwrap_or_default());

        Ok(Some(datetime))
    }

    pub fn next_birthday(&self) -> Result<Option<DateTime<Utc>>, TrancerError> {
        let Some(birthday) = &self.birthday_date()? else {
            return Ok(None);
        };

        let today = Utc::now().date_naive();
        let birth_month = birthday.month();
        let birth_day = birthday.day();

        let this_year = today.year();
        let this_year_birthday = Utc.with_ymd_and_hms(this_year, birth_month, birth_day, 0, 0, 0);

        let next = if let Some(date) = this_year_birthday.single() {
            if date.date_naive() >= today {
                date
            } else {
                // Next year's birthday
                Utc.with_ymd_and_hms(this_year + 1, birth_month, birth_day, 0, 0, 0)
                    .single()
                    .ok_or_else(|| TrancerError::Generic("Invalid birthday in next year".into()))?
            }
        } else {
            // Handle invalid date (like Feb 29 on non-leap year)
            Utc.with_ymd_and_hms(this_year + 1, birth_month, birth_day, 0, 0, 0)
                .single()
                .ok_or_else(|| TrancerError::Generic("Invalid birthday date".into()))?
        };

        Ok(Some(next))
    }

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
            UserData::from_row,
        );

        match result {
            Ok(ok) => {
                {
                    let mut lock = PRONOUN_SET_CACHE.write().unwrap();
                    lock.remove(&ok.user_id.clone());
                    lock.insert(ok.user_id.clone(), ok.pronoun_set.clone());
                }
                Ok(ok)
            }
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
            UserData::from_row,
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
            UserData::from_row,
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
        let sql = format!(
            "UPDATE user_data SET {} = {} + ?3 WHERE user_id = ?1 AND guild_id = ?2",
            key.as_str(),
            key.as_str()
        );

        db.run(&sql, &[&self.user_id, &self.guild_id, &value])?;
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
