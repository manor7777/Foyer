//! Hooks the player up to the platform's system media session (Android notification shade, lock
//! screen and headset buttons) and picture-in-picture. On other platforms the platform calls are
//! no-ops, so this costs one cheap timer tick a second.

use super::*;
use platform::MediaAction;

impl Ctx {
    /// A button on the system's media controls.
    pub fn media_action(&self, a: MediaAction) {
        let Some(ui) = self.ui.upgrade() else { return };
        if !ui.get_playing() {
            return;
        }
        let toggle = |ui: &AppWindow| ui.invoke_player_key("pause".into());
        match a {
            MediaAction::Play => {
                if ui.get_player_paused() {
                    toggle(&ui);
                }
            }
            MediaAction::Pause => {
                if !ui.get_player_paused() {
                    toggle(&ui);
                }
            }
            MediaAction::Next => {
                if ui.get_player_has_next() {
                    ui.invoke_player_key("next".into());
                }
            }
            MediaAction::Prev => {
                if ui.get_player_has_prev() {
                    ui.invoke_player_key("prev".into());
                }
            }
            MediaAction::Stop => ui.invoke_player_key("quit".into()),
            MediaAction::Seek(ms) => ui.invoke_player_seek(ms as f32 / 1000.0),
            MediaAction::Forward => ui.invoke_player_seek((ui.get_player_pos() + 10.0).min(ui.get_player_dur())),
            MediaAction::Rewind => ui.invoke_player_seek((ui.get_player_pos() - 10.0).max(0.0)),
        }
    }

    /// Once a second: publish what is playing, keep Android's auto picture-in-picture armed, and
    /// follow the system PiP window (it shrinks the app, so the UI must hide its controls).
    pub fn sysmedia_tick(&self, armed: &std::cell::Cell<Option<(bool, i32)>>) {
        let Some(ui) = self.ui.upgrade() else { return };
        let active = ui.get_playing() && ui.get_player_started();
        let aspect = ui.get_video_aspect();
        if active {
            let sub = String::new();
            platform::media_update(ui.get_player_title().as_str(), &sub, !ui.get_player_paused(), (ui.get_player_pos() * 1000.0) as i64, (ui.get_player_dur() * 1000.0) as i64);
        } else {
            platform::media_stop();
        }
        let key = Some((active, (aspect * 100.0) as i32));
        if armed.replace(key) != key {
            platform::set_pip_auto(active, aspect);
        }
        let in_pip = active && platform::in_system_pip();
        if in_pip != ui.get_pip() && (in_pip || ui.get_pip()) && cfg!(target_os = "android") {
            ui.set_pip(in_pip);
        }
    }
}
