use crate::cmd_import_map;

mod add_money;
mod balance;
mod craft;
mod daily;
mod leaderboard_money;
mod pay;
mod remove_money;
mod xp;

cmd_import_map!(
    balance,
    xp,
    daily,
    leaderboard_money,
    pay,
    craft,
    add_money,
    remove_money
);
