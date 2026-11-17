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

/// Starts a resize drag from an edge or corner of a frameless window (picture-in-picture).
/// `dir`: 0 north, 1 south, 2 east, 3 west, 4 north-east, 5 north-west, 6 south-east, 7 south-west.
pub fn drag_resize(w: &slint::Window, dir: i32) {
    use winit::window::ResizeDirection as R;
    let d = match dir {
        0 => R::North,
        1 => R::South,
        2 => R::East,
        3 => R::West,
        4 => R::NorthEast,
        5 => R::NorthWest,
        6 => R::SouthEast,
        7 => R::SouthWest,
        _ => return,
    };
    w.with_winit_window(|win| {
        let _ = win.drag_resize_window(d);
    });
}

/// Moves a finished download into the user's Downloads\Foyer folder. Returns where it went.
pub fn save_to_downloads(src: &std::path::Path, name: &str) -> Result<String, String> {
    let home = std::env::var_os("USERPROFILE").or_else(|| std::env::var_os("HOME")).ok_or("no home folder")?;
    let dir = std::path::PathBuf::from(home).join("Downloads").join("Foyer");
    std::fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let dest = dir.join(name);
    if std::fs::rename(src, &dest).is_err() {
        std::fs::copy(src, &dest).map_err(|e| e.to_string())?;
        let _ = std::fs::remove_file(src);
    }
    Ok(dest.to_string_lossy().into_owned())
}

// Resizing the frameless picture-in-picture window by hand: asking Windows to resize it and then correcting the
// shape afterwards made it judder, so on Windows Foyer follows the mouse itself and moves + resizes in one call.

/// True where `cursor_pos`, `left_button_down` and `set_window_rect` work.
pub const CUSTOM_RESIZE: bool = cfg!(windows);

/// Mouse position on screen in physical pixels.
#[cfg(windows)]
pub fn cursor_pos() -> Option<(i32, i32)> {
    use windows_sys::Win32::{Foundation::POINT, UI::WindowsAndMessaging::GetCursorPos};
    let mut p = POINT { x: 0, y: 0 };
    // SAFETY: plain out-parameter call.
    (unsafe { GetCursorPos(&mut p) } != 0).then_some((p.x, p.y))
}

#[cfg(windows)]
pub fn left_button_down() -> bool {
    use windows_sys::Win32::UI::Input::KeyboardAndMouse::{GetAsyncKeyState, VK_LBUTTON};
    // SAFETY: plain call.
    unsafe { GetAsyncKeyState(VK_LBUTTON as i32) as u16 & 0x8000 != 0 }
}

/// Moves and resizes the window (outer rectangle, physical pixels) in one step.
#[cfg(windows)]
pub fn set_window_rect(w: &slint::Window, x: i32, y: i32, width: i32, height: i32) {
    use windows_sys::Win32::UI::WindowsAndMessaging::{SetWindowPos, SWP_NOACTIVATE, SWP_NOZORDER};
    use winit::raw_window_handle::{HasWindowHandle, RawWindowHandle};
    w.with_winit_window(|win| {
        if let Ok(RawWindowHandle::Win32(h)) = win.window_handle().map(|h| h.as_raw()) {
            // SAFETY: the handle belongs to this live window.
            unsafe { SetWindowPos(h.hwnd.get() as _, std::ptr::null_mut(), x, y, width, height, SWP_NOZORDER | SWP_NOACTIVATE) };
        }
    });
}

#[cfg(not(windows))]
pub fn cursor_pos() -> Option<(i32, i32)> {
    None
}
#[cfg(not(windows))]
pub fn left_button_down() -> bool {
    false
}
#[cfg(not(windows))]
pub fn set_window_rect(_w: &slint::Window, _x: i32, _y: i32, _width: i32, _height: i32) {}
