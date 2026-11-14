use std::path::{Path, PathBuf};

pub fn form_factor() -> super::FormFactor {
    super::FormFactor::Desktop
}

pub const SUPPORTS_MATERIAL: bool = true;

pub fn data_base() -> PathBuf {
    std::env::var_os("APPDATA").map(PathBuf::from).unwrap_or_else(std::env::temp_dir)
}

pub fn os_version() -> String {
    use std::os::windows::process::CommandExt;
    std::process::Command::new("cmd")
        .args(["/C", "ver"])
        .creation_flags(0x0800_0000)
        .output()
        .map(|o| String::from_utf8_lossy(&o.stdout).trim().to_string())
        .unwrap_or_default()
}

pub fn open_folder(dir: &Path) {
    let _ = std::process::Command::new("explorer").arg(dir).spawn();
}

/// Identity Windows shows on toasts. Without a registered one notify-rust borrows PowerShell's, so
/// every Foyer notification said "Windows PowerShell".
const APP_ID: &str = "Foyer.Foyer";

pub fn system_notify(title: &str, body: &str) {
    let _ = notify_rust::Notification::new().appname("Foyer").app_id(APP_ID).summary(title).body(body).show();
}

/// Registers Foyer's name and icon for toast notifications (HKCU, no admin needed). Safe to call on
/// every start; runs in the background because it starts two short-lived `reg` processes.
pub fn register_app_identity() {
    std::thread::spawn(|| {
        use std::os::windows::process::CommandExt;
        let dir = foyer_core::data_dir();
        let _ = std::fs::create_dir_all(&dir);
        let icon = dir.join("app-icon.png");
        if std::fs::metadata(&icon).map(|m| m.len() == 0).unwrap_or(true) {
            let _ = std::fs::write(&icon, include_bytes!("../../assets/app.png"));
        }
        let key = format!("HKCU\\Software\\Classes\\AppUserModelId\\{APP_ID}");
        let set = |name: &str, value: &str| {
            let _ = std::process::Command::new("reg")
                .args(["add", &key, "/v", name, "/t", "REG_SZ", "/d", value, "/f"])
                .creation_flags(0x0800_0000)
                .output();
        };
        set("DisplayName", "Foyer");
        set("IconUri", &icon.to_string_lossy());
    });
}

/// Window material: 0 solid, 1 Mica, 2 Acrylic (Windows 11; needs the transparent window).
pub fn set_material(window: &slint::Window, idx: usize) {
    use slint::winit_030::{winit::platform::windows::{BackdropType, WindowExtWindows}, WinitWindowAccessor};
    let kind = match idx {
        1 => BackdropType::MainWindow,
        2 => BackdropType::TransientWindow,
        _ => BackdropType::None,
    };
    window.with_winit_window(|w| w.set_system_backdrop(kind));
}

/// The accent colour set in Windows (Settings > Personalization > Colours), as 0xRRGGBB.
pub fn system_accent() -> Option<u32> {
    use windows_sys::Win32::System::Registry::{RegGetValueW, HKEY_CURRENT_USER, RRF_RT_REG_DWORD};
    let sub: Vec<u16> = "Software\\Microsoft\\Windows\\DWM\0".encode_utf16().collect();
    let val: Vec<u16> = "AccentColor\0".encode_utf16().collect();
    let mut data: u32 = 0;
    let mut size: u32 = 4;
    // SAFETY: valid NUL-terminated wide strings and a correctly sized DWORD out buffer.
    let rc = unsafe {
        RegGetValueW(HKEY_CURRENT_USER, sub.as_ptr(), val.as_ptr(), RRF_RT_REG_DWORD, std::ptr::null_mut(), &mut data as *mut u32 as *mut _, &mut size)
    };
    if rc != 0 {
        return None;
    }
    // stored as 0xAABBGGRR
    Some(((data & 0xFF) << 16) | (data & 0xFF00) | ((data >> 16) & 0xFF))
}


/// Names of the installed display adapters (Windows registry), e.g. "Intel(R) UHD Graphics".
pub fn gpu_names() -> Vec<String> {
    use windows_sys::Win32::System::Registry::{RegCloseKey, RegEnumKeyExW, RegGetValueW, RegOpenKeyExW, HKEY_LOCAL_MACHINE, KEY_READ, RRF_RT_REG_SZ};
    let base: Vec<u16> = r"SYSTEM\CurrentControlSet\Control\Class\{4d36e968-e325-11cd-be10-08002be10318}".encode_utf16().chain(Some(0)).collect();
    let mut out = Vec::new();
    // SAFETY: valid NUL-terminated wide strings, correctly sized buffers, handle closed after use.
    unsafe {
        let mut key = std::ptr::null_mut();
        if RegOpenKeyExW(HKEY_LOCAL_MACHINE, base.as_ptr(), 0, KEY_READ, &mut key) != 0 {
            return out;
        }
        let mut i = 0;
        loop {
            let mut name = [0u16; 64];
            let mut len = name.len() as u32;
            if RegEnumKeyExW(key, i, name.as_mut_ptr(), &mut len, std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut(), std::ptr::null_mut()) != 0 {
                break;
            }
            i += 1;
            let sub: Vec<u16> = String::from_utf16_lossy(&name[..len as usize]).encode_utf16().chain(Some(0)).collect();
            let val: Vec<u16> = "DriverDesc".encode_utf16().chain(Some(0)).collect();
            let mut buf = [0u16; 256];
            let mut size = (buf.len() * 2) as u32;
            if RegGetValueW(key, sub.as_ptr(), val.as_ptr(), RRF_RT_REG_SZ, std::ptr::null_mut(), buf.as_mut_ptr() as *mut _, &mut size) == 0 {
                let n = (size as usize / 2).saturating_sub(1).min(buf.len());
                let d = String::from_utf16_lossy(&buf[..n]);
                if !d.is_empty() && !d.to_lowercase().contains("basic") && !d.to_lowercase().contains("remote") {
                    out.push(d);
                }
            }
        }
        RegCloseKey(key);
    }
    out
}


/// Phones turn to landscape while a video plays; nothing to do here.
pub fn set_landscape(_on: bool) {}
