//! Accent-colour maths.

pub fn rgb_to_hsv(rgb: u32) -> (f32, f32, f32) {
    let (r, g, b) = (((rgb >> 16) & 0xFF) as f32 / 255.0, ((rgb >> 8) & 0xFF) as f32 / 255.0, (rgb & 0xFF) as f32 / 255.0);
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let d = max - min;
    let h = if d == 0.0 {
        0.0
    } else if max == r {
        60.0 * (((g - b) / d).rem_euclid(6.0))
    } else if max == g {
        60.0 * ((b - r) / d + 2.0)
    } else {
        60.0 * ((r - g) / d + 4.0)
    };
    (h, if max == 0.0 { 0.0 } else { d / max }, max)
}

pub fn parse_hex(s: &str) -> Option<u32> {
    let t = s.trim().trim_start_matches('#');
    match t.len() {
        6 => u32::from_str_radix(t, 16).ok(),
        3 => {
            let v = u32::from_str_radix(t, 16).ok()?;
            let (r, g, b) = ((v >> 8) & 0xF, (v >> 4) & 0xF, v & 0xF);
            Some(((r * 17) << 16) | ((g * 17) << 8) | (b * 17))
        }
        _ => None,
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn parses_hex() {
        assert_eq!(parse_hex("#60CDFF"), Some(0x60CDFF));
        assert_eq!(parse_hex("60cdff"), Some(0x60CDFF));
        assert_eq!(parse_hex("#fa0"), Some(0xFFAA00));
        assert_eq!(parse_hex("nope"), None);
        assert_eq!(parse_hex("#12345"), None);
    }

    #[test]
    fn rgb_hsv_roundtrip_matches_known_colours() {
        let (h, s, v) = rgb_to_hsv(0xFF0000);
        assert!((h - 0.0).abs() < 0.01 && (s - 1.0).abs() < 0.01 && (v - 1.0).abs() < 0.01);
        let (h, _, _) = rgb_to_hsv(0x00FF00);
        assert!((h - 120.0).abs() < 0.01);
        let (h, _, _) = rgb_to_hsv(0x0000FF);
        assert!((h - 240.0).abs() < 0.01);
        // the default accent
        let (h, s, v) = rgb_to_hsv(0x60CDFF);
        assert!((h - 199.0).abs() < 1.0 && (s - 0.62).abs() < 0.01 && (v - 1.0).abs() < 0.01);
        // grey has no saturation
        assert_eq!(rgb_to_hsv(0x808080).1, 0.0);
    }
}
