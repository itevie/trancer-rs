use crate::cmd_util::TrancerError;
use crate::database::Database;
use crate::impl_from_row;
use rand::Rng;
use rusqlite::ToSql;
use serde::{Deserialize, Serialize};
use serenity::all::{Message, UserId};
use serenity::client::Context;

impl_from_row!(Spiral, SpiralFiends {
   id: u32,
   link: String,
   sent_by: Option<String>,
   created_at: String,
   file_name: Option<String>
});

impl Spiral {
    pub async fn add(
        ctx: &Context,
        link: String,
        author: UserId,
        file_name: String,
    ) -> rusqlite::Result<Spiral> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        let mut result = db.get_many(
            "INSERT INTO spirals (link, sent_by, file_name) VALUES (?1, ?2, ?3) RETURNING *;",
            &[&link, &author.get(), &file_name],
            Spiral::from_row,
        )?;

        Ok(result.remove(0))
    }

    pub async fn get_all(ctx: &Context) -> rusqlite::Result<Vec<Spiral>> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        db.get_many("SELECT * FROM spirals", &[], Spiral::from_row)
    }

    pub async fn get_random(ctx: &Context) -> rusqlite::Result<Spiral> {
        let all = Spiral::get_all(ctx).await?;
        let mut rng = rand::thread_rng();
        Ok(all[rng.gen_range(0..all.len())].clone())
    }

    pub async fn get_by_link(ctx: &Context, link: &str) -> rusqlite::Result<Option<Spiral>> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        let spirals = db.get_many(
            "SELECT * FROM spirals WHERE link = ?1",
            &[&link],
            Spiral::from_row,
        )?;

        Ok(spirals.into_iter().next())
    }

    pub async fn get_from_message(
        ctx: &Context,
        message: &Message,
    ) -> Result<Spiral, TrancerError> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        if let Some(ref reference) = message.referenced_message {
            let spiral = db.get_many(
                "SELECT * FROM spirals WHERE link = ?1",
                &[&reference.content],
                Spiral::from_row,
            )?;

            if spiral.len() > 0 {
                return Ok(spiral.get(0).unwrap().clone());
            }
        }

        Err(TrancerError::Generic(
            "Could not find a spiral from that message".to_string(),
        ))
    }

    pub async fn update_key<T>(
        &self,
        ctx: &Context,
        key: SpiralFiends,
        value: T,
    ) -> rusqlite::Result<()>
    where
        T: ToSql + Send + Sync + 'static,
    {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();
        let sql = format!("UPDATE spirals SET {} = ?1 WHERE id = ?2", key.as_str());

        db.run(&sql, &[&value, &self.id])?;
        Ok(())
    }
}
