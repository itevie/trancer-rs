use std::collections::HashMap;
use std::sync::LazyLock;

pub static CRAFTING_RECIPES: LazyLock<HashMap<String, HashMap<&'static str, u32>>> =
    LazyLock::new(|| {
        HashMap::from([
            (
                "stone-pickaxe".to_string(),
                HashMap::from([("rock", 3), ("stick", 1)]),
            ),
            (
                "emerald-pickaxe".to_string(),
                HashMap::from([("stick", 1), ("emerald", 3)]),
            ),
            (
                "pendulum".to_string(),
                HashMap::from([("stick", 1), ("emerald", 1)]),
            ),
        ])
    });
