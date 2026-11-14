//! Playback quality: direct play versus a transcode preset.

/// Playback quality: Auto picks direct play unless the connection clearly can't carry the file.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Quality {
    Auto,
    Direct,
    /// Index into PRESETS: server transcode capped at that bitrate.
    Max(usize),
}

pub struct Preset {
    pub bps: u32,
    pub height: u32,
    pub short: &'static str,
    pub long: &'static str,
}

pub const PRESETS: [Preset; 9] = [
    Preset { bps: 40_000_000, height: 1080, short: "40 Mbps", long: "40 Mbps · 1080p" },
    Preset { bps: 20_000_000, height: 1080, short: "20 Mbps", long: "20 Mbps · 1080p" },
    Preset { bps: 12_000_000, height: 1080, short: "12 Mbps", long: "12 Mbps · 1080p" },
    Preset { bps: 8_000_000, height: 1080, short: "8 Mbps", long: "8 Mbps · 1080p" },
    Preset { bps: 4_000_000, height: 720, short: "4 Mbps", long: "4 Mbps · 720p" },
    Preset { bps: 3_000_000, height: 720, short: "3 Mbps", long: "3 Mbps · 720p" },
    Preset { bps: 2_000_000, height: 480, short: "2 Mbps", long: "2 Mbps · 480p" },
    Preset { bps: 1_500_000, height: 480, short: "1.5 Mbps", long: "1.5 Mbps · 480p" },
    Preset { bps: 720_000, height: 360, short: "720 kbps", long: "720 kbps · 360p" },
];

impl Quality {
    pub fn from_cfg(s: &str) -> Quality {
        match s {
            "" | "direct" => Quality::Direct,
            "auto" => Quality::Auto,
            n => n
                .parse::<u32>()
                .ok()
                .and_then(|bps| PRESETS.iter().position(|p| p.bps == bps))
                .map_or(Quality::Auto, Quality::Max),
        }
    }

    pub fn to_cfg(self) -> String {
        match self {
            Quality::Auto => "auto".into(),
            Quality::Direct => "direct".into(),
            Quality::Max(i) => PRESETS[i].bps.to_string(),
        }
    }
}

/// Decides direct play (None) or a transcode preset. For Auto: direct unless a measurement
/// shows the link is slower than the file needs.
pub fn decide(q: Quality, src_bps: Option<i64>, measured: Option<f64>) -> (Option<usize>, String) {
    match q {
        Quality::Direct => (None, "Direct".into()),
        Quality::Max(i) => (Some(i), PRESETS[i].short.into()),
        Quality::Auto => {
            let (Some(src), Some(m)) = (src_bps.filter(|b| *b > 0), measured) else {
                return (None, "Direct".into());
            };
            if m >= src as f64 * 1.1 {
                return (None, "Direct".into());
            }
            let budget = m * 0.8;
            let idx = PRESETS.iter().position(|p| p.bps as f64 <= budget).unwrap_or(PRESETS.len() - 1);
            if PRESETS[idx].bps as i64 >= src {
                (None, "Direct".into())
            } else {
                (Some(idx), PRESETS[idx].short.into())
            }
        }
    }
}

pub fn quality_label(q: Quality, resolved: &str) -> String {
    match q {
        Quality::Auto if resolved.is_empty() => "Auto".into(),
        Quality::Auto => format!("Auto – {resolved}"),
        Quality::Direct => "Direct".into(),
        Quality::Max(i) => PRESETS[i].short.into(),
    }
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn auto_prefers_direct() {
        // fast link: direct
        assert_eq!(decide(Quality::Auto, Some(15_000_000), Some(100e6)).0, None);
        // unknown bitrate or failed test: direct
        assert_eq!(decide(Quality::Auto, None, Some(1e6)).0, None);
        assert_eq!(decide(Quality::Auto, Some(15_000_000), None).0, None);
        // just enough headroom: direct
        assert_eq!(decide(Quality::Auto, Some(15_000_000), Some(16.6e6)).0, None);
    }

    #[test]
    fn auto_steps_down_when_link_is_slow() {
        // 25 Mbps file over a 14 Mbps link: budget 11.2 Mbps -> 8 Mbps preset
        let (p, label) = decide(Quality::Auto, Some(25_000_000), Some(14e6));
        assert_eq!(p.map(|i| PRESETS[i].bps), Some(8_000_000));
        assert_eq!(label, "8 Mbps");
        // very slow link bottoms out at the lowest preset
        let (p, _) = decide(Quality::Auto, Some(25_000_000), Some(0.3e6));
        assert_eq!(p, Some(PRESETS.len() - 1));
        // 3 Mbps file over a 2.5 Mbps link: budget 2.0 Mbps -> the 2 Mbps preset
        assert_eq!(decide(Quality::Auto, Some(3_000_000), Some(2.5e6)).0.map(|i| PRESETS[i].bps), Some(2_000_000));
        // a tiny file never "transcodes" up to a preset bigger than itself
        assert_eq!(decide(Quality::Auto, Some(700_000), Some(0.6e6)).0, None);
    }

    #[test]
    fn explicit_choices_and_config_roundtrip() {
        // a fresh install (nothing saved yet) plays the original file
        assert_eq!(Quality::from_cfg(""), Quality::Direct);
        assert_eq!(decide(Quality::Direct, Some(1), Some(1.0)).0, None);
        assert_eq!(decide(Quality::Max(4), None, None).0, Some(4));
        for q in [Quality::Auto, Quality::Direct, Quality::Max(0), Quality::Max(8)] {
            assert_eq!(Quality::from_cfg(&q.to_cfg()), q);
        }
        assert_eq!(quality_label(Quality::Auto, "Direct"), "Auto – Direct");
        assert_eq!(quality_label(Quality::Auto, "10 Mbps"), "Auto – 10 Mbps");
    }
}
