use crate::{enum_with_sql, impl_from_row};

enum_with_sql!(Rarity {
    Common = "common",
    Uncommon = "uncommon",
    Rare = "rare",
    Epic = "epic",
    Legendary = "legendary"
});

impl_from_row!(Card, CardField {
    id: u32,
    deck: u32,
    description: Option<String>,
    name: String,
    link: String,
    file_name: String,
    rarity: String,
    created_at: String
});
