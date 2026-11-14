//! Human-readable durations, sizes and bitrates.

use emby_api::TICKS_PER_SEC;

pub fn duration_label(ticks: i64) -> String {
    let mins = ticks / TICKS_PER_SEC / 60;
    if mins >= 60 {
        format!("{}h {}m", mins / 60, mins % 60)
    } else {
        format!("{mins}m")
    }
}

pub fn clock(secs: i64) -> String {
    format!("{}:{:02}:{:02}", secs / 3600, secs / 60 % 60, secs % 60)
}

pub fn fmt_size(bytes: i64) -> String {
    let b = bytes as f64;
    if b >= 1e9 {
        format!("{:.2} GB", b / 1e9)
    } else {
        format!("{:.0} MB", b / 1e6)
    }
}

pub fn fmt_bps(bps: i64) -> String {
    if bps >= 1_000_000 {
        format!("{:.1} Mbps", bps as f64 / 1e6)
    } else {
        format!("{} kbps", bps / 1000)
    }
}

