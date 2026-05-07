use crate::cmd_import_map;

mod count_ruins;
mod current_count;
mod manage_server_settings;
mod slowmode;
mod verify;

cmd_import_map!(
    slowmode,
    verify,
    current_count,
    count_ruins,
    manage_server_settings
);
