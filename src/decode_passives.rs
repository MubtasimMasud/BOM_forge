use regex::Regex;

/// Decode a resistor value string (e.g. "4.7k", "4k7", "47R") into ohms (u32).
/// Returns None if the string can't be parsed or would overflow.
pub fn decode_resistance(s: &str) -> Option<u32> {
    // 1) normalize: lowercase, strip "ohm", "Ω" and whitespace
    let mut t = s.to_lowercase();
    for pat in &["ohms", "ohm", "ω"] {
        t = t.replace(pat, "");
    }
    let token = t.split_whitespace().next()?;

    // 2) simple "47r" or "4.7r" → ohms
    if let Some(rest) = token.strip_suffix('r') {
        if let Ok(val) = rest.parse::<f64>() {
            let ohm = val.round();
            if ohm <= u32::MAX as f64 {
                return Some(ohm as u32);
            }
        }
    }

    // 3) code form: e.g. "4k7", "2m2", "4r7" (r = decimal sep for ohm)
    lazy_static::lazy_static! {
        static ref CODE_RE: Regex = Regex::new(r"^(\d+)([kmr])(\d+)$").unwrap();
    }
    if let Some(cap) = CODE_RE.captures(token) {
        let intp: f64 = cap[1].parse().ok()?;
        let frac = &cap[3];
        let unit = cap[2].chars().next().unwrap();
        let num = format!("{intp:.0}.{frac}").parse::<f64>().ok()?;
        let mul = match unit {
            'r' => 1.0,
            'k' => 1_000.0,
            'm' => 1_000_000.0,
            _ => unreachable!(),
        };
        let ohm = (num * mul).round();
        return if ohm <= u32::MAX as f64 {
            Some(ohm as u32)
        } else {
            None
        };
    }

    // 4) normal form: decimal + optional scale suffix (k, m, g)
    lazy_static::lazy_static! {
        static ref NORM_RE: Regex = Regex::new(r"^(\d*\.?\d+)([kmg]?)$").unwrap();
    }
    if let Some(cap) = NORM_RE.captures(token) {
        let num: f64 = cap[1].parse().ok()?;
        let mul = match &cap[2] {
            "k" => 1_000.0,
            "m" => 1_000_000.0,
            "g" => 1_000_000_000.0,
            _ => 1.0,
        };
        let ohm = (num * mul).round();
        return if ohm <= u32::MAX as f64 {
            Some(ohm as u32)
        } else {
            None
        };
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn res_r_suffix() {
        assert_eq!(decode_resistance("47R"), Some(47));
        assert_eq!(decode_resistance("4.7R"), Some(5)); // 4.7 → round to 5 Ω
    }

    #[test]
    fn res_existing() {
        assert_eq!(decode_resistance("4.7k"), Some(4_700));
        assert_eq!(decode_resistance("4k7"), Some(4_700));
        assert_eq!(decode_resistance("4.7k ohms"), Some(4_700));
        assert_eq!(decode_resistance("4.7k 0805 Ω"), Some(4_700));
        assert_eq!(decode_resistance("10M"), Some(10_000_000));
        assert_eq!(decode_resistance("220"), Some(220));
    }
}
