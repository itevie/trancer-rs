use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::util::embeds::create_embed;
use image::{ImageFormat, Rgb, RgbImage};
use rand::Rng;
use serenity::all::{Colour, CreateAttachment, CreateMessage};
use std::io::Cursor;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "randomcolour".to_string(),
        t: TrancerCommandType::Help,
        description: "Get some basic details about the bot!".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["randomcolor".to_string(), "randcolor".to_string(), "randcol".to_string(), "randcolour".to_string()]),
            ..Default::default()
        },

        handler: trancer_handler!(|_ctx, _args| {
            let image_bytes = create_random_square_bytes(256);
            let attachment = CreateAttachment::bytes(image_bytes.0, "square.png");

            Ok(TrancerResponseType::Big(CreateMessage::new().embed(
                create_embed().title("Your random colour!")
                    .colour(Colour::from_rgb(image_bytes.2, image_bytes.3, image_bytes.4))
                    .description(format!("Your random colour is {}", image_bytes.1))
                    .thumbnail("attachment://square.png")
            ).add_file(attachment)))
        }),
    }
}

fn create_random_square_bytes(size: u32) -> (Vec<u8>, String, u8, u8, u8) {
    let mut rng = rand::thread_rng();

    let colour = Rgb([rng.gen::<u8>(), rng.gen::<u8>(), rng.gen::<u8>()]);

    let [r, g, b] = colour.0;

    let hex = format!("#{:02X}{:02X}{:02X}", r, g, b);

    let mut img = RgbImage::new(size, size);

    for pixel in img.pixels_mut() {
        *pixel = colour;
    }

    // Write image to memory buffer
    let mut bytes: Vec<u8> = Vec::new();
    img.write_to(&mut Cursor::new(&mut bytes), ImageFormat::Png)
        .unwrap();

    (bytes, hex, r, g, b)
}
