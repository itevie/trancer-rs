use crate::database::Database;
use crate::impl_from_row;
use chrono::Utc;
use serenity::client::Context;

impl_from_row!(
    CommandCreation,
    CommandCreationField {
        name: String,
        created_at: String
    }
);

impl CommandCreation {
    pub async fn insert_commands(ctx: &Context, command_names: Vec<String>) -> rusqlite::Result<()> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        let iso_string = Utc::now().to_rfc3339();

        for command in command_names {
            db.run(
                "INSERT OR IGNORE INTO command_creations (name, created_at) VALUES (?1, ?2)",
                &[&command, &iso_string],
            )?
        }

        Ok(())
    }

    pub async fn get_all(ctx: &Context) -> rusqlite::Result<Vec<CommandCreation>> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        db.get_many(
            "SELECT * FROM command_creations;",
            &[],
            |r| CommandCreation::from_row(r),
        )
    }
}
