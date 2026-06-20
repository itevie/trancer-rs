use crate::cmd_import_map;

mod feed_dawn;
mod get_dawnagotchi;
mod obtain_dawn;
mod play_dawn;
mod revive_dawn;
mod water_dawn;

cmd_import_map!(
    get_dawnagotchi,
    obtain_dawn,
    feed_dawn,
    play_dawn,
    water_dawn,
    revive_dawn
);
