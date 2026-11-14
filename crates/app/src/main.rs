#![cfg_attr(all(not(debug_assertions), windows), windows_subsystem = "windows")]

fn main() -> Result<(), slint::PlatformError> {
    foyer::run()
}
