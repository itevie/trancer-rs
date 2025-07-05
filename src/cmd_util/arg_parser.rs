use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::TrancerError;
use std::collections::HashMap;

pub trait CommandArgumentStruct {
    fn construct(parts: HashMap<String, PCACV>) -> Result<Box<Self>, TrancerError>
    where
        Self: Sized;
}

#[macro_export]
macro_rules! command_argument_struct {
    ($name:ident {$($i:ident: $t:ty, $n:path), *}) => {
        #[derive(Debug, Clone)]
        pub struct $name {
            $(pub $i: $t,)*
        }

        impl CommandArgumentStruct for $name {
            fn construct(parts: HashMap<String, PCACV>) -> Result<Box<Self>, TrancerError> {
                Ok(Box::from(Self {
                    $(
                        $i: match match parts.get(stringify!($i)) {
                          Some(some) => some,
                            None => return Err(TrancerError::ArgumentConstructor(format!("Failed to construct param: {} for {}, it was not provided", stringify!($i), stringify!($name))))
                        } {
                            $n(v) => v.clone(),
                            e => return Err(TrancerError::ArgumentConstructor(format!("Failed to construct param: {} for {}, it was an invalid type: {:?}", stringify!($i), stringify!($name), e)))
                        }
                    ),*
                }))
            }
        }
    };
}

#[derive(Debug)]
pub struct ParsedArguments {
    pub args: Vec<String>,
    pub wick: HashMap<String, String>,
    pub original: Vec<String>,
    pub original_content: String,
}

static WICK_CHAR: &'static str = "?";

/// PreCommandArgumentConstructionValue
#[derive(Debug)]
pub enum PCACV {
    Number(i32),
    OpNumber(Option<i32>),
    String(String),
    OpString(Option<String>),
}

impl PCACV {
    pub fn from_arg(
        arg: &Argument,
        required: bool,
        value: Option<String>,
    ) -> Result<PCACV, TrancerError> {
        let Some(value) = value else {
            return if required {
                Err(TrancerError::ArgumentOptionalConversion(
                    "The value is required".to_string(),
                ))
            } else {
                match arg.t {
                    ArgType::Number { min: _, max: _ } => Ok(PCACV::OpNumber(None)),
                    _ => panic!("Could not handle {:?}", arg.t),
                }
            };
        };

        match arg.t {
            ArgType::Number { min: _, max: _ } => {
                let ok = match value.parse::<i32>() {
                    Ok(ok) => ok,
                    Err(e) => {
                        return Err(TrancerError::ArgumentConstructor(format!(
                            "Failed to parse number: {}",
                            e
                        )))
                    }
                };

                Ok(if required {
                    PCACV::Number(ok)
                } else {
                    PCACV::OpNumber(Some(ok))
                })
            }
            ArgType::String { flags: _ } => Ok(if required {
                PCACV::String(value)
            } else {
                PCACV::OpString(Some(value))
            }),
            _ => Err(TrancerError::NotImplemented(format!(
                "Cannot handle converting {:?} to PCACV",
                arg.t
            ))),
        }
    }
}

pub fn parse_args(contents: String) -> Result<ParsedArguments, TrancerError> {
    let mut parsed = ParsedArguments {
        args: Vec::new(),
        wick: HashMap::new(),
        original: Vec::new(),
        original_content: contents.clone(),
    };

    let mut chars = contents.chars().peekable();
    let mut wick: Option<String> = None;
    let mut in_wick_name = false;
    let mut current = String::new();

    while let Some(c) = chars.next() {
        match c {
            '\\' => {
                if let Some(next) = chars.next() {
                    current.push(next);
                }
            }
            ' ' => {
                if wick.is_some() {
                    continue;
                } else if in_wick_name {
                    wick = Some(std::mem::take(&mut current));
                    in_wick_name = false;
                } else {
                    parsed.args.push(std::mem::take(&mut current));
                }
            }
            '?' => {
                if let Some(w) = wick.take() {
                    parsed.wick.insert(w, std::mem::take(&mut current));
                    in_wick_name = true;
                } else if !in_wick_name {
                    parsed.args.push(std::mem::take(&mut current));
                    in_wick_name = true;
                } else {
                    current.clear();
                }
            }
            _ => current.push(c),
        }
    }

    if !current.is_empty() {
        match wick {
            Some(w) => {
                parsed.wick.insert(w, current);
            }
            None => parsed.args.push(current),
        }
    }

    Ok(parsed)
}

pub fn map_and_validate<T>(
    args: ParsedArguments,
    arg_schema: TrancerArguments,
) -> Result<Box<T>, TrancerError>
where
    T: CommandArgumentStruct,
{
    let mut arg_map: HashMap<String, PCACV> = HashMap::new();

    for i in 0..arg_schema.args.len() {
        let arg = &arg_schema.args[i];
        let required = arg_schema.required >= i;
        let value = if arg.details.wick_style.is_none() {
            match args.args.get(i) {
                Some(v) => v,
                None => {
                    if required {
                        return Err(TrancerError::MissingPositionalArgument(
                            arg.name.to_string(),
                            arg.clone(),
                        ));
                    } else {
                        arg_map.insert(arg.name.clone(), PCACV::from_arg(&arg, required, None)?);
                        continue;
                    }
                }
            }
        } else {
            match args.wick.get(&arg.name) {
                Some(v) => v,
                None => {
                    if required {
                        return Err(TrancerError::MissingPositionalArgument(
                            arg.name.to_string(),
                            arg.clone(),
                        ));
                    } else {
                        arg_map.insert(arg.name.clone(), PCACV::from_arg(&arg, required, None)?);
                        continue;
                    }
                }
            }
        }
        .clone();

        if let Some(ref must_be) = arg.details.must_be {
            if value != *must_be {
                return Err(TrancerError::ArgumentMustBeFailed(
                    must_be.clone(),
                    arg.clone(),
                ));
            }
        }

        if let Some(ref one_of) = arg.details.one_of {
            if one_of.contains(&value) {
                return Err(TrancerError::ArgumentOneOfFailed(
                    one_of.clone(),
                    arg.clone(),
                ));
            }
        }

        arg_map.insert(
            arg.name.clone(),
            PCACV::from_arg(&arg, required, Some(value))?,
        );
    }

    println!("{:#?}", arg_map);

    T::construct(arg_map)
}
