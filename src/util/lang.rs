use chrono::{DateTime, TimeZone};
use serenity::all::User;
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

pub fn list<T: Into<String>, T2: Into<String>>(data: Vec<(T, T2)>) -> String {
    data.into_iter()
        .map(|x| format!("**{}**: {}", x.0.into(), x.1.into()))
        .collect::<Vec<String>>()
        .join("\n")
}

pub fn currency(val: i32) -> String {
    format!("**{val} 🌀**")
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
