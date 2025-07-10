// This file really moos!
type Cow = &'static str;

pub struct PartialItem {
    pub name: Cow,
    pub price: u32,
    pub description: Option<Cow>,
    pub weight: f64,
    pub emoji: Option<Cow>,
    pub tag: Option<Cow>,
    pub buyable: bool,
    pub droppable: bool,
    pub max: Option<u32>,
}

static VERY_COMMON: (u32, f64) = (5, 0.9);
static COMMON: (u32, f64) = (10, 0.8);
static UNCOMMONISH: (u32, f64) = (15, 0.75);
static UNCOMMON: (u32, f64) = (15, 0.6);
static RARISH: (u32, f64) = (25, 0.5);
static RARE: (u32, f64) = (40, 0.2);
static VERY_RARE: (u32, f64) = (55, 0.1);
static EPIC: (u32, f64) = (80, 0.05);
static LEGENDARY: (u32, f64) = (125, 0.04);
static MYTHIC: (u32, f64) = (400, 0.01);
static DIVINE: (u32, f64) = (1250, 0.005);

const fn fish(name: Cow, price_weight: (u32, f64), emoji: Cow) -> PartialItem {
    PartialItem {
        name,
        price: price_weight.0,
        description: None,
        weight: price_weight.1,
        emoji: Some(emoji),
        tag: None,
        buyable: false,
        droppable: true,
        max: None,
    }
}

const fn fishd(name: Cow, price_weight: (u32, f64), emoji: Cow, description: Cow) -> PartialItem {
    PartialItem {
        description: Some(description),
        ..fish(name, price_weight, emoji)
    }
}

const fn basic(
    name: Cow,
    price: u32,
    weight: f64,
    description: Option<Cow>,
    emoji: Cow,
) -> PartialItem {
    PartialItem {
        name,
        price,
        description,
        weight,
        emoji: Some(emoji),
        tag: None,
        buyable: true,
        droppable: true,
        max: None,
    }
}

const fn tagged(
    name: Cow,
    price: u32,
    weight: f64,
    description: Option<Cow>,
    emoji: Cow,
    tag: Cow,
) -> PartialItem {
    PartialItem {
        tag: Some(tag),
        ..basic(name, price, weight, description, emoji)
    }
}

const fn taggednb(
    name: Cow,
    price: u32,
    weight: f64,
    description: Option<Cow>,
    emoji: Cow,
    tag: Cow,
) -> PartialItem {
    PartialItem {
        buyable: false,
        ..tagged(name, price, weight, description, emoji, tag)
    }
}

const fn mineral(name: Cow, price: u32, weight: f64, emoji: Cow) -> PartialItem {
    taggednb(name, price, weight, None, emoji, "mineral")
}

const fn collectable(name: Cow, price: u32, description: Cow, emoji: Cow) -> PartialItem {
    PartialItem {
        name,
        price,
        description: Some(description),
        weight: 0.0,
        emoji: Some(emoji),
        tag: Some("collectable"),
        buyable: false,
        droppable: false,
        max: None,
    }
}

