use crate::cmd_util::{TrancerError, TrancerResponseType, TrancerRunnerContext};
use crate::database::Database;
use crate::impl_from_row;
use crate::util::cached_usernames::get_cached_username;
use crate::util::db_date::DbDate;
use crate::util::embeds::create_embed;
use crate::util::pagination::{paginate, Field, PaginationDataType, PaginationOptions};
use rusqlite::Error::QueryReturnedNoRows;
use serde::{Deserialize, Serialize};
use serenity::all::{
    Channel, ChannelId, Context, CreateEmbed, CreateEmbedFooter, GuildId, Message, MessageId, User,
    UserId,
};

impl_from_row!(Quote, QuoteField {
   id: u32,
   content: String,
   author_id: String,
   message_id: Option<String>,
   channel_id: Option<String>,
   server_id: String,
   created_at: DbDate,
   created_by: Option<String>,
   last_guessed: DbDate
});

#[derive(Debug, Clone)]
pub struct FetchedQuoteData {
    message: Option<Message>,
    author: User,
    channel: Option<Channel>,
}

#[derive(Debug, Clone)]
pub struct QuoteList(pub Vec<Quote>);

impl Quote {
    pub async fn get(ctx: &Context, id: u32) -> rusqlite::Result<Option<Quote>> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        let result = db.get_one(
            "SELECT * FROM quotes WHERE id = ?1",
            &[&id],
            Quote::from_row,
        );

        match result {
            Ok(ok) => Ok(Some(ok)),
            Err(QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub async fn all(ctx: &Context) -> rusqlite::Result<QuoteList> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        Ok(QuoteList(db.get_many(
            "SELECT * FROM quotes",
            &[],
            Quote::from_row,
        )?))
    }

    pub async fn add(
        ctx: &Context,
        message: &Message,
        created_by: String,
    ) -> rusqlite::Result<Quote> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        db.run(
            "INSERT INTO quotes (content, author_id, server_id, message_id, channel_id, created_by)
             VALUES (?, ?, ?, ?, ?, ?)",
            &[
                &message.content,
                &message.author.id.to_string(),
                &message.guild_id.unwrap().to_string(),
                &message.id.to_string(),
                &message.channel_id.to_string(),
                &created_by,
            ],
        )?;

        // Get last inserted row
        let quote = db.get_one(
            "SELECT * FROM quotes ORDER BY id DESC LIMIT 1",
            &[],
            Quote::from_row,
        )?;

        Ok(quote)
    }

