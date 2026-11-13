//! libmpv playback, rendered into the host's own OpenGL surface (mpv render API), so the host
//! can draw its UI over the video, handle the mouse and keyboard itself, and go fullscreen
//! like any other window.
//!
//! mpv-1.dll is loaded once and never unloaded (mpv does not survive being unloaded and
//! reloaded, which crashed the second playback in earlier builds).
//!
//! Threading: `play` blocks on its own thread and owns the mpv core. The host calls
//! `Surface::render` from its GL thread each frame (Slint's BeforeRendering notifier); that
//! thread owns the mpv render context.
//!
//! Defaults carry over the fixes from FixEmby.ps1 (large read-ahead, sequential HLS reads).

use libloading::Library;
use std::ffi::{c_char, c_double, c_int, c_void, CStr, CString};
use std::path::PathBuf;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::mpsc::Receiver;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

#[derive(Debug, Clone, PartialEq)]
pub enum Outcome {
    /// Reached the end of the file cleanly. The only outcome that may trigger auto-next.
    Finished,
    /// The user stopped.
    Stopped,
    /// Could not load or play. Callers must not advance or write a position.
    Failed(String),
}

/// An external subtitle file (served by Emby) to attach once the file has loaded.
#[derive(Debug, Clone)]
pub struct ExtSub {
    pub url: String,
    pub title: String,
    pub lang: String,
}

#[derive(Debug, Clone)]
pub struct Options {
    pub title: String,
    pub start_secs: f64,
    pub external_subs: Vec<ExtSub>,
    /// 1-based audio track to select once the file loads (direct play only).
    pub audio_pos: Option<i64>,
    pub hw_decode: bool,
    /// 0 auto (copy-back), 1 auto + convert to a plain 10-bit layout, 2 dxva2-copy, 3 nvdec, 4 zero-copy auto.
    pub hw_mode: u8,
    /// When set, mpv writes its full verbose log here (audio device, underruns, decoder...).
    pub debug_log: Option<String>,
    /// Stay paused on the last frame at the end instead of closing (the host then sends `Finish`).
    pub keep_open: bool,
}

/// One audio or subtitle track in the current file.
#[derive(Debug, Clone)]
pub struct Track {
    /// "audio" or "sub"
    pub kind: String,
    pub id: i64,
    pub label: String,
    pub selected: bool,
}

/// Input forwarded from the host.
#[derive(Debug, Clone)]
pub enum Control {
    Quit,
    /// End playback as if the file had reached its end.
    Finish,
    TogglePause,
    SeekBy(i32),
    VolumeBy(i32),
    ToggleMute,
    CycleSubtitles,
    CycleAudio,
    SetPause(bool),
    SeekTo(f64),
    SetVolume(i32),
    SetMute(bool),
    SelectAudio(i64),
    /// Shift audio relative to video by this many seconds (positive = later).
    AudioDelayBy(f64),
    ResetAudioDelay,
    /// -1 turns subtitles off
    SelectSub(i64),
    AddSubtitle { url: String, title: String, lang: String },
}

/// Live playback state, polled a few times a second.
#[derive(Debug, Clone)]
pub struct Tick {
    pub position: f64,
    pub duration: f64,
    pub paused: bool,
    /// Sitting at the end of the file (only with `keep_open`).
    pub eof: bool,
    pub volume: f64,
    pub muted: bool,
    pub audio: String,
    pub subtitle: String,
    pub audio_delay: f64,
    /// Playback diagnostics (A/V sync, dropped frames), present every ~10 s.
    pub diag: Option<String>,
    /// Present on the first tick and whenever the track list changed.
    pub tracks: Option<Vec<Track>>,
}

// ---------------------------------------------------------------- C API

#[repr(C)]
struct MpvEvent {
    event_id: c_int,
    error: c_int,
    reply_userdata: u64,
    data: *mut c_void,
}

#[repr(C)]
struct MpvEventEndFile {
    reason: c_int,
    error: c_int,
}

#[repr(C)]
struct RenderParam {
    kind: c_int,
    data: *mut c_void,
}

