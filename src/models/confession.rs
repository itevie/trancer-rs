use crate::impl_from_row;

impl_from_row!(Confession, ConfessionField {
    id: u32,
    user_id: String,
    channel_id: String,
    message_id: String,
    content: String,
    created_at: String
});