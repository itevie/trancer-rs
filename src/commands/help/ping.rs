use std::sync::Arc;
use crate::command::{TrancerCommand, TrancerResponseType};

pub fn init() -> Vec<TrancerCommand> {
    vec![
        TrancerCommand {
            name: "ping".to_string(),
            description: "This is a test".to_string(),
            aliases: None,
            flags: None,

            handler: Arc::new(|ctx, msg| Box::pin(async move {
                TrancerResponseType::Content("Hi!".to_string())
            }))
        }
    ]
}