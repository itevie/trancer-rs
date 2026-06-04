use crate::cmd_import_map;

mod add_money;
mod balance;
mod buy;
mod craft;
mod daily;
mod fish;
mod leaderboard_money;
mod list_jobs;
mod mine;
mod missions;
mod pay;
mod recipies;
mod remove_money;
mod rigged_coinflip;
mod select_job;
mod shop;
mod work;
mod xp;

cmd_import_map!(
    balance,
    xp,
    daily,
    leaderboard_money,
    pay,
    craft,
    add_money,
    remove_money,
    fish,
    select_job,
    list_jobs,
    shop,
    recipies,
    work,
    rigged_coinflip,
    missions,
    buy,
    mine
);
