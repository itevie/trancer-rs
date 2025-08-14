use crate::database::Database;
use crate::impl_from_row;
use serenity::all::UserId;
use serenity::client::Context;

impl_from_row!(
    AquiredItem,
    AquiredItemField {
        item_id: u32,
        user_id: String,
        amount: u32,
        protected: bool
    }
);

impl AquiredItem {
    pub async fn give_item_to(
        ctx: &Context,
        user_id: UserId,
        item_id: u32,
        amount: u32,
    ) -> rusqlite::Result<()> {
        let data_lock = ctx.data.read().await;
        let database = data_lock.get::<Database>().unwrap();

        database.run(
            "INSERT INTO aquired_items (item_id, user_id, amount) VALUES (?1, ?2, ?3)\
                ON CONFLICT(item_id, user_id) DO UPDATE SET \
                    amount = amount + excluded.amount\
            ",
            &[&item_id, &user_id.to_string(), &amount],
        )
    }

    pub async fn remove_item_from(
        ctx: &Context,
        user_id: UserId,
        item_id: u32,
        amount: u32,
    ) -> rusqlite::Result<()> {
        let data_lock = ctx.data.read().await;
        let database = data_lock.get::<Database>().unwrap();

        database.run(
            "INSERT INTO aquired_items (item_id, user_id, amount) VALUES (?1, ?2, 0)\
                ON CONFLICT(item_id, user_id) DO UPDATE SET \
                    amount = MIN(amount - ?3, 0)\
            ",
            &[&item_id, &user_id.to_string(), &amount],
        )
    }

    pub async fn fetch_all_for(
        ctx: &Context,
        user_id: UserId,
    ) -> rusqlite::Result<Vec<AquiredItem>> {
        let data_lock = ctx.data.read().await;
        let database = data_lock.get::<Database>().unwrap();

        database.get_many(
            "SELECT * FROM aquired_items WHERE user_id = ?1",
            &[&user_id.to_string()],
            |x| AquiredItem::from_row(x),
        )
    }
}
