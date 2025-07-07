use crate::cmd_util::CommandTrait;
use crate::command_argument_struct;
use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use std::collections::HashMap;
use crate::TrancerError;

mod help;
mod hypnosis;

pub fn init() -> Vec<Box<dyn CommandTrait>> {
    let mut commands = vec![];
    commands.extend(help::init());
    commands.extend(hypnosis::init());
    commands
}

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
                .map_err(|x| TrancerError::ReplyError(x)),
        }
    };
}

command_argument_struct!(CommandHasNoArgs {});