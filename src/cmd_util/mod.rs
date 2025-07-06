use crate::cmd_util::arg_parser::{map_and_validate, CommandArgumentStruct, ParsedArguments};
use crate::cmd_util::args::TrancerArguments;
use serenity::all::{GuildChannel, Message};
use serenity::builder::CreateMessage;
use serenity::client::Context;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub mod arg_parser;
pub mod args;
pub mod flag;
pub mod trancer_error;
pub mod types;

pub(crate) use flag::TrancerFlag;
pub(crate) use trancer_error::*;

/// Returned from all commands
pub enum TrancerResponseType {
    /// Do nothing after the command is executed
    None,

    /// Reply to the user with a new message of String
    Content(String),

    /// For returning a full MessageCreate
    Big(CreateMessage),
}

/// The future returned from commands
pub type TrancerFuture<'a> =
    Pin<Box<dyn Future<Output = Result<TrancerResponseType, TrancerError>> + Send + 'a>>;

/// The type of the command handler
pub type TrancerHandler<T> =
    Arc<dyn Fn(TrancerRunnerContext, T) -> TrancerFuture<'static> + Send + Sync>;

/// This is just some magic to allow typing to work
pub trait CommandTrait: Send + Sync {
    /// The function to run the command
    fn run(&self, ctx: TrancerRunnerContext, args: ParsedArguments) -> TrancerFuture;

    /// Get the name of the command
    fn name(&self) -> String;

    /// Get the description of the command
    fn description(&self) -> String;

    /// Get the other details of the command
    fn details(&self) -> TrancerDetails;
}

pub struct TrancerCommand<T: CommandArgumentStruct> {
    pub name: String,
    pub t: TrancerCommandType,
    pub description: String,
    pub details: TrancerDetails,
    pub handler: TrancerHandler<T>,
}

impl<T: CommandArgumentStruct + Send + 'static + std::fmt::Debug> CommandTrait
    for TrancerCommand<T>
{
    fn run(&self, ctx: TrancerRunnerContext, args: ParsedArguments) -> TrancerFuture {
        Box::pin(async move {
            let arg_schema = self.details().arguments;

            let mapped_args = if let Some(arg_schema) = arg_schema {
                map_and_validate::<T>(args, arg_schema, &ctx).await?
            } else {
                T::construct(HashMap::new())?
            };

            (self.handler)(ctx, *mapped_args).await
        })
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn description(&self) -> String {
        self.description.clone()
    }

    fn details(&self) -> TrancerDetails {
        self.details.clone()
    }
}

#[derive(Clone)]
pub struct TrancerRunnerContext {
    pub sy: Context,
    pub msg: Message,
    pub channel: GuildChannel,
    pub server_settings: ServerSettings,
    pub user_data: UserData,
}

#[derive(Default, Clone)]
pub struct TrancerDetails {
    pub aliases: Option<Vec<String>>,
    pub flags: Option<Vec<TrancerFlag>>,
    pub arguments: Option<TrancerArguments>,
}

/// Helps create the handler for commands
macro_rules! trancer_handler {
    (|$ctx:ident, $args:ident| $body:block) => {
        std::sync::Arc::new(move |$ctx, $args| {
            Box::pin(async move $body)
        })
    };
}
pub(crate) use trancer_handler;
use crate::cmd_util::types::TrancerCommandType;
use crate::models::server_settings::ServerSettings;
use crate::models::user_data::UserData;
