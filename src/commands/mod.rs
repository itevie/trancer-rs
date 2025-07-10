use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, ArgumentDetails, TrancerArguments};
use crate::cmd_util::{ArgumentError, TrancerError};
use crate::command_argument_struct;
use serenity::all::User;
use std::collections::HashMap;

mod economy;
mod fun;
mod help;
mod hypnosis;
mod server;

#[macro_export]
macro_rules! cmd_import_map {
    ($($idents:tt),*) => {
        use crate::cmd_util::CommandTrait;
        pub fn init() -> Vec<Box<dyn CommandTrait>> {
            let mut commands = vec![];
            $(commands.extend($idents::init());)*
            commands
        }
    };
}

cmd_import_map!(help, hypnosis, economy, server, fun);

#[macro_export]
macro_rules! command_file {
    ($($body:expr),*) => {
        pub fn init() -> Vec<Box<dyn CommandTrait>> {
            vec![
                $(
                    Box::from($body),
                ),*
            ]
        }
    };
}

#[macro_export]
macro_rules! reply {
    ($ctx:expr, $body:expr) => {
        match $ctx
            .msg
            .channel_id
            .send_message(&$ctx.sy.http, $body.reference_message(&$ctx.msg))
            .await
        {
            Ok(ok) => Ok(ok),
            Err(_) => $ctx
                .msg
                .channel_id
                .send_message(&$ctx.sy.http, $body)
                .await
                .map_err(|x| {
                    error!("Failed to send message: {}", x);
                    TrancerError::ReplyError(x)
                }),
        }
    };
}

command_argument_struct!(CommandHasNoArgs {});
command_argument_struct!(OnlyUserArgs {
    user: User, PCACV::User
});

pub fn only_user_args(allow_bots: bool, infer: bool) -> TrancerArguments {
    TrancerArguments {
        required: 1,
        args: vec![Argument {
            name: "user".to_string(),
            t: ArgType::User { allow_bots, infer },
            details: ArgumentDetails::default(),
        }],
    }
}
