use crate::util::config::CONFIG;
use tracing::Instrument;

pub static TIME_BETWEEN: u32 = 120000;
pub static XP_ECO_REWARD: u32 = 100;

pub fn calculate_level(xp: u32) -> u32 {
    let mut level = 0;

    for i in CONFIG.xp.levels.iter() {
        if xp >= *i {
            level += 1;
        } else {
            break;
        }
    }

    if xp > *CONFIG.xp.levels.last().unwrap() {
        let rem = xp - *CONFIG.xp.levels.last().unwrap();
        level += (rem as f64 / CONFIG.xp.after as f64).trunc() as u32;
    }

    level.checked_sub(1).unwrap_or(0)
}

pub fn xp_for_next_level(xp: u32) -> u32 {
    let level = calculate_level(xp);
    let next = get_xp_for_level(level + 1);
    next - xp
}

pub fn get_xp_for_level(level: u32) -> u32 {
    if let Some(x) = CONFIG.xp.levels.get(level as usize) {
        *x
    } else {
        let last = CONFIG.xp.levels.len() - 1;
        let lvl = CONFIG.xp.levels.last().unwrap();
        lvl + (level - last as u32) * CONFIG.xp.after
    }
}
