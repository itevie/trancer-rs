use crate::cmd_import_map;

mod badges_add;
mod badges_for;
mod badges_remove;

cmd_import_map!(badges_for, badges_add, badges_remove);
