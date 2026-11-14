//! Real picture-in-picture and the system media session (notification shade, lock screen and
//! headset controls) for the playing video. Everything is best effort: failures are logged.
//!
//! cargo-apk ships no Java code, so the MediaSession callback class is a tiny dex built by CI
//! (`.github/scripts/build-mediacb.sh`) and loaded here with an InMemoryDexClassLoader.
use super::with_env;
use crate::platform::MediaAction;
use jni::objects::{GlobalRef, JClass, JObject, JValue};
use jni::sys::{jint, jlong};
use jni::{JNIEnv, NativeMethod};
use std::ffi::c_void;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Mutex, OnceLock};

static DEX: &[u8] = include_bytes!("../../android/mediacb.dex");
const CHANNEL: &str = "foyer_playback";
const NOTE_ID: i32 = 4242;
const CB_CLASS: &str = "app.foyer.player.MediaCb";

struct Session {
    session: GlobalRef,
    title: String,
    dur_ms: i64,
}

static SESSION: Mutex<Option<Session>> = Mutex::new(None);
static CB: OnceLock<GlobalRef> = OnceLock::new();
type Handler = Box<dyn Fn(MediaAction) + Send>;
static HANDLER: Mutex<Option<Handler>> = Mutex::new(None);
static ASKED_PERMISSION: AtomicBool = AtomicBool::new(false);

pub fn set_media_handler(f: Handler) {
    *HANDLER.lock().unwrap() = Some(f);
}

extern "system" fn native_action(_env: JNIEnv, _cls: JClass, code: jint, arg: jlong) {
    let action = match code {
        1 => MediaAction::Play,
        2 => MediaAction::Pause,
        3 => MediaAction::Next,
        4 => MediaAction::Prev,
        5 => MediaAction::Stop,
        6 => MediaAction::Seek(arg),
        7 => MediaAction::Forward,
        8 => MediaAction::Rewind,
        _ => return,
    };
    if let Some(h) = HANDLER.lock().unwrap().as_ref() {
        h(action);
    }
}

fn sdk(env: &mut JNIEnv<'_>) -> jni::errors::Result<i32> {
    env.get_static_field("android/os/Build$VERSION", "SDK_INT", "I")?.i()
}

fn as_class(g: &GlobalRef) -> JClass<'static> {
    // SAFETY: the GlobalRef (kept in a static) outlives every use of the borrowed class handle.
    JClass::from(unsafe { JObject::from_raw(g.as_obj().as_raw()) })
}

// ---------------------------------------------------------------- picture-in-picture

fn pip_params<'a>(env: &mut JNIEnv<'a>, aspect: f32, auto: bool) -> jni::errors::Result<JObject<'a>> {
    let builder = env.new_object("android/app/PictureInPictureParams$Builder", "()V", &[])?;
    // Android only accepts ratios between about 1:2.39 and 2.39:1.
    let ratio = if aspect.is_finite() && aspect > 0.0 { aspect.clamp(0.42, 2.38) } else { 16.0 / 9.0 };
    let rational = env.new_object("android/util/Rational", "(II)V", &[JValue::Int((ratio * 1000.0) as i32), JValue::Int(1000)])?;
    env.call_method(&builder, "setAspectRatio", "(Landroid/util/Rational;)Landroid/app/PictureInPictureParams$Builder;", &[JValue::Object(&rational)])?;
    if sdk(env)? >= 31 {
        env.call_method(&builder, "setAutoEnterEnabled", "(Z)Landroid/app/PictureInPictureParams$Builder;", &[JValue::Bool(auto as u8)])?;
    }
    env.call_method(&builder, "build", "()Landroid/app/PictureInPictureParams;", &[])?.l()
}

/// Shrinks the app into Android's floating picture-in-picture window. False if it can't.
pub fn enter_pip(aspect: f32) -> bool {
    with_env(|env, act| {
        if sdk(env)? < 26 {
            return Ok(false);
        }
        let params = pip_params(env, aspect, false)?;
        env.call_method(act, "enterPictureInPictureMode", "(Landroid/app/PictureInPictureParams;)Z", &[JValue::Object(&params)])?.z()
    })
    .map_err(|e| eprintln!("enter picture-in-picture failed: {e}"))
    .unwrap_or(false)
}

