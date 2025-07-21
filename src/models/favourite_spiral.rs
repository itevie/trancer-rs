use crate::database::Database;
use crate::impl_from_row;
use crate::models::spiral::Spiral;
use rand::Rng;
use serenity::all::UserId;
use serenity::client::Context;

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
            |r| Spiral::from_row(r),
        )
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
}
