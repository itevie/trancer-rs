pub struct StatusTheme {
    pub red: &'static str,
    pub orange: &'static str,
    pub green: &'static str,
}

#[derive(Copy, Clone)]
pub enum Theme {
    Circles,
    Squares,
    Fruit,
    Hearts,
    Books,
    Flowers,
}

impl Theme {
    pub fn get(self) -> StatusTheme {
        match self {
            Theme::Circles => StatusTheme {
                red: "🔴",
                orange: "🟠",
                green: "🟢",
            },
            Theme::Squares => StatusTheme {
                red: "🟥",
                orange: "🟧",
                green: "🟩",
            },
            Theme::Fruit => StatusTheme {
                red: "🍎",
                orange: "🍊",
                green: "🍏",
            },
            Theme::Hearts => StatusTheme {
                red: "❤️",
                orange: "🧡",
                green: "💚",
            },
            Theme::Books => StatusTheme {
                red: "📕",
                orange: "📙",
                green: "📗",
            },
            Theme::Flowers => StatusTheme {
                red: "🌹",
                orange: "🌻",
                green: "🥬",
            },
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            Theme::Circles => "circles",
            Theme::Squares => "squares",
            Theme::Fruit => "fruit",
            Theme::Hearts => "hearts",
            Theme::Books => "books",
            Theme::Flowers => "flowers",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "circles" => Some(Self::Circles),
            "squares" => Some(Self::Squares),
            "fruit" => Some(Self::Fruit),
            "hearts" => Some(Self::Hearts),
            "books" => Some(Self::Books),
            "flowers" => Some(Self::Flowers),
            _ => None,
        }
    }
}

/// 🔑 This is your "all types as a list"
pub const ALL_THEME_NAMES: &[&str] = &["circles", "squares", "fruit", "hearts", "books", "flowers"];
