use crate::impl_from_row;

impl_from_row!(
    GiveawayEntry,
    GiveawayEntryField {
        giveaway_id: String,
        author_id: String
    }
);
