use crate::cmd_import_map;

mod command_info;
mod profile;
mod ping;
mod commands_new;
mod commands_all;
mod invite;
mod avatar;

cmd_import_map!(profile, command_info, ping, commands_new, commands_all, invite, avatar);