use crate::cmd_import_map;

mod about;
mod avatar;
mod category;
mod command_info;
mod commands_all;
mod commands_new;
mod define;
mod github;
mod invite;
mod ping;
mod profile;

cmd_import_map!(
    profile,
    command_info,
    ping,
    commands_new,
    commands_all,
    invite,
    avatar,
    define,
    category,
    github,
    about
);
