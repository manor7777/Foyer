//! Android and Android TV. Talks to Android's Java APIs (notifications, package installer,
//! permissions) through JNI. Everything here is best effort: a failure is logged and returned,
//! never a crash.
use jni::objects::{GlobalRef, JObject, JValue};
use jni::{JNIEnv, JavaVM};
use std::ffi::c_void;
use std::path::{Path, PathBuf};
use std::sync::OnceLock;

#[path = "android_media.rs"]
mod android_media;
pub use android_media::*;
#[path = "android_notify.rs"]
mod android_notify;
pub use android_notify::*;

pub const SUPPORTS_MATERIAL: bool = false;

/// Android TV reports itself through UiModeManager (UI_MODE_TYPE_TELEVISION = 4).
pub fn form_factor() -> super::FormFactor {
    with_env(|env, act| {
        let service = env.new_string("uimode")?;
        let mgr = env.call_method(act, "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;", &[JValue::Object(&service)])?.l()?;
        Ok(env.call_method(&mgr, "getCurrentModeType", "()I", &[])?.i()? == 4)
    })
    .map(|tv| if tv { super::FormFactor::Tv } else { super::FormFactor::Touch })
    .unwrap_or(super::FormFactor::Touch)
}

struct AndroidCtx {
    vm: JavaVM,
    activity: GlobalRef,
}

static CTX: OnceLock<AndroidCtx> = OnceLock::new();

/// Called once from `android_main` with the process's JavaVM and the activity object.
pub fn init(vm_ptr: *mut c_void, activity_ptr: *mut c_void) {
    // SAFETY: both pointers come from android-activity and stay valid for the life of the process.
    let Ok(vm) = (unsafe { JavaVM::from_raw(vm_ptr.cast()) }) else { return };
    let activity = {
        let Ok(mut env) = vm.attach_current_thread() else { return };
        let obj = unsafe { JObject::from_raw(activity_ptr as jni::sys::jobject) };
        let Ok(global) = env.new_global_ref(&obj) else { return };
        global
    };
    let _ = CTX.set(AndroidCtx { vm, activity });
}

/// Runs `f` with a JNIEnv attached to this thread and the activity. A pending Java exception is
/// cleared and turned into an error.
fn with_env<T>(f: impl FnOnce(&mut JNIEnv<'_>, &JObject<'_>) -> jni::errors::Result<T>) -> Result<T, String> {
    let ctx = CTX.get().ok_or("Android is not ready yet")?;
    let mut env = ctx.vm.attach_current_thread().map_err(|e| e.to_string())?;
    let result = f(&mut env, ctx.activity.as_obj());
    if env.exception_check().unwrap_or(false) {
        let _ = env.exception_describe();
        let _ = env.exception_clear();
        return Err("An Android call failed".into());
    }
    result.map_err(|e| e.to_string())
}

pub fn data_base() -> PathBuf {
    // Set from the activity's files dir at startup (see the Android entry point).
    std::env::var_os("FOYER_DATA_DIR").map(PathBuf::from).unwrap_or_else(std::env::temp_dir)
}

pub fn os_version() -> String {
    with_env(|env, _| {
        let sdk = env.get_static_field("android/os/Build$VERSION", "SDK_INT", "I")?.i()?;
        Ok(format!("Android (API {sdk})"))
    })
    .unwrap_or_else(|_| "Android".into())
}

/// MediaCodec does the decoding, so there is no GPU choice to make.
pub fn gpu_names() -> Vec<String> {
    Vec::new()
}

/// Material You (Android 12+): the accent colour derived from the wallpaper, as 0xRRGGBB.
pub fn system_accent() -> Option<u32> {
    with_env(|env, act| {
        let sdk = env.get_static_field("android/os/Build$VERSION", "SDK_INT", "I")?.i()?;
        if sdk < 31 {
            return Ok(None);
        }
        let id = env.get_static_field("android/R$color", "system_accent1_500", "I")?.i()?;
        let res = env.call_method(act, "getResources", "()Landroid/content/res/Resources;", &[])?.l()?;
        let argb = env.call_method(&res, "getColor", "(ILandroid/content/res/Resources$Theme;)I", &[JValue::Int(id), JValue::Object(&JObject::null())])?.i()?;
        Ok(Some((argb as u32) & 0x00FF_FFFF))
    })
    .ok()
    .flatten()
}

pub fn set_material(_window: &slint::Window, _idx: usize) {}

pub fn open_folder(_dir: &Path) {}

