use crate::impl_from_row;

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