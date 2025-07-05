/// The base struct for defining all the arguments
#[derive(Debug, Clone)]
pub struct TrancerArguments {
    /// How many arguments are required
    pub required: usize,

    /// All the argument definitions
    pub args: Vec<Argument>,
}

/// An individual argument
#[derive(Debug, Clone)]
pub struct Argument {
    /// The name of this argument
    pub name: String,

    /// Extra details about this argument
    pub details: ArgumentDetails,

    /// The type of this argument
    pub t: ArgType,
}

/// More details about the argument
#[derive(Default, Clone, Debug)]
pub struct ArgumentDetails {
    /// The description of this argument, shown in help menus
    pub description: Option<String>,

    /// What this argument must be
    pub must_be: Option<String>,

    /// A list of what this argument is allowed to be
    pub one_of: Option<Vec<String>>,

    /// Whether this is a wick style argument (like ?name John)
    pub wick_style: Option<WickArgumentOptions>,
}

/// Options for arguments which are defined as wick
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
        infer: bool,
    },
}

#[derive(Debug, Clone)]
pub enum StringArgTypeFlag {
    TakeContent,
    TakeRest,
}
