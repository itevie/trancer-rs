use crate::database::Database;
use crate::impl_from_row;
use serenity::all::{Context, GuildId, UserId};

impl_from_row!(
    QotdQuestion,
    QotdQuestionFields {
        id: u32,
        server_id: u64,
        suggestor: u64,
        question: String,
        asked: bool
    }
);

impl QotdQuestion {
    pub async fn exists(
        ctx: &Context,
        question: String,
        server_id: GuildId,
    ) -> rusqlite::Result<bool> {
        let lock = ctx.data.read().await;
        let db = lock.get::<Database>().unwrap();

        let result = db.get_many(
            "SELECT * FROM qotd_questions WHERE question = ?1 AND server_id = ?2;",
            &[&question, &server_id.get()],
            QotdQuestion::from_row,
        )?;

        Ok(result.len() != 0)
    }

    pub async fn fetch_all(
        ctx: &Context,
        server_id: GuildId,
    ) -> rusqlite::Result<Vec<QotdQuestion>> {
        let lock = ctx.data.read().await;
        let db = lock.get::<Database>().unwrap();

        db.get_many(
            "SELECT * FROM qotd_questions WHERE server_id = ?1;",
            &[&server_id.get()],
            QotdQuestion::from_row,
        )
    }

    pub async fn add_question(
        ctx: &Context,
        question: String,
        server_id: GuildId,
        suggester: UserId,
    ) -> rusqlite::Result<QotdQuestion> {
        let lock = ctx.data.read().await;
        let db = lock.get::<Database>().unwrap();

        db.get_one(
            "INSERT INTO qotd_questions (question, server_id, suggestor) VALUES (?1, ?2, ?3) RETURNING *;",
            &[&question, &server_id.get(), &suggester.get()],
            QotdQuestion::from_row,
        )
    }

    pub async fn set_asked(&self, ctx: &Context, asked: bool) -> rusqlite::Result<()> {
        let lock = ctx.data.read().await;
        let db = lock.get::<Database>().unwrap();

        db.run(
            "UPDATE qotd_questions SET asked = ?2 WHERE id = ?1",
            &[&self.id, &asked],
        )?;

        Ok(())
    }
}