    pub async fn get_from(ctx: &Context, quotee: UserId) -> rusqlite::Result<QuoteList> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        Ok(QuoteList(db.get_many(
            "SELECT * FROM quotes WHERE author_id = ?1",
            &[&quotee.to_string()],
            Quote::from_row,
        )?))
    }

    pub async fn get_by_message(ctx: &Context, msg: MessageId) -> rusqlite::Result<Option<Quote>> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        match db.get_one(
            "SELECT * FROM quotes WHERE message_id = ?1",
            &[&msg.to_string()],
            Quote::from_row,
        ) {
            Ok(ok) => Ok(Some(ok)),
            Err(QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub async fn get_by(ctx: &Context, quoter: UserId) -> rusqlite::Result<QuoteList> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        Ok(QuoteList(db.get_many(
            "SELECT * FROM quotes WHERE created_by = ?1",
            &[&quoter.to_string()],
            Quote::from_row,
        )?))
    }

    pub async fn get_fetched_data(
        &self,
        context: &Context,
        get_message: bool,
    ) -> Result<FetchedQuoteData, TrancerError> {
        let channel = if let Some(id) = self.channel_id.clone() {
            Some(id.parse::<ChannelId>()?.to_channel(&context).await?)
        } else {
            None
        };

        let message = if let (Some(channel), Some(message_id), true) =
            (channel.clone(), self.message_id.clone(), get_message)
        {
            Some(
                context
                    .http
                    .get_message(channel.id(), message_id.parse()?)
                    .await?,
            )
        } else {
            None
        };

        let author = self.author_id.parse::<UserId>()?.to_user(context).await?;

        Ok(FetchedQuoteData {
            message: message.clone(),
            author: author.clone(),
            channel: channel.clone(),
        })
    }

    pub async fn has_similar_quote(
        ctx: &Context,
        content: String,
        server_id: GuildId,
        user_id: UserId,
    ) -> rusqlite::Result<Option<Quote>> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        match db.get_one(
            "SELECT * FROM quotes WHERE LOWER(content) = ?1 AND server_id = ?2 AND author_id = ?3",
            &[&content, &server_id.to_string(), &user_id.to_string()],
            Quote::from_row,
        ) {
            Ok(ok) => Ok(Some(ok)),
            Err(QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn message_link(data: FetchedQuoteData) -> String {
        if let (Some(message), Some(channel)) = (data.message, data.channel) {
            format!(
                "[[Link]](https://discord.com/channels/{}/{})",
                channel.guild().unwrap().id,
                message.id
            )
        } else {
            String::new()
        }
    }

    pub async fn to_embed(&self, context: &Context) -> Result<CreateEmbed, TrancerError> {
        let FetchedQuoteData {
            message,
            author: _,
            channel: _,
        } = self.get_fetched_data(context, true).await?;

        let content = if let Some(ref message) = message {
            message.content.clone()
        } else {
            self.content.clone()
        };
        let _created_at = self.created_at.0.to_rfc3339();

        //TODO: Get message references
        //TODO: Check for embeds/files too
        //TODO: Create footer

        let description = String::from(content);

        let embed = create_embed()
            .title(format!("Quote #{}", self.id))
            .description(description);

        Ok(embed)
    }

    pub async fn gen_embed(&self, ctx: &Context) -> Result<CreateEmbed, TrancerError> {
        let mut embed = create_embed()
            .title("Quote")
            .footer(CreateEmbedFooter::new(format!("Quote #{}", self.id)));

        let mut fallback = || {
            embed = embed.clone().description(if self.content.is_empty() {
                format!("*{}*", self.content)
            } else {
                "".to_string()
            });
        };

        if self.message_id.is_none() && self.channel_id.is_none() {
            fallback();
            return Ok(embed);
        }

        let channel = match ChannelId::new(self.channel_id.clone().unwrap().parse()?)
            .to_channel(ctx)
            .await
        {
            Ok(ok) => ok,
            Err(_) => {
                fallback();
                return Ok(embed);
            }
        };

        let channel = channel
            .guild()
            .ok_or_else(|| serenity::Error::Other("Not a guild channel"))?;

        let mut message = match channel
            .message(
                &ctx.http,
                MessageId::new(self.message_id.clone().unwrap().parse::<u64>()?),
            )
            .await
        {
            Ok(ok) => ok,
            Err(_) => {
                fallback();
                return Ok(embed);
            }
        };

        let mut description = if !message.content.is_empty() {
            message.content
        } else {
            self.content.clone()
        };

        let mut amount = 0;

        while let Some(reference) = &message.referenced_message {
            if amount >= 5 {
                break;
            }

            let ref_msg = reference;

            description.push_str(&format!(
                "\n:arrow_right_hook: {}",
                if ref_msg.content.is_empty() {
                    "".to_string()
                } else {
                    format!("*{}*", ref_msg.content)
                }
            ));

            // if !is_game {
            //     description.push_str(&format!(
            //         " - {}",
            //         make_footer(ref_msg.author.name.clone(), Some(ref_msg), true)?
            //     ));
            // }

            message = (**ref_msg).clone();
            amount += 1;
        }

        Ok(embed.clone())
    }

    // pub async fn generate_embed(
    //     ctx: &Context,
    //     quote: &Quote,
    //     is_game: bool,
    // ) -> Result<CreateEmbed, TrancerError> {
    //     let mut embed = CreateEmbed::new().title("Quote");
    //
    //     // --- Footer ---
    //     embed = embed.footer(CreateEmbedFooter::new(if is_game {
    //         "Use guess (guess) to guess! (NO PREFIX)".to_string()
    //     } else {
    //         format!("Quote #{}", quote.id)
    //     }));
    //
    //     // --- Helper: make footer text ---
    //     let make_footer = |username: String,
    //                        message: Option<&Message>,
    //                        small: bool|
    //                        -> Result<String, TrancerError> {
    //         let mut result = username;
    //
    //         if !small {
    //             let date = message
    //                 .map(|m| m.timestamp.timestamp())
    //                 .unwrap_or_else(|| DateTime::<Utc>::from_naive_utc_and_offset(quote.created_at.0.naive_utc(), Utc).timestamp());
    //
    //             let time = match DateTime::from_timestamp(date, 0) {
    //                 Some(ok) => ok,
    //                 None => return Err(TrancerError::Generic("Failed to get timestamp".to_string()))
    //             };
    //
    //             result.push_str(&format!(" - {}", time.format("%a %b %d %Y")));
    //         }
    //
    //         if let Some(msg) = message {
    //             if let Some(guild_id) = msg.guild_id {
    //                 result.push_str(&format!(
    //                     " - [[Message Link]](https://discord.com/channels/{}/{}/{})",
    //                     guild_id.get(),
    //                     msg.channel_id.get(),
    //                     msg.id.get()
    //                 ));
    //             }
    //         }
    //
    //         Ok(result)
    //     };
    //
    //     // --- Fallback ---
    //     let mut fallback = || -> Result<CreateEmbed, TrancerError> {
    //         let mut desc = String::new();
    //
    //         if !quote.content.is_empty() {
    //             desc.push_str(&format!("*{}*\n", quote.content));
    //         }
    //
    //         desc.push_str(&format!(
    //             " - {}",
    //             make_footer(quote.author_id.to_string(), None, false)?
    //         ));
    //
    //         Ok(embed.clone().description(desc))
    //     };
    //
    //     // --- If no message reference ---
    //     if quote.message_id.is_none() || quote.channel_id.is_none() {
    //         return Ok(fallback()?);
    //     }
    //
    //     // --- Try fetch message ---
    //     let result: Result<CreateEmbed, serenity::Error> = async {
    //         let channel = ChannelId::new(quote.channel_id.unwrap().parse().unwrap())
    //             .to_channel(ctx)
    //             .await?;
    //
    //         let channel = channel.guild().ok_or_else(|| {
    //             serenity::Error::Other("Not a guild channel")
    //         })?;
    //
    //         let description = if let Some(msg_id) = quote.message_id {
    //             let message = channel.message(&ctx.http, MessageId::new(msg_id.parse::<u64>().unwrap())).await?;
    //
    //             if message.content.is_empty() {
    //                 quote.content
    //             } else {
    //                 message.content
    //             }
    //         } else {
    //             quote.content
    //         };
    //
    //         if !is_game {
    //             description.push_str(&format!(
    //                 "\n - {}",
    //                 make_footer(message.author.name.clone(), Some(&message), false)?
    //             ));
    //         }
    //
    //         // --- References chain ---
    //         let mut amount = 0;
    //
    //         while let Some(reference) = &message.referenced_message {
    //             if amount >= 5 {
    //                 break;
    //             }
    //
    //             let ref_msg = reference;
    //
    //             description.push_str(&format!(
    //                 "\n:arrow_right_hook: {}",
    //                 if ref_msg.content.is_empty() {
    //                     "".to_string()
    //                 } else {
    //                     format!("*{}*", ref_msg.content)
    //                 }
    //             ));
    //
    //             if !is_game {
    //                 description.push_str(&format!(
    //                     " - {}",
    //                     make_footer(ref_msg.author.name.clone(), Some(ref_msg), true)?
    //                 ));
    //             }
    //
    //             message = (**ref_msg).clone();
    //             amount += 1;
    //         }
    //
    //         embed = embed.description(description);
    //
    //         // --- Embedded content ---
    //         if let Some(msg_embed) = message.embeds.first() {
    //             embed = embed.field(
    //                 format!("Embed: {}", msg_embed.title.clone().unwrap_or("(No title)".into())),
    //                 msg_embed
    //                     .description
    //                     .clone()
    //                     .unwrap_or("(No description)".into()),
    //                 false,
    //             );
    //
    //             if let Some(thumbnail) = &msg_embed.thumbnail {
    //                 embed = embed.thumbnail(thumbnail.url.clone());
    //             }
    //
    //             if let Some(image) = &msg_embed.image {
    //                 embed = embed.image(image.url.clone());
    //             }
    //
    //             for field in &msg_embed.fields {
    //                 embed = embed.field(&field.name, &field.value, field.inline);
    //             }
    //         }
    //
    //         // --- Attachments ---
    //         if let Some(att) = message.attachments.first() {
    //             if let Some(content_type) = &att.content_type {
    //                 if [
    //                     "image/png",
    //                     "image/jpeg",
    //                     "image/jpg",
    //                     "image/gif",
    //                     "image/gifv",
    //                 ]
    //                     .contains(&content_type.as_str())
    //                 {
    //                     embed = embed.image(att.proxy_url.clone());
    //                 }
    //             }
    //         }
    //
    //         Ok(embed)
    //     }
    //         .await;
    //
    //     // --- Fallback on error ---
    //     match result {
    //         Ok(embed) => Ok(embed),
    //         Err(_) => Ok(fallback()?),
    //     }
    // }
}

pub enum QuoteListPaginationType {
    From(User),
    By(User),
}

impl QuoteList {
    pub async fn paginate(
        &self,
        ctx: TrancerRunnerContext,
        t: QuoteListPaginationType,
    ) -> Result<TrancerResponseType, TrancerError> {
        paginate(PaginationOptions {
            ctx,
            embed: create_embed().title(match t {
                QuoteListPaginationType::From(user) => format!("Quotes from {}", user.name),
                QuoteListPaginationType::By(user) => format!("Quotes by {}", user.name),
            }),
            page_size: 20,
            data: PaginationDataType::Field(
                self.0
                    .iter()
                    .rev()
                    .map(|x| Field {
                        value: format!("*{}*", x.content),
                        name: format!(
                            "{}, (Quote #{})",
                            get_cached_username(x.author_id.clone()),
                            x.id
                        ),
                        inline: false,
                    })
                    .collect(),
            ),
        })
        .await?;

        Ok(TrancerResponseType::None)
    }
}
