//! Settings (config.json), where Foyer keeps its files, and the home-row order.

use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::OnceLock;

static BASE: OnceLock<PathBuf> = OnceLock::new();

/// Called once at start-up with the folder the OS keeps app data in (`%APPDATA%`, or the app's
/// files folder on Android).
pub fn set_data_base(base: PathBuf) {
    let _ = BASE.set(base);
}

/// Foyer's own data folder.
pub fn data_dir() -> PathBuf {
    let base = BASE.get().cloned().unwrap_or_else(std::env::temp_dir);
    let dir = base.join("Foyer");
    // Carry over settings and sign-in from the pre-rename folder.
    let old = base.join("EmbyRevived");
    if !dir.exists() && old.exists() {
        let _ = std::fs::rename(&old, &dir);
    }
    dir
}

/// A series the user asked to be told about. `known` is every episode id seen so far.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Follow {
    pub id: String,
    pub name: String,
    pub known: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Hide {
    pub id: String,
    pub name: String,
}


fn yes() -> bool {
    true
}

// Container-level default: any field missing from an older config.json falls back to its default,
// so adding a setting can never make an existing file unreadable.
#[derive(Serialize, Deserialize, Clone)]
#[serde(default)]
pub struct Config {
    /// The active server (mirrors its entry in `servers`).
    pub server: String,
    #[serde(default)]
    pub kind: emby_api::Kind,
    /// Show the server switcher in the sidebar and keep several servers signed in.
    #[serde(default)]
    pub multi_server: bool,
    #[serde(default)]
    pub servers: Vec<SavedServer>,
    pub user_id: String,
    pub user_name: String,
    pub token: String,
    pub device_id: String,
    #[serde(default)]
    pub search_history: Vec<String>,
    /// "" = never chosen (Original/direct); "auto"; "direct"; or a preset bitrate in bps.
    #[serde(default)]
    pub quality: String,
    #[serde(default = "yes")]
    pub autoplay_next: bool,
    /// Show the time of day in the player's top-right corner.
    #[serde(default = "yes")]
    pub show_clock: bool,
    /// 24-hour clock instead of 12-hour (e.g. 17:05 rather than 5:05pm).
    #[serde(default)]
    pub clock_24h: bool,
    #[serde(default)]
    pub auto_skip_intro: bool,
    /// Seconds the rewind / skip-forward buttons and arrow keys jump.
    #[serde(default = "default_rewind_secs")]
    pub rewind_secs: i32,
    #[serde(default = "default_forward_secs")]
    pub forward_secs: i32,
    #[serde(default = "yes")]
    pub hw_decode: bool,
    #[serde(default)]
    pub hw_mode: usize,
    #[serde(default)]
    pub always_on_top: bool,
    /// Index into SUB_LANGS.
    #[serde(default)]
    pub sub_lang: usize,
    /// The subtitle language last picked on a detail page (3-letter code), or "off"; empty = no preference.
    #[serde(default)]
    pub sub_pref: String,
    /// 0 solid, 1 Mica, 2 Acrylic.
    #[serde(default)]
    pub material: usize,
    /// 0 = follow the Windows accent colour, 1 = the picked colour below.
    #[serde(default = "default_accent_mode")]
    pub accent_mode: usize,
    /// Picked accent colour as 0xRRGGBB.
    #[serde(default = "default_accent_rgb")]
    pub accent_rgb: u32,
    /// Keep the sidebar expanded at any window size.
    #[serde(default)]
    pub nav_pinned: bool,
    /// Fade the background to a hovered poster's banner art.
    #[serde(default)]
    pub hover_backdrop: bool,
    /// Library sort order (index into SORTS).
    #[serde(default)]
    pub library_sort: usize,
    #[serde(default)]
    pub seerr_enabled: bool,
    #[serde(default)]
    pub seerr_url: String,
    /// Seerr session cookie (the only credential kept; no password or API key).
    #[serde(default)]
    pub seerr_cookie: String,
    /// Home rows the user switched off: continue, nextup, requests, media, latest.
    #[serde(default)]
    pub hidden_rows: Vec<String>,
    #[serde(default)]
    pub row_order: Vec<String>,
    #[serde(default = "yes")]
    pub notify_enabled: bool,
    #[serde(default = "yes")]
    pub notify_movies: bool,
    #[serde(default = "yes")]
    pub notify_episodes: bool,
    #[serde(default = "yes")]
    pub notify_windows: bool,
    /// 0 = every batch of new episodes, 1 = only when a whole season is ready.
    #[serde(default)]
    pub notify_mode: usize,
    /// Index into seerr::NOTIFY_INTERVALS (minutes).
    #[serde(default = "default_notify_interval")]
    pub notify_interval: usize,
    #[serde(default = "yes")]
    pub notify_issues: bool,
    #[serde(default = "yes")]
    pub notify_my_issues: bool,
    /// Look for a newer release when Foyer starts.
    #[serde(default = "yes")]
    pub update_check: bool,
    /// Install it automatically (applies on the next start).
    #[serde(default = "yes")]
    pub update_install: bool,
    #[serde(default)]
    pub debug_logging: bool,
    /// Closing the window hides it to the tray icon instead of quitting.
    #[serde(default)]
    pub close_to_tray: bool,
    /// Index into CARD_SCALES; 2 = default size.
    #[serde(default = "default_card_scale")]
    pub card_scale: usize,
    #[serde(default)]
    pub card_auto: bool,
    /// Last window size (physical pixels) and whether it was maximised.
    #[serde(default)]
    pub win_w: u32,
    #[serde(default)]
    pub win_h: u32,
    #[serde(default)]
    pub win_max: bool,
    /// Series the user wants new-episode notifications for.
    #[serde(default)]
    pub followed: Vec<Follow>,
    /// Blank the episode descriptions of unwatched episodes.
    pub hide_spoilers: bool,
    /// Take a title off the Seerr watchlist once it has been watched.
    pub watchlist_autoremove: bool,
    /// Last Emby server version we announced an update for.
    pub server_update_seen: String,
    /// Titles hidden from every list.
    pub hidden_items: Vec<Hide>,
    /// Movies, series and seasons removed from Continue Watching / Next Up.
    pub cw_hidden: Vec<Hide>,
}

