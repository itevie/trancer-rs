use crate::cmd_import_map;

mod badges_add;
mod badges_defined_list;
mod badges_for;
mod badges_remove;
mod badges_who;

cmd_import_map!(
    badges_for,
    badges_add,
    badges_remove,
    badges_defined_list,
    badges_who
);
