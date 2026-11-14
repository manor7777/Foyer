//! Linux and macOS: enough to build and run the shared code for development and tests.
use std::path::{Path, PathBuf};

pub fn form_factor() -> super::FormFactor {
    super::FormFactor::Desktop
}

pub const SUPPORTS_MATERIAL: bool = false;

pub fn data_base() -> PathBuf {
    std::env::var_os("XDG_DATA_HOME")
        .map(PathBuf::from)
        .or_else(|| std::env::var_os("HOME").map(|h| PathBuf::from(h).join(".local/share")))
        .unwrap_or_else(std::env::temp_dir)
}

pub fn os_version() -> String {
    String::new()
}

pub fn gpu_names() -> Vec<String> {
    Vec::new()
}

pub fn system_accent() -> Option<u32> {
    None
}

pub fn set_material(_window: &slint::Window, _idx: usize) {}

pub fn open_folder(_dir: &Path) {}

pub fn system_notify(title: &str, body: &str) {
    let _ = notify_rust::Notification::new().appname("Foyer").summary(title).body(body).show();
}

/// Phones turn to landscape while a video plays; nothing to do here.
pub fn set_landscape(_on: bool) {}
