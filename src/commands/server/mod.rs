use crate::cmd_import_map;

mod count_ruins;
mod current_count;
mod slowmode;
mod verify;

cmd_import_map!(slowmode, verify, current_count, count_ruins);
