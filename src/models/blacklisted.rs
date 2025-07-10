use crate::{enum_with_sql, impl_from_row};

enum_with_sql!(BlacklistType {
    Channel = "channel",
    Command = "command"
});

impl_from_row!(
    Blacklisted,
    BlacklistedField {
        r#type: String,
        server_id: String,
        key: String
    }
);
