use serenity::all::Message;
use serenity::client::Context;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

pub mod flag;
pub mod trancer_error;
pub mod types;
pub mod args;
mod arg_parser;

pub(crate) use flag::TrancerFlag;
pub(crate) use trancer_error::*;
pub(crate) use types::*;

pub enum TrancerResponseType {
    None,
    Content(String),
}

pub type TrancerHandler = Arc<dyn Fn(Context, Message) -> TrancerFuture + Send + Sync>;
pub type TrancerFuture =
    Pin<Box<dyn Future<Output = Result<TrancerResponseType, TrancerError>> + Send + 'static>>;

pub struct TrancerCommand {
    pub name: String,
    pub description: String,
    pub details: TrancerDetails,

    pub handler: TrancerHandler,
}

#[derive(Default)]
pub struct TrancerDetails {
    pub aliases: Option<Vec<String>>,
    pub flags: Option<Vec<TrancerFlag>>,
    pub arguments: Option<TrancerArguments>,
}

macro_rules! trancer_handler {
    (|$ctx:ident, $msg:ident| $body:block) => {
        std::sync::Arc::new(move |$ctx, $msg| {
            Box::pin(async move $body)
        })
    };
}

pub(crate) use trancer_handler;
use crate::cmd_util::args::TrancerArguments;
