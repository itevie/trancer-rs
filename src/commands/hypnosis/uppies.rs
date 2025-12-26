use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use rand::Rng;

static PHRASES: &[&str] = &["Up up up! All the way up!", "*up up up!*"];

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "uppies".to_string(),
        t: TrancerCommandType::Hypnosis,
        description: "Up up up! All the way up!".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["up".to_string()]),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let mut rng = rand::thread_rng();
            let item = PHRASES.get(rng.gen_range(0..PHRASES.len())).unwrap();
            Ok(TrancerResponseType::Content(item.to_string()))
        }),
    }
}
