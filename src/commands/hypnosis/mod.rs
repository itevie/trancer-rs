mod trigger_send;
mod triggers_view;

use crate::cmd_util::CommandTrait;

pub fn init() -> Vec<Box<dyn CommandTrait>> {
    let mut commands = vec![];
    commands.extend(trigger_send::init());
    commands.extend(triggers_view::init());
    commands
}
