use crate::cmd_import_map;

mod add_favourite_spiral;
mod favourite_spiral;
mod remove_favourite_spiral;
mod spiral;

cmd_import_map!(
    spiral,
    add_favourite_spiral,
    remove_favourite_spiral,
    favourite_spiral
);