pub fn in_system_pip() -> bool {
    with_env(|env, act| {
        if sdk(env)? < 26 {
            return Ok(false);
        }
        env.call_method(act, "isInPictureInPictureMode", "()Z", &[])?.z()
    })
    .unwrap_or(false)
}

/// While a video plays, pressing Home (or swiping up) shrinks it into picture-in-picture by itself
/// (Android 12+). Also tells Android the video's shape.
pub fn set_pip_auto(on: bool, aspect: f32) {
    let _ = with_env(|env, act| {
        if sdk(env)? < 31 {
            return Ok(());
        }
        let params = pip_params(env, aspect, on)?;
        env.call_method(act, "setPictureInPictureParams", "(Landroid/app/PictureInPictureParams;)V", &[JValue::Object(&params)])?;
        Ok(())
    });
}

// ---------------------------------------------------------------- media session

fn load_callback_class(env: &mut JNIEnv<'_>, act: &JObject<'_>) -> jni::errors::Result<GlobalRef> {
    if let Some(c) = CB.get() {
        return Ok(c.clone());
    }
    let parent = env.call_method(act, "getClassLoader", "()Ljava/lang/ClassLoader;", &[])?.l()?;
    // SAFETY: DEX is 'static, and the loader only reads it.
    let buf = unsafe { env.new_direct_byte_buffer(DEX.as_ptr() as *mut u8, DEX.len())? };
    let loader = env.new_object("dalvik/system/InMemoryDexClassLoader", "(Ljava/nio/ByteBuffer;Ljava/lang/ClassLoader;)V", &[JValue::Object(&buf), JValue::Object(&parent)])?;
    let name = env.new_string(CB_CLASS)?;
    let class = env.call_method(&loader, "loadClass", "(Ljava/lang/String;)Ljava/lang/Class;", &[JValue::Object(&name)])?.l()?;
    let class = JClass::from(class);
    env.register_native_methods(&class, &[NativeMethod { name: "nativeAction".into(), sig: "(IJ)V".into(), fn_ptr: native_action as *mut c_void }])?;
    let global = env.new_global_ref(&class)?;
    let _ = CB.set(global.clone());
    Ok(global)
}

fn create_session(env: &mut JNIEnv<'_>, act: &JObject<'_>) -> jni::errors::Result<Session> {
    let class = load_callback_class(env, act)?;
    let callback = env.new_object(as_class(&class), "()V", &[])?;
    let tag = env.new_string("Foyer")?;
    let session = env.new_object("android/media/session/MediaSession", "(Landroid/content/Context;Ljava/lang/String;)V", &[JValue::Object(act), JValue::Object(&tag)])?;
    // The app's own thread has no Looper, so the callbacks run on the main one.
    let looper = env.call_static_method("android/os/Looper", "getMainLooper", "()Landroid/os/Looper;", &[])?.l()?;
    let handler = env.new_object("android/os/Handler", "(Landroid/os/Looper;)V", &[JValue::Object(&looper)])?;
    env.call_method(&session, "setCallback", "(Landroid/media/session/MediaSession$Callback;Landroid/os/Handler;)V", &[JValue::Object(&callback), JValue::Object(&handler)])?;
    env.call_method(&session, "setActive", "(Z)V", &[JValue::Bool(1)])?;
    Ok(Session { session: env.new_global_ref(&session)?, title: String::new(), dur_ms: -1 })
}

fn notification_manager<'a>(env: &mut JNIEnv<'a>, act: &JObject<'_>) -> jni::errors::Result<JObject<'a>> {
    let service = env.new_string("notification")?;
    env.call_method(act, "getSystemService", "(Ljava/lang/String;)Ljava/lang/Object;", &[JValue::Object(&service)])?.l()
}

