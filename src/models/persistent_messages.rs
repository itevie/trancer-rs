use crate::database::Database;
use crate::impl_from_row;
use rusqlite::ToSql;
use serenity::all::{ChannelId, Context, GuildId, MessageId};

impl_from_row!(
    PersistentMessages,
    PersistentMessagesFields {
        id: u32,
        name: String,
        message_id: String,
        channel_id: String,
        server_id: String
    }
);

impl PersistentMessages {
    pub async fn fetch(ctx: &Context, name: String, server_id: GuildId) -> rusqlite::Result<Self> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        db.get_one(
            "SELECT * FROM persistent_messages WHERE server_id = ?1 AND name = ?2 LIMIT 1",
            &[&server_id.to_string(), &name.to_string()],
            PersistentMessages::from_row,
        )
    }

    pub async fn create(
        ctx: &Context,
        name: String,
        server_id: GuildId,
        channel_id: ChannelId,
        message_id: MessageId,
    ) -> rusqlite::Result<Self> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        db.get_one(
            "INSERT INTO persistent_messages (name, message_id, channel_id, server_id) VALUES (?1, ?2, ?3, ?4) RETURNING *;",
            &[&name, &message_id.to_string(), &channel_id.to_string(), &server_id.to_string()],
            PersistentMessages::from_row,
        )
    }

    pub async fn update_key<T>(
        &self,
        ctx: &Context,
        key: PersistentMessagesFields,
        value: T,
    ) -> rusqlite::Result<()>
    where
        T: ToSql + Send + Sync + 'static,
    {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();
        let sql = format!(
            "UPDATE persistent_messages SET {} = ?1 WHERE name = ?2 AND server_id = ?3",
            key.as_str()
        );

        db.run(&sql, &[&value, &self.name, &self.server_id])?;
        Ok(())
    }
}
