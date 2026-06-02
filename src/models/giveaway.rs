use crate::impl_from_row;
use serde::{Deserialize, Serialize};

impl_from_row!(Giveaway, GiveawayField {
    id: String,
    what: String,
    message_id: String,
    channel_id: String,
    author_id: String,
    min_level: Option<u32>
});
