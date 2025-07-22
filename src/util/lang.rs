use crate::models::item::Item;
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

pub fn pron(user1: &User, user2: &User) -> String {
    if user1 == user2 { "your" } else { "their" }.to_string()
}

pub fn pronu(user1: &User, user2: &User) -> String {
    if user1 == user2 { "Your" } else { "Their" }.to_string()
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

static PROGRESS_BAR_EMPTY: &'static str = "░";
static PROGRESS_BAR_FILLED: &'static str = "█";
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
        "**{}{}{}{}**",
        if amount == 0 {
            String::new()
        } else {
            format!("{} ", amount)
        },
        item.emoji.unwrap_or(String::new()),
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

    let mut finished = items.get(0).unwrap().clone();

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
