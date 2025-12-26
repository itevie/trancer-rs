use std::fmt::Display;

macro_rules! trancer_command_type {
    ($i:ident, {$($field:ident, $emoji:expr),*}) => {
        #[derive(Debug, Clone, Copy, Hash, Eq, PartialEq)]
        pub enum $i {
            $($field),*
        }

        impl $i {
            pub fn all() -> &'static [&'static str] {
                &[$(stringify!($field)),*]
            }

            pub fn emoji(&self) -> &'static str {
                match self {
                    $(Self::$field => $emoji),*
                }
            }
        }

        impl Display for $i {
            fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
                match *self {
                    $($i::$field => write!(f, "{}", stringify!($field).to_lowercase())),*
                }
            }
        }

        impl From<String> for $i {
            fn from(s: String) -> Self {
                match s.as_str() {
                    $(stringify!($field) => $i::$field),*,
                    _ => $i::Unknown
                }
            }
        }
    };
}

trancer_command_type!(TrancerCommandType, {
    Analytics, "📈",
    Dawnagotchi, "🏳‍🌈",
    Ranks, "🌭",
    Economy, "🌀",
    Cards, "🎴",
    Badges, "🥇",
    Booster, ":pink_heart:",
    Counting, "🔢",
    Spirals, "😵‍💫",
    Quotes, "🗨️",
    Help, "📖",
    Minecraft, ":green_heart:",
    Hypnosis, "😵‍💫",
    Uncategorized, "❓",
    Fun, "🎮",
    Admin, "🛠️",
    Messages, "💬",
    Leaderboards, "🏆",
    Games, "🎮️",
    Actions, "👊",
    Ai, "🤖",
    Marriage, "💍",
    Reporting, "⚔️",
    Qotd, "❓",
    Voice, "📞",
    Confessions, "🤫",
    FileDirectory, "📁",
    Unknown, "❓"
});
