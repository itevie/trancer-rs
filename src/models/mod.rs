pub mod aquired_badge;
pub mod aquired_item;
pub mod blacklisted;
pub mod card;
pub mod command_creation;
pub mod confession;
pub mod dawnagotchi;
pub mod economy;
pub mod giveaway;
pub mod giveaway_entry;
pub mod item;
pub mod ratelimit;
pub mod server_settings;
pub mod user_data;
pub mod user_imposition;

#[macro_export]
macro_rules! enum_with_sql {
    ($name:ident {$($f:ident = $str:expr),*}) => {
        use rusqlite::types::{FromSql, FromSqlResult, FromSqlError};

        #[derive(Debug, Clone)]
        pub enum $name {
            $($f),*
        }

        impl FromSql for $name {
            fn column_result(value: rusqlite::types::ValueRef<'_>) -> FromSqlResult<Self> {
                let s = value.as_str()?;
                match s {
                    $($str => Ok($name::$f),)*
                    _ => Err(FromSqlError::InvalidType),
                }
            }
        }
    };
}
