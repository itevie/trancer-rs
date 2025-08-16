use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::CommandHasNoArgs;
use crate::util::embeds::create_embed;
use rand::Rng;
use serenity::all::CreateMessage;

static POSITIVE: &[&str] = &[
    "It is certain",
    "It is decidedly so",
    "Without a doubt",
    "Yes definitely",
    "You may rely on it",
    "As I see it, yes",
    "Most likely",
    "Outlook good",
    "Yes",
    "Signs point to yes",
];

static NEUTRAL: &[&str] = &[
    "Reply hazy, try again",
    "Ask again later",
    "Better not tell you now",
    "Cannot predict now",
    "Concentrate and ask again",
];

static NEGATIVE: &[&str] = &[
    "Don't count on it",
    "My reply is no",
    "My sources say no",
    "Outlook not so good",
    "Very doubtful",
];

static EIGHT_BALL_IMAGE: &str = "https://cdn.discordapp.com/attachments/1257417475621130351/1353034264257761433/8ball.png?ex=67e02eda&is=67dedd5a&hm=c52e79c925edde50c2c2b14f642931c4305e98c014e6d348f3a6718ca368e580&";

command_file! {
    TrancerCommand::<CommandHasNoArgs> {
        name: "8ball".to_string(),
        t: TrancerCommandType::Fun,
        description: "This is a test".to_string(),
        details: TrancerDetails {
            aliases: Some(vec!["8b".to_string(), "magic8ball".to_string()]),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let mut all = Vec::new();
            all.extend(NEUTRAL.to_vec());
            all.extend(NEGATIVE.to_vec());
            all.extend(POSITIVE.to_vec());

            let mut rng = rand::thread_rng();
            let r = all[rng.gen_range(0..all.len())];

            Ok(TrancerResponseType::Big(CreateMessage::new().embed(create_embed().title("Magical 8ball")
            .description(r)
            .color(
                match r {
                    x if POSITIVE.contains(&x) => (0, 255, 0),
                    x if NEUTRAL.contains(&x) => (255, 255, 0),
                    x if NEGATIVE.contains(&x) => (255, 0, 0),
                    _ => unreachable!(),
                }
            ).thumbnail(EIGHT_BALL_IMAGE))))
        }),
    }
}
