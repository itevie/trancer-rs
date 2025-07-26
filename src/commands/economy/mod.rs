use crate::cmd_import_map;

mod balance;
mod daily;
mod leaderboard_money;
mod pay;
mod xp;

cmd_import_map!(balance, xp, daily, leaderboard_money, pay);