// No cursor, frame or movable window on Android.
pub fn set_cursor_visible(_w: &slint::Window, _visible: bool) {}
pub fn drag_window(_w: &slint::Window) {}
pub fn drag_resize(_w: &slint::Window, _dir: i32) {}
pub fn set_decorations(_w: &slint::Window, _on: bool) {}
pub fn monitor_area(_w: &slint::Window) -> Option<(i32, i32, u32, u32)> {
    None
}
pub fn bring_to_front(_w: &slint::Window) {}
pub fn watch_window_events(_w: &slint::Window, _on_back: impl Fn() + 'static, _on_cursor_left: impl Fn() + 'static) {}

// ---------------------------------------------------------------- notifications

const CHANNEL: &str = "foyer";

/// Shows an Android notification. On Android 13+ the first call asks for the notification
/// permission instead (nothing is shown until it has been granted).
#[allow(dead_code)]
pub fn system_notify(title: &str, body: &str) {
    let result = with_env(|env, act| {
        let sdk = env.get_static_field("android/os/Build$VERSION", "SDK_INT", "I")?.i()?;
        if sdk >= 33 {
            let perm = env.new_string("android.permission.POST_NOTIFICATIONS")?;
            let granted = env.call_method(act, "checkSelfPermission", "(Ljava/lang/String;)I", &[JValue::Object(&perm)])?.i()?;
            if granted != 0 {
                let string_class = env.find_class("java/lang/String")?;
                let list = env.new_object_array(1, &string_class, &perm)?;
                env.call_method(act, "requestPermissions", "([Ljava/lang/String;I)V", &[JValue::Object(&list), JValue::Int(1)])?;
                return Ok(());
            }
        }
        let id = env.new_string(CHANNEL)?;
        let name = env.new_string("Foyer")?;
        let service = env.new_string("notification")?;
        let manager = env.call_method(act, "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;", &[JValue::Object(&service)])?.l()?;
        let channel = env.new_object("android/app/NotificationChannel", "(Ljava/lang/String;Ljava/lang/CharSequence;I)V", &[JValue::Object(&id), JValue::Object(&name), JValue::Int(3)])?;
        env.call_method(&manager, "createNotificationChannel", "(Landroid/app/NotificationChannel;)V", &[JValue::Object(&channel)])?;
        let icon = env.get_static_field("android/R$drawable", "ic_dialog_info", "I")?.i()?;
        let builder = env.new_object("android/app/Notification$Builder", "(Landroid/content/Context;Ljava/lang/String;)V", &[JValue::Object(act), JValue::Object(&id)])?;
        let title = env.new_string(title)?;
        let body = env.new_string(body)?;
        env.call_method(&builder, "setContentTitle", "(Ljava/lang/CharSequence;)Landroid/app/Notification$Builder;", &[JValue::Object(&title)])?;
        env.call_method(&builder, "setContentText", "(Ljava/lang/CharSequence;)Landroid/app/Notification$Builder;", &[JValue::Object(&body)])?;
        env.call_method(&builder, "setSmallIcon", "(I)Landroid/app/Notification$Builder;", &[JValue::Int(icon)])?;
        env.call_method(&builder, "setAutoCancel", "(Z)Landroid/app/Notification$Builder;", &[JValue::Bool(1)])?;
        let notification = env.call_method(&builder, "build", "()Landroid/app/Notification;", &[])?.l()?;
        let unique = (std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0) & 0x7fff_ffff) as i32;
        env.call_method(&manager, "notify", "(ILandroid/app/Notification;)V", &[JValue::Int(unique), JValue::Object(&notification)])?;
        Ok(())
    });
    if let Err(e) = result {
        eprintln!("notification failed: {e}");
    }
}

// ---------------------------------------------------------------- updates

/// While a video plays on a phone or tablet, turn to landscape (and let go again afterwards).
/// SCREEN_ORIENTATION_SENSOR_LANDSCAPE = 6, SCREEN_ORIENTATION_UNSPECIFIED = -1.
pub fn set_landscape(on: bool) {
    if let Err(e) = with_env(|env, act| {
        env.call_method(act, "setRequestedOrientation", "(I)V", &[JValue::Int(if on { 6 } else { -1 })])?;
        Ok(())
    }) {
        eprintln!("setRequestedOrientation failed: {e}");
    }
    // Hide the navigation bar too, which includes Samsung's taskbar on foldables.
    if let Err(e) = android_media::set_immersive(on) {
        eprintln!("immersive mode failed: {e}");
    }
}

/// Back with nowhere left to go: send the app to the background (like Home) instead of letting
/// Android finish the activity.
pub fn move_to_back() {
    if let Err(e) = with_env(|env, act| {
        env.call_method(act, "moveTaskToBack", "(Z)Z", &[JValue::Bool(1)])?;
        Ok(())
    }) {
        eprintln!("moveTaskToBack failed: {e}");
    }
}

static APP: OnceLock<slint::android::AndroidApp> = OnceLock::new();

