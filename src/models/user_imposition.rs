use crate::database::Database;
use crate::impl_from_row;
use crate::models::user_data::HypnoStatus;
use serenity::all::{Context, UserId};
use std::collections::HashMap;

impl_from_row!(
    UserImposition,
    UserImpositionFields {
        user_id: String,
        what: String,
        is_bombardable: bool,
        tags: String,
    }
);

#[derive(PartialEq, Clone, Debug)]
pub enum ImpositionTag {
    Green,
    Yellow,
    Red,
    Bombard,
    ByOthers,
}

impl From<HypnoStatus> for ImpositionTag {
    fn from(status: HypnoStatus) -> Self {
        match status {
            HypnoStatus::Green => ImpositionTag::Green,
            HypnoStatus::Yellow => ImpositionTag::Yellow,
            HypnoStatus::Red => ImpositionTag::Red,
        }
    }
}

pub struct ImpositionTagList(pub Vec<ImpositionTag>);

impl ImpositionTagList {
    pub fn has(&self, tag: ImpositionTag) -> bool {
        self.0.contains(&tag)
    }

    pub fn has_all(&self, tags: &[ImpositionTag]) -> bool {
        for i in tags.iter() {
            if !self.0.contains(i) {
                return false;
            }
        }

        true
    }

    pub fn all(&self) -> Vec<ImpositionTag> {
        self.0.clone()
    }
}

impl UserImposition {
    pub fn tags(&self) -> ImpositionTagList {
        let parts = self.tags.split(';').collect::<Vec<&str>>();
        let map: HashMap<&str, ImpositionTag> = HashMap::from([
            ("green", ImpositionTag::Green),
            ("yellow", ImpositionTag::Yellow),
            ("red", ImpositionTag::Red),
            ("bombard", ImpositionTag::Bombard),
            ("by others", ImpositionTag::ByOthers),
        ]);

        let mut tags: Vec<ImpositionTag> = Vec::new();
        for (k, v) in map {
            if parts.contains(&k) {
                tags.push(v);
            }
        }

        ImpositionTagList(tags)
    }

    pub async fn fetch_all_for(
        ctx: &Context,
        user_id: UserId,
    ) -> rusqlite::Result<Vec<UserImposition>> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        db.get_many(
            "SELECT * FROM user_imposition WHERE user_id = ?1",
            &[&user_id.to_string()],
            |r| UserImposition::from_row(r),
        )
    }

    pub async fn fetch_all_for_by_tag(
        ctx: &Context,
        user_id: UserId,
        tags: &[ImpositionTag],
    ) -> rusqlite::Result<Vec<UserImposition>> {
        let all = Self::fetch_all_for(ctx, user_id).await?;
        Ok(all
            .iter()
            .filter(|x| x.tags().has_all(tags))
            .cloned()
            .collect::<Vec<UserImposition>>())
    }

    pub async fn has(ctx: &Context, user_id: UserId, what: String) -> rusqlite::Result<bool> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        let result = db.get_one(
            "SELECT * FROM user_imposition WHERE user_id = ?1 AND what = $2",
            &[&user_id.to_string(), &what],
            |r| UserImposition::from_row(r),
        );

        match result {
            Ok(_) => Ok(true),
            Err(rusqlite::Error::QueryReturnedNoRows) => Ok(false),
            Err(e) => Err(e),
        }
    }

    pub async fn create(
        ctx: &Context,
        user_id: UserId,
        what: String,
    ) -> rusqlite::Result<UserImposition> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        db.get_one(
            "INSERT INTO user_imposition (user_id, what) VALUES (?1, ?2) RETURNING *",
            &[&user_id.to_string(), &what],
            |r| UserImposition::from_row(r),
        )
    }

    pub async fn remove(ctx: &Context, user_id: UserId, what: String) -> rusqlite::Result<()> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        db.run(
            "DELETE FROM user_imposition WHERE user_id = ?1 AND what = ?2",
            &[&user_id.to_string(), &what],
        )
    }

    pub async fn set_tags(ctx: &Context, user_id: UserId, tags: String) -> rusqlite::Result<()> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        db.run(
            "UPDATE user_imposition SET tags = ?1 WHERE user_id = ?2",
            &[&tags, &user_id.to_string()],
        )
    }
}
