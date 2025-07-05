#[derive(Debug, Clone)]
pub struct TrancerArguments {
    pub required: usize,
    pub args: Vec<Argument>,
}

#[derive(Debug, Clone)]
pub struct Argument {
    pub name: String,
    pub details: ArgumentDetails,
    pub t: ArgType,
}

#[derive(Default, Clone, Debug)]
pub struct ArgumentDetails {
    pub description: Option<String>,
    pub must_be: Option<String>,
    pub one_of: Option<Vec<String>>,
    pub wick_style: Option<WickArgumentOptions>,
}

#[derive(Default, Debug, Clone)]
pub struct WickArgumentOptions {
    pub aliases: Option<Vec<String>>,
}

#[derive(Debug, Clone)]
pub enum ArgType {
    Any,
    Boolean,
    String {
        flags: Option<Vec<StringArgTypeFlag>>,
    },
    Attachment,
    Number {
        min: Option<i32>,
        max: Option<i32>,
    },
    Array {
        inner: Box<ArgType>,
    },
    Currency {
        min: Option<i32>,
        max: Option<i32>,
        allow_negative: bool,
    },
    User {
        allow_bots: bool,
    },
}

#[derive(Debug, Clone)]
pub enum StringArgTypeFlag {
    TakeContent,
    TakeRest,
}