pub static ALL_ITEMS: &'static [PartialItem] = &[
    // ---- Fish -----
    fish("cod", VERY_COMMON, "<:cod:1322128982027534367>"),
    fish("common-fish", COMMON, "<:common_fish:1321758129872048189>"),
    fish("salmon", UNCOMMONISH, "<:salmon:1322129073811623991>"),
    fish(
        "uncommon-fish",
        UNCOMMON,
        "<:uncommon_fish:1321758154715041873>",
    ),
    fish(
        "wide-salmon",
        UNCOMMON,
        "<:wide_salmon:1322129134423379991>",
    ),
    fish("pufferfish", RARISH, "<:pufferfish:1322129056086491136>"),
    fish("rare-fish", RARE, "<:rare_fish:1321758169004773419>"),
    fish("spiral-fish", RARE, "<:spiral_fish:1322129093684101193>"),
    fishd(
        "scottish-fish",
        RARE,
        "<:scotish_fish:1321843773415886899>",
        "SCOTLAND FOREVER!",
    ),
    fishd(
        "cute-fishy",
        VERY_RARE,
        "<:cute_fish:1321857857636798566>",
        "Aw... such a cutie patootie fishie",
    ),
    fish(
        "cookie-fish",
        VERY_RARE,
        "<:cookie_fish:1321843822115684483>",
    ),
    fishd(
        "transparent-fish",
        VERY_RARE,
        "<:transparent_fish:1321843791241678869>",
        "Woah.",
    ),
    fish("epic-fish", EPIC, "<:epic_fish:1321758183882227755>"),
    fish(
        "basking-shark",
        EPIC,
        "<:basking_shark:1325557096808452107>",
    ),
    fish("angle-fish", LEGENDARY, "<:angel_fish:1325262607812395079>"),
    fish("devil-fish", LEGENDARY, "<:devil_fish:1325262632776892487>"),
    fish("mythic-fish", MYTHIC, "<:mythic_fish:1321758197178175588>"),
    fishd(
        "dawn-fish",
        MYTHIC,
        "<:dawn_fish:1325556803546779678>",
        "According to all known laws of hypnosis, Dawn is a fish... allegedly.",
    ),
    fish("weed-fish", MYTHIC, "<:weed_fish:1322130669479923762>"),
    fish("fish", MYTHIC, "<:fish:1322130721548009512>"),
    fishd(
        "gay-fish",
        MYTHIC,
        "<:gay_fish:1321843756592271441>",
        "Gay!!!!",
    ),
    fishd(
        "trans-fish",
        DIVINE,
        "<:trans_fish:1321845160492925029>",
        "Your average trans fish.",
    ),
    fishd(
        "british-fish",
        DIVINE,
        "<:british_fish:1321758209983381534>",
        "Pip pip cheerio!",
    ),
    // ----- Special Fish -----
    fishd(
        "catfish",
        (2, 0.3),
        "<:cat_fish:1322128963837104158>",
        "Such rare... very nothing... much scam...",
    ),
    fish(
        "you-are-never-getting-this-fish",
        (5000, 0.0005),
        "<:you_are_never_getting_this_fish:1321758224222912604>",
    ),
    fishd(
        "we-are-number-one-fish",
        (20_000, 0.00001),
        "<:we_are_number_one_fish:1322129116006449234>",
        "If you wanna be a villain number one, you have to catch a fishie on the run.",
    ),
    // ----- Useful Items -----
    basic(
        "card-pull",
        100,
        0.6,
        Some("Buy this, and pull a card using the `pull` command!"),
        "<:card_pull:1321761564314964010>",
    ),
    basic(
        "hair-dye",
        150,
        0.05,
        Some("Dye the hair of your Dawn! Use the `dyehair` command to do so!"),
        "<:hair_dye:1325556895309758576>",
    ),
    basic(
        "hair",
        30,
        0.5,
        Some("Feed your Dawn some hair and it'll be a lot less hungry!"),
        "<:hair:1325556878494928957>",
    ),
    basic(
        "juicebox",
        15,
        0.55,
        Some("Give your Dawn some juice and it'll be a lot less thirsty!"),
        "<:juicebox:1322129011899502602>",
    ),
    basic(
        "pendulum",
        75,
        0.15,
        Some("A pendulum! It goes this way and that. (good for playing with Dawn!)"),
        "<:pendulum:1325556858869645443>",
    ),
    basic(
        "fishing-rod",
        250,
        0.1,
        Some("You can fish more frequently. This has a 10% chance of breaking."),
        "<:fishing_rod:1321761522699210802>",
    ),
    // ----- Accessories -----
    tagged(
        "pacifier",
        100,
        0.2,
        Some("Good for calming down."),
        "<:paci:1358918815584620877>",
        "accessory",
    ),
    tagged(
        "hair-bow",
        2500,
        0.005,
        Some("Make your Dawn all cutesy with a hair bow!"),
        "<:bow:1368265911139565578>",
        "accessory",
    ),
    // ----- Resources -----
    basic(
        "stick",
        5,
        0.9,
        Some("A stick."),
        "<:stick:1321761484174524498>",
    ),
    tagged(
        "rock",
        1,
        0.5,
        None,
        "<:rock:1321761504386744391>",
        "mineral",
    ),
    // ----- Minerals -----
    mineral("dirt", 1, 0.6, "<:dirt:1328997326932676648>"),
    mineral("coal", 5, 0.35, "<:coal:1325546270500196443>"),
    mineral("iron", 15, 0.3, "<:iron:1325546289261318294>"),
    mineral("copper", 25, 0.2, "<:copper:1342883570259198063>"),
    mineral("silver", 40, 0.15, "<:silver:1342883725796446269>"),
    mineral("gold", 50, 0.1, "<:gold:1325546308689199155>"),
    mineral("amethyst", 60, 0.08, "<:amythest:1342885723749220455>"),
    mineral("emerald", 70, 0.05, "<:emerald:1328997361925492828>"),
    mineral("sapphire", 85, 0.03, "<:sapphire:1342883648482840678>"),
    mineral("ruby", 110, 0.009, "<:ruby:1342883623422132387>"),
    mineral("diamond", 120, 0.003, "<:diamond:1328997377889140817>"),
    mineral("spiral", 150, 0.001, "<:spiral:1328997344343228460>"),
    // ----- Pickaxes -----
    tagged(
        "stone-pickaxe",
        10,
        0.5,
        None,
        "<:stone_pickaxe:1325548046188286053>",
        "pickaxe",
    ),
    taggednb(
        "emerald-pickaxe",
        200,
        0.0001,
        Some("Gives you 10% extra luck when mining."),
        "<:emerald_pickaxe:1342888386175963248>",
        "pickaxe",
    ),
    // ----- Collectables -----
    collectable(
        "christmas-cookie",
        1000,
        "A tasty cookie given on 25/12/2024!",
        "<:chirtmas_cookie:1321761548372279337>",
    ),
    collectable(
        "easter-2025-egg",
        1000,
        "An ugly egg given on Easter 2025!",
        "<:egg:1366070379302359050>",
    ),
    collectable(
        "1st-birthday-cake",
        5000,
        "Twilight's first birthday cake :heart:",
        ":birthday:",
    ),
    // ----- Misc -----
    PartialItem {
        name: "lottery-ticket",
        price: 250,
        description: Some("Enter into the lottery. Check the `lottery` command!"),
        weight: 0.0,
        emoji: Some("<:lottery_ticket:1322129029368909917>"),
        tag: None,
        buyable: true,
        droppable: true,
        max: Some(5),
    },
];