type GetProc = unsafe extern "C" fn(*mut c_void, *const c_char) -> *mut c_void;

#[repr(C)]
struct GlInitParams {
    get_proc_address: GetProc,
    get_proc_address_ctx: *mut c_void,
    extra_exts: *const c_char, // present in older mpv versions; must be null
}

#[repr(C)]
struct GlFbo {
    fbo: c_int,
    w: c_int,
    h: c_int,
    internal_format: c_int,
}

const EV_SHUTDOWN: c_int = 1;
const EV_END_FILE: c_int = 7;
const EV_FILE_LOADED: c_int = 8;
const END_EOF: c_int = 0;
const END_ERROR: c_int = 4;

const PARAM_API_TYPE: c_int = 1;
const PARAM_GL_INIT: c_int = 2;
const PARAM_GL_FBO: c_int = 3;
const PARAM_FLIP_Y: c_int = 4;

#[derive(Clone, Copy)]
struct Api {
    create: unsafe extern "C" fn() -> *mut c_void,
    set_opt: unsafe extern "C" fn(*mut c_void, *const c_char, *const c_char) -> c_int,
    init: unsafe extern "C" fn(*mut c_void) -> c_int,
    command: unsafe extern "C" fn(*mut c_void, *const *const c_char) -> c_int,
    get_prop: unsafe extern "C" fn(*mut c_void, *const c_char) -> *mut c_char,
    free: unsafe extern "C" fn(*mut c_void),
    wait: unsafe extern "C" fn(*mut c_void, c_double) -> *mut MpvEvent,
    destroy: unsafe extern "C" fn(*mut c_void),
    render_create: unsafe extern "C" fn(*mut *mut c_void, *mut c_void, *mut RenderParam) -> c_int,
    render_set_update_cb: unsafe extern "C" fn(*mut c_void, Option<unsafe extern "C" fn(*mut c_void)>, *mut c_void),
    render_update: unsafe extern "C" fn(*mut c_void) -> u64,
    render_render: unsafe extern "C" fn(*mut c_void, *mut RenderParam) -> c_int,
    render_free: unsafe extern "C" fn(*mut c_void),
    render_report_swap: unsafe extern "C" fn(*mut c_void),
}

/// Where a downloaded libmpv lives (%APPDATA%\Foyer\mpv).
pub fn engine_dir() -> PathBuf {
    std::env::var_os("APPDATA").map(PathBuf::from).unwrap_or_else(std::env::temp_dir).join("Foyer").join("mpv")
}

fn dll_candidates() -> Vec<PathBuf> {
    let mut dirs = Vec::new();
    if let Ok(exe) = std::env::current_exe() {
        if let Some(dir) = exe.parent() {
            dirs.push(dir.to_path_buf());
            dirs.push(dir.join("libmpv"));
        }
    }
    dirs.push(engine_dir());
    let mut v = Vec::new();
    for d in dirs {
        for n in ["mpv-1.dll", "libmpv-2.dll"] {
            v.push(d.join(n));
        }
    }
    v
}

/// True when a libmpv DLL is present (next to the app, or downloaded).
pub fn available() -> bool {
    dll_candidates().iter().any(|p| p.exists())
}

fn load_api() -> Result<Api, String> {
    let mut last = String::from("the video engine (libmpv) is not installed yet");
    for p in dll_candidates() {
        if !p.exists() {
            continue;
        }
        // Leaked on purpose: never unloaded.
        let lib: &'static Library = match unsafe { Library::new(&p) } {
            Ok(l) => Box::leak(Box::new(l)),
            Err(e) => {
                last = format!("{}: {e}", p.display());
                continue;
            }
        };
        macro_rules! sym {
            ($n:literal) => {
                *unsafe { lib.get($n) }.map_err(|e| format!("{}: {e}", String::from_utf8_lossy($n)))?
            };
        }
        return Ok(Api {
            create: sym!(b"mpv_create"),
            set_opt: sym!(b"mpv_set_option_string"),
            init: sym!(b"mpv_initialize"),
            command: sym!(b"mpv_command"),
            get_prop: sym!(b"mpv_get_property_string"),
            free: sym!(b"mpv_free"),
            wait: sym!(b"mpv_wait_event"),
            destroy: sym!(b"mpv_terminate_destroy"),
            render_create: sym!(b"mpv_render_context_create"),
            render_set_update_cb: sym!(b"mpv_render_context_set_update_callback"),
            render_update: sym!(b"mpv_render_context_update"),
            render_render: sym!(b"mpv_render_context_render"),
            render_free: sym!(b"mpv_render_context_free"),
            render_report_swap: sym!(b"mpv_render_context_report_swap"),
        });
    }
    Err(last)
}