/// The ongoing "now playing" notification that carries the session's controls.
fn post_notification(env: &mut JNIEnv<'_>, act: &JObject<'_>, session: &JObject<'_>, title: &str, subtitle: &str) -> jni::errors::Result<()> {
    if sdk(env)? >= 33 {
        let perm = env.new_string("android.permission.POST_NOTIFICATIONS")?;
        let granted = env.call_method(act, "checkSelfPermission", "(Ljava/lang/String;)I", &[JValue::Object(&perm)])?.i()?;
        if granted != 0 {
            if !ASKED_PERMISSION.swap(true, Ordering::Relaxed) {
                let string_class = env.find_class("java/lang/String")?;
                let list = env.new_object_array(1, &string_class, &perm)?;
                env.call_method(act, "requestPermissions", "([Ljava/lang/String;I)V", &[JValue::Object(&list), JValue::Int(1)])?;
            }
            return Ok(());
        }
    }
    let manager = notification_manager(env, act)?;
    let id = env.new_string(CHANNEL)?;
    let name = env.new_string("Playback")?;
    // IMPORTANCE_LOW: no sound, still shown.
    let channel = env.new_object("android/app/NotificationChannel", "(Ljava/lang/String;Ljava/lang/CharSequence;I)V", &[JValue::Object(&id), JValue::Object(&name), JValue::Int(2)])?;
    env.call_method(&manager, "createNotificationChannel", "(Landroid/app/NotificationChannel;)V", &[JValue::Object(&channel)])?;

    let builder = env.new_object("android/app/Notification$Builder", "(Landroid/content/Context;Ljava/lang/String;)V", &[JValue::Object(act), JValue::Object(&id)])?;
    let icon = env.get_static_field("android/R$drawable", "ic_media_play", "I")?.i()?;
    let (t, s) = (env.new_string(title)?, env.new_string(subtitle)?);
    env.call_method(&builder, "setContentTitle", "(Ljava/lang/CharSequence;)Landroid/app/Notification$Builder;", &[JValue::Object(&t)])?;
    env.call_method(&builder, "setContentText", "(Ljava/lang/CharSequence;)Landroid/app/Notification$Builder;", &[JValue::Object(&s)])?;
    env.call_method(&builder, "setSmallIcon", "(I)Landroid/app/Notification$Builder;", &[JValue::Int(icon)])?;
    env.call_method(&builder, "setOngoing", "(Z)Landroid/app/Notification$Builder;", &[JValue::Bool(1)])?;
    env.call_method(&builder, "setVisibility", "(I)Landroid/app/Notification$Builder;", &[JValue::Int(1)])?;
    let token = env.call_method(session, "getSessionToken", "()Landroid/media/session/MediaSession$Token;", &[])?.l()?;
    let style = env.new_object("android/app/Notification$MediaStyle", "()V", &[])?;
    env.call_method(&style, "setMediaSession", "(Landroid/media/session/MediaSession$Token;)Landroid/app/Notification$MediaStyle;", &[JValue::Object(&token)])?;
    env.call_method(&builder, "setStyle", "(Landroid/app/Notification$Style;)Landroid/app/Notification$Builder;", &[JValue::Object(&style)])?;

    // Tapping it comes back to the app.
    let pm = env.call_method(act, "getPackageManager", "()Landroid/content/pm/PackageManager;", &[])?.l()?;
    let pkg = env.call_method(act, "getPackageName", "()Ljava/lang/String;", &[])?.l()?;
    let launch = env.call_method(&pm, "getLaunchIntentForPackage", "(Ljava/lang/String;)Landroid/content/Intent;", &[JValue::Object(&pkg)])?.l()?;
    if !launch.is_null() {
        // FLAG_IMMUTABLE | FLAG_UPDATE_CURRENT
        let pending = env
            .call_static_method("android/app/PendingIntent", "getActivity", "(Landroid/content/Context;ILandroid/content/Intent;I)Landroid/app/PendingIntent;", &[JValue::Object(act), JValue::Int(0), JValue::Object(&launch), JValue::Int(0x0400_0000 | 0x0800_0000)])?
            .l()?;
        env.call_method(&builder, "setContentIntent", "(Landroid/app/PendingIntent;)Landroid/app/Notification$Builder;", &[JValue::Object(&pending)])?;
    }
    let notification = env.call_method(&builder, "build", "()Landroid/app/Notification;", &[])?.l()?;
    env.call_method(&manager, "notify", "(ILandroid/app/Notification;)V", &[JValue::Int(NOTE_ID), JValue::Object(&notification)])?;
    Ok(())
}

