mod manage_imposition;

use crate::cmd_util::CommandTrait;

pub fn init() -> Vec<Box<dyn CommandTrait>> {
    let mut commands = vec![];
    commands.extend(manage_imposition::init());
    commands
}
