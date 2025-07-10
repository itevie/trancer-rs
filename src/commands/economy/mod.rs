use crate::cmd_util::CommandTrait;

mod balance;
mod xp;

pub fn init() -> Vec<Box<dyn CommandTrait>> {
    let mut commands = vec![];
    commands.extend(balance::init());
    commands.extend(xp::init());
    commands
}