fn api() -> Result<Api, String> {
    // Only a success is cached, so a later download can still be picked up.
    static API: std::sync::Mutex<Option<Api>> = std::sync::Mutex::new(None);
    let mut g = API.lock().unwrap();
    if let Some(a) = *g {
        return Ok(a);
    }
    let a = load_api()?;
    *g = Some(a);
    Ok(a)
}

// ---------------------------------------------------------------- video surface

type GlLoader<'a> = &'a dyn Fn(&CStr) -> *const c_void;

/// Address of the host's current GL loader (a `&dyn Fn`), valid only inside `Surface::render`.
static GL_LOADER: AtomicUsize = AtomicUsize::new(0);

unsafe extern "C" fn gl_trampoline(_ctx: *mut c_void, name: *const c_char) -> *mut c_void {
    let p = GL_LOADER.load(Ordering::Acquire);
    if p == 0 || name.is_null() {
        return std::ptr::null_mut();
    }
    let f = &*(p as *const GlLoader);
    f(CStr::from_ptr(name)) as *mut c_void
}

unsafe extern "C" fn on_update(ctx: *mut c_void) {
    let inner = &*(ctx as *const SurfaceInner);
    (inner.redraw)();
}

struct SurfState {
    api: Option<Api>,
    mpv: usize,    // mpv core handle, published by the player thread (0 = none)
    render: usize, // mpv render context, owned by the GL thread (0 = none)
    release: bool, // player asked the GL thread to free the render context
}

struct SurfaceInner {
    state: Mutex<SurfState>,
    redraw: Box<dyn Fn() + Send + Sync>,
}

/// Shared between the host's GL thread and the player thread.
#[derive(Clone)]
pub struct Surface(Arc<SurfaceInner>);

impl Surface {
    /// `redraw` asks the host to repaint (called from any thread).
    pub fn new(redraw: impl Fn() + Send + Sync + 'static) -> Self {
        Surface(Arc::new(SurfaceInner {
            state: Mutex::new(SurfState { api: None, mpv: 0, render: 0, release: false }),
            redraw: Box::new(redraw),
        }))
    }

    /// Draws the current video frame into the bound framebuffer 0 of size `w` x `h`.
    /// Call from the GL thread (Slint's BeforeRendering). Cheap when nothing is playing.
    pub fn render(&self, loader: GlLoader, w: i32, h: i32) {
        let mut st = self.0.state.lock().unwrap();
        let Some(api) = st.api else { return };
        // SAFETY: render context pointers are only used here, under the lock, on the GL thread.
        unsafe {
            if st.release || st.mpv == 0 {
                if st.render != 0 {
                    (api.render_set_update_cb)(st.render as *mut c_void, None, std::ptr::null_mut());
                    (api.render_free)(st.render as *mut c_void);
                    st.render = 0;
                }
                return;
            }
            let loader_ref: GlLoader = loader;
            GL_LOADER.store(&loader_ref as *const GlLoader as usize, Ordering::Release);
            if st.render == 0 {
                let mut init = GlInitParams { get_proc_address: gl_trampoline, get_proc_address_ctx: std::ptr::null_mut(), extra_exts: std::ptr::null() };
                let api_type = CString::new("opengl").unwrap();
                let mut params = [
                    RenderParam { kind: PARAM_API_TYPE, data: api_type.as_ptr() as *mut c_void },
                    RenderParam { kind: PARAM_GL_INIT, data: &mut init as *mut GlInitParams as *mut c_void },
                    RenderParam { kind: 0, data: std::ptr::null_mut() },
                ];
                let mut ctx: *mut c_void = std::ptr::null_mut();
                if (api.render_create)(&mut ctx, st.mpv as *mut c_void, params.as_mut_ptr()) >= 0 && !ctx.is_null() {
                    (api.render_set_update_cb)(ctx, Some(on_update), Arc::as_ptr(&self.0) as *mut c_void);
                    st.render = ctx as usize;
                }
            }
            if st.render != 0 {
                let ctx = st.render as *mut c_void;
                (api.render_update)(ctx);
                let mut fbo = GlFbo { fbo: 0, w, h, internal_format: 0 };
                let mut flip: c_int = 1;
                let mut params = [
                    RenderParam { kind: PARAM_GL_FBO, data: &mut fbo as *mut GlFbo as *mut c_void },
                    RenderParam { kind: PARAM_FLIP_Y, data: &mut flip as *mut c_int as *mut c_void },
                    RenderParam { kind: 0, data: std::ptr::null_mut() },
                ];
                (api.render_render)(ctx, params.as_mut_ptr());
            }
            GL_LOADER.store(0, Ordering::Release);
        }
    }

