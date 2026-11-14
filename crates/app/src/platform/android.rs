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

/// Saves the (already verified) APK to the Downloads folder and opens Android's installer on it.
/// If Foyer isn't yet allowed to install apps, opens that settings page and returns a message
/// asking the user to allow it and try again.
pub fn install_package(apk: &[u8], file_name: &str) -> Result<(), String> {
    with_env(|env, act| {
        let sdk = env.get_static_field("android/os/Build$VERSION", "SDK_INT", "I")?.i()?;
        if sdk < 29 {
            return Err(jni::errors::Error::JavaException);
        }
        let pm = env.call_method(act, "getPackageManager", "()Landroid/content/pm/PackageManager;", &[])?.l()?;
        let allowed = env.call_method(&pm, "canRequestPackageInstalls", "()Z", &[])?.z()?;
        if !allowed {
            let pkg = env.call_method(act, "getPackageName", "()Ljava/lang/String;", &[])?.l()?;
            let pkg: String = env.get_string(&pkg.into())?.into();
            let uri_class = env.find_class("android/net/Uri")?;
            let spec = env.new_string(format!("package:{pkg}"))?;
            let uri = env.call_static_method(uri_class, "parse", "(Ljava/lang/String;)Landroid/net/Uri;", &[JValue::Object(&spec)])?.l()?;
            let action = env.new_string("android.settings.MANAGE_UNKNOWN_APP_SOURCES")?;
            let intent = env.new_object("android/content/Intent", "(Ljava/lang/String;Landroid/net/Uri;)V", &[JValue::Object(&action), JValue::Object(&uri)])?;
            env.call_method(&intent, "addFlags", "(I)Landroid/content/Intent;", &[JValue::Int(0x1000_0000)])?;
            env.call_method(act, "startActivity", "(Landroid/content/Intent;)V", &[JValue::Object(&intent)])?;
            return Err(jni::errors::Error::NullPtr("install permission"));
        }

        let resolver = env.call_method(act, "getContentResolver", "()Landroid/content/ContentResolver;", &[])?.l()?;
        let values = env.new_object("android/content/ContentValues", "()V", &[])?;
        let put = |env: &mut JNIEnv<'_>, k: &str, v: &str| -> jni::errors::Result<()> {
            let (k, v) = (env.new_string(k)?, env.new_string(v)?);
            env.call_method(&values, "put", "(Ljava/lang/String;Ljava/lang/String;)V", &[JValue::Object(&k), JValue::Object(&v)])?;
            Ok(())
        };
        put(env, "_display_name", file_name)?;
        put(env, "mime_type", "application/vnd.android.package-archive")?;
        put(env, "relative_path", "Download/Foyer")?;
        let collection = env.get_static_field("android/provider/MediaStore$Downloads", "EXTERNAL_CONTENT_URI", "Landroid/net/Uri;")?.l()?;
        let uri = env
            .call_method(&resolver, "insert", "(Landroid/net/Uri;Landroid/content/ContentValues;)Landroid/net/Uri;", &[JValue::Object(&collection), JValue::Object(&values)])?
            .l()?;
        if uri.is_null() {
            return Err(jni::errors::Error::NullPtr("downloads uri"));
        }
        let stream = env.call_method(&resolver, "openOutputStream", "(Landroid/net/Uri;)Ljava/io/OutputStream;", &[JValue::Object(&uri)])?.l()?;
        for chunk in apk.chunks(256 * 1024) {
            let bytes = env.byte_array_from_slice(chunk)?;
            env.call_method(&stream, "write", "([B)V", &[JValue::Object(&bytes)])?;
        }
        env.call_method(&stream, "flush", "()V", &[])?;
        env.call_method(&stream, "close", "()V", &[])?;

        let action = env.new_string("android.intent.action.VIEW")?;
        let mime = env.new_string("application/vnd.android.package-archive")?;
        let intent = env.new_object("android/content/Intent", "(Ljava/lang/String;)V", &[JValue::Object(&action)])?;
        env.call_method(&intent, "setDataAndType", "(Landroid/net/Uri;Ljava/lang/String;)Landroid/content/Intent;", &[JValue::Object(&uri), JValue::Object(&mime)])?;
        // FLAG_GRANT_READ_URI_PERMISSION | FLAG_ACTIVITY_NEW_TASK
        env.call_method(&intent, "addFlags", "(I)Landroid/content/Intent;", &[JValue::Int(0x1 | 0x1000_0000)])?;
        env.call_method(act, "startActivity", "(Landroid/content/Intent;)V", &[JValue::Object(&intent)])?;
        Ok(())
    })
    .map_err(|e| match e.as_str() {
        s if s.contains("install permission") => "Allow Foyer to install apps in the settings page that just opened, then try the update again.".to_string(),
        "An Android call failed" | "Java exception was thrown" => "Android couldn't open the installer for this update.".to_string(),
        s => format!("Couldn't hand the update to Android ({s})."),
    })
}

/// While a video plays on a phone or tablet, turn to landscape (and let go again afterwards).
/// SCREEN_ORIENTATION_SENSOR_LANDSCAPE = 6, SCREEN_ORIENTATION_UNSPECIFIED = -1.
pub fn set_landscape(on: bool) {
    if let Err(e) = with_env(|env, act| {
        env.call_method(act, "setRequestedOrientation", "(I)V", &[JValue::Int(if on { 6 } else { -1 })])?;
        Ok(())
    }) {
        eprintln!("setRequestedOrientation failed: {e}");
    }
}

/// Windows registers its toast identity here; nothing to do elsewhere.
pub fn register_app_identity() {}
