use crate::cmd_util::CommandTrait;

mod command_info;
mod profile;
mod ping;
mod new_commands;
mod all_commands;

pub fn init() -> Vec<Box<dyn CommandTrait>> {
    let mut commands = vec![];
    commands.extend(profile::init());
    commands.extend(command_info::init());
    commands.extend(ping::init());
    commands.extend(new_commands::init());
    commands.extend(all_commands::init());
    commands
}


