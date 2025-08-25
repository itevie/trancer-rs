use chrono::{DateTime, NaiveDateTime, TimeZone, Utc};
use rusqlite::types::{FromSql, FromSqlError, FromSqlResult, ValueRef};

#[derive(Debug, Clone)]
pub struct DbDate(pub DateTime<Utc>);

// chatgpt
impl FromSql for DbDate {
    fn column_result(value: ValueRef<'_>) -> FromSqlResult<Self> {
        match value {
            ValueRef::Integer(i) => {
                // Node.js Date.now() (ms since epoch)
                Ok(DbDate(Utc.timestamp_millis(i)))
            }
            ValueRef::Real(f) => {
                // Just in case it's stored as REAL
                Ok(DbDate(Utc.timestamp_millis(f as i64)))
            }
            ValueRef::Text(t) => {
                let s = std::str::from_utf8(t)
                    .map_err(|_| FromSqlError::Other(Box::new(std::fmt::Error)))?;

                // Try RFC3339 first
                if let Ok(dt) = DateTime::parse_from_rfc3339(s) {
                    return Ok(DbDate(dt.with_timezone(&Utc)));
                }

                // Try SQL-like "YYYY-MM-DD HH:MM:SS"
                if let Ok(naive) = NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S") {
                    return Ok(DbDate(DateTime::<Utc>::from_naive_utc_and_offset(
                        naive, Utc,
                    )));
                }

                Err(FromSqlError::Other(Box::new(std::fmt::Error)))
            }
            _ => Err(FromSqlError::InvalidType),
        }
    }
}
