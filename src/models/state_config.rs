use crate::database::Database;
use crate::impl_from_row;
use rusqlite::ToSql;
use serenity::all::Context;

impl_from_row!(StateConfig, StateConfigFields {
   last_backup: Option<String>,
   last_lottery: Option<String>,
   last_qotd: Option<String>
});

impl StateConfig {
    pub async fn fetch(ctx: &Context) -> Self {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        match db.get_one("SELECT * FROM config;", &[], StateConfig::from_row) {
            Ok(ok) => ok,
            Err(_) => db
                .get_one(
                    "INSERT INTO config DEFAULT VALUES RETURNING *",
                    &[],
                    StateConfig::from_row,
                )
                .unwrap(),
        }
    }

    pub async fn update_key<T>(
        &self,
        ctx: &Context,
        key: StateConfigFields,
        value: T,
    ) -> rusqlite::Result<()>
    where
        T: ToSql + Send + Sync + 'static,
    {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();
        let sql = format!("UPDATE config SET {} = ?1", key.as_str());

        db.run(&sql, &[&value])?;
        Ok(())
    }
}
