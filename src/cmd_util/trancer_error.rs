use std::fmt;

#[derive(Debug)]
pub enum TrancerError {
    Sqlite(rusqlite::Error),
    Serenity(serenity::Error),
}

impl std::error::Error for TrancerError {}

impl fmt::Display for TrancerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TrancerError::Sqlite(err) => write!(f, "Database error: {}", err),
            TrancerError::Serenity(err) => write!(f, "Serenity error: {}", err),
        }
    }
}

impl From<rusqlite::Error> for TrancerError {
    fn from(err: rusqlite::Error) -> Self {
        TrancerError::Sqlite(err)
    }
}


impl From<serenity::Error> for TrancerError {
    fn from(err: serenity::Error) -> Self {
        TrancerError::Serenity(err.into())
    }
}