//! Window operations that go through winit, shared by the Windows and desktop backends.
//! Android has no cursor, window frame or movable window, so it supplies no-ops instead.
use slint::winit_030::{winit, EventResult, WinitWindowAccessor};

pub fn set_cursor_visible(w: &slint::Window, visible: bool) {
    w.with_winit_window(|win| win.set_cursor_visible(visible));
}

pub fn drag_window(w: &slint::Window) {
    w.with_winit_window(|win| {
        let _ = win.drag_window();
    });
}

pub fn set_decorations(w: &slint::Window, on: bool) {
    w.with_winit_window(|win| win.set_decorations(on));
}

/// Position and size of the monitor the window is on, in physical pixels.
pub fn monitor_area(w: &slint::Window) -> Option<(i32, i32, u32, u32)> {
    w.with_winit_window(|win| win.current_monitor().map(|m| (m.position().x, m.position().y, m.size().width, m.size().height))).flatten()
}

/// Un-minimises and focuses the window.
pub fn bring_to_front(w: &slint::Window) {
    w.with_winit_window(|win| {
        win.set_minimized(false);
        win.focus_window();
    });
}

/// `on_back` runs for the mouse's Back thumb button and the keyboard's Browser Back key;
/// `on_cursor_left` when the pointer leaves the window.
pub fn watch_window_events(w: &slint::Window, on_back: impl Fn() + 'static, on_cursor_left: impl Fn() + 'static) {
    use winit::event::{ElementState, MouseButton, WindowEvent};
    use winit::keyboard::{Key, NamedKey};
    w.on_winit_window_event(move |_, ev| {
        let back = match ev {
            WindowEvent::MouseInput { state: ElementState::Pressed, button: MouseButton::Back, .. } => true,
            WindowEvent::KeyboardInput { event, .. } => event.state == ElementState::Pressed && event.logical_key == Key::Named(NamedKey::BrowserBack),
            _ => false,
        };
        if back {
            on_back();
            return EventResult::PreventDefault;
        }
        if matches!(ev, WindowEvent::CursorLeft { .. }) {
            on_cursor_left();
        }
        EventResult::Propagate
    });
}
