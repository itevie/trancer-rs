use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::util::embeds::create_embed;
use crate::util::pagination::{paginate, Field, PaginationDataType, PaginationOptions};
use crate::{command_argument_struct, command_file};
use reqwest::StatusCode;
use serde::Deserialize;
use std::collections::HashMap;

static BASE: &'static str = "https://api.dictionaryapi.dev/api/v2/entries/en";

#[derive(Debug, Deserialize, Clone)]
pub struct DictionaryEntry {
    pub word: String,
    pub phonetic: Option<String>,
    pub phonetics: Vec<Phonetic>,
    pub origin: Option<String>,
    pub meanings: Vec<Meaning>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Phonetic {
    pub text: Option<String>,
    pub audio: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Meaning {
    #[serde(rename = "partOfSpeech")]
    pub part_of_speech: String,
    pub definitions: Vec<Definition>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Definition {
    pub definition: String,
    pub example: Option<String>,
    pub synonyms: Vec<String>,
    pub antonyms: Vec<String>,
}

command_argument_struct!(DefineArgs {
    what: String, PCACV::String
});

command_file! {
    TrancerCommand::<DefineArgs> {
        name: "define".to_string(),
        t: TrancerCommandType::Help,
        description: "Get a word's definition!".to_string(),
        details: TrancerDetails {
            arguments: Some(TrancerArguments {
                required: 1,
                args: vec![Argument {
                    name: "what".to_string(),
                    t: ArgType::String { flags: None },
                    details: Default::default(),
                }]
            }),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let response = reqwest::get(format!("{BASE}/{}", args.what).as_str()).await?;

            if response.status() == StatusCode::NOT_FOUND {
                return Ok(TrancerResponseType::Content("Sorry! I couldn't find a definition for that word.".to_string()));
            }

            let binding = response.json::<Vec<DictionaryEntry>>().await?;
            let json = match binding.get(0) {
                Some(ok) => ok.clone(),
                None => {
                    return Ok(TrancerResponseType::Content("Sorry! I couldn't find a definition for that word.".to_string()));
                }
            };

            paginate(PaginationOptions {
                ctx,
                embed: create_embed().title(
                    format!("{} ({})", json.word, json.phonetic.as_ref().unwrap_or(&"/???/".to_string()))
                ),
                page_size: 10,
                data: PaginationDataType::Field(json.meanings.into_iter().flat_map(|m| {
                    m.definitions.iter().map(|d| {
                       Field {
                            name: format!("{} ({})", json.word, m.part_of_speech.clone()),
                            inline: false,
                            value: format!("{}{}", d.clone().definition, if let Some(ref example) = d.example.clone() {
                                format!("\n> Example: {}", example)
                            } else {
                                "".to_string()
                            }),
                        }
                    }).collect::<Vec<_>>()
                }).collect::<Vec<_>>()),
            }).await?;

            Ok(TrancerResponseType::None)
        }),
    }
}
