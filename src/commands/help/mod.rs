use crate::cmd_util::CommandTrait;

mod command_info;
mod profile;
mod ping;
mod commands_new;
mod commands_all;

pub fn init() -> Vec<Box<dyn CommandTrait>> {
    let mut commands = vec![];
    commands.extend(profile::init());
    commands.extend(command_info::init());
    commands.extend(ping::init());
    commands.extend(commands_new::init());
    commands.extend(commands_all::init());
    commands
}


