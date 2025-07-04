use std::collections::HashMap;
use crate::cmd_util::TrancerError;

pub struct ParsedArguments {
    pub args: HashMap<String, String>,
    pub wick: HashMap<String, String>,
    pub original: Vec<String>,
    pub original_content: String,
}

static WICK_CHAR: &'static str = "?";

pub fn parse_args(contents: String) -> Result<ParsedArguments, TrancerError> {
    let mut parsed = ParsedArguments {
        args: HashMap::new(),
        wick: HashMap::new(),
        original: Vec::new(),
        original_content: contents.clone(),
    };

    Ok(parsed)
}