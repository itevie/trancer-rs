use crate::cmd_util::arg_parser::CommandArgumentStruct;
use crate::cmd_util::{CommandTrait};

mod command_info;
mod ping;

pub fn init() -> Vec<Box<dyn CommandTrait>> {
    let mut commands = vec![];
    commands.extend(ping::init());
    commands.extend(command_info::init());
    commands
}
