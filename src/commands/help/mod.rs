use crate::cmd_import_map;

mod about;
mod avatar;
mod category;
mod command_info;
mod commands_all;
mod commands_new;
mod define;
mod github;
mod help;
mod invite;
mod ping;
mod profile;
mod set_pronoun;

cmd_import_map!(
    about,
    avatar,
    category,
    command_info,
    commands_all,
    commands_new,
    define,
    github,
    help,
    invite,
    ping,
    profile,
    set_pronoun
);
