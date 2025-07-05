use serenity::all::{CreateEmbed, Timestamp};

pub fn create_embed() -> CreateEmbed {
    base_embed()
        .timestamp(Timestamp::now())
}

pub fn base_embed() -> CreateEmbed {
    CreateEmbed::new()
        .colour((255, 0, 0))
}
