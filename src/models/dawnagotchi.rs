use crate::impl_from_row;

impl_from_row!(Dawnagotchi, DawnagotchiField {
    id: u32,
    owner_id: String,
    created_at: String,

    hair_color_hex: String,

    alive: bool,

    next_feed: String,
    next_drink: String,
    next_play: String,

    acc_face: Option<u32>,
    acc_hair: Option<u32>,
});
