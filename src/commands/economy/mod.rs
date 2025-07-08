use crate::cmd_util::CommandTrait;

mod balance;

pub fn init() -> Vec<Box<dyn CommandTrait>> {
    let mut commands = vec![];
    commands.extend(balance::init());
    commands
}
