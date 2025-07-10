static LEVELS: &[u32] = &[
    25, 50, 75, 100, 150, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1200,
    1400, 1600, 1800, 2000, 2500, 5000, 5500, 6000, 6500, 7000, 7500, 8000, 8500,
    9000, 9500, 10000, 11000,
];
pub fn after_those() -> u32 {
    LEVELS.last().unwrap() - LEVELS.get(LEVELS.len() - 2).unwrap()
}

pub static MIN_XP: u16 = 0;
pub static MAX_XP: u16 = 5;
pub static TIME_BETWEEN: u32 = 120000;
pub static XP_ECO_REWARD: u32 = 100;

pub fn calculate_level(xp: u32) -> u32 {
    let mut level = 0;

    for i in LEVELS.iter() {
        if xp >= *i {
            level += 1;
        } else {
            break;
        }
    }

    if xp > *LEVELS.last().unwrap() {
        let rem = xp - *LEVELS.last().unwrap();
        level += (rem as f64 / after_those() as f64).trunc() as u32;
    }

    level - 1
}

pub fn xp_for_next_level(xp: u32) -> u32 {
    let level = calculate_level(xp);
    let next = get_xp_for_level(level + 1);
    next - xp
}

pub fn get_xp_for_level(level: u32) -> u32 {
    if let Some(x) = LEVELS.get(level as usize) {
        *x
    } else {
        let last = LEVELS.len() - 1;
        let lvl = LEVELS.last().unwrap();
        lvl + (level - last as u32) * after_those()
    }
}