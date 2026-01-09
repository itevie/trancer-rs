use crate::database::Database;
use crate::impl_from_row;
use crate::models::spiral::Spiral;
use rand::Rng;
use serenity::all::UserId;
use serenity::client::Context;
use tracing::Instrument;

impl_from_row!(
    FavouriteSpiral,
    FavouriteSpiralField {
        id: u32,
        user_id: String
    }
);

impl FavouriteSpiral {
    pub async fn get_all_for(ctx: &Context, user_id: UserId) -> rusqlite::Result<Vec<Spiral>> {
        let lock = ctx.data.read().await;
        let db = lock.get::<Database>().unwrap();

        db.get_many(
            "SELECT s.*
                FROM spirals s
                INNER JOIN user_favourite_spirals f
                ON s.id = f.id
                WHERE f.user_id = ?1",
            &[&user_id.to_string()],
            Spiral::from_row,
        )
    }

    pub async fn exists(ctx: &Context, user_id: UserId, spiral: u32) -> rusqlite::Result<bool> {
        let lock = ctx.data.read().await;
        let db = lock.get::<Database>().unwrap();

        let result = db.get_many(
            "SELECT * FROM user_favourite_spirals WHERE id = ?1 AND user_id = ?2",
            &[&spiral, &user_id.to_string()],
            FavouriteSpiral::from_row,
        )?;

        Ok(result.len() != 0)
    }

    pub async fn get_random_for(
        ctx: &Context,
        user_id: UserId,
    ) -> rusqlite::Result<Option<Spiral>> {
        let all = FavouriteSpiral::get_all_for(ctx, user_id).await?;

        if all.is_empty() {
            return Ok(None);
        }

        let mut rng = rand::thread_rng();

        Ok(all.get(rng.gen_range(0..all.len())).cloned())
    }

    pub async fn add(ctx: &Context, user_id: UserId, spiral: u32) -> rusqlite::Result<()> {
        let lock = ctx.data.read().await;
        let db = lock.get::<Database>().unwrap();

        db.run(
            "INSERT INTO user_favourite_spirals (id, user_id) VALUES (?1, ?2)",
            &[&spiral, &user_id.to_string()],
        )
    }

    pub async fn remove(ctx: &Context, user_id: UserId, spiral: u32) -> rusqlite::Result<()> {
        let lock = ctx.data.read().await;
        let db = lock.get::<Database>().unwrap();

        db.run(
            "DELETE FROM user_favourite_spirals WHERE id = ?1 AND user_id = ?2",
            &[&spiral, &user_id.to_string()],
        )
    }
}
