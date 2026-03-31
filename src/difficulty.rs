pub fn estimate_attempts(prefix: &str, suffix: &str) -> (usize, f64) {
    let p = prefix.trim_start_matches("0x").len();
    let s = suffix.trim_start_matches("0x").len();

    let total_chars = p + s;
    let expected_attempts = 16_f64.powi(total_chars as i32);

    (total_chars, expected_attempts)
}
