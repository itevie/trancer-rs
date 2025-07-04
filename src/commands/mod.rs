use once_cell::sync::Lazy;
use crate::cmd_util::TrancerCommand;

mod help;

pub static COMMANDS: Lazy<Vec<TrancerCommand>> = Lazy::new(|| {
    init()
});

pub fn init() -> Vec<TrancerCommand> {
    let mut commands = vec![];
    commands.extend(help::init());
    commands
}


#[macro_export]
macro_rules! command_file {
    ($($body:expr),*) => {
        pub fn init() -> Vec<TrancerCommand> {
            vec![
                $(
                    $body
                )*
            ]
        }
    };
}