use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{content_response, trancer_handler, TrancerDetails};
use crate::cmd_util::{TrancerCommand, TrancerError};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::models::spiral::{Spiral, SpiralFiends};
use crate::util::config::CONFIG;
use crate::util::r3::{create_r2_client, upload_to_r2};
use std::env;
use std::path::Path;
use tokio::io::AsyncWriteExt;

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "addspiral".to_string(),
        t: TrancerCommandType::Help,
        description: "This is a test".to_string(),
        details: TrancerDetails {
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, _args| {
            let Some(reference) = ctx.msg.referenced_message else {
                return Err(TrancerError::NonScary("You need to reply to a message with a spiral!".to_string()))
            };

            let mut links: Vec<String> = vec![];

            // Check for attachment
            if reference.attachments.len() != 0 {
                for attachment in &reference.attachments {
                    if attachment.content_type.clone().unwrap_or("no".to_string()) != "image/gif" {
                        continue;
                    }

                    links.push(attachment.proxy_url.clone());
                }
            }

            // Check for explicit link
            if reference.content.starts_with("https://") {
                links.push(reference.content.clone());
            }

            if links.len() == 0 {
                return Err(TrancerError::NonScary("I couldn't get any spirals from that message!\nTry sending a link to one, or attaching them as a file! :cyclone:".to_string()))
            }

            let mut errors: Vec<String> = vec![];

            for link in links {
                if let Some(_) = Spiral::get_by_link(&ctx.sy, &link).await? {
                    errors.push(format!("<{}> has already been added!", link));
                    continue;
                }

                let path = match download_image(&link, &*(CONFIG.general.data_dir.clone() + "/temp_spirals")).await {
                    Ok(ok) => ok,
                    Err(err) => {
                        errors.push(format!("Failed to download <{}>: `{}`", link, err.to_string()));
                        continue;
                    }
                };

                let spiral = Spiral::add(&ctx.sy, link.clone(), reference.author.id, path.clone()).await?;

                if let Ok(_) = env::var("S3_ENDPOINT") {
                    // Upload to S3 bucket
                    let key = format!("spirals/{}.gif", spiral.id);
                    let client = create_r2_client();
                    match upload_to_r2(&client, "trancer", &key, &path).await {
                        Ok(_) => {},
                        Err(err) => {
                            errors.push(format!("Failed to upload to S3 <{}>: `{}`", link.clone(), err.to_string()));
                            continue;
                        }
                    }

                    spiral.update_key(&ctx.sy, SpiralFiends::link, format!("https://trancerspirals.dawn.rest/{}", key)).await?;
                } else {
                    println!("S3 is null");
                }
            }

            if errors.len() > 0 {
                return Err(TrancerError::NonScary(format!("The following errors happened:\n{}", errors.join("\n"))));
            }

            // TODO: Add helping money

            Ok(content_response("Spirals added! Thanks :cyclone:"))
        }),
    }
}

pub async fn download_image(url: &str, folder: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = &normalize_tenor(&normalize_imgur(url));
    let response = reqwest::get(url).await?;

    if !response.status().is_success() {
        return Err(format!("Failed to download: {}", url).into());
    }

    let headers = response.headers().clone();
    let bytes = response.bytes().await?;

    // Try to get extension from URL
    let mut ext = url
        .split('?')
        .next()
        .and_then(|u| u.split('.').last())
        .unwrap_or("")
        .to_string();

    // Fallback to content-type
    if ext.is_empty() || ext.len() > 5 {
        if let Some(content_type) = headers.get("content-type") {
            let ct = content_type.to_str().unwrap_or("");
            ext = match ct {
                "image/gif" => "gif",
                "image/png" => "png",
                "image/jpeg" => "jpg",
                "image/webp" => "webp",
                _ => "dat",
            }
            .to_string();
        }
    }

    // Generate filename (you probably want something better later)
    let filename = format!("{:x}.{}", md5::compute(url), ext);
    let path = Path::new(folder).join(&filename);

    // Ensure folder exists
    tokio::fs::create_dir_all(folder).await?;

    let mut file = tokio::fs::File::create(&path).await?;
    file.write_all(&bytes).await?;

    Ok(path.to_string_lossy().to_string())
}

fn normalize_imgur(url: &str) -> String {
    if url.contains("imgur.com") && !url.contains("i.imgur.com") {
        let id = url.split('/').last().unwrap_or("");
        return format!("https://i.imgur.com/{}.gif", id);
    }
    url.to_string()
}

fn normalize_tenor(url: &str) -> String {
    // Already direct media link
    if url.contains("media.tenor.com") {
        return url.to_string();
    }

    // Handle tenor.com/view/...-ID
    if url.contains("tenor.com") {
        if let Some(id) = url.split('-').last() {
            // Strip query params if any
            let id = id.split('?').next().unwrap_or(id);
            return format!("https://media.tenor.com/{}/tenor.gif", id);
        }
    }

    url.to_string()
}
