use crate::cmd_util::types::TrancerCommandType;
use crate::cmd_util::CommandTrait;
use crate::cmd_util::{trancer_handler, TrancerDetails};
use crate::cmd_util::{TrancerCommand, TrancerResponseType};
use crate::command_file;
use crate::commands::{only_user_args, OnlyUserArgs};
use rand::Rng;

static BITE_LIP: &'static str = "<:bite_lips:1315469148004028537>";

command_file! {
    TrancerCommand::<OnlyUserArgs> {
        name: "rizz".to_string(),
        t: TrancerCommandType::Fun,
        description: "Rizz someone up!".to_string(),
        details: TrancerDetails {
            arguments: Some(only_user_args(true, false)),
            ..Default::default()
        },

        handler: trancer_handler!(|ctx, args| {
            let mut rng = rand::thread_rng();
            let amount = rng.gen_range(0..100);

            Ok(TrancerResponseType::Content(if ctx.msg.author.id == args.user.id {
                format!("Damn... you trying to rizz yourself up {BITE_LIP}? Ok.. ok... you ultimately rizz yourself up with **{amount}%** rizz!")
            } else {
                format!("Ok... **{}** are you ready for **{}'s** amazing rizz {BITE_LIP}? Well... it was **{amount}%** effective :fire:"
                ,args.user.name, ctx.msg.author.name)
            }))
        }),
    }
}