/// Keeps the system's idea of "what is playing" current. Call about once a second while a video
/// plays; the notification is only re-posted when the title changes.
pub fn media_update(title: &str, subtitle: &str, playing: bool, pos_ms: i64, dur_ms: i64) {
    let result = with_env(|env, act| {
        let mut guard = SESSION.lock().unwrap();
        if guard.is_none() {
            *guard = Some(create_session(env, act)?);
        }
        let s = guard.as_mut().expect("session just created");
        let session = s.session.as_obj();
        if s.title != title || s.dur_ms != dur_ms {
            let builder = env.new_object("android/media/MediaMetadata$Builder", "()V", &[])?;
            for (key, val) in [("android.media.metadata.TITLE", title), ("android.media.metadata.ARTIST", subtitle)] {
                let (k, v) = (env.new_string(key)?, env.new_string(val)?);
                env.call_method(&builder, "putString", "(Ljava/lang/String;Ljava/lang/String;)Landroid/media/MediaMetadata$Builder;", &[JValue::Object(&k), JValue::Object(&v)])?;
            }
            let k = env.new_string("android.media.metadata.DURATION")?;
            env.call_method(&builder, "putLong", "(Ljava/lang/String;J)Landroid/media/MediaMetadata$Builder;", &[JValue::Object(&k), JValue::Long(dur_ms.max(0))])?;
            let meta = env.call_method(&builder, "build", "()Landroid/media/MediaMetadata;", &[])?.l()?;
            env.call_method(session, "setMetadata", "(Landroid/media/MediaMetadata;)V", &[JValue::Object(&meta)])?;
            if s.title != title {
                post_notification(env, act, session, title, subtitle)?;
            }
            s.title = title.to_string();
            s.dur_ms = dur_ms;
        }
        // STOP | PAUSE | PLAY | REWIND | SKIP_TO_PREVIOUS | SKIP_TO_NEXT | FAST_FORWARD | SEEK_TO | PLAY_PAUSE
        let actions: i64 = 1 | 2 | 4 | 8 | 16 | 32 | 64 | 256 | 512;
        let state = env.new_object("android/media/session/PlaybackState$Builder", "()V", &[])?;
        env.call_method(&state, "setActions", "(J)Landroid/media/session/PlaybackState$Builder;", &[JValue::Long(actions)])?;
        let now = env.call_static_method("android/os/SystemClock", "elapsedRealtime", "()J", &[])?.j()?;
        // STATE_PLAYING = 3, STATE_PAUSED = 2
        env.call_method(
            &state,
            "setState",
            "(IJFJ)Landroid/media/session/PlaybackState$Builder;",
            &[JValue::Int(if playing { 3 } else { 2 }), JValue::Long(pos_ms.max(0)), JValue::Float(if playing { 1.0 } else { 0.0 }), JValue::Long(now)],
        )?;
        let built = env.call_method(&state, "build", "()Landroid/media/session/PlaybackState;", &[])?.l()?;
        env.call_method(session, "setPlaybackState", "(Landroid/media/session/PlaybackState;)V", &[JValue::Object(&built)])?;
        Ok(())
    });
    if let Err(e) = result {
        eprintln!("media session update failed: {e}");
    }
}

/// Playback ended: drop the session and its notification.
pub fn media_stop() {
    let Some(s) = SESSION.lock().unwrap().take() else { return };
    let _ = with_env(|env, act| {
        env.call_method(s.session.as_obj(), "setActive", "(Z)V", &[JValue::Bool(0)])?;
        env.call_method(s.session.as_obj(), "release", "()V", &[])?;
        let manager = notification_manager(env, act)?;
        env.call_method(&manager, "cancel", "(I)V", &[JValue::Int(NOTE_ID)])?;
        Ok(())
    });
}
