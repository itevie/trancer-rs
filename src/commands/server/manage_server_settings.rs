use crate::cmd_util::arg_parser::{CommandArgumentStruct, PCACV};
use crate::cmd_util::args::{ArgType, Argument, TrancerArguments};
use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{content_response, trancer_handler, TrancerDetails};
use crate::cmd_util::{ArgumentError, TrancerCommand, TrancerError, TrancerResponseType};
use crate::models::server_settings::ServerSettingsFields;
use crate::util::embeds::create_embed;
use crate::{command_argument_struct, command_file};
use rusqlite::ToSql;
use serenity::all::CreateMessage;
use std::collections::HashMap;

pub type SettingValue = Box<dyn ToSql + Send + Sync>;

pub struct SettingDefinition {
    pub field: ServerSettingsFields,
    pub description: &'static str,
    pub valid_values: Option<&'static [&'static str]>,
    pub max_length: Option<u32>,

    pub validator: fn(&str) -> Result<SettingValue, String>,
}

static VALID_BOOLEAN_VALUES: Option<&'static [&'static str]> = Some(&["true", "false"]);
fn validate_boolean(value: &str) -> Result<SettingValue, String> {
    if value == "true" {
        Ok(Box::new(true))
    } else if value == "false" {
        Ok(Box::new(false))
    } else {
        Err("Must be true or false".into())
    }
}

pub fn settings_registry() -> HashMap<&'static str, SettingDefinition> {
    HashMap::from([
        (
            "prefix",
            SettingDefinition {
                field: ServerSettingsFields::prefix,

                description: "The bot prefix",
                valid_values: None,
                max_length: Some(3),

                validator: |value| Ok(Box::new(value.to_string())),
            },
        ),
        (
            "random_replies",
            SettingDefinition {
                field: ServerSettingsFields::prefix,

                description: "Whether or not Trancer should send random messages every so often",
                valid_values: VALID_BOOLEAN_VALUES,
                max_length: None,

                validator: validate_boolean,
            },
        ),
        (
            "react_bot",
            SettingDefinition {
                field: ServerSettingsFields::prefix,

                description: "Should Trancer respond to things when you @ it?",
                valid_values: VALID_BOOLEAN_VALUES,
                max_length: None,

                validator: validate_boolean,
            },
        ),
        (
            "streak_reactions",
            SettingDefinition {
                field: ServerSettingsFields::prefix,

                description:
                    "Should Trancer react with the :fire: emoji when a user's streak has increased?",
                valid_values: VALID_BOOLEAN_VALUES,
                max_length: None,

                validator: validate_boolean,
            },
        ),
        (
            "streak_end_reactions",
            SettingDefinition {
                field: ServerSettingsFields::prefix,

                description: "Should Trancer send a message when a user's streak has been reset?",
                valid_values: VALID_BOOLEAN_VALUES,
                max_length: None,

                validator: validate_boolean,
            },
        ),
    ])
}

command_argument_struct!(ManageServerSettingsArgs {
    key: Option<String>, PCACV::OpString,
    value: Option<String>, PCACV::OpString
});

command_file! {
    TrancerCommand::<ManageServerSettingsArgs> {
        name: "set".to_string(),
        t: TrancerCommandType::Help,
        description: "Manage Server Settings".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["manageserversetting".to_string(), "setsetting".to_string(), "setting".to_string()]),
            arguments: Some(TrancerArguments {
                required: 0,
                args: vec![
                    Argument {
                        t: ArgType::String { flags: None },
                        name: "key".to_string(),
                        details: Default::default()
                    },

                    Argument {
                        t: ArgType::String { flags: None },
                        name: "value".to_string(),
                        details: Default::default()
                    }
                ]
            }),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let Some(key) = args.key else {
                return Ok(TrancerResponseType::Big(
                    CreateMessage::new()
                        .embed(
                            create_embed()
                                .title("List of settings")
                        )
                ));
            };

            let settings = settings_registry();

            if !settings.contains_key(&key.as_str()) {
                return Err(TrancerError::NonScary("That is not a valid setting!".to_string()));
            }

            let setting = settings.get(&key.as_str()).unwrap().clone();

            let Some(value) = args.value else {
                return Ok(TrancerResponseType::Big(
                    CreateMessage::new()
                        .embed(
                            create_embed()
                                .title("Settings details")
                        )
                ));
            };

            if let Some(valid_values) = setting.valid_values {
                if !valid_values.contains(&value.as_str()) {
                    return Err(TrancerError::NonScary(
                        format!("That is not a valid value for this setting!\nValid values: {}", valid_values.join(", "))
                    ));
                }
            }

            if let Some(max_length) = setting.max_length {
                if value.len() > max_length as usize {
                    return Err(TrancerError::NonScary(
                        format!("The length of the value can only be {}!", max_length)
                    ));
                }
            }

            let result = match (setting.validator)(&value) {
                Ok(ok) => ok,
                Err(err) => {
                     return Err(TrancerError::NonScary(
                        format!("The validator for this setting failed!\nError: {}", err)
                    ));
                }
            };


            ctx.server_settings.update_key(&ctx.sy, setting.field, result).await?;

            Ok(content_response("Setting updated! :cyclone:".to_string()))
        }),
    }
}
