use crate::cmd_util::TrancerCommand;

mod ping;
mod command_info;

pub fn init() -> Vec<TrancerCommand> {
    let mut commands = vec![];
    commands.extend(ping::init());
    commands.extend(command_info::init());
    commands
}
