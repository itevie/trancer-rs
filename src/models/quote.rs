use crate::cmd_util::{TrancerError, TrancerResponseType, TrancerRunnerContext};
use crate::database::Database;
use crate::impl_from_row;
use crate::util::cached_usernames::get_cached_username;
use crate::util::db_date::DbDate;
use crate::util::embeds::create_embed;
use crate::util::pagination::{paginate, Field, PaginationDataType, PaginationOptions};
use rusqlite::Error::QueryReturnedNoRows;
use serenity::all::{Channel, ChannelId, Context, CreateEmbed, Message, User, UserId};

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

    pub async fn get_from(ctx: &Context, quotee: UserId) -> rusqlite::Result<QuoteList> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        Ok(QuoteList(db.get_many(
            "SELECT * FROM quotes WHERE author_id = ?1",
            &[&quotee.to_string()],
            Quote::from_row,
        )?))
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
            author,
            channel,
        } = self.get_fetched_data(context, true).await?;

        let content = if let Some(ref message) = message {
            message.content.clone()
        } else {
            self.content.clone()
        };
        let created_at = self.created_at.0.to_rfc3339();

        //TODO: Get message references
        //TODO: Check for embeds/files too
        //TODO: Create footer

        let description = String::from(content);

        let mut embed = create_embed()
            .title(format!("Quote #{}", self.id))
            .description(description);

        Ok(embed)
    }
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
