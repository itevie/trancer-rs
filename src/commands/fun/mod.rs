mod birthdays;
mod cowsay;
mod eight_ball;
mod math_eval;
mod random_colour;
mod rate;
mod rizz;
mod set_birthday;

use crate::cmd_import_map;

cmd_import_map!(
    rizz,
    rate,
    eight_ball,
    birthdays,
    cowsay,
    set_birthday,
    random_colour,
    math_eval
);
