use std::collections::HashMap;
use std::sync::LazyLock;

pub struct PronounSet {
    pub sub: &'static str,
    pub obj: &'static str,
    pub poss_adj: &'static str,
    pub poss_prn: &'static str,
    pub reflex: &'static str,
}

pub static ALL_PRONOUNS: LazyLock<HashMap<&'static str, PronounSet>> = LazyLock::new(|| {
    HashMap::from([
        (
            "she",
            PronounSet {
                sub: "she",
                obj: "her",
                poss_adj: "her",
                poss_prn: "hers",
                reflex: "herself",
            },
        ),
        (
            "he",
            PronounSet {
                sub: "he",
                obj: "him",
                poss_adj: "his",
                poss_prn: "his",
                reflex: "himself",
            },
        ),
        (
            "they",
            PronounSet {
                sub: "they",
                obj: "them",
                poss_adj: "their",
                poss_prn: "theirs",
                reflex: "themself",
            },
        ),
    ])
});
