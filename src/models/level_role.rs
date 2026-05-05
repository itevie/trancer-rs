use crate::database::Database;
use crate::impl_from_row;
use rusqlite::Error;
use serenity::all::{Context, GuildId};

impl_from_row!(LevelRole, LevelRoleFields {
   server_id: String,
   role_id: Option<String>,
   level: u32
});

impl LevelRole {
    pub async fn fetch_all(ctx: &Context, server_id: GuildId) -> rusqlite::Result<Vec<LevelRole>> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        db.get_many(
            "SELECT * FROM level_roles WHERE server_id = ?1",
            &[&server_id.to_string()],
            LevelRole::from_row,
        )
    }

    pub async fn fetch_by_level(
        ctx: &Context,
        server_id: GuildId,
        level: u32,
    ) -> rusqlite::Result<Option<LevelRole>> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        match db.get_one(
            "SELECT * FROM level_roles WHERE server_id = ?1 AND level = ?2 LIMIT 1",
            &[&server_id.to_string(), &level],
            LevelRole::from_row,
        ) {
            Ok(value) => Ok(Some(value)),
            Err(Error::QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub async fn delete(&self, ctx: &Context) -> rusqlite::Result<()> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        db.run(
            "DELETE FROM level_roles WHERE server_id = ?1 AND role_id = ?2 AND level = ?3",
            &[&self.server_id, &self.role_id, &self.level],
        )?;

        Ok(())
    }
}
