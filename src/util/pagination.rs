use crate::cmd_util::{TrancerError, TrancerRunnerContext};
use crate::reply;
use serenity::all::{
    ButtonStyle, CreateActionRow, CreateButton, CreateEmbedFooter, CreateMessage, EditMessage,
};
use serenity::builder::CreateEmbed;
use serenity::futures::StreamExt;
use std::time::Duration;
use tracing::{error};

pub struct PaginationOptions {
    pub ctx: TrancerRunnerContext,
    pub embed: CreateEmbed,
    pub page_size: usize,
    pub data: PaginationDataType,
}

pub struct Field {
    name: String,
    value: String,
    inline: bool,
}

pub enum PaginationDataType {
    Description {
        data: Vec<String>,
        base_description: Option<String>,
    },
    Field(Vec<Field>),
}

impl PaginationDataType {
    pub fn len(&self) -> usize {
        match &self {
            PaginationDataType::Description {
                data,
                base_description: _,
            } => data.len(),
            PaginationDataType::Field(data) => data.len(),
        }
    }
}

macro_rules! p_err {
    ($expr:expr) => {
        match $expr {
            Ok(ok) => Ok(ok),
            Err(err) => Err(TrancerError::Generic(
                "Pagination error: ".to_string() + &err.to_string(),
            )),
        }
    };
}

pub async fn paginate(op: PaginationOptions) -> Result<(), TrancerError> {
    let old_embed = op.embed.clone();
    let mut current_index: usize = 0;

    let modify_embed = |current_index: usize| {
        let mut embed = old_embed.clone();
        embed = if op.data.len() == 0 {
            embed.description("*No items to show here!*")
        } else {
            match op.data {
                PaginationDataType::Description {
                    ref data,
                    ref base_description,
                } => embed.description(
                    base_description
                        .clone()
                        .map(|x| x + "\n\n")
                        .unwrap_or("".to_string())
                        + data[current_index..(current_index + op.page_size).min(data.len())]
                            .join(&"\n")
                            .as_str(),
                ),
                PaginationDataType::Field(ref data) => todo!(),
            }
        };

        embed = embed.footer(CreateEmbedFooter::new(format!(
            "Page {} / {} ({} items)",
            current_index / op.page_size + 1,
            (op.data.len() as f64 / op.page_size as f64).ceil() as usize,
            op.data.len()
        )));
        embed
    };

    if op.data.len() < op.page_size + 1 {
        p_err!(reply!(
            op.ctx,
            CreateMessage::new()
                .embed(modify_embed(current_index))
                .reference_message(&op.ctx.msg)
        ))?;
        return Ok(());
    }

    let buttons = CreateActionRow::Buttons(vec![
        CreateButton::new("first-page")
            .style(ButtonStyle::Secondary)
            .label("<<<"),
        CreateButton::new("page-prev")
            .style(ButtonStyle::Primary)
            .label("<"),
        CreateButton::new("page-search")
            .style(ButtonStyle::Success)
            .label("🔍️"),
        CreateButton::new("page-next")
            .style(ButtonStyle::Primary)
            .label(">"),
        CreateButton::new("last-page")
            .style(ButtonStyle::Secondary)
            .label(">>>"),
    ]);

    let mut msg = p_err!(
        op.ctx
            .msg
            .channel_id
            .send_message(
                &op.ctx.sy,
                CreateMessage::new()
                    .embed(modify_embed(current_index))
                    .reference_message(&op.ctx.msg)
                    .components(vec![buttons]),
            )
            .await
    )?;

    let mut collector = msg
        .await_component_interactions(&op.ctx.sy)
        .timeout(Duration::from_secs(5 * 60))
        .stream();

    while let Some(ref i) = collector.next().await {
        p_err!(i.defer(&op.ctx.sy).await)?;
        match i.data.custom_id.as_str() {
            "first-page" => {
                current_index = 0;
            }
            "page-prev" => {
                if current_index < op.page_size {
                    return Ok(());
                }
                current_index -= op.page_size;
            }
            "page-next" => {
                if current_index >= op.data.len() - op.page_size {
                    return Ok(());
                }
                current_index += op.page_size;
            }
            "last-page" => {
                current_index = op.data.len() - (op.data.len() % op.page_size);
            }
            _ => {
                p_err!(reply!(
                    op.ctx,
                    CreateMessage::new().content("Search is not implemented yet!".to_string())
                ))?;
            }
        }

        p_err!(
            msg.edit(
                &op.ctx.sy,
                EditMessage::new().embed(modify_embed(current_index)),
            )
            .await
        )?;
    }

    p_err!(msg
        .edit(&op.ctx.sy, EditMessage::new().components(vec![]))
        .await
        .map(|_| ()))
}
