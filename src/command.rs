use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use serenity::all::Message;
use serenity::client::Context;

pub enum TrancerResponseType {
    None,
    Content(String),
}

pub type TrancerHandler = Arc<dyn Fn(&Context, &Message) -> TrancerFuture + Send + Sync>;
pub type TrancerFuture = Pin<Box<dyn Future<Output = TrancerResponseType> + Send + 'static>>;

pub enum TrancerFlag {
    EachAliasHasItsOwnCommand,
    Ignore,
    NeedsReference,
    AdminOnly,
    BotServerOnly,
    BotOwnerOnly,
    TwilightBoosterOnly
}

pub enum TrancerCommandType {
    Analytics,
    Dawnagotchi,
    Ranks,
    Economy,
    Cards,
    Badges,
    Booster,
    Counting,
    Spirals,
    Quotes,
    Help,
    Minecraft,
    Hypnosis,
    Uncategorized,
    Fun,
    Admin,
    Messages,
    Leaderboards,
    Games,
    Actions,
    Ai,
    Marriage,
    Reporting,
    Qotd,
    Voice,
    Confessions,
    FileDirectory
}

pub struct TrancerCommand {
    pub name: String,
    pub description: String,
    pub aliases: Option<Vec<String>>,
    pub flags: Option<Vec<TrancerFlag>>,

    pub handler: TrancerHandler,
}