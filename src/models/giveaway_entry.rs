use crate::impl_from_row;
use serde::{Deserialize, Serialize};

impl_from_row!(
    GiveawayEntry,
    GiveawayEntryField {
        giveaway_id: String,
        author_id: String
    }
);
