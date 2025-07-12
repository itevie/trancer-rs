mod birthdays;
mod cowsay;
mod eight_ball;
mod rate;
mod rizz;

use crate::cmd_import_map;

cmd_import_map!(rizz, rate, eight_ball, birthdays, cowsay);
