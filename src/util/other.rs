pub fn random_number_from_string(input: &str, min: i32, max: i32) -> i32 {
    // Hash function matching JavaScript's `>>> 0`
    let mut hash: u32 = 0;
    for c in input.chars() {
        hash = hash.wrapping_mul(31).wrapping_add(c as u32);
    }

    // Get a pseudorandom fractional value from the seed
    let seed = hash as f64;
    let random_fraction = (seed.sin() * 10000.0).fract(); // value in [0.0, 1.0)

    // Scale and map to range
    let range_size = (max - min + 1) as f64;
    let scaled = (random_fraction * range_size).floor() as i32;

    scaled + min
}
