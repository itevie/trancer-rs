#[derive(Debug, Clone)]
pub enum TrancerFlag {
    EachAliasHasItsOwnCommand,
    Ignore,
    NeedsReference,
    AdminOnly,
    BotServerOnly,
    BotOwnerOnly,
    TwilightBoosterOnly,
}
