use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::{ArgumentError, TrancerError, TrancerRunnerContext};
use crate::util::lang::currency;
use serenity::all::{User, UserId};
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
                            None => return Err(ArgumentError::Constructor(format!("Failed to construct param: {} for {}, it was not provided", stringify!($i), stringify!($name))))?
                        } {
                            $n(v) => v.clone(),
                            e => return Err(ArgumentError::Constructor((format!("Failed to construct param: {} for {}, it was an invalid type: {:?}", stringify!($i), stringify!($name), e))))?
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
    User(User),
    OpUser(Option<User>),
}

impl PCACV {
    pub async fn from_arg(
        arg: &Argument,
        required: bool,
        value: Option<String>,
        ctx: &TrancerRunnerContext,
    ) -> Result<PCACV, TrancerError> {
        let Some(mut value) = value else {
            return if required {
                Err(ArgumentError::OptionalConversion(
                    "The value is required".to_string(),
                    arg.clone(),
                ))?
            } else {
                match arg.t {
                    ArgType::Number { min: _, max: _ } => Ok(PCACV::OpNumber(None)),
                    ArgType::String { flags: _ } => Ok(PCACV::OpString(None)),
                    ArgType::Currency {
                        range: _,
                        allow_negative: _,
                    } => Ok(PCACV::OpNumber(None)),
                    ArgType::User {
                        allow_bots: _,
                        infer: _,
                    } => Ok(PCACV::OpUser(None)),
                    _ => panic!("Could not handle {:?}", arg.t),
                }
            };
        };

        match arg.t {
            ArgType::Number { min, max } => {
                let ok = match value.parse::<i32>() {
                    Ok(ok) => ok,
                    Err(e) => {
                        return Err(ArgumentError::Parser(
                            format!("Failed to parse number: {}", e),
                            arg.clone(),
                        ))?
                    }
                };

                if let Some(min) = min {
                    if ok < min {
                        return Err(ArgumentError::InvalidInput(
                            format!("Minimum amount is {}", min),
                            arg.clone(),
                        ))?;
                    }
                }

                if let Some(max) = max {
                    if ok > max {
                        return Err(ArgumentError::InvalidInput(
                            format!("Maximum amount is: {}", max),
                            arg.clone(),
                        ))?;
                    }
                }

                Ok(if required {
                    PCACV::Number(ok)
                } else {
                    PCACV::OpNumber(Some(ok))
                })
            }
            ArgType::Currency {
                ref range,
                allow_negative,
            } => {
                let ok = match value.parse::<i32>() {
                    Ok(ok) => ok,
                    Err(e) => {
                        return Err(ArgumentError::Parser(
                            format!("Failed to parse number: {}", e),
                            arg.clone(),
                        ))?
                    }
                };

                if !allow_negative && ok < 0 {
                    Err(ArgumentError::InvalidInput(
                        "Currency cannot be negative".to_string(),
                        arg.clone(),
                    ))?
                }

                if ok > ctx.economy.balance {
                    Err(ArgumentError::InvalidInput(
                        format!(
                            "You do not have {}, you have {}",
                            currency(ok),
                            currency(ctx.economy.balance)
                        ),
                        arg.clone(),
                    ))?
                }

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
            ArgType::User {
                allow_bots,
                infer: _,
            } => {
                value.retain(|c| c != '<' && c != '>' && c != '@');
                let Ok(id) = value.parse::<u64>() else {
                    return Err(ArgumentError::Parser(
                        format!("Invalid user: {}", value),
                        arg.clone(),
                    ))?;
                };

                let Ok(user) = ctx.sy.http.get_user(UserId::from(id)).await else {
                    return Err(ArgumentError::Parser(
                        format!("Could not fetch user: {}", value),
                        arg.clone(),
                    ))?;
                };

                if !allow_bots && user.bot {
                    return Err(ArgumentError::InvalidInput(
                        "Bots cannot be used here!".to_string(),
                        arg.clone(),
                    ))?;
                }

                Ok(if required {
                    PCACV::User(user.to_owned())
                } else {
                    PCACV::OpUser(Some(user.to_owned()))
                })
            }
            _ => Err(TrancerError::NotImplemented(format!(
                "Cannot handle converting {:?} to PCACV",
                arg.t
            ))),
        }
    }
}

pub fn infer_user(ctx: &TrancerRunnerContext) -> User {
    if let Some(ref msg_ref) = ctx.msg.referenced_message {
        msg_ref.author.clone()
    } else {
        ctx.msg.author.clone()
    }
}

pub async fn map_and_validate<T>(
    args: ParsedArguments,
    arg_schema: TrancerArguments,
    ctx: &TrancerRunnerContext,
) -> Result<Box<T>, TrancerError>
where
    T: CommandArgumentStruct,
{
    let mut arg_map: HashMap<String, PCACV> = HashMap::new();

    for i in 0..arg_schema.args.len() {
        let arg = &arg_schema.args[i];
        let required = arg_schema.required > i;

        let value = {
            let val_opt = if arg.details.wick_style.is_none() {
                args.args.get(i)
            } else {
                args.wick.get(&arg.name)
            };

            match val_opt {
                Some(v) => v.clone(),
                None => {
                    if let ArgType::User {
                        allow_bots: _,
                        infer: true,
                    } = arg.t
                    {
                        infer_user(&ctx).id.to_string()
                    } else if required {
                        let err = if arg.details.wick_style.is_none() {
                            ArgumentError::MissingPositional(arg.name.to_string(), arg.clone())
                        } else {
                            ArgumentError::MissingWick(arg.name.to_string(), arg.clone())
                        };
                        return Err(err)?;
                    } else {
                        arg_map.insert(
                            arg.name.clone(),
                            PCACV::from_arg(&arg, required, None, &ctx).await?,
                        );
                        continue;
                    }
                }
            }
        };

        if let Some(ref must_be) = arg.details.must_be {
            if value != *must_be {
                return Err(ArgumentError::MustBeFailed(must_be.clone(), arg.clone()))?;
            }
        }

        if let Some(ref one_of) = arg.details.one_of {
            if !one_of.contains(&value) {
                return Err(ArgumentError::OneOfFailed(one_of.clone(), arg.clone()))?;
            }
        }

        arg_map.insert(
            arg.name.clone(),
            PCACV::from_arg(&arg, required, Some(value), &ctx).await?,
        );
    }

    T::construct(arg_map)
}

pub fn parse_args(contents: String) -> ParsedArguments {
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

    parsed
}
