use crate::cmd_util::TrancerError;
use crate::database::Database;
use crate::impl_from_row;
use rand::Rng;
use serenity::all::Message;
use serenity::client::Context;

impl_from_row!(Spiral, SpiralFiends {
   id: u32,
   link: String,
   sent_by: Option<String>,
   created_at: String,
   file_name: Option<String>
});

impl Spiral {
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
}
