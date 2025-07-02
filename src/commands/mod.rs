use crate::command::TrancerCommand;

mod help;

pub fn init() -> Vec<TrancerCommand> {
    let mut commands = vec![];
    commands.extend(help::init());
    commands
}

