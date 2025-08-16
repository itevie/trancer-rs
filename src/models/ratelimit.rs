use crate::database::Database;
use crate::impl_from_row;
use chrono::{TimeZone, Utc};
use rusqlite::Error::QueryReturnedNoRows;
use serenity::all::UserId;
use serenity::client::Context;

impl_from_row!(
    Ratelimit,
    RatelimitField {
        user_id: String,
        command_name: String,
        last_used: String
    }
);

impl Ratelimit {
    pub async fn fetch(
        ctx: &Context,
        user_id: UserId,
        command_name: String,
    ) -> rusqlite::Result<Ratelimit> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        let result = db.get_one(
            "SELECT * FROM ratelimits WHERE user_id = ?1 AND command_name = ?2 LIMIT 1",
            &[&user_id.to_string(), &command_name],
            Ratelimit::from_row,
        );

        match result {
            Ok(ok) => Ok(ok),
            Err(QueryReturnedNoRows) => db.get_one(
                "INSERT INTO ratelimits (user_id, command_name, last_used) VALUES (?1, ?2, ?3) RETURNING *",
                &[&user_id.to_string(), &command_name.to_string(), &Utc.timestamp_opt(0, 0).unwrap().to_rfc3339()],
                Ratelimit::from_row
            ),
            Err(e) => Err(e),
        }
    }

    pub async fn update(
        ctx: &Context,
        user_id: UserId,
        command_name: String,
    ) -> rusqlite::Result<()> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        db.run(
            "UPDATE ratelimits SET last_used = ?1 WHERE user_id = ?2 AND command_name = ?3",
            &[
                &Utc::now().to_rfc3339(),
                &user_id.to_string(),
                &command_name,
            ],
        )
    }
}
