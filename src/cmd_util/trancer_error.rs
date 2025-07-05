use crate::cmd_util::args::Argument;
use std::fmt;

#[derive(Debug)]
pub enum TrancerError {
    Sqlite(rusqlite::Error),
    Serenity(serenity::Error),
    ArgumentConstructor(String),
    MissingPositionalArgument(String, Argument),
    MissingWickArgument(String, Argument),
    ArgumentMustBeFailed(String, Argument),
    ArgumentOneOfFailed(Vec<String>, Argument),
    ArgumentConversion(String),
    ArgumentOptionalConversion(String),
    NotImplemented(String),
}

impl std::error::Error for TrancerError {}

impl fmt::Display for TrancerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TrancerError::Sqlite(err) => write!(f, "Database error: {}", err),
            TrancerError::Serenity(err) => write!(f, "Serenity error: {}", err),
            TrancerError::ArgumentConstructor(err) => {
                write!(f, "Argument constructor error: {}", err)
            }
            TrancerError::MissingPositionalArgument(_, arg) => {
                write!(f, "The {} is missing", arg.name)
            }
            TrancerError::MissingWickArgument(_, arg) => write!(f, "The {} is missing", arg.name),
            TrancerError::ArgumentMustBeFailed(must, arg) => {
                write!(f, "The {} argument must be {}", arg.name, must)
            }
            TrancerError::ArgumentOneOfFailed(oneOf, arg) => write!(
                f,
                "The {} argument must be one of: {}",
                arg.name,
                oneOf.join(", ")
            ),
            TrancerError::ArgumentConversion(err) => {
                write!(f, "Failed to convert into wanted type: {}", err)
            }
            TrancerError::ArgumentOptionalConversion(err) => {
                write!(f, "The argument was required: {}", err)
            }
            TrancerError::NotImplemented(err) => {
                write!(f, "This feature is not implemented yet: {}", err)
            }
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
