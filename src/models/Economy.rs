use rusqlite::Error::QueryReturnedNoRows;
use rusqlite::ToSql;
use serenity::all::UserId;
use serenity::client::Context;
use crate::database::Database;
use crate::impl_from_row;

impl_from_row!(Economy, EconomyFields,
    user_id: String,
    balance: i32,
    last_fish: i32,
    lasy_daily: i32,
    lasy_dawn_care: i32,
    last_dawn_care_all_100: i32,
    from_messaging: i32,
    from_vc: i32,
    from_commands: i32,
    from_gambling: i32,
    from_gambling_lost: i32,
    from_helping: i32,
    work_xp: i32,
    mine_xp: i32,
    fish_xp: i32,
    job: Option<String>,
    mission_tokens: i32,
);

impl Economy {
    pub async fn fetch(ctx: &Context, user_id: UserId) -> rusqlite::Result<Economy> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        match db.get_one(
            "SELECT * FROM economy WHERE user_id = ?1 LIMIT 1;",
            &[&user_id.to_string()],
            |r| Economy::from_row(r)
        ) {
            Ok(ok) => Ok(ok),
            Err(QueryReturnedNoRows) => Economy::create(ctx, user_id),
            Err(e) => Err(e)
        }
    }

    pub async fn create(ctx: &Context, user_id: UserId) -> rusqlite::Result<Economy> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        db.get_one(
            "INSERT INTO economy (user_id) VALUES (?1) RETURNING *",
            &[&user_id.to_string()],
            |r| Economy::from_row(r)
        )
    }

    pub async fn increment(
        &self,
        ctx: &Context,
        key: EconomyFields,
        value: i32,
    ) -> rusqlite::Result<()> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();
        let sql = format!(
            "UPDATE economy SET {} = {} + ?2 WHERE user_id = ?1",
            key.as_str(),
            key.as_str()
        );

        db.run(&sql, &[&self.user_id, &value])?;
        Ok(())
    }

    pub async fn update_key<T>(
        &self,
        ctx: &Context,
        key: EconomyFields,
        value: T,
    ) -> rusqlite::Result<()>
    where
        T: ToSql + Send + Sync + 'static,
    {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();
        let sql = format!(
            "UPDATE economy SET {} = ?1 WHERE user_id = ?2",
            key.as_str()
        );

        db.run(&sql, &[&value, &self.user_id])?;
        Ok(())
    }
}