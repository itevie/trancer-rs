// use crate::commands::CommandTrait;
// use serenity::all::EditMessage;
// use serenity::all::CreateButton;
// use serenity::all::CreateActionRow;
// use serenity::all::ButtonStyle;
// use std::time::Duration;
// use crate::cmd_util::{content_response, trancer_handler, TrancerDetails, TrancerResponseType};
// use crate::cmd_util::{TrancerCommand, TrancerError};
// use crate::models::quote::Quote;
// use crate::{command_file, confirm_action, reply};
// use serenity::all::{Message, ChannelId};
// use serenity::builder::{CreateMessage};
// use std::collections::HashMap;
// use rusqlite::fallible_iterator::FallibleIterator;
// use crate::cmd_util::types::TrancerCommandType;
// use crate::commands::error;
//
// command_file! {
//     TrancerCommand::<()> {
//         name: "quote".to_string(),
//         t: TrancerCommandType::Quotes,
//         description: "Reply to a funny message and it will be saved!".to_string(),
//         details: TrancerDetails {
//             ..Default::default()
//         },
//
//         handler: trancer_handler!(|ctx, _args| {
//             let msg = &ctx.msg;
//
//             // --- Check reference ---
//             let reference = match &msg.message_reference {
//                 Some(r) => r,
//                 None => {
//                     return Err(TrancerError::NonScary(
//                         "Please reply to a message!".to_string()
//                     ));
//                 }
//             };
//
//             let ref_msg = msg.channel_id
//                 .message(&ctx.sy, reference.message_id.unwrap())
//                 .await?;
//
//             // --- Prevent quoting bot outputs ---
//             if ref_msg.author.id == ctx.sy.cache.current_user().id {
//                 if ref_msg.content.starts_with("According to")
//                     || ref_msg.content.starts_with("amazing rizz")
//                 {
//                     return Err(TrancerError::NonScary(
//                         "Nuh uh! Can't quote the results of those commands".to_string()
//                     ));
//                 }
//             }
//
//             // --- Prevent self quote ---
//             if ref_msg.author.id == msg.author.id {
//                 return Err(TrancerError::NonScary(
//                     "You cannot quote yourself! :cyclone:".to_string()
//                 ));
//             }
//
//             // --- Check if already quoted ---
//             if let Some(existing) = Quote::get_by_message(&ctx.sy, ref_msg.id).await? {
//                 return Err(TrancerError::NonScary(
//                     format!(
//                         "Sadly, that quote has already been quoted! :cyclone: (id: #{})",
//                         existing.id
//                     )
//                 ));
//             }
//
//             // --- Check similar ---
//             let similar = Quote::has_similar_quote(
//                 &ctx.sy,
//                 ref_msg.content.to_lowercase(),
//                 msg.guild_id.unwrap(),
//                 ref_msg.author.id
//             ).await?;
//
//             // --- Confirm (pseudo helper, same idea as ConfirmAction) ---
//             let confirmed = if let Some(similar) = similar {
//                 let embed = Quote::generate_embed(&ctx.sy, &similar, false)
//                     .await?
//                     .title("That quote is too similar to this quote! Is it worth it?");
//
//                 let (confirmed, _) = confirm_action!(
//                     ctx,
//                     CreateMessage::new().embed(embed)
//                 );
//
//                 confirmed
//             } else {
//                 true // autoYes
//             };
//
//             if !confirmed {
//                 return Ok(TrancerResponseType::None);
//             }
//
//             // --- Insert quote ---
//             let quote = Quote::add(&ctx.sy, &ref_msg, msg.author.id).await?;
//
//             let embed = Quote::generate_embed(&quote).await?;
//
//             // --- Decide message type ---
//             // let send_options = if ref_msg.attachments.is_empty()
//             //     && !ref_msg.content.is_empty()
//             //     && ref_msg.content.len() < 50
//             // {
//             //     let image = Quote::generate_quote_image(&quote).await?;
//             //
//             //     CreateMessage::new()
//             //         .add_file(image)
//             // } else {
//                 CreateMessage::new()
//                     .embed(embed)
//             // };
//
//             // --- Send to quotes channel if configured ---
//             if let Some(channel_id) = ctx.server_settings.quotes_channel_id {
//                 let channel = ChannelId::new(channel_id.parse().unwrap());
//
//                 let _ = channel.send_message(&ctx.sy, send_options.clone()).await;
//             }
//
//             // --- Return message ---
//             ctx.msg.channel_id
//                 .send_message(&ctx.serenity, send_options)
//                 .await?;
//
//             Ok(())
//         }),
//     }
// }