/// Called once from `android_main`, so the window flags below can be changed later.
pub fn set_app(app: slint::android::AndroidApp) {
    let _ = APP.set(app);
}

/// Keeps the screen on (no screensaver, no sleep) while a video plays. Android applies the flag on
/// its UI thread; only changes are sent, as the player reports its state several times a second.
pub fn keep_screen_on(on: bool) {
    use slint::android::android_activity::WindowManagerFlags as F;
    use std::sync::atomic::{AtomicU8, Ordering};
    static LAST: AtomicU8 = AtomicU8::new(2);
    // Re-apply every 15 s while on: a flag change can be lost around activity/window changes (the TV went to sleep mid-film).
    static LAST_AT: std::sync::Mutex<Option<std::time::Instant>> = std::sync::Mutex::new(None);
    let stale = on && LAST_AT.lock().map(|mut t| { let s = t.map_or(true, |i| i.elapsed().as_secs() >= 15); if s { *t = Some(std::time::Instant::now()); } s }).unwrap_or(false);
    if LAST.swap(on as u8, Ordering::Relaxed) == on as u8 && !stale {
        return;
    }
    if let Some(app) = APP.get() {
        if on {
            app.set_window_flags(F::KEEP_SCREEN_ON, F::empty());
        } else {
            app.set_window_flags(F::empty(), F::KEEP_SCREEN_ON);
        }
    }
}

/// Windows registers its toast identity here; nothing to do elsewhere.
pub fn register_app_identity() {}

/// Copies the log files to Downloads/Foyer. Returns where they went.
pub fn export_logs(dir: &Path) -> Result<String, String> {
    let stamp = chrono::Local::now().format("%Y%m%d-%H%M%S");
    let mut n = 0;
    for e in std::fs::read_dir(dir).map_err(|e| e.to_string())?.flatten() {
        let p = e.path();
        let Some(name) = p.file_name().and_then(|f| f.to_str()) else { continue };
        android_media::save_to_downloads(&p.to_string_lossy(), &format!("foyer-{stamp}-{name}"))?;
        n += 1;
    }
    if n == 0 {
        return Err("There are no log files yet.".into());
    }
    Ok("Saved to the Downloads/Foyer folder on this device.".into())
}

/// Moves a finished download into the public Downloads/Foyer folder.
pub fn save_to_downloads(src: &Path, name: &str, _custom_dir: &str) -> Result<String, String> {
    android_media::save_to_downloads(&src.to_string_lossy(), name)?;
    let _ = std::fs::remove_file(src);
    Ok("Downloads/Foyer".into())
}

pub const CUSTOM_RESIZE: bool = false;
pub fn cursor_pos() -> Option<(i32, i32)> {
    None
}
pub fn left_button_down() -> bool {
    false
}
pub fn set_window_rect(_w: &slint::Window, _x: i32, _y: i32, _width: i32, _height: i32) {}

/// The system font size setting (1.0 = normal; Settings > Display > Font size).
pub fn font_scale() -> f32 {
    with_env(|env, act| {
        let res = env.call_method(act, "getResources", "()Landroid/content/res/Resources;", &[])?.l()?;
        let cfg = env.call_method(&res, "getConfiguration", "()Landroid/content/res/Configuration;", &[])?.l()?;
        env.get_field(&cfg, "fontScale", "F")?.f()
    })
    .unwrap_or(1.0)
}

/// Wi-Fi drops multicast packets unless an app holds a multicast lock, so Cast devices (found with mDNS) stay
/// invisible without one. Hold it while searching.
pub fn multicast_lock(on: bool) {
    static LOCK: std::sync::Mutex<Option<GlobalRef>> = std::sync::Mutex::new(None);
    let res = with_env(|env, act| {
        let mut held = LOCK.lock().unwrap();
        if on {
            if held.is_none() {
                let svc = env.new_string("wifi")?;
                let wifi = env.call_method(act, "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;", &[JValue::Object(&svc)])?.l()?;
                let tag = env.new_string("foyer")?;
                let lock = env.call_method(&wifi, "createMulticastLock", "(Ljava/lang/String;)Landroid/net/wifi/WifiManager$MulticastLock;", &[JValue::Object(&tag)])?.l()?;
                env.call_method(&lock, "setReferenceCounted", "(Z)V", &[JValue::Bool(0)])?;
                env.call_method(&lock, "acquire", "()V", &[])?;
                *held = Some(env.new_global_ref(&lock)?);
            }
        } else if let Some(lock) = held.take() {
            env.call_method(&lock, "release", "()V", &[])?;
        }
        Ok(())
    });
    if let Err(e) = res {
        eprintln!("multicast lock failed: {e}");
    }
}
