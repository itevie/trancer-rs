use crate::database::Database;
use crate::impl_from_row;
use serenity::all::{Context, GuildId, UserId};

impl_from_row!(
    SwearJar,
    SwearJarField {
        user_id: String,
        server_id: String,
        word: String,
        uses: u32,
    }
);

impl SwearJar {
    pub async fn get_all(ctx: &Context) -> rusqlite::Result<Vec<SwearJar>> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        db.get_many("SELECT * FROM swear_jar", &[], SwearJar::from_row)
    }

    pub async fn create(
        ctx: &Context,
        user_id: UserId,
        server_id: GuildId,
        word: String,
        count: u32,
    ) -> rusqlite::Result<()> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        db.run(
            "INSERT INTO swear_jar (user_id, server_id, word, uses) VALUES (?1, ?2, ?3, ?4)\
        ON CONFLICT(user_id, server_id, word)\
        DO UPDATE SET uses = uses + ?4\
        ",
            &[
                &user_id.to_string(),
                &server_id.to_string(),
                &word.to_string(),
                &count,
            ],
        )
    }
}
