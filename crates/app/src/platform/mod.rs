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
