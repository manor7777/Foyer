//! Android and Android TV. Stubs for now; each is replaced as the matching step of
//! docs/ANDROID-PLAN.md lands (data dir from the app's files dir, Material You accent,
//! system notifications through JNI).
use std::path::{Path, PathBuf};

pub const SUPPORTS_MATERIAL: bool = false;

pub fn data_base() -> PathBuf {
    // Set from the activity's files dir at startup (see the Android entry point).
    std::env::var_os("FOYER_DATA_DIR").map(PathBuf::from).unwrap_or_else(std::env::temp_dir)
}

pub fn os_version() -> String {
    "Android".into()
}

/// MediaCodec does the decoding, so there is no GPU choice to make.
pub fn gpu_names() -> Vec<String> {
    Vec::new()
}

pub fn system_accent() -> Option<u32> {
    None
}

pub fn set_material(_window: &slint::Window, _idx: usize) {}

pub fn open_folder(_dir: &Path) {}

pub fn system_notify(_title: &str, _body: &str) {}

// No cursor, frame or movable window on Android.
pub fn set_cursor_visible(_w: &slint::Window, _visible: bool) {}
pub fn drag_window(_w: &slint::Window) {}
pub fn set_decorations(_w: &slint::Window, _on: bool) {}
pub fn monitor_area(_w: &slint::Window) -> Option<(i32, i32, u32, u32)> {
    None
}
pub fn bring_to_front(_w: &slint::Window) {}
pub fn watch_window_events(_w: &slint::Window, _on_back: impl Fn() + 'static, _on_cursor_left: impl Fn() + 'static) {}
