use crate::cmd_util::TrancerError;
use crate::database::Database;
use crate::impl_from_row;
use crate::util::config::CONFIG;
use chrono::{DateTime, Utc};
use image::ImageFormat;
use rusqlite::ToSql;
use serenity::all::{Context, GuildId, UserId};
use std::io::Cursor;

impl_from_row!(Dawnagotchi, DawnagotchiField {
    id: u32,
    owner_id: String,
    created_at: String,

    hair_color_hex: String,

    alive: bool,

    next_feed: i64,
    next_drink: i64,
    next_play: i64,

    acc_face: Option<u32>,
    acc_hair: Option<u32>,
});

pub struct DawnRequirements {
    pub feed: i32,
    pub drink: i32,
    pub play: i32,
}

impl Dawnagotchi {
    pub async fn fetch(ctx: &Context, user_id: UserId) -> rusqlite::Result<Self> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        let result = db.get_one(
            "SELECT * FROM dawnagotchi WHERE owner_id = ?1 AND ALIVE = true LIMIT 1",
            &[&user_id.to_string()],
            Dawnagotchi::from_row,
        );

        match result {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn create(ctx: &Context, user_id: UserId) -> rusqlite::Result<Self> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        let result = db.get_one(
            "INSERT INTO dawnagotchi (owner_id, next_feed, next_drink, next_play) VALUES (?1, ?2, ?2, ?2) RETURNING *;",
            &[&user_id.to_string(), &Utc::now().timestamp_millis()],
            Dawnagotchi::from_row,
        );

        match result {
            Ok(result) => Ok(result),
            Err(e) => Err(e),
        }
    }

    pub async fn feed(&self, ctx: &Context) -> Result<(), TrancerError> {
        let requirements = self.get_requirements();

        if requirements.feed >= 100 {
            return Err(TrancerError::NonScary(
                "Your Dawn is not hungry!".to_string(),
            ));
        }

        let new_time = self.next_feed + CONFIG.dawnagotchi.feed_time_add;

        self.update_key(ctx, DawnagotchiField::next_feed, new_time)
            .await?;

        Ok(())
    }

    pub async fn water(&self, ctx: &Context) -> Result<(), TrancerError> {
        let requirements = self.get_requirements();

        if requirements.drink >= 100 {
            return Err(TrancerError::NonScary(
                "Your Dawn is not thirsty!".to_string(),
            ));
        }

        let new_time = self.next_drink + CONFIG.dawnagotchi.drink_time_add;

        self.update_key(ctx, DawnagotchiField::next_drink, new_time)
            .await?;

        Ok(())
    }

    pub async fn play(&self, ctx: &Context) -> Result<(), TrancerError> {
        let requirements = self.get_requirements();

        if requirements.play >= 100 {
            return Err(TrancerError::NonScary(
                "Your Dawn isn't feeling whimsicle!".to_string(),
            ));
        }

        let new_time = self.next_play + CONFIG.dawnagotchi.play_time_add;

        self.update_key(ctx, DawnagotchiField::next_play, new_time)
            .await?;

        Ok(())
    }

    pub async fn update_key<T>(
        &self,
        ctx: &Context,
        key: DawnagotchiField,
        value: T,
    ) -> rusqlite::Result<()>
    where
        T: ToSql + Send + Sync + 'static,
    {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();
        let sql = format!(
            "UPDATE dawnagotchi SET {} = ?1 WHERE owner_id = ?2 AND alive = true",
            key.as_str()
        );

        db.run(&sql, &[&value, &self.owner_id])?;
        Ok(())
    }

    pub fn get_requirements(&self) -> DawnRequirements {
        DawnRequirements {
            feed: calculate_requirement_from_date(self.next_feed),
            drink: calculate_requirement_from_date(self.next_drink),
            play: calculate_requirement_from_date(self.next_play),
        }
    }

    pub fn make_dawn_image(&self) -> Vec<u8> {
        let mut base = image::open("src/images/dawn/base_dawn.png")
            .unwrap()
            .to_rgba8();

        replace_ff00ed(&mut base, hex_to_rgb(&self.hair_color_hex));

        let requirements = self.get_requirements();

        if requirements.feed < 25 {
            overlay_image(&mut base, "src/images/dawn/acc_food.png");
        }

        if requirements.drink < 25 {
            overlay_image(&mut base, "src/images/dawn/acc_water.png");
        }

        if requirements.play < 25 {
            overlay_image(&mut base, "src/images/dawn/acc_play.png");
        }

        let mut bytes: Vec<u8> = Vec::new();
        base.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
            .unwrap();

        bytes
    }
}

pub fn calculate_requirement_from_date(expected: i64) -> i32 {
    let now = Utc::now().timestamp_millis();

    let hours_until_expected = (expected - now) as f64 / 3_600_000.0;

    let percentage = (50.0 + (hours_until_expected * 50.0) / 24.0).round();
    percentage.clamp(0.0, 100.0) as i32
}

use image::imageops::overlay;
use image::{Rgba, RgbaImage};

pub fn replace_ff00ed(img: &mut RgbaImage, to: [u8; 3]) {
    let from = [255, 0, 237];

    for pixel in img.pixels_mut() {
        let [r, g, b, a] = pixel.0;

        if [r, g, b] == from {
            *pixel = Rgba([to[0], to[1], to[2], a]);
        }
    }
}

pub fn hex_to_rgb(hex: &str) -> [u8; 3] {
    let hex = hex.trim_start_matches('#');

    let r = u8::from_str_radix(&hex[0..2], 16).unwrap();
    let g = u8::from_str_radix(&hex[2..4], 16).unwrap();
    let b = u8::from_str_radix(&hex[4..6], 16).unwrap();

    [r, g, b]
}

pub fn overlay_image(base: &mut RgbaImage, path: &str) {
    let overlay_img = image::open(path)
        .expect("Failed to open overlay image")
        .to_rgba8();

    // Equivalent to SRC_OVER with full opacity
    overlay(base, &overlay_img, 0, 0);
}
