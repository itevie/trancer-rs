use crate::command::TrancerCommand;

mod ping;

pub fn init() -> Vec<TrancerCommand> {
    let mut commands = vec![];
    commands.extend(ping::init());
    commands
}