    /// Tells mpv a frame was presented, so it can time video against audio accurately.
    pub fn report_swap(&self) {
        let st = self.0.state.lock().unwrap();
        if let (Some(api), true) = (st.api, st.render != 0) {
            // SAFETY: see `render`.
            unsafe { (api.render_report_swap)(st.render as *mut c_void) };
        }
    }

    /// The host's GL context is going away: free the render context now.
    pub fn teardown(&self) {
        let mut st = self.0.state.lock().unwrap();
        if let (Some(api), true) = (st.api, st.render != 0) {
            // SAFETY: see `render`.
            unsafe {
                (api.render_set_update_cb)(st.render as *mut c_void, None, std::ptr::null_mut());
                (api.render_free)(st.render as *mut c_void);
            }
            st.render = 0;
        }
    }

    fn publish(&self, api: Api, mpv: *mut c_void) {
        {
            let mut st = self.0.state.lock().unwrap();
            st.api = Some(api);
            st.mpv = mpv as usize;
            st.release = false;
        }
        (self.0.redraw)();
    }

    /// Asks the GL thread to free the render context and waits for it. Returns false if it
    /// could not be released in time (window hidden); the caller must then not destroy mpv.
    fn unpublish(&self) -> bool {
        {
            self.0.state.lock().unwrap().release = true;
        }
        let deadline = Instant::now() + Duration::from_secs(3);
        loop {
            (self.0.redraw)();
            if self.0.state.lock().unwrap().render == 0 {
                break;
            }
            if Instant::now() > deadline {
                return false;
            }
            std::thread::sleep(Duration::from_millis(20));
        }
        self.0.state.lock().unwrap().mpv = 0;
        true
    }
}

// ---------------------------------------------------------------- player

/// Applied to every mpv instance.
pub const DEFAULT_OPTIONS: &[(&str, &str)] = &[
    ("vo", "libmpv"),
    ("hwdec", "auto-copy-safe"),
    // Newer libmpv builds produced corrupt (noisy) frames in the shared GL context with zero-copy
    // hardware decode and direct rendering; copy-back decode and plain uploads are reliable.
    ("vd-lavc-dr", "no"),
    ("opengl-pbo", "no"),
    ("keep-open", "no"),
    ("osc", "no"), // the host draws the controls
    ("osd-level", "0"),
    ("input-default-bindings", "no"),
    ("input-vo-keyboard", "no"),
    ("idle", "no"),
    // from FixEmby.ps1
    ("cache", "yes"),
    ("demuxer-max-bytes", "1GiB"),
    ("demuxer-max-back-bytes", "256MiB"),
    ("stream-buffer-size", "4MiB"),
    ("cache-pause-wait", "10"),
    ("demuxer-lavf-o", "http_persistent=0,http_multiple=0"),
];

