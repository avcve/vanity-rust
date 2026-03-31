pub fn normalize_hex(input: &str) -> String {
    input.trim_start_matches("0x").to_lowercase()
}

pub fn matches_pattern(address: &str, prefix: &str, suffix: &str) -> bool {
    let normalized = normalize_hex(address);
    let p = normalize_hex(prefix);
    let s = normalize_hex(suffix);

    if !p.is_empty() && !normalized.starts_with(&p) {
        return false;
    }

    if !s.is_empty() && !normalized.ends_with(&s) {
        return false;
    }

    true
}
