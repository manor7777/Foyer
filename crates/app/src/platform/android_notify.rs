//! Rich Android notifications: a picture (the poster) and a tap that opens the right page.
//!
//! cargo-apk's NativeActivity never hands the app a *new* intent while it is running, so a tap on
//! a notification is found in two ways: the intent that started the app (`launch_note_target`), and,
//! while the process is alive, by noticing that one of our notifications disappeared just as the
//! window got focus (`poll_note_tap`). Swiping a notification away does not count, because the
//! window has no focus then.
use super::{with_env, RichNote};
use jni::objects::{JObject, JObjectArray, JValue};
use jni::JNIEnv;
use std::sync::Mutex;
use std::time::{Duration, Instant};

/// Notifications we posted that are still on screen: (Android id, target).
static POSTED: Mutex<Vec<(i32, String)>> = Mutex::new(Vec::new());
/// Targets of notifications that just vanished, waiting for the window to take focus.
static VANISHED: Mutex<Vec<(String, Instant)>> = Mutex::new(Vec::new());
static LAUNCH_TAKEN: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

const EXTRA: &str = "foyer_target";

fn sdk(env: &mut JNIEnv<'_>) -> jni::errors::Result<i32> {
    env.get_static_field("android/os/Build$VERSION", "SDK_INT", "I")?.i()
}

/// Shows (or, on Android 13+ without permission, asks for permission to show) a notification with
/// the poster as its large picture. Tapping it brings the app back and goes to `n.target`.
pub fn system_notify_rich(n: &RichNote) {
    let result = with_env(|env, act| {
        if sdk(env)? >= 33 {
            let perm = env.new_string("android.permission.POST_NOTIFICATIONS")?;
            let granted = env.call_method(act, "checkSelfPermission", "(Ljava/lang/String;)I", &[JValue::Object(&perm)])?.i()?;
            if granted != 0 {
                let string_class = env.find_class("java/lang/String")?;
                let list = env.new_object_array(1, &string_class, &perm)?;
                env.call_method(act, "requestPermissions", "([Ljava/lang/String;I)V", &[JValue::Object(&list), JValue::Int(1)])?;
                return Ok(());
            }
        }
        let id = env.new_string(super::CHANNEL)?;
        let name = env.new_string("Foyer")?;
        let service = env.new_string("notification")?;
        let manager = env.call_method(act, "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;", &[JValue::Object(&service)])?.l()?;
        let channel = env.new_object("android/app/NotificationChannel", "(Ljava/lang/String;Ljava/lang/CharSequence;I)V", &[JValue::Object(&id), JValue::Object(&name), JValue::Int(3)])?;
        env.call_method(&manager, "createNotificationChannel", "(Landroid/app/NotificationChannel;)V", &[JValue::Object(&channel)])?;

        let builder = env.new_object("android/app/Notification$Builder", "(Landroid/content/Context;Ljava/lang/String;)V", &[JValue::Object(act), JValue::Object(&id)])?;
        let icon = env.get_static_field("android/R$drawable", "ic_dialog_info", "I")?.i()?;
        let (title, body) = (env.new_string(&n.title)?, env.new_string(&n.body)?);
        env.call_method(&builder, "setContentTitle", "(Ljava/lang/CharSequence;)Landroid/app/Notification$Builder;", &[JValue::Object(&title)])?;
        env.call_method(&builder, "setContentText", "(Ljava/lang/CharSequence;)Landroid/app/Notification$Builder;", &[JValue::Object(&body)])?;
        env.call_method(&builder, "setSmallIcon", "(I)Landroid/app/Notification$Builder;", &[JValue::Int(icon)])?;
        env.call_method(&builder, "setAutoCancel", "(Z)Landroid/app/Notification$Builder;", &[JValue::Bool(1)])?;
        // Long text can be expanded.
        let style = env.new_object("android/app/Notification$BigTextStyle", "()V", &[])?;
        env.call_method(&style, "bigText", "(Ljava/lang/CharSequence;)Landroid/app/Notification$BigTextStyle;", &[JValue::Object(&body)])?;
        env.call_method(&builder, "setStyle", "(Landroid/app/Notification$Style;)Landroid/app/Notification$Builder;", &[JValue::Object(&style)])?;

        if let Some(bytes) = &n.image_bytes {
            let arr = env.byte_array_from_slice(bytes)?;
            let bmp = env.call_static_method("android/graphics/BitmapFactory", "decodeByteArray", "([BII)Landroid/graphics/Bitmap;", &[JValue::Object(&arr), JValue::Int(0), JValue::Int(bytes.len() as i32)])?.l()?;
            if !bmp.is_null() {
                env.call_method(&builder, "setLargeIcon", "(Landroid/graphics/Bitmap;)Landroid/app/Notification$Builder;", &[JValue::Object(&bmp)])?;
            }
        }

        // The tap: start the app (or bring it forward) carrying where to go.
        let android_id = (n.id & 0x7fff_ffff) as i32;
        if !n.target.is_empty() {
            let pm = env.call_method(act, "getPackageManager", "()Landroid/content/pm/PackageManager;", &[])?.l()?;
            let pkg = env.call_method(act, "getPackageName", "()Ljava/lang/String;", &[])?.l()?;
            let launch = env.call_method(&pm, "getLaunchIntentForPackage", "(Ljava/lang/String;)Landroid/content/Intent;", &[JValue::Object(&pkg)])?.l()?;
            if !launch.is_null() {
                let (k, v) = (env.new_string(EXTRA)?, env.new_string(&n.target)?);
                env.call_method(&launch, "putExtra", "(Ljava/lang/String;Ljava/lang/String;)Landroid/content/Intent;", &[JValue::Object(&k), JValue::Object(&v)])?;
                // FLAG_ACTIVITY_SINGLE_TOP: reuse the running app instead of starting another.
                env.call_method(&launch, "addFlags", "(I)Landroid/content/Intent;", &[JValue::Int(0x2000_0000)])?;
                // FLAG_IMMUTABLE | FLAG_UPDATE_CURRENT
                let pending = env
                    .call_static_method("android/app/PendingIntent", "getActivity", "(Landroid/content/Context;ILandroid/content/Intent;I)Landroid/app/PendingIntent;", &[JValue::Object(act), JValue::Int(android_id), JValue::Object(&launch), JValue::Int(0x0400_0000 | 0x0800_0000)])?
                    .l()?;
                env.call_method(&builder, "setContentIntent", "(Landroid/app/PendingIntent;)Landroid/app/Notification$Builder;", &[JValue::Object(&pending)])?;
            }
        }
        let notification = env.call_method(&builder, "build", "()Landroid/app/Notification;", &[])?.l()?;
        env.call_method(&manager, "notify", "(ILandroid/app/Notification;)V", &[JValue::Int(android_id), JValue::Object(&notification)])?;
        if !n.target.is_empty() {
            let mut posted = POSTED.lock().unwrap();
            posted.push((android_id, n.target.clone()));
            let len = posted.len();
            if len > 30 {
                posted.drain(..len - 30);
            }
        }
        Ok(())
    });
    if let Err(e) = result {
        eprintln!("rich notification failed: {e}");
    }
}

