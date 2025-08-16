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
    pub fn validate_badge_name(name: String) -> rusqlite::Result<()> {
        match ALL_DEFINED_BADGES.iter().find(|x| x.id == name) {
            Some(_) => Ok(()),
            _ => Err(rusqlite::Error::InvalidColumnName(format!(
                "{} is not a valid badge ID",
                name
            ))),
        }
    }

    pub async fn get_all_for(
        context: &Context,
        user_id: UserId,
    ) -> rusqlite::Result<AquiredBadgeVec> {
        let data_lock = context.data.read().await;
        let database = data_lock.get::<Database>().unwrap();

        Ok(AquiredBadgeVec(database.get_many(
            "SELECT * FROM aquired_badges WHERE user = ?1",
            &[&user_id.to_string()],
            Self::from_row,
        )?))
    }

    pub async fn get_all_by_badge<T: Into<String>>(
        context: &Context,
        name: T,
    ) -> rusqlite::Result<AquiredBadgeVec> {
        let name = name.into();
        AquiredBadge::validate_badge_name(name.clone())?;
        let data_lock = context.data.read().await;
        let database = data_lock.get::<Database>().unwrap();

        Ok(AquiredBadgeVec(database.get_many(
            "SELECT * FROM aquired_badges WHERE badge_name = ?1",
            &[&name],
            Self::from_row,
        )?))
    }

    pub async fn has<T: Into<String>>(
        context: &Context,
        user_id: UserId,
        badge: T,
    ) -> rusqlite::Result<bool> {
        let s = badge.into();
        AquiredBadge::validate_badge_name(s.clone())?;
        Ok(AquiredBadge::get_all_for(context, user_id)
            .await?
            .0
            .iter()
            .any(|b| b.badge_name == s))
    }

    pub async fn add_for<T: Into<String>>(
        context: &Context,
        user_id: UserId,
        badge: T,
    ) -> rusqlite::Result<()> {
        let data_lock = context.data.read().await;
        let database = data_lock.get::<Database>().unwrap();

        let s = badge.into();
        AquiredBadge::validate_badge_name(s.clone())?;

        database.run(
            "INSERT INTO aquired_badges (user, badge_name, date_aquired) VALUES (?1, ?2, ?3)",
            &[&user_id.to_string(), &s, &chrono::Utc::now().to_rfc3339()],
        )
    }

    pub async fn remove_for<T: Into<String>>(
        context: &Context,
        user_id: UserId,
        badge: T,
    ) -> rusqlite::Result<()> {
        let data_lock = context.data.read().await;
        let database = data_lock.get::<Database>().unwrap();

        let s = badge.into();
        AquiredBadge::validate_badge_name(s.clone())?;

        database.run(
            "DELETE FROM aquired_badges WHERE user = ?1 AND badge_name = ?2",
            &[&user_id.to_string(), &s],
        )
    }
}

impl AquiredBadgeVec {
    pub fn as_defined(&self) -> Vec<&DefinedBadge> {
        self.0
            .clone()
            .iter()
            .filter_map(|x| ALL_DEFINED_BADGES.iter().find(|y| y.id == x.badge_name))
            .collect::<Vec<_>>()
    }
}
