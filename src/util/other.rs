use crate::cmd_util::{generic, TrancerError};
use crate::util::lang::permission_names;
use serenity::all::{Member, Permissions, Role};
use serenity::client::Context;

pub fn random_number_from_string(input: &str, min: i32, max: i32) -> i32 {
    let mut hash: u32 = 0;
    for c in input.chars() {
        hash = hash.wrapping_mul(31).wrapping_add(c as u32);
    }

    let seed = hash as f64;
    let random_fraction = (seed.sin() * 10000.0).fract(); // value in [0.0, 1.0)

    let range_size = (max - min + 1) as f64;
    let scaled = (random_fraction * range_size).floor() as i32;

    scaled + min
}

pub async fn give_role(ctx: &Context, member: &Member, role: &Role) -> Result<(), TrancerError> {
    let blacklisted = &[
        Permissions::MANAGE_CHANNELS,
        Permissions::MANAGE_ROLES,
        Permissions::MANAGE_EVENTS,
        Permissions::MANAGE_MESSAGES,
        Permissions::MANAGE_GUILD,
        Permissions::MANAGE_GUILD_EXPRESSIONS,
        Permissions::MANAGE_WEBHOOKS,
        Permissions::MANAGE_THREADS,
        Permissions::MANAGE_WEBHOOKS,
        Permissions::ADMINISTRATOR,
    ];

    for i in blacklisted {
        if role.permissions.contains(*i) {
            return Err(generic(format!(
                "Could not give the role as it has a dangerous permission: {}",
                permission_names(*i)
            )));
        }
    }

    member
        .add_role(&ctx, role)
        .await
        .map_err(|x| TrancerError::Serenity(x))
}
