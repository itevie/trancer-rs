use crate::cmd_util::{TrancerError, TrancerResponseType, TrancerRunnerContext};
use crate::util::embeds::create_embed;
use crate::util::pagination::{paginate, Field, PaginationDataType, PaginationOptions};
use reqwest::StatusCode;
use serde::Deserialize;

static BASE: &str = "https://api.dictionaryapi.dev/api/v2/entries/en";

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

pub async fn handle_define_message(
    ctx: &TrancerRunnerContext,
    what: String,
) -> Result<TrancerResponseType, TrancerError> {
    let response = reqwest::get(format!("{BASE}/{}", what).as_str()).await?;

    if response.status() == StatusCode::NOT_FOUND {
        return Ok(TrancerResponseType::Content(
            "Sorry! I couldn't find a definition for that word.".to_string(),
        ));
    }

    let binding = response.json::<Vec<DictionaryEntry>>().await?;
    let json = match binding.first() {
        Some(ok) => ok.clone(),
        None => {
            return Ok(TrancerResponseType::Content(
                "Sorry! I couldn't find a definition for that word.".to_string(),
            ));
        }
    };

    paginate(PaginationOptions {
        ctx: ctx.clone(),
        embed: create_embed().title(format!(
            "{} ({})",
            json.word,
            json.phonetic.as_ref().unwrap_or(&"/???/".to_string())
        )),
        page_size: 10,
        data: PaginationDataType::Field(
            json.meanings
                .into_iter()
                .flat_map(|m| {
                    m.definitions
                        .iter()
                        .map(|d| Field {
                            name: format!("{} ({})", json.word, m.part_of_speech.clone()),
                            inline: false,
                            value: format!(
                                "{}{}",
                                d.clone().definition,
                                if let Some(ref example) = d.example.clone() {
                                    format!("\n> Example: {}", example)
                                } else {
                                    "".to_string()
                                }
                            ),
                        })
                        .collect::<Vec<_>>()
                })
                .collect::<Vec<_>>(),
        ),
    })
    .await?;

    Ok(TrancerResponseType::None)
}
