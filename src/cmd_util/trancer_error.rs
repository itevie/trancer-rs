use crate::cmd_util::args::Argument;
use std::fmt;
use chrono::ParseError;

#[derive(Debug)]
pub enum ArgumentError {
    MissingPositional(String, Argument),
    MissingWick(String, Argument),
    MustBeFailed(String, Argument),
    OneOfFailed(Vec<String>, Argument),
    Conversion(String, Argument),
    OptionalConversion(String, Argument),
    Constructor(String),
    Parser(String, Argument),
}

#[derive(Debug)]
pub enum TrancerError {
    Sqlite(rusqlite::Error),
    Serenity(serenity::Error),
    Argument(ArgumentError),
    NotImplemented(String),
    ReplyError(serenity::Error),
    Generic(String)
}

impl std::error::Error for TrancerError {}

impl fmt::Display for TrancerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TrancerError::Sqlite(err) => write!(f, "Database error: {}", err),
            TrancerError::Serenity(err) => write!(f, "Serenity error: {}", err),
            TrancerError::Argument(err) => match err {
                ArgumentError::Constructor(err) => {
                    write!(f, "Argument constructor error: {}", err)
                }
                ArgumentError::MissingPositional(_, arg) => {
                    write!(f, "The {} is missing", arg.name)
                }
                ArgumentError::MissingWick(_, arg) => write!(f, "The {} is missing", arg.name),
                ArgumentError::MustBeFailed(must, arg) => {
                    write!(f, "The {} argument must be {}", arg.name, must)
                }
                ArgumentError::OneOfFailed(one_of, arg) => write!(
                    f,
                    "The {} argument must be one of: {}",
                    arg.name,
                    one_of.join(", ")
                ),
                ArgumentError::Conversion(err, _) => {
                    write!(f, "Failed to convert into wanted type: {}", err)
                }
                ArgumentError::OptionalConversion(err, _) => {
                    write!(f, "The argument was required: {}", err)
                }
                ArgumentError::Parser(err, _) => write!(f, "Failed to parse argument: {}", err),
            },
            TrancerError::NotImplemented(err) => {
                write!(f, "This feature is not implemented yet: {}", err)
            }
            TrancerError::ReplyError(err) => write!(f, "Reply error: {}", err),
            TrancerError::Generic(err) => write!(f, "{}", err),
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

impl From<ArgumentError> for TrancerError {
    fn from(err: ArgumentError) -> Self {
        TrancerError::Argument(err)
    }
}

impl From<ParseError> for TrancerError {
    fn from(err: ParseError) -> Self {
        TrancerError::Generic(format!("Failed to parse date/time: {} ({:?})", err.to_string(), err.kind()))
    }
}