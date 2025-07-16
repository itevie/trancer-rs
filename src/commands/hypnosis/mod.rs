mod trigger_send;
mod triggers_add;
mod triggers_remove;
mod triggers_view;
mod uppies;

use crate::cmd_import_map;

cmd_import_map!(
    trigger_send,
    triggers_view,
    triggers_add,
    triggers_remove,
    uppies
);
