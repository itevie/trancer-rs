use crate::cmd_import_map;

mod slowmode;
mod verify;

cmd_import_map!(slowmode, verify);
