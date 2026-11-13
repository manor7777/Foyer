//! Windows media controls: the flyout / lock screen, media keys and remote-control apps.
//! They show what's playing and send play / pause / stop / next / previous back to the player.
use crate::AppWindow;
use player::Control;
use slint::winit_030::{winit::raw_window_handle::{HasWindowHandle, RawWindowHandle}, WinitWindowAccessor};
use slint::ComponentHandle;
use std::{cell::RefCell, sync::mpsc::Sender};
use windows::{
    core::{factory, HSTRING},
    Foundation::TypedEventHandler,
    Media::{MediaPlaybackStatus, MediaPlaybackType, SystemMediaTransportControls, SystemMediaTransportControlsButton as Btn},
    Win32::{Foundation::HWND, System::WinRT::ISystemMediaTransportControlsInterop},
};

struct Media {
    smtc: SystemMediaTransportControls,
    token: i64,
    last: (String, bool, bool, bool),
}

impl Media {
    fn new(hwnd: isize, tx: Sender<Control>) -> windows::core::Result<Self> {
        let interop = factory::<SystemMediaTransportControls, ISystemMediaTransportControlsInterop>()?;
        // SAFETY: hwnd is this process's live top-level window.
        let smtc: SystemMediaTransportControls = unsafe { interop.GetForWindow(HWND(hwnd as *mut _)) }?;
        smtc.SetIsEnabled(true)?;
        smtc.SetIsPlayEnabled(true)?;
        smtc.SetIsPauseEnabled(true)?;
        smtc.SetIsStopEnabled(true)?;
        let token = smtc.ButtonPressed(&TypedEventHandler::new(move |_, args: windows::core::Ref<'_, windows::Media::SystemMediaTransportControlsButtonPressedEventArgs>| {
            if let Some(a) = args.as_ref() {
                let _ = match a.Button()? {
                    Btn::Play => tx.send(Control::SetPause(false)),
                    Btn::Pause => tx.send(Control::SetPause(true)),
                    Btn::Stop => tx.send(Control::Quit),
                    Btn::Next => Ok(crate::goto_with(&tx, false)),
                    Btn::Previous => Ok(crate::goto_with(&tx, true)),
                    _ => Ok(()),
                };
            }
            Ok(())
        }))?;
        Ok(Self { smtc, token, last: Default::default() })
    }

    fn update(&mut self, title: &str, paused: bool, prev: bool, next: bool) -> windows::core::Result<()> {
        if self.last.0 != title {
            let du = self.smtc.DisplayUpdater()?;
            du.SetType(MediaPlaybackType::Video)?;
            du.VideoProperties()?.SetTitle(&HSTRING::from(title))?;
            du.Update()?;
        }
        self.smtc.SetPlaybackStatus(if paused { MediaPlaybackStatus::Paused } else { MediaPlaybackStatus::Playing })?;
        self.smtc.SetIsPreviousEnabled(prev)?;
        self.smtc.SetIsNextEnabled(next)?;
        self.last = (title.to_string(), paused, prev, next);
        Ok(())
    }
}

impl Drop for Media {
    fn drop(&mut self) {
        // The window's controls object is shared, so detach our handler and hide the entry.
        let _ = self.smtc.RemoveButtonPressed(self.token);
        let _ = self.smtc.SetPlaybackStatus(MediaPlaybackStatus::Closed);
        let _ = self.smtc.SetIsEnabled(false);
    }
}

thread_local! {
    // Outer None: not tried yet this session. Inner None: Windows refused, don't retry every tick.
    static CUR: RefCell<Option<Option<Media>>> = const { RefCell::new(None) };
}

/// Call on the UI thread with the current player state (cheap: only talks to Windows on change).
pub fn sync(ui: &AppWindow, tx: &Sender<Control>) {
    CUR.with(|c| {
        let mut c = c.borrow_mut();
        let m = c.get_or_insert_with(|| {
            let hwnd = ui.window().with_winit_window(|w| match w.window_handle().ok()?.as_raw() {
                RawWindowHandle::Win32(h) => Some(h.hwnd.get()),
                _ => None,
            });
            hwnd.flatten().and_then(|h| Media::new(h, tx.clone()).ok())
        });
        if let Some(m) = m {
            let _ = m.update(&ui.get_player_title(), ui.get_player_paused(), ui.get_player_has_prev(), ui.get_player_has_next());
        }
    });
}

/// Playback is over: take the entry out of the flyout.
pub fn close() {
    CUR.with(|c| *c.borrow_mut() = None);
}
