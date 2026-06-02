use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use crate::database::Database;
use crate::impl_from_row;
use crate::models::aquired_item::AquiredItem;
use crate::models::economy::{Economy, MoneyAddReason};
use crate::models::server_settings::ServerSettingsFields;
use crate::models::user_data::UserData;
use crate::trancer_config::all_missions::{base_random_rewards, get_defined_missions};
use crate::util::config::CONFIG;
use crate::util::other::random_range;
use crate::util::random_rewards::{
    englishify_random_reward, generate_random_rewards, give_random_reward, RandomRewardResult,
};
use chrono::{DateTime, Utc};
use rand::prelude::SliceRandom;
use rand::rngs::StdRng;
use rand::SeedableRng;
use rusqlite::ToSql;
use serde::{Deserialize, Serialize};
use serenity::all::{Context, GuildId, UserId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MissionDetails {
    pub items: Vec<AquiredItem>,
    pub eco: Economy,
    pub user_data: UserData,
}

impl_from_row!(
    TrancerMission,
    MissionFields {
        id: u32,
        r#for: String,
        created_at: String,
        name: String,
        completed: bool,
        completed_at: String,
        rewards: String,
        old: String
    }
);

impl TrancerMission {
    pub async fn get_for(
        ctx: &Context,
        user_id: UserId,
    ) -> Result<Vec<TrancerMission>, TrancerError> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        let today = Utc::now().format("%Y-%m-%d").to_string();

        // Check for today's missions
        let current_missions = db.get_many(
            "
            SELECT *
            FROM missions
            WHERE `for` = ?
              AND DATE(created_at) = ?
            ",
            &[&user_id.to_string(), &today.clone()],
            TrancerMission::from_row,
        )?;

        if !current_missions.is_empty() {
            return Ok(current_missions);
        }

        let ms = get_defined_missions();

        // No missions today, generate 3
        let mut mission_names: Vec<String> = ms.clone().keys().map(|s| s.to_string()).collect();

        let mut rng = StdRng::from_entropy();
        mission_names.shuffle(&mut rng);

        let selected = mission_names.into_iter().take(3).collect::<Vec<_>>();

        let old = Self::get_mission_details(ctx, user_id).await?;
        let json = match serde_json::to_string(&old) {
            Ok(ok) => ok,
            Err(err) => {
                return Err(TrancerError::Generic(format!(
                    "Serialization error: {}",
                    err
                )))
            }
        };

        for name in &selected {
            let base = base_random_rewards();
            let rewards = base
                .get(&ms.get(name.as_str()).unwrap().difficulty)
                .unwrap();
            println!("{:?}", rewards.clone());
            let reward = generate_random_rewards(ctx, rewards.clone()).await?;

            let reward_json = match serde_json::to_string(&reward) {
                Ok(ok) => ok,
                Err(err) => {
                    return Err(TrancerError::Generic(format!(
                        "Serialization error: {}",
                        err
                    )))
                }
            };

            db.run(
                "
                INSERT INTO missions (
                    `for`,
                    created_at,
                    name,
                    completed,
                    completed_at,
                    rewards,
                    old
                )
                VALUES (?, ?, ?, 0, '', ?, ?)
                ",
                &[
                    &user_id.to_string(),
                    &Utc::now().to_rfc3339(),
                    &name.clone(),
                    &reward_json,
                    &json,
                ],
            )?;
        }

        // Fetch newly created missions
        Ok(db.get_many(
            "
            SELECT *
            FROM missions
            WHERE `for` = ?
              AND DATE(created_at) = ?
            ",
            &[&user_id.to_string(), &today],
            TrancerMission::from_row,
        )?)
    }

    pub async fn check_missions(ctx: &TrancerRunnerContext) -> Result<String, TrancerError> {
        let user_missions = TrancerMission::get_for(&ctx.sy, ctx.user_id).await?;

        let mut missions: Vec<String> = vec![];
        let mut missions_completed: Vec<String> = vec![];

        let all_missions = get_defined_missions();

        for mission in user_missions {
            let target_mission = all_missions.get(mission.name.as_str()).unwrap();
            let result = (target_mission.check)(ctx.clone(), mission.clone()).await?;

            if result >= 100 {
                let rewards = mission.json_rewards()?;
                give_random_reward(&ctx.sy, ctx.user_id, &rewards, MoneyAddReason::Commands)
                    .await?;
                mission
                    .update_key(&ctx.sy, MissionFields::completed, true)
                    .await?;
                mission
                    .update_key(&ctx.sy, MissionFields::completed, Utc::now().to_rfc3339())
                    .await?;
                missions_completed.push(format!(
                    "{}: {}",
                    target_mission.description,
                    englishify_random_reward(rewards)
                ));
            } else {
                missions.push(format!("{}: {}%", target_mission.description, result))
            }
        }

        let mut finished_string = "".to_string();

        if missions_completed.len() > 0 {
            finished_string += &format!(
                "**You completed the following missions!**\n{}",
                missions_completed.join("\n")
            )
        }

        if missions.len() > 0 {
            if missions_completed.len() > 0 {
                finished_string += "\n";
            }

            finished_string += &format!("**Your missions:**\n{}", missions.join("\n"))
        }

        Ok(finished_string)
    }

    pub async fn get_mission_details(
        ctx: &Context,
        user_id: UserId,
    ) -> Result<MissionDetails, TrancerError> {
        Ok(MissionDetails {
            items: AquiredItem::fetch_all_for(ctx, user_id).await?,
            eco: Economy::fetch(ctx, user_id).await?,
            user_data: UserData::fetch(ctx, user_id, CONFIG.server.id.parse::<GuildId>()?).await?,
        })
    }

    pub fn json_details(&self) -> Result<MissionDetails, TrancerError> {
        match serde_json::from_str(&self.old) {
            Ok(ok) => Ok(ok),
            Err(err) => Err(TrancerError::Generic(format!(
                "Json deserialization error: {}",
                err
            ))),
        }
    }

    pub fn json_rewards(&self) -> Result<RandomRewardResult, TrancerError> {
        match serde_json::from_str(&self.rewards) {
            Ok(ok) => Ok(ok),
            Err(err) => Err(TrancerError::Generic(format!(
                "Json deserialization error: {}",
                err
            ))),
        }
    }

    pub async fn update_key<T>(
        &self,
        ctx: &Context,
        key: MissionFields,
        value: T,
    ) -> rusqlite::Result<()>
    where
        T: ToSql + Send + Sync + 'static,
    {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();
        let sql = format!("UPDATE missions SET {} = ?1 WHERE 'for' = ?2", key.as_str());

        db.run(&sql, &[&value, &self.r#for])?;
        Ok(())
    }
}
