use crate::database::Database;
use crate::impl_from_row;
use rusqlite::Error::QueryReturnedNoRows;
use rusqlite::ToSql;
use serenity::all::UserId;
use serenity::client::Context;
use std::fmt::Display;

impl_from_row!(Economy, EconomyFields {
    user_id: String,
    balance: i32,
    last_fish: u64,
    last_daily: u64,
    last_dawn_care: u64,
    last_dawn_care_all_100: u64,
    from_messaging: i32,
    from_vc: i32,
    from_commands: i32,
    from_gambling: i32,
    from_gambling_lost: i32,
    from_helping: i32,
    from_mc: i32,
    from_mc_lost: i32,
    work_xp: i32,
    mine_xp: i32,
    fish_xp: i32,
    job: Option<String>,
    mission_tokens: i32,
});

pub enum MoneyAddReason {
    Gambling,
    Commands,
    Messaging,
    Vc,
    Helping,
}

impl Display for MoneyAddReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MoneyAddReason::Gambling => write!(f, "gambling"),
            MoneyAddReason::Commands => write!(f, "commands"),
            MoneyAddReason::Messaging => write!(f, "messaging"),
            MoneyAddReason::Vc => write!(f, "vc"),
            MoneyAddReason::Helping => write!(f, "helping"),
        }
    }
}

impl Economy {
    pub async fn fetch(ctx: &Context, user_id: UserId) -> rusqlite::Result<Economy> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        let result = db.get_one(
            "SELECT * FROM economy WHERE user_id = ?1 LIMIT 1;",
            &[&user_id.to_string()],
            Economy::from_row,
        );

        match result {
            Ok(ok) => Ok(ok),
            Err(QueryReturnedNoRows) => Economy::create(ctx, user_id).await,
            Err(e) => Err(e),
        }
    }

    pub async fn fetch_all(ctx: &Context) -> rusqlite::Result<Vec<Economy>> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        db.get_many("SELECT * FROM economy", &[], Economy::from_row)
    }

    pub async fn create(ctx: &Context, user_id: UserId) -> rusqlite::Result<Economy> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        db.get_one(
            "INSERT INTO economy (user_id) VALUES (?1) RETURNING *",
            &[&user_id.to_string()],
            Economy::from_row,
        )
    }

    pub async fn add_money(
        &self,
        ctx: &Context,
        amount: u32,
        reason: Option<MoneyAddReason>,
    ) -> rusqlite::Result<()> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        let sql = if let Some(reason) = reason {
            format!(
                "UPDATE economy SET balance = balance + ?1, from_{} = from_{} + ?1 WHERE user_id + ?2",
                reason, reason
            )
        } else {
            "UPDATE economy SET balance = balance + ?1 WHERE user_id = ?2".to_string()
        };

        db.run(sql, &[&amount, &self.user_id.to_string()])
    }

    pub async fn remove_money(
        &self,
        ctx: &Context,
        amount: u32,
        gambling_related: bool,
    ) -> rusqlite::Result<()> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        let sql = if gambling_related {
            "UPDATE economy SET balance = balance - ?1, from_gambling_lost = from_gambling_lost + ?1 WHERE user_id + ?2".to_string()
        } else {
            "UPDATE economy SET balance = balance - ?1 WHERE user_id + ?2".to_string()
        };

        db.run(sql, &[&amount, &self.user_id.to_string()])
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
