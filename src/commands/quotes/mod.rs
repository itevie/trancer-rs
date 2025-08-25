mod quote_get;
mod quoted_by;
mod quotes_from;
mod quotes_leaderboard;

use crate::cmd_import_map;

cmd_import_map!(quote_get, quotes_from, quoted_by, quotes_leaderboard);
