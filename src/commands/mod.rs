use crate::cmd_util::arg_parser::CommandArgumentStruct;
use crate::cmd_util::{CommandTrait, TrancerCommand};
use once_cell::sync::Lazy;

mod help;

pub fn init() -> Vec<Box<dyn CommandTrait>> {
    let mut commands = vec![];
    commands.extend(help::init());
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
