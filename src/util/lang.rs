use crate::models::item::Item;
use crate::models::user_data::PRONOUN_SET_CACHE;
use crate::trancer_config::all_pronouns::ALL_PRONOUNS;
use crate::util::config::CONFIG;
use chrono::{DateTime, TimeZone};
use serenity::all::{Permissions, User};
use std::fmt::Display;

pub fn pronoun<S: Into<String>>(user1: &User, user2: &User, same_prn: S, diff_prn: S) -> String {
    let same_prn = same_prn.into();
    let diff_prn = diff_prn.into();

    if user1 == user2 {
        same_prn.replace("$name", &user1.name)
    } else {
        diff_prn.replace("$name", &user2.name)
    }
}

pub enum PronounType {
    Subject,
    Object,
    PossessiveAdjective,
    PossessivePronoun,
    Reflexive,
}

pub fn pronoun_for(user: &User, t: PronounType) -> &str {
    let binding = PRONOUN_SET_CACHE.read().unwrap();
    let prn = binding
        .get(&user.id.to_string())
        .cloned()
        .unwrap_or("they".to_string());
    let set = ALL_PRONOUNS
        .get(prn.as_str())
        .unwrap_or_else(|| ALL_PRONOUNS.get(&"they").unwrap());
    match t {
        PronounType::Subject => set.sub,
        PronounType::Object => set.obj,
        PronounType::PossessiveAdjective => set.poss_adj,
        PronounType::PossessivePronoun => set.poss_prn,
        PronounType::Reflexive => set.reflex,
    }
}

pub fn pronoun_base(user1: &User, user2: &User, t: PronounType, p: bool) -> String {
    let val = if user1 == user2 {
        "your"
    } else {
        pronoun_for(user2, t)
    }
    .to_string();

    if p {
        proper(val)
    } else {
        val
    }
}

pub fn pron(user1: &User, user2: &User) -> String {
    pronoun_base(user1, user2, PronounType::PossessiveAdjective, false)
}

pub fn pronu(user1: &User, user2: &User) -> String {
    pronoun_base(user1, user2, PronounType::PossessiveAdjective, true)
}

pub fn proper<T: Into<String>>(value: T) -> String {
    let value = value.into();
    value
        .split(" ")
        .map(|s| {
            if s.is_empty() {
                "".to_string()
            } else if s.len() == 1 {
                s.to_uppercase()
            } else {
                s.to_string()
                    .chars()
                    .next()
                    .unwrap()
                    .to_string()
                    .to_uppercase()
                    + &*s[1..].to_string()
            }
        })
        .collect::<Vec<String>>()
        .join(" ")
}

pub fn list<T: Into<String>, T2: Into<String>>(data: Vec<(T, T2)>) -> String {
    data.into_iter()
        .map(|x| format!("**{}**: {}", x.0.into(), x.1.into()))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn success<T: Into<String>>(val: T) -> String {
    format!(":green_circle: {}", val.into())
}

pub fn date<T>(dt: DateTime<T>) -> String
where
    T: TimeZone,
    T::Offset: Display,
{
    dt.format("%Y/%m/%d").to_string()
}

pub fn date_time<T>(dt: DateTime<T>) -> String
where
    T: TimeZone,
    T::Offset: Display,
{
    dt.format("%Y/%m/%d %H:%M:%S").to_string()
}

static PROGRESS_BAR_EMPTY: &str = "░";
static PROGRESS_BAR_FILLED: &str = "█";
pub fn make_percentage(percentage: f64, length: u8) -> String {
    let percentage_per = 100f64 / length as f64;
    let amount = (percentage / percentage_per)
        .round()
        .clamp(0., length as f64);

    PROGRESS_BAR_FILLED.repeat(amount as usize)
        + &PROGRESS_BAR_EMPTY.repeat(length as usize - amount as usize)
}

pub fn permission_names(perms: Permissions) -> String {
    Permissions::all()
        .iter()
        .filter(|&p| perms.contains(p))
        .map(|p| match p {
            Permissions::ADMINISTRATOR => "Administrator",
            Permissions::MANAGE_GUILD => "Manage Server",
            Permissions::MANAGE_CHANNELS => "Manage Channels",
            Permissions::MANAGE_MESSAGES => "Manage Messages",
            Permissions::KICK_MEMBERS => "Kick Members",
            Permissions::BAN_MEMBERS => "Ban Members",
            Permissions::SEND_MESSAGES => "Send Messages",
            Permissions::VIEW_CHANNEL => "View Channel",
            Permissions::MENTION_EVERYONE => "Mention Everyone",
            Permissions::EMBED_LINKS => "Embed Links",
            Permissions::READ_MESSAGE_HISTORY => "Read Message History",
            // Add more as needed...
            _ => "Unknown",
        })
        .collect::<Vec<&'static str>>()
        .join(", ")
}

pub fn warn<T: Into<String>>(what: T) -> String {
    format!(":warning: {}", what.into())
}

pub fn currency<T: Into<i64>>(amount: T) -> String {
    format!("**{}{}**", amount.into(), CONFIG.economy.symbol)
}

pub fn item_text(item: Item, amount: u32) -> String {
    format!(
        "**{} {} {}{}**",
        if amount == 0 {
            String::new()
        } else {
            format!("{} ", amount)
        },
        item.emoji.unwrap_or_default(),
        item.name,
        if amount == 0 {
            ""
        } else if amount != 1 {
            "s"
        } else {
            ""
        }
    )
}

pub fn englishify_list(items: Vec<String>, use_or: bool) -> String {
    if items.is_empty() {
        return String::new();
    }

    let mut finished = String::new();

    for i in 0..items.len() {
        let sep = if i == items.len() - 1 {
            if use_or {
                " or "
            } else {
                " and "
            }
        } else {
            ", "
        };

        finished.push_str(&format!("{}{}", sep, items[i]));
    }

    finished.clone()
}
