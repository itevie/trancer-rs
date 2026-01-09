use crate::cmd_import_map;

mod swears_for;
mod total_swears;
mod word_leaderboard;

cmd_import_map!(word_leaderboard, total_swears, swears_for);