pub fn play(url: &str, opts: &Options, controls: &Receiver<Control>, surface: &Surface, on_tick: &mut dyn FnMut(Tick)) -> Outcome {
    match play_inner(url, opts, controls, surface, on_tick) {
        Ok(o) => o,
        Err(e) => Outcome::Failed(e),
    }
}

fn play_inner(url: &str, opts: &Options, controls: &Receiver<Control>, surface: &Surface, on_tick: &mut dyn FnMut(Tick)) -> Result<Outcome, String> {
    let api = api()?;
    // SAFETY: function pointers come from libmpv's documented C API with matching signatures;
    // the core handle is only used on this thread until destroyed below.
    unsafe {
        let ctx = (api.create)();
        if ctx.is_null() {
            return Err("mpv_create failed".into());
        }
        let c = |s: &str| CString::new(s).unwrap_or_default();
        let set = |k: &str, v: &str| {
            let (k, v) = (c(k), c(v));
            (api.set_opt)(ctx, k.as_ptr(), v.as_ptr());
        };
        for (k, v) in DEFAULT_OPTIONS {
            set(k, v);
        }
        set("force-media-title", &opts.title);
        if opts.keep_open {
            set("keep-open", "yes");
        }
        if !opts.hw_decode {
            set("hwdec", "no");
        } else {
            match opts.hw_mode {
                2 => set("hwdec", "dxva2-copy"),
                3 => set("hwdec", "nvdec"),
                4 => set("hwdec", "auto-safe"),
                m => {
                    set("hwdec", "auto-copy-safe");
                    if m == 1 {
                        set("vf", "format=yuv420p10le");
                    }
                }
            }
        }
        if let Some(path) = &opts.debug_log {
            set("log-file", path);
            set("msg-level", "all=v");
        }
        // Optional user overrides, one `key=value` per line (# comments), applied last.
        if let Ok(text) = std::fs::read_to_string(engine_dir().with_file_name("mpv.conf")) {
            for line in text.lines().map(str::trim).filter(|l| !l.is_empty() && !l.starts_with('#')) {
                if let Some((k, v)) = line.split_once('=') {
                    set(k.trim(), v.trim());
                }
            }
        }
        if opts.start_secs > 1.0 {
            set("start", &format!("{:.3}", opts.start_secs));
        }
        let rc = (api.init)(ctx);
        if rc < 0 {
            (api.destroy)(ctx);
            return Err(format!("mpv_initialize failed ({rc})"));
        }

        let run = |args: &[&str]| {
            let owned: Vec<CString> = args.iter().map(|a| c(a)).collect();
            let mut argv: Vec<*const c_char> = owned.iter().map(|a| a.as_ptr()).collect();
            argv.push(std::ptr::null());
            (api.command)(ctx, argv.as_ptr())
        };
        let read_str = |name: &str| -> Option<String> {
            let n = c(name);
            let p = (api.get_prop)(ctx, n.as_ptr());
            if p.is_null() {
                return None;
            }
            let v = CStr::from_ptr(p).to_string_lossy().into_owned();
            (api.free)(p as *mut c_void);
            Some(v)
        };

        surface.publish(api, ctx);
        if run(&["loadfile", url]) < 0 {
            surface.unpublish();
            (api.destroy)(ctx);
            return Err("mpv rejected the file".into());
        }

        let mut outcome = Outcome::Stopped;
        let mut last_poll = Instant::now() - Duration::from_secs(1);
        let mut last_diag = Instant::now();
        let mut tracks_due: Option<Instant> = Some(Instant::now() + Duration::from_millis(800));
        'play: loop {
            let mut forced_poll = false;
            while let Ok(ctl) = controls.try_recv() {
                forced_poll = true;
                match ctl {
                    Control::Quit => {
                        run(&["quit"]);
                    }
                    Control::Finish => {
                        outcome = Outcome::Finished;
                        break 'play;
                    }
                    Control::TogglePause => {
                        run(&["cycle", "pause"]);
                    }
                    Control::SeekBy(s) => {
                        run(&["seek", &s.to_string()]);
                    }
                    Control::VolumeBy(v) => {
                        run(&["add", "volume", &v.to_string()]);
                    }
                    Control::ToggleMute => {
                        run(&["cycle", "mute"]);
                    }
                    Control::CycleSubtitles => {
                        run(&["cycle", "sub"]);
                    }
                    Control::CycleAudio => {
                        run(&["cycle", "audio"]);
                    }
                    Control::SetPause(p) => {
                        run(&["set", "pause", if p { "yes" } else { "no" }]);
                    }
                    Control::SeekTo(s) => {
                        run(&["seek", &format!("{s:.3}"), "absolute"]);
                    }
                    Control::SetVolume(v) => {
                        run(&["set", "volume", &v.to_string()]);
                    }
                    Control::SetMute(m) => {
                        run(&["set", "mute", if m { "yes" } else { "no" }]);
                    }
                    Control::AudioDelayBy(d) => {
                        run(&["add", "audio-delay", &format!("{d:.3}")]);
                    }
                    Control::ResetAudioDelay => {
                        run(&["set", "audio-delay", "0"]);
                    }
                    Control::SelectAudio(id) => {
                        run(&["set", "aid", &id.to_string()]);
                        tracks_due = Some(Instant::now() + Duration::from_millis(300));
                    }
                    Control::SelectSub(id) => {
                        run(&["set", "sid", &if id < 0 { "no".to_string() } else { id.to_string() }]);
                        tracks_due = Some(Instant::now() + Duration::from_millis(300));
                    }
                    Control::AddSubtitle { url, title, lang } => {
                        run(&["sub-add", &url, "select", &title, &lang]);
                        tracks_due = Some(Instant::now() + Duration::from_millis(500));
                    }
                }
            }
            let ev = &*(api.wait)(ctx, 0.03);
            match ev.event_id {
                EV_SHUTDOWN => break,
                EV_FILE_LOADED => {
                    if let Some(a) = opts.audio_pos {
                        run(&["set", "aid", &a.to_string()]);
                    }
                    for sub in &opts.external_subs {
                        run(&["sub-add", &sub.url, "auto", &sub.title, &sub.lang]);
                    }
                    tracks_due = Some(Instant::now() + Duration::from_millis(500));
                }
                EV_END_FILE => {
                    let e = &*(ev.data as *const MpvEventEndFile);
                    outcome = match e.reason {
                        END_EOF => Outcome::Finished,
                        END_ERROR => Outcome::Failed(format!("playback failed (mpv error {})", e.error)),
                        _ => Outcome::Stopped,
                    };
                    break;
                }
                _ => {}
            }
            if !forced_poll && last_poll.elapsed() < Duration::from_millis(250) {
                continue;
            }
            last_poll = Instant::now();
            if let Some(position) = read_str("time-pos").and_then(|s| s.parse::<f64>().ok()) {
                let tracks = match tracks_due {
                    Some(t) if Instant::now() >= t => {
                        tracks_due = None;
                        Some(read_tracks(&read_str))
                    }
                    _ => None,
                };
                on_tick(Tick {
                    position,
                    duration: read_str("duration").and_then(|s| s.parse().ok()).unwrap_or(0.0),
                    paused: read_str("pause").as_deref() == Some("yes"),
                    eof: opts.keep_open && read_str("eof-reached").as_deref() == Some("yes"),
                    volume: read_str("volume").and_then(|s| s.parse().ok()).unwrap_or(100.0),
                    muted: read_str("mute").as_deref() == Some("yes"),
                    audio: track_label(&read_str, "audio"),
                    subtitle: track_label(&read_str, "sub"),
                    audio_delay: read_str("audio-delay").and_then(|s| s.parse().ok()).unwrap_or(0.0),
                    diag: if last_diag.elapsed() >= Duration::from_secs(if opts.debug_log.is_some() { 2 } else { 10 }) {
                        last_diag = Instant::now();
                        let g = |k: &str| read_str(k).unwrap_or_else(|| "-".into());
                        let extra = if opts.debug_log.is_some() {
                            format!(
                                " | ao={} dev={} aparams={} aout={} abitrate={} cache={}s cachespeed={} buffering={} pausedcache={} coreidle={} vsync={} vspeed={} aspeed={} dispfps={}",
                                g("current-ao"),
                                g("audio-device"),
                                g("audio-params"),
                                g("audio-out-params"),
                                g("audio-bitrate"),
                                g("demuxer-cache-duration"),
                                g("cache-speed"),
                                g("cache-buffering-state"),
                                g("paused-for-cache"),
                                g("core-idle"),
                                g("video-sync"),
                                g("video-speed-correction"),
                                g("audio-speed-correction"),
                                g("display-fps")
                            )
                        } else {
                            String::new()
                        };
                        Some(format!(
                            "avsync={} drops(vo/dec)={}/{} delayed={} hwdec={} codec={} fps(container/est)={}/{}{extra}",
                            g("avsync"),
                            g("frame-drop-count"),
                            g("decoder-frame-drop-count"),
                            g("vo-delayed-frame-count"),
                            g("hwdec-current"),
                            g("video-format"),
                            g("container-fps"),
                            g("estimated-vf-fps")
                        ))
                    } else {
                        None
                    },
                    tracks,
                });
            }
        }
        // The render context must be freed (on the GL thread) before the core is destroyed.
        if surface.unpublish() {
            (api.destroy)(ctx);
        }
        if let Some(path) = &opts.debug_log {
            redact_log(path);
        }
        Ok(outcome)
    }
}

