use crate::util::config::CONFIG;
use serenity::client::Context;
use sled::IVec;
use std::path::PathBuf;
use std::sync::OnceLock;

static DB: OnceLock<sled::Db> = OnceLock::new();

pub fn init_cached_usernames_database() {
    let path = &PathBuf::from(CONFIG.general.data_dir.clone()).join("cached_usernames.db");
    let db = sled::open(path).expect("Failed to open sled DB");
    DB.set(db).expect("DB already initialized");
}

pub fn get_cached_username(user_id: String) -> String {
    DB.get()
        .unwrap()
        .get(user_id.clone())
        .unwrap_or(None)
        .map(|x| String::from_utf8_lossy(&x).to_string())
        .unwrap_or(user_id)
}

pub fn set_cached_username(user_id: String, value: String) {
    let _ = DB
        .get()
        .unwrap()
        .insert(user_id, IVec::from(value.into_bytes()));
}

pub async fn load_from_sy_cache(ctx: &Context) {
    for guild_id in ctx.cache.guilds() {
        if let Some(guild) = ctx.cache.guild(guild_id) {
            for member in guild.members.values() {
                let user = member.user.clone();
                set_cached_username(user.id.to_string(), user.name);
            }
        }
    }
}