pub const CARD_SCALES: [f32; 5] = [0.6, 0.8, 1.0, 1.15, 1.3];

fn default_card_scale() -> usize {
    2
}

fn default_notify_interval() -> usize {
    1
}

fn default_rewind_secs() -> i32 {
    10
}

fn default_forward_secs() -> i32 {
    30
}

/// Choices offered in Settings for the rewind / skip-forward step.
/// After this many episodes autoplay in a row, stop and ask whether you're still watching.
pub const AUTOPLAY_STREAK_LIMIT: u32 = 5;

pub const SKIP_STEPS: [i32; 7] = [5, 10, 15, 30, 45, 60, 90];

pub fn skip_step_index(secs: i32) -> i32 {
    SKIP_STEPS.iter().position(|&v| v == secs).unwrap_or(1) as i32
}

static DEFAULT_ACCENT_MODE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(1);

/// Platforms with a system accent (Windows, Android's Material You) make "follow the system" the
/// default for new installs. Call before loading the config.
pub fn set_default_accent_mode(mode: usize) {
    DEFAULT_ACCENT_MODE.store(mode, std::sync::atomic::Ordering::Relaxed);
}

fn default_accent_mode() -> usize {
    DEFAULT_ACCENT_MODE.load(std::sync::atomic::Ordering::Relaxed)
}

fn default_accent_rgb() -> u32 {
    0x60CDFF
}