/// mpv logs the stream URL, which carries the access token: blank it out of the debug log.
fn redact_log(path: &str) {
    let Ok(text) = std::fs::read_to_string(path) else { return };
    let mut out = String::with_capacity(text.len());
    for line in text.lines() {
        let mut rest = line;
        while let Some(i) = rest.find("api_key=") {
            out.push_str(&rest[..i + 8]);
            let tail = &rest[i + 8..];
            let end = tail.find(|c: char| c == '&' || c.is_whitespace() || c == '"' || c == '\'').unwrap_or(tail.len());
            out.push_str("REDACTED");
            rest = &tail[end..];
        }
        out.push_str(rest);
        out.push('\n');
    }
    let _ = std::fs::write(path, out);
}

fn track_label(read: &dyn Fn(&str) -> Option<String>, kind: &str) -> String {
    let title = read(&format!("current-tracks/{kind}/title")).filter(|s| !s.is_empty());
    let lang = read(&format!("current-tracks/{kind}/lang")).filter(|s| !s.is_empty());
    match (lang, title) {
        (Some(l), Some(t)) => format!("{l} · {t}"),
        (Some(l), None) => l,
        (None, Some(t)) => t,
        (None, None) => {
            if read(&format!("current-tracks/{kind}/id")).is_some() {
                "On".into()
            } else {
                "Off".into()
            }
        }
    }
}

