use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::commands::CommandHasNoArgs;
use crate::{command_argument_struct, command_file};
use rand::Rng;
use std::collections::HashMap;

static PHRASES: &'static [&'static str] = &["Up up up! All the way up!", "*up up up!*"];

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "uppies".to_string(),
        t: TrancerCommandType::Help,
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
