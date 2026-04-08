use crate::util::config::CONFIG;

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

    level.saturating_sub(1)
}

pub struct CalcEverything {
    pub level: u32,
    pub next_level: u32,
    pub current_level_xp: u32,
    pub next_level_xp: u32,
    pub needed_xp: u32,
    pub progress: f64,
    pub amount_progress: u32,
    pub most: f64,
    pub least: f64,
    pub average: f64,
}

pub fn calc_everything(xp: u32) -> CalcEverything {
    let level = calculate_level(xp);
    let next_level = level + 1;

    let current_level_xp = get_xp_for_level(level);
    let next_level_xp = get_xp_for_level(next_level);
    let needed_xp = next_level_xp - current_level_xp;

    let progress = if needed_xp == 0 {
        100.0
    } else {
        (xp.saturating_sub(current_level_xp) as f64 / needed_xp as f64) * 100.0
    };
    let amount_progress = next_level_xp - xp;

    let remaining_xp = next_level_xp.saturating_sub(xp) as f64;

    let time_between = CONFIG.xp.time_between as f64 / 60_000.0;

    let most = remaining_xp / (CONFIG.xp.min as f64 * time_between);
    let least = remaining_xp / (CONFIG.xp.max as f64 * time_between);
    let average = remaining_xp / (((CONFIG.xp.min + CONFIG.xp.max) as f64 / 2.0) * time_between);

    CalcEverything {
        level,
        next_level,
        current_level_xp,
        next_level_xp,
        needed_xp,
        progress,
        amount_progress,
        most,
        least,
        average,
    }
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
