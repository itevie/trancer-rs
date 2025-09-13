use crate::{enum_with_sql, impl_from_row};

enum_with_sql!(Rarity {
    Common = "common",
    Uncommon = "uncommon",
    Rare = "rare",
    Epic = "epic",
    Mythic = "mythic"
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

// Ideas for cards
// - Remove rarity from cards
// - Each card has the set of rarities
// - Make aquired_cards have a rarity
// - So like, legendary dawn, common dawn etc.
// - For the existing aquired cards, give them the same rarity that card is but as aquired_cards
//   So like, if they have 2 Dawns, they'd get 2 mythic dawns
// - They could be listed like Dawn [2x common 1x rare], but with emojis
// - Also store the image files in the cloudflare bucket
// - People that are level 10 or whatever can make their own card without asking
