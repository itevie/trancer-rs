pub struct TrancerArguments {
    pub required: i32,
    pub args: Vec<Argument>
}

pub enum StringArgTypeFlag {
    TakeContent,
    TakeRest,
}

pub struct Argument {
    pub name: String,
    pub details: ArgumentDetails,
    pub t: ArgType,
}

#[derive(Default)]
pub struct ArgumentDetails {
    pub description: Option<String>,
    pub must_be: Option<String>,
    pub one_of: Option<Vec<String>>,
    pub wick_style: Option<WickArgumentOptions>,
}

#[derive(Default)]
pub struct WickArgumentOptions {
    pub aliases: Option<Vec<String>>,
}

pub enum ArgType {
    Any,
    Boolean,
    String { flags: Option<Vec<StringArgTypeFlag>> },
    Attachment,
    Number { min: Option<i32>, max: Option<i32> },
    Array { inner: Box<ArgType> },
    Currency { min: Option<i32>, max: Option<i32>, allow_negative: bool },
    User { allow_bots: bool },
}