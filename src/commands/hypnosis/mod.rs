mod trigger_send;
mod triggers_add;
mod triggers_view;

use crate::cmd_import_map;

cmd_import_map!(trigger_send, triggers_view, triggers_add);