impl Default for Config {
    fn default() -> Self {
        Config {
            server: String::new(),
            kind: emby_api::Kind::Emby,
            multi_server: false,
            servers: Vec::new(),
            user_id: String::new(),
            user_name: String::new(),
            token: String::new(),
            device_id: String::new(),
            search_history: Vec::new(),
            quality: String::new(),
            autoplay_next: true,
            show_clock: true,
            clock_24h: false,
            hw_decode: true,
            auto_skip_intro: false,
            rewind_secs: default_rewind_secs(),
            forward_secs: default_forward_secs(),
            hw_mode: 0,
            always_on_top: false,
            sub_lang: 0,
            sub_pref: String::new(),
            material: 0,
            accent_mode: default_accent_mode(),
            accent_rgb: default_accent_rgb(),
            nav_pinned: false,
            hover_backdrop: false,
            library_sort: 0,
            seerr_enabled: false,
            seerr_url: String::new(),
            seerr_cookie: String::new(),
            hidden_rows: Vec::new(),
            row_order: Vec::new(),
            notify_enabled: true,
            notify_movies: true,
            notify_episodes: true,
            notify_windows: true,
            notify_mode: 0,
            notify_interval: default_notify_interval(),
            notify_issues: true,
            notify_my_issues: true,
            update_check: true,
            update_install: true,
            debug_logging: false,
            close_to_tray: false,
            card_scale: default_card_scale(),
            card_auto: false,
            win_w: 0,
            win_h: 0,
            win_max: false,
            followed: Vec::new(),
            hide_spoilers: false,
            watchlist_autoremove: false,
            server_update_seen: String::new(),
            hidden_items: Vec::new(),
            cw_hidden: Vec::new(),
        }
    }
}


#[derive(Serialize, Deserialize, Clone)]
pub struct SavedServer {
    pub kind: emby_api::Kind,
    pub url: String,
    pub user_id: String,
    pub user_name: String,
    pub token: String,
}


pub fn load_config() -> Config {
    let path = data_dir().join("config.json");
    let mut c: Config = match std::fs::read_to_string(&path) {
        Ok(text) => match serde_json::from_str(&text) {
            Ok(c) => c,
            Err(e) => {
                // Never silently replace a file we couldn't read: keep it so nothing is lost.
                crate::log(&format!("config.json unreadable ({e}); kept as config.bad.json"));
                let _ = std::fs::copy(&path, data_dir().join("config.bad.json"));
                Config::default()
            }
        },
        Err(_) => Config::default(),
    };
    if c.device_id.is_empty() {
        c.device_id = uuid::Uuid::new_v4().simple().to_string();
        save_config(&c);
    }
    c
}

// shortcut: token is stored in plain JSON under %APPDATA%; move to Windows Credential Manager.
pub fn save_config(c: &Config) {
    let _ = std::fs::create_dir_all(data_dir());
    if let Ok(s) = serde_json::to_string_pretty(c) {
        // Write to a temp file and rename, so a crash mid-write can't leave a truncated config.
        let tmp = data_dir().join("config.json.tmp");
        if std::fs::write(&tmp, s).is_ok() {
            let _ = std::fs::rename(&tmp, data_dir().join("config.json"));
        }
    }
}


pub const HOME_ROWS: [(&str, &str, &str); 6] = [
    ("media", "My Media", "Shortcuts to your libraries."),
    ("continue", "Continue Watching", "Shows what you were part-way through."),
    ("nextup", "Next Up", "The episode after the one you are on."),
    ("requests", "My requests", "Your recent Seerr requests with their status. Needs Requests to be enabled."),
    ("latest", "Recently added", "Titles added to each library on the server lately, whatever their age."),
    ("released", "Recently released", "Movies and shows with the newest release or air dates."),
];

/// The saved row order, with any rows it doesn't mention appended in the default order.
pub fn home_order(cfg: &Config) -> Vec<String> {
    let mut out: Vec<String> = cfg.row_order.iter().filter(|k| HOME_ROWS.iter().any(|r| r.0 == k.as_str())).cloned().collect();
    out.dedup();
    for (k, _, _) in HOME_ROWS {
        if !out.iter().any(|o| o == k) {
            out.push(k.to_string());
        }
    }
    out
}


#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn old_config_without_new_fields_still_loads() {
        let old = r#"{"server":"http://x:8096","user_id":"u","user_name":"n","token":"t","device_id":"d","debug_logging":true}"#;
        let c: Config = serde_json::from_str(old).expect("old config must still parse");
        assert_eq!(c.token, "t");
        assert!(c.debug_logging);
        assert_eq!(c.hw_mode, 0);
        assert!(c.row_order.is_empty());
        assert!(c.hw_decode && c.notify_issues);
    }
}
