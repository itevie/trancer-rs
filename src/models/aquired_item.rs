use crate::impl_from_row;

impl_from_row!(
    AquiredItem,
    AquiredItemField {
        item_id: u32,
        user_id: String,
        amount: u32,
        protected: bool
    }
);
