//! Everything that differs per operating system lives behind this module, so the rest of the app
//! (UI, Emby/Seerr logic, playback control) is the same on every platform.
//!
//! Each backend exports the same functions:
//! - `data_base()`: folder that holds the app's data folder
//! - `os_version()`, `gpu_names()`: diagnostics and hardware-decode choice
//! - `system_accent()`: the OS accent colour as 0xRRGGBB, if it has one
//! - `SUPPORTS_MATERIAL` / `set_material()`: Mica/Acrylic window backdrops
//! - `system_notify()`: a system notification (the in-app banner is separate)
//! - `open_folder()`: reveal a folder in the file manager, where there is one
//! - window ops (`set_cursor_visible`, `drag_window`, `set_decorations`, `monitor_area`, `bring_to_front`,
//!   `watch_window_events`): winit-backed on desktop, no-ops on Android

#[cfg(not(target_os = "android"))]
mod winit_ops;
#[cfg(not(target_os = "android"))]
pub use winit_ops::*;

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::*;

#[cfg(target_os = "android")]
mod android;
#[cfg(target_os = "android")]
pub use android::*;

#[cfg(not(any(windows, target_os = "android")))]
mod desktop;
#[cfg(not(any(windows, target_os = "android")))]
pub use desktop::*;

/// What the app is running on. Decides tap-target sizes and whether a remote's D-pad drives focus.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FormFactor {
    /// Mouse and keyboard (Windows, or Linux for development).
    Desktop,
    /// Android phone or tablet.
    Touch,
    /// Android TV / Google TV / Chromecast with Google TV (remote control only).
    Tv,
}

/// A button pressed on the system's media controls (notification shade, lock screen, headset).
#[derive(Clone, Copy, Debug)]
#[cfg_attr(not(target_os = "android"), allow(dead_code))]
pub enum MediaAction {
    Play,
    Pause,
    Next,
    Prev,
    Stop,
    Forward,
    Rewind,
    /// Absolute position in milliseconds.
    Seek(i64),
}

// Real picture-in-picture and the system media session exist on Android only (Windows has its own
// media-key integration in `media.rs`); everywhere else these do nothing.
#[cfg(not(target_os = "android"))]
pub fn enter_pip(_aspect: f32) -> bool {
    false
}
#[cfg(not(target_os = "android"))]
pub fn in_system_pip() -> bool {
    false
}
#[cfg(not(target_os = "android"))]
pub fn set_pip_auto(_on: bool, _aspect: f32) {}
#[cfg(not(target_os = "android"))]
pub fn set_media_handler(_f: Box<dyn Fn(MediaAction) + Send>) {}
#[cfg(not(target_os = "android"))]
pub fn media_update(_title: &str, _subtitle: &str, _playing: bool, _pos_ms: i64, _dur_ms: i64) {}
#[cfg(not(target_os = "android"))]
pub fn media_stop() {}
