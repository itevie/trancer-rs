use crate::database::Database;
use crate::impl_from_row;
use crate::trancer_config::all_badges::{DefinedBadge, ALL_DEFINED_BADGES};
use serenity::all::UserId;
use serenity::client::Context;

impl_from_row!(
    AquiredBadge,
    AquiredBadgeField {
        user: String,
        badge_name: String,
        date_aquired: String,
    }
);

pub struct AquiredBadgeVec(pub Vec<AquiredBadge>);

impl AquiredBadge {
    pub async fn get_all_for(
        context: &Context,
        user_id: UserId,
    ) -> rusqlite::Result<AquiredBadgeVec> {
        let data_lock = context.data.read().await;
        let database = data_lock.get::<Database>().unwrap();

        Ok(AquiredBadgeVec(database.get_many(
            "SELECT * FROM aquired_badges WHERE user = ?1",
            &[&user_id.to_string()],
            |r| Self::from_row(r),
        )?))
    }
}

impl AquiredBadgeVec {
    pub fn as_defined(&self) -> Vec<&DefinedBadge> {
        self.0
            .clone()
            .iter()
            .map(|x| ALL_DEFINED_BADGES.iter().find(|y| y.id == x.badge_name))
            .filter_map(|x| x)
            .collect::<Vec<_>>()
    }
}
