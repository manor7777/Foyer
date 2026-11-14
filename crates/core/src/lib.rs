//! Platform-neutral logic for Foyer. See Cargo.toml.

mod config;
mod format;
mod quality;
pub mod color;

pub use config::*;
pub use format::*;
pub use quality::*;

use std::sync::OnceLock;

type LogFn = fn(&str);
static LOG: OnceLock<LogFn> = OnceLock::new();

/// The app registers its log writer once at start-up; the core logs through it.
pub fn set_log(f: LogFn) {
    let _ = LOG.set(f);
}

pub(crate) fn log(msg: &str) {
    if let Some(f) = LOG.get() {
        f(msg);
    }
}