fn read_tracks(read: &dyn Fn(&str) -> Option<String>) -> Vec<Track> {
    let n: usize = read("track-list/count").and_then(|s| s.parse().ok()).unwrap_or(0);
    let mut out = Vec::new();
    for i in 0..n {
        let Some(kind) = read(&format!("track-list/{i}/type")) else { continue };
        if kind != "audio" && kind != "sub" {
            continue;
        }
        let Some(id) = read(&format!("track-list/{i}/id")).and_then(|s| s.parse::<i64>().ok()) else { continue };
        let field = |k: &str| read(&format!("track-list/{i}/{k}")).filter(|s| !s.is_empty());
        let yes = |k: &str| field(k).as_deref() == Some("yes");
        let mut parts: Vec<String> = Vec::new();
        if let Some(l) = field("lang") {
            parts.push(l.to_uppercase());
        }
        if let Some(t) = field("title") {
            parts.push(t);
        }
        if let Some(c) = field("codec") {
            parts.push(c.to_uppercase());
        }
        if kind == "audio" {
            if let Some(ch) = field("demux-channel-count") {
                parts.push(format!("{ch}ch"));
            }
        }
        if yes("forced") {
            parts.push("Forced".into());
        }
        if yes("external") {
            parts.push("External".into());
        }
        let label = if parts.is_empty() { format!("Track {id}") } else { parts.join(" · ") };
        out.push(Track { kind, id, label, selected: yes("selected") });
    }
    out
}
