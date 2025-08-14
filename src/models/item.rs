use crate::cmd_util::{generic, TrancerError};
use crate::database::Database;
use crate::impl_from_row;
use crate::trancer_config::all_items::ALL_ITEMS_DEF;
use once_cell::sync::OnceCell;
use rusqlite::Error::QueryReturnedNoRows;
use serenity::client::Context;
use std::collections::HashMap;

impl_from_row!(Item, ItemField {
    id: u32,
    name: String,
    price: u32,
    description: Option<String>,
    droppable: bool,
    weight: f64,
    tag: Option<String>,
    buyable: bool,
    emoji: Option<String>,
    max: Option<u32>
});

pub static ALL_ITEMS: OnceCell<Vec<Item>> = OnceCell::new();

pub fn get_item(item_id: u32) -> Result<Item, TrancerError> {
    let Some(all) = ALL_ITEMS.get() else {
        return Err(generic("Couldn't get ALL_ITEMS"));
    };

    match all.iter().find(|x| x.id == item_id) {
        Some(some) => Ok(some.clone()),
        None => Err(generic(format!("Item ID {item_id} does not exist."))),
    }
}

pub fn get_item_name<T: Into<String>>(name: T) -> Result<Item, TrancerError> {
    let name = name.into();
    let Some(all) = ALL_ITEMS.get() else {
        return Err(generic("Couldn't get ALL_ITEMS"));
    };

    match all.iter().find(|x| x.name == name) {
        Some(some) => Ok(some.clone()),
        None => Err(generic(format!("Item with name {name} does not exist."))),
    }
}

impl Item {
    pub async fn get(ctx: &Context, id: u32) -> rusqlite::Result<Option<Item>> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        let result = db.get_one("SELECT * FROM items WHERE id = ?1", &[&id], |r| {
            Item::from_row(r)
        });

        match result {
            Ok(ok) => Ok(Some(ok)),
            Err(QueryReturnedNoRows) => Ok(None),
            Err(e) => Err(e),
        }
    }

    pub fn get_all() -> Vec<Item> {
        ALL_ITEMS.get().unwrap().clone()
    }

    pub async fn get_all_db(ctx: &Context) -> rusqlite::Result<Vec<Item>> {
        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        db.get_many("SELECT * FROM items;", &[], |r| Item::from_row(r))
    }

    pub async fn insert_all(ctx: &Context) -> rusqlite::Result<()> {
        let items = Item::get_all_db(&ctx).await?;

        let data_lock = ctx.data.read().await;
        let db = data_lock.get::<Database>().unwrap();

        for item in ALL_ITEMS_DEF.iter() {
            let Some(already_item) = items.iter().find(|x| x.name == item.name) else {
                db.run(
                    "INSERT INTO items (name, price, description, weight, droppable, tag, buyable, emoji, max) VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9)",
                    &[&item.name, &item.price, &item.description, &item.weight, &item.droppable, &item.tag, &item.buyable, &item.emoji, &item.max],
                )?;
                continue;
            };

            if already_item.price != item.price {
                db.run(
                    "UPDATE items SET price = ?1 WHERE id = ?2",
                    &[&item.price, &already_item.id],
                )?;
            }

            if already_item.description != item.description.map(|x| x.to_string()) {
                db.run(
                    "UPDATE items SET description = ?1 WHERE id = ?2",
                    &[&item.description, &already_item.id],
                )?;
            }

            if already_item.weight != item.weight {
                db.run(
                    "UPDATE items SET weight = ?1 WHERE id = ?2",
                    &[&item.weight, &already_item.id],
                )?
            }

            if already_item.droppable != item.droppable {
                db.run(
                    "UPDATE items SET droppable = ?1 WHERE id = ?2",
                    &[&item.droppable, &already_item.id],
                )?;
            }

            if already_item.tag != item.tag.map(|x| x.to_string()) {
                db.run(
                    "UPDATE items SET tag = ?1 WHERE id = ?2",
                    &[&item.tag, &already_item.id],
                )?;
            }

            if already_item.buyable != item.buyable {
                db.run(
                    "UPDATE items SET buyable = ?1 WHERE id = ?2",
                    &[&item.buyable, &already_item.id],
                )?;
            }

            if already_item.emoji != item.emoji.map(|x| x.to_string()) {
                db.run(
                    "UPDATE items SET emoji = ?1 WHERE id = ?2",
                    &[&item.emoji, &already_item.id],
                )?;
            }

            if already_item.max != item.max {
                db.run(
                    "UPDATE items SET max = ?1 WHERE id = ?2",
                    &[&item.max, &already_item.id],
                )?;
            }
        }

        Ok(())
    }
}