/// Ids of our notifications that are still showing.
fn active_ids(env: &mut JNIEnv<'_>, act: &JObject<'_>) -> jni::errors::Result<Vec<i32>> {
    let service = env.new_string("notification")?;
    let manager = env.call_method(act, "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;", &[JValue::Object(&service)])?.l()?;
    let arr: JObjectArray = env.call_method(&manager, "getActiveNotifications", "()[Landroid/service/notification/StatusBarNotification;", &[])?.l()?.into();
    let len = env.get_array_length(&arr)?;
    let mut ids = Vec::with_capacity(len as usize);
    for i in 0..len {
        let sbn = env.get_object_array_element(&arr, i)?;
        ids.push(env.call_method(&sbn, "getId", "()I", &[])?.i()?);
    }
    Ok(ids)
}

/// Call about once a second. Returns the target of a notification that was just tapped.
pub fn poll_note_tap() -> Option<String> {
    if POSTED.lock().unwrap().is_empty() && VANISHED.lock().unwrap().is_empty() {
        return None;
    }
    with_env(|env, act| {
        let active = active_ids(env, act)?;
        {
            let mut posted = POSTED.lock().unwrap();
            let mut vanished = VANISHED.lock().unwrap();
            posted.retain(|(id, target)| {
                if active.contains(id) {
                    true
                } else {
                    vanished.push((target.clone(), Instant::now()));
                    false
                }
            });
            vanished.retain(|(_, t)| t.elapsed() < Duration::from_secs(4));
        }
        if env.call_method(act, "hasWindowFocus", "()Z", &[])?.z()? {
            let mut vanished = VANISHED.lock().unwrap();
            let tapped = vanished.pop().map(|(t, _)| t);
            vanished.clear();
            return Ok(tapped);
        }
        Ok(None)
    })
    .ok()
    .flatten()
}

/// Where the notification that started the app wanted to go (read once).
pub fn launch_note_target() -> Option<String> {
    if LAUNCH_TAKEN.swap(true, std::sync::atomic::Ordering::SeqCst) {
        return None;
    }
    with_env(|env, act| {
        let intent = env.call_method(act, "getIntent", "()Landroid/content/Intent;", &[])?.l()?;
        if intent.is_null() {
            return Ok(None);
        }
        let key = env.new_string(EXTRA)?;
        let v = env.call_method(&intent, "getStringExtra", "(Ljava/lang/String;)Ljava/lang/String;", &[JValue::Object(&key)])?.l()?;
        if v.is_null() {
            return Ok(None);
        }
        let target: String = env.get_string(&v.into())?.into();
        env.call_method(&intent, "removeExtra", "(Ljava/lang/String;)V", &[JValue::Object(&key)])?;
        Ok(Some(target).filter(|t| !t.is_empty()))
    })
    .ok()
    .flatten()
}
