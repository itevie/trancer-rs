use crate::impl_from_row;

impl_from_row!(
    AquiredBadge,
    AquiredBadgeField {
        user: String,
        badge_name: String,
        aquired_at: String,
    }
);
