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
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
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
    /// Select it on load (the server flags it default/forced).
    pub select: bool,
}

#[derive(Debug, Clone)]
pub struct Options {
    pub title: String,
    pub start_secs: f64,
    pub external_subs: Vec<ExtSub>,
    /// Embedded text subtitles are supplied as `external_subs` (fetched from the server), so mpv's own
    /// copies are hidden and never selected: selecting one makes mpv demux the whole remote stream and freeze.
    pub decoupled: bool,
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
    /// Shift subtitles by this many seconds (positive = later); 0 resets.
    SubDelayBy(f64),
    ResetSubDelay,
    ResetAudioDelay,
    /// -1 turns subtitles off
    SelectSub(i64),
    AddSubtitle { url: String, title: String, lang: String },
    /// Subtitle vertical position, mpv `sub-pos` (100 = bottom, lower = higher up). Fractions are used when
    /// this mpv accepts them, so the slide with the controls is smooth.
    SubPos(f64),
    /// Save the current frame (with subtitles) to this PNG path.
    Screenshot(String),
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
    /// Display aspect ratio of the video (0 = unknown).
    pub aspect: f64,
    pub audio: String,
    pub subtitle: String,
    pub audio_delay: f64,
    pub sub_delay: f64,
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
const PARAM_BLOCK_FOR_TARGET_TIME: c_int = 12;

#[derive(Clone, Copy)]
struct Api {
    create: unsafe extern "C" fn() -> *mut c_void,
    set_opt: unsafe extern "C" fn(*mut c_void, *const c_char, *const c_char) -> c_int,
    init: unsafe extern "C" fn(*mut c_void) -> c_int,
    command: unsafe extern "C" fn(*mut c_void, *const *const c_char) -> c_int,
    command_async: unsafe extern "C" fn(*mut c_void, u64, *const *const c_char) -> c_int,
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
    if cfg!(target_os = "android") {
        return true; // libmpv.so is packaged in the APK
    }
    dll_candidates().iter().any(|p| p.exists())
}

/// Android loads by bare name: the system linker finds libmpv.so among the APK's native libraries.
fn lib_candidates() -> Vec<PathBuf> {
    if cfg!(target_os = "android") {
        vec![PathBuf::from("libmpv.so")]
    } else {
        dll_candidates()
    }
}

fn load_api() -> Result<Api, String> {
    let mut last = String::from("the video engine (libmpv) is not installed yet");
    for p in lib_candidates() {
        if !cfg!(target_os = "android") && !p.exists() {
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
            command_async: sym!(b"mpv_command_async"),
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
/// While set, rendering does not wait for each video frame's display time. mpv's wait throttles every redraw to the
/// video's frame rate (24 fps), which made the controls' fade and the subtitle slide look choppy.
static UNBLOCKED: AtomicBool = AtomicBool::new(false);
static LOG_HOOK: std::sync::OnceLock<fn(&str)> = std::sync::OnceLock::new();

/// The host's diagnostic logger, so the player's teardown steps show up in its debug log.
pub fn set_log(f: fn(&str)) {
    let _ = LOG_HOOK.set(f);
}

fn plog(msg: &str) {
    if let Some(f) = LOG_HOOK.get() {
        f(msg);
    }
}

/// Destroys the mpv core. On Android `mpv_terminate_destroy` has been seen never to return, which left the
/// player thread (and so the UI, which waits for it) stuck on a black screen. It runs on a helper thread and is
/// given a few seconds; after that the instance is abandoned (leaked) so the player can close.
fn destroy_core(api: Api, ctx: *mut c_void) {
    let t0 = Instant::now();
    plog("player: destroying the mpv core");
    let p = ctx as usize;
    let (tx, rx) = std::sync::mpsc::channel();
    let spawned = std::thread::Builder::new().name("mpv-destroy".into()).spawn(move || {
        // SAFETY: the pointer is the core handle this player created, and nothing else uses it any more.
        unsafe { (api.destroy)(p as *mut c_void) };
        let _ = tx.send(());
    });
    match spawned {
        Ok(_) => match rx.recv_timeout(Duration::from_secs(3)) {
            Ok(()) => plog(&format!("player: mpv core destroyed in {} ms", t0.elapsed().as_millis())),
            Err(_) => plog("player: the mpv core did not shut down within 3 s: abandoned, so the player can close"),
        },
        // SAFETY: as above.
        Err(_) => unsafe { (api.destroy)(ctx) },
    }
}

/// Called by the UI: true while it is animating (the player controls are showing or fading).
pub fn set_unblocked(on: bool) {
    UNBLOCKED.store(on, Ordering::Relaxed);
}

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
    gl: Option<GlFns>, // resolved once per GL context
    mini: Option<MiniTex>, // off-screen target used while the video is shown as a small in-app image
}

/// Off-screen render target (GL thread only): mpv draws here and the host shows the texture as an image.
#[derive(Clone, Copy)]
struct MiniTex {
    fbo: u32,
    tex: u32,
    w: i32,
    h: i32,
}

const GL_TEXTURE_2D: u32 = 0x0DE1;
const GL_FRAMEBUFFER: u32 = 0x8D40;

/// The few GL entry points needed to manage the off-screen target.
#[derive(Clone, Copy)]
struct GlFns {
    gen_tex: unsafe extern "system" fn(i32, *mut u32),
    del_tex: unsafe extern "system" fn(i32, *const u32),
    bind_tex: unsafe extern "system" fn(u32, u32),
    tex_image: unsafe extern "system" fn(u32, i32, i32, i32, i32, i32, u32, u32, *const c_void),
    tex_param: unsafe extern "system" fn(u32, u32, i32),
    gen_fbo: unsafe extern "system" fn(i32, *mut u32),
    del_fbo: unsafe extern "system" fn(i32, *const u32),
    bind_fbo: unsafe extern "system" fn(u32, u32),
    fbo_tex: unsafe extern "system" fn(u32, u32, u32, u32, i32),
    get_int: unsafe extern "system" fn(u32, *mut i32),
    active_tex: unsafe extern "system" fn(u32),
    pixel_store: unsafe extern "system" fn(u32, i32),
    attrib_off: unsafe extern "system" fn(u32),
    // State the host's renderer relies on, saved and put back around mpv's drawing (see `save`).
    use_program: unsafe extern "system" fn(u32),
    bind_buffer: unsafe extern "system" fn(u32, u32),
    viewport: unsafe extern "system" fn(i32, i32, i32, i32),
    scissor: unsafe extern "system" fn(i32, i32, i32, i32),
    enable: unsafe extern "system" fn(u32),
    disable: unsafe extern "system" fn(u32),
    is_enabled: unsafe extern "system" fn(u32) -> u8,
    blend_sep: unsafe extern "system" fn(u32, u32, u32, u32),
    blend_eq_sep: unsafe extern "system" fn(u32, u32),
    color_mask: unsafe extern "system" fn(u8, u8, u8, u8),
    depth_mask: unsafe extern "system" fn(u8),
    stencil_mask: unsafe extern "system" fn(u32),
    get_attrib: unsafe extern "system" fn(u32, u32, *mut i32),
    attrib_on: unsafe extern "system" fn(u32),
    get_error: unsafe extern "system" fn() -> u32,
    vao: Option<unsafe extern "system" fn(u32)>,
    bind_rbo: Option<unsafe extern "system" fn(u32, u32)>,
}

/// Everything in `GlFns::save` (indexes mirror the order it reads them).
#[derive(Clone, Copy, Default)]
struct GlSaved {
    program: i32,
    buffers: [i32; 3], // array, element array, pixel unpack
    vao: i32,
    fbo: i32,
    rbo: i32,
    active: i32,
    tex2d: [i32; 4],
    tex_ext: [i32; 4],
    viewport: [i32; 4],
    scissor: [i32; 4],
    color_mask: [i32; 4],
    caps: [bool; 6], // scissor, blend, depth, stencil, cull, dither
    blend: [i32; 6], // src rgb, dst rgb, src alpha, dst alpha, equation rgb, equation alpha
    depth_mask: i32,
    stencil_mask: i32,
    unpack: [i32; 4], // alignment, row length, skip pixels, skip rows
    attribs: [bool; 8],
}

impl GlFns {
    unsafe fn load(loader: GlLoader) -> Option<GlFns> {
        macro_rules! f {
            ($n:literal) => {{
                let p = loader(CStr::from_bytes_with_nul_unchecked(concat!($n, " ").as_bytes()));
                if p.is_null() {
                    return None;
                }
                std::mem::transmute(p)
            }};
        }
        Some(GlFns {
            gen_tex: f!("glGenTextures"),
            del_tex: f!("glDeleteTextures"),
            bind_tex: f!("glBindTexture"),
            tex_image: f!("glTexImage2D"),
            tex_param: f!("glTexParameteri"),
            gen_fbo: f!("glGenFramebuffers"),
            del_fbo: f!("glDeleteFramebuffers"),
            bind_fbo: f!("glBindFramebuffer"),
            fbo_tex: f!("glFramebufferTexture2D"),
            get_int: f!("glGetIntegerv"),
            active_tex: f!("glActiveTexture"),
            pixel_store: f!("glPixelStorei"),
            attrib_off: f!("glDisableVertexAttribArray"),
            use_program: f!("glUseProgram"),
            bind_buffer: f!("glBindBuffer"),
            viewport: f!("glViewport"),
            scissor: f!("glScissor"),
            enable: f!("glEnable"),
            disable: f!("glDisable"),
            is_enabled: f!("glIsEnabled"),
            blend_sep: f!("glBlendFuncSeparate"),
            blend_eq_sep: f!("glBlendEquationSeparate"),
            color_mask: f!("glColorMask"),
            depth_mask: f!("glDepthMask"),
            stencil_mask: f!("glStencilMask"),
            get_attrib: f!("glGetVertexAttribiv"),
            attrib_on: f!("glEnableVertexAttribArray"),
            get_error: f!("glGetError"),
            vao: {
                let p = loader(CStr::from_bytes_with_nul_unchecked(b"glBindVertexArray\0"));
                (!p.is_null()).then(|| std::mem::transmute(p))
            },
            bind_rbo: {
                let p = loader(CStr::from_bytes_with_nul_unchecked(b"glBindRenderbuffer\0"));
                (!p.is_null()).then(|| std::mem::transmute(p))
            },
        })
    }

    /// Reads the GL state mpv is about to disturb. mpv draws with its own shaders, buffers and settings and does
    /// not put them back; on Android the UI renderer then drew new text and icons wrongly (white squares, black
    /// boxes around letters), because its uploads and draws ran on top of mpv's leftovers.
    unsafe fn save(&self) -> GlSaved {
        let g = |e: u32| {
            let mut v = 0i32;
            (self.get_int)(e, &mut v);
            v
        };
        let g4 = |e: u32| {
            let mut v = [0i32; 4];
            (self.get_int)(e, v.as_mut_ptr());
            v
        };
        let mut st = GlSaved { active: g(0x84E0), program: g(0x8B8D), fbo: g(0x8CA6), ..Default::default() };
        st.buffers = [g(0x8894), g(0x8895), g(0x88EF)];
        if self.vao.is_some() {
            st.vao = g(0x85B5);
        }
        if self.bind_rbo.is_some() {
            st.rbo = g(0x8CA7);
        }
        for u in 0..4u32 {
            (self.active_tex)(0x84C0 + u);
            st.tex2d[u as usize] = g(0x8069);
            st.tex_ext[u as usize] = g(0x8D67);
        }
        (self.active_tex)(st.active as u32);
        st.viewport = g4(0x0BA2);
        st.scissor = g4(0x0C10);
        st.color_mask = g4(0x0C23);
        for (i, cap) in [0x0C11u32, 0x0BE2, 0x0B71, 0x0B90, 0x0B44, 0x0BD0].into_iter().enumerate() {
            st.caps[i] = (self.is_enabled)(cap) != 0;
        }
        for (i, e) in [0x80C9u32, 0x80C8, 0x80CB, 0x80CA, 0x8009, 0x883D].into_iter().enumerate() {
            st.blend[i] = g(e);
        }
        st.depth_mask = g(0x0B72);
        st.stencil_mask = g(0x0B98);
        for (i, e) in [0x0CF5u32, 0x0CF2, 0x0CF4, 0x0CF3].into_iter().enumerate() {
            st.unpack[i] = g(e);
        }
        for i in 0..8u32 {
            let mut on = 0i32;
            (self.get_attrib)(i, 0x8622, &mut on);
            st.attribs[i as usize] = on != 0;
        }
        self.clear_errors();
        st
    }

    /// Sets the state mpv assumes: the GL defaults. The UI renderer leaves non-default settings behind (for example the
    /// row length it used for its glyph atlas), and mpv, which only sets what it needs for the job at hand, then
    /// uploads video frames sheared into static.
    unsafe fn defaults(&self) {
        for (e, v) in [(0x0CF5u32, 4), (0x0CF2, 0), (0x0CF4, 0), (0x0CF3, 0)] {
            (self.pixel_store)(e, v);
        }
        (self.bind_buffer)(0x88EC, 0);
        (self.bind_buffer)(0x8892, 0);
        (self.bind_buffer)(0x8893, 0);
        (self.use_program)(0);
        (self.active_tex)(0x84C0);
        for cap in [0x0C11u32, 0x0BE2, 0x0B71, 0x0B90, 0x0B44] {
            (self.disable)(cap);
        }
        (self.enable)(0x0BD0);
        (self.blend_sep)(1, 0, 1, 0);
        (self.blend_eq_sep)(0x8006, 0x8006);
        (self.color_mask)(1, 1, 1, 1);
        (self.depth_mask)(1);
        (self.stencil_mask)(0xFFFF_FFFF);
        self.clear_errors();
    }

    /// Puts back what `save` read.
    unsafe fn restore(&self, st: &GlSaved) {
        (self.use_program)(st.program as u32);
        (self.bind_buffer)(0x8892, st.buffers[0] as u32);
        (self.bind_buffer)(0x8893, st.buffers[1] as u32);
        (self.bind_buffer)(0x88EC, st.buffers[2] as u32);
        if let Some(vao) = self.vao {
            vao(st.vao as u32);
        }
        (self.bind_fbo)(GL_FRAMEBUFFER, st.fbo as u32);
        if let Some(rbo) = self.bind_rbo {
            rbo(0x8D41, st.rbo as u32);
        }
        for u in 0..4u32 {
            (self.active_tex)(0x84C0 + u);
            (self.bind_tex)(GL_TEXTURE_2D, st.tex2d[u as usize] as u32);
            (self.bind_tex)(0x8D65, st.tex_ext[u as usize] as u32);
        }
        (self.active_tex)(st.active as u32);
        let v = st.viewport;
        (self.viewport)(v[0], v[1], v[2], v[3]);
        let c = st.scissor;
        (self.scissor)(c[0], c[1], c[2], c[3]);
        for (i, cap) in [0x0C11u32, 0x0BE2, 0x0B71, 0x0B90, 0x0B44, 0x0BD0].into_iter().enumerate() {
            if st.caps[i] {
                (self.enable)(cap);
            } else {
                (self.disable)(cap);
            }
        }
        let b = st.blend;
        (self.blend_sep)(b[0] as u32, b[1] as u32, b[2] as u32, b[3] as u32);
        (self.blend_eq_sep)(b[4] as u32, b[5] as u32);
        let m = st.color_mask;
        (self.color_mask)(m[0] as u8, m[1] as u8, m[2] as u8, m[3] as u8);
        (self.depth_mask)(st.depth_mask as u8);
        (self.stencil_mask)(st.stencil_mask as u32);
        for (i, e) in [0x0CF5u32, 0x0CF2, 0x0CF4, 0x0CF3].into_iter().enumerate() {
            (self.pixel_store)(e, st.unpack[i]);
        }
        for i in 0..8u32 {
            if st.attribs[i as usize] {
                (self.attrib_on)(i);
            } else {
                (self.attrib_off)(i);
            }
        }
        self.clear_errors();
    }

    /// Queries and restores above can raise harmless errors on older GL versions; don't leave them for the host to see.
    unsafe fn clear_errors(&self) {
        for _ in 0..16 {
            if (self.get_error)() == 0 {
                break;
            }
        }
    }

    unsafe fn free(&self, m: MiniTex) {
        (self.del_fbo)(1, &m.fbo);
        (self.del_tex)(1, &m.tex);
    }

    unsafe fn create(&self, w: i32, h: i32) -> MiniTex {
        let (mut prev_tex, mut tex, mut fbo) = (0i32, 0u32, 0u32);
        (self.get_int)(0x8069, &mut prev_tex);
        (self.gen_tex)(1, &mut tex);
        (self.bind_tex)(GL_TEXTURE_2D, tex);
        (self.tex_image)(GL_TEXTURE_2D, 0, 0x8058, w, h, 0, 0x1908, 0x1401, std::ptr::null());
        (self.tex_param)(GL_TEXTURE_2D, 0x2801, 0x2601);
        (self.tex_param)(GL_TEXTURE_2D, 0x2800, 0x2601);
        (self.tex_param)(GL_TEXTURE_2D, 0x2802, 0x812F);
        (self.tex_param)(GL_TEXTURE_2D, 0x2803, 0x812F);
        (self.bind_tex)(GL_TEXTURE_2D, prev_tex as u32);
        let mut prev_fbo = 0i32;
        (self.get_int)(0x8CA6, &mut prev_fbo);
        (self.gen_fbo)(1, &mut fbo);
        (self.bind_fbo)(GL_FRAMEBUFFER, fbo);
        (self.fbo_tex)(GL_FRAMEBUFFER, 0x8CE0, GL_TEXTURE_2D, tex, 0);
        (self.bind_fbo)(GL_FRAMEBUFFER, prev_fbo as u32);
        MiniTex { fbo, tex, w, h }
    }
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
            state: Mutex::new(SurfState { api: None, mpv: 0, render: 0, release: false, gl: None, mini: None }),
            redraw: Box::new(redraw),
        }))
    }

    /// Draws the current video frame into the bound framebuffer 0 of size `w` x `h`.
    /// Call from the GL thread (Slint's BeforeRendering). Cheap when nothing is playing.
    /// With `mini` = Some((w, h)) the video goes into an off-screen texture of that size instead and
    /// the texture id is returned, for the host to draw as an image.
    pub fn render(&self, loader: GlLoader, w: i32, h: i32, mini: Option<(i32, i32)>) -> Option<u32> {
        let mut st = self.0.state.lock().unwrap();
        let Some(api) = st.api else { return None };
        let mut tex_out = None;
        // SAFETY: render context pointers are only used here, under the lock, on the GL thread.
        unsafe {
            if st.release || st.mpv == 0 {
                if st.render != 0 {
                    (api.render_set_update_cb)(st.render as *mut c_void, None, std::ptr::null_mut());
                    (api.render_free)(st.render as *mut c_void);
                    st.render = 0;
                }
                return None;
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
                if st.gl.is_none() {
                    st.gl = GlFns::load(loader);
                }
                let gl = st.gl;
                // Off-screen target: (re)create it to match, or drop it when no longer wanted.
                let mut target = None;
                if let Some(gl) = &gl {
                    match (mini, st.mini) {
                        (Some((mw, mh)), cur) => {
                            if cur.is_none_or(|m| m.w != mw || m.h != mh) {
                                if let Some(m) = cur {
                                    gl.free(m);
                                }
                                st.mini = Some(gl.create(mw.max(2), mh.max(2)));
                            }
                            target = st.mini;
                        }
                        (None, Some(m)) => {
                            gl.free(m);
                            st.mini = None;
                        }
                        (None, None) => {}
                    }
                }
                let mut prev_fbo = 0i32;
                if let (Some(gl), Some(_)) = (&gl, target) {
                    (gl.get_int)(0x8CA6, &mut prev_fbo);
                }
                let mut fbo = match target {
                    Some(m) => GlFbo { fbo: m.fbo as c_int, w: m.w, h: m.h, internal_format: 0x8058 },
                    None => GlFbo { fbo: 0, w, h, internal_format: 0 },
                };
                let mut flip: c_int = if target.is_some() { 0 } else { 1 };
                // By default mpv waits inside render for the frame's display time. In the small player that wait
                // stalls the whole UI (drags, hovers, the page behind), so don't wait there.
                let mut block: c_int = if target.is_some() || UNBLOCKED.load(Ordering::Relaxed) { 0 } else { 1 };
                let mut params = [
                    RenderParam { kind: PARAM_GL_FBO, data: &mut fbo as *mut GlFbo as *mut c_void },
                    RenderParam { kind: PARAM_FLIP_Y, data: &mut flip as *mut c_int as *mut c_void },
                    RenderParam { kind: PARAM_BLOCK_FOR_TARGET_TIME, data: &mut block as *mut c_int as *mut c_void },
                    RenderParam { kind: 0, data: std::ptr::null_mut() },
                ];
                // Android: mpv's leftovers corrupted the UI's text and icons (see `GlFns::save`).
                let saved = if cfg!(target_os = "android") { gl.as_ref().map(|g| g.save()) } else { None };
                if let (Some(gl), Some(_)) = (&gl, &saved) {
                    gl.defaults();
                }
                (api.render_render)(ctx, params.as_mut_ptr());
                // mpv leaves GL state behind that the UI renderer then trips over (garbled text and icons).
                if let Some(gl) = &gl {
                    (gl.active_tex)(0x84C0);
                    (gl.pixel_store)(0x0CF5, 4);
                    for i in 2..8 {
                        (gl.attrib_off)(i);
                    }
                }
                if let (Some(gl), Some(m)) = (&gl, target) {
                    (gl.bind_fbo)(GL_FRAMEBUFFER, prev_fbo as u32);
                    tex_out = Some(m.tex);
                }
                if let (Some(gl), Some(saved)) = (&gl, &saved) {
                    gl.restore(saved);
                }
            }
            GL_LOADER.store(0, Ordering::Release);
        }
        tex_out
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
        st.gl = None;
        st.mini = None;
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
        } else if cfg!(target_os = "android") {
            set("ao", "audiotrack,opensles");
            // MediaCodec. "mediacodec" hands decoded frames to the GL surface without copying (fast);
            // "-copy" copies every frame back to memory first, which on weak TV chips (Chromecast
            // with Google TV) managed only a few frames a second. Automatic tries zero-copy and
            // falls back to copying if the device can't do it.
            set(
                "hwdec",
                match opts.hw_mode {
                    4 => "mediacodec",
                    2 => "mediacodec-copy",
                    _ => "mediacodec,mediacodec-copy",
                },
            );
            // Read ahead less: a TV stick has 1-2 GB of memory for everything.
            set("demuxer-max-bytes", "150MiB");
            set("demuxer-max-back-bytes", "50MiB");
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
            // sub-add downloads the file before returning; a slow server must not freeze this loop.
            if args[0] == "sub-add" { (api.command_async)(ctx, 0, argv.as_ptr()) } else { (api.command)(ctx, argv.as_ptr()) }
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
        let mut sub_ids: Vec<i64> = Vec::new(); // selectable subtitle track ids, for cycling
        let mut sub_float: Option<bool> = None; // does this mpv accept a fractional sub-pos?
        'play: loop {
            let mut forced_poll = false;
            let mut sub_pos: Option<f64> = None;
            while let Ok(ctl) = controls.try_recv() {
                // Sub-position nudges arrive every frame while the controls fade; only the latest matters,
                // and they must not trigger the (slow) property poll below.
                if let Control::SubPos(p) = ctl {
                    sub_pos = Some(p);
                    continue;
                }
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
                        let cur = read_str("sid").and_then(|s| s.parse::<i64>().ok());
                        let next = match cur.and_then(|c| sub_ids.iter().position(|i| *i == c)) {
                            Some(p) => sub_ids.get(p + 1).copied(),
                            None => sub_ids.first().copied(),
                        };
                        run(&["set", "sid", &next.map_or("no".to_string(), |i| i.to_string())]);
                        tracks_due = Some(Instant::now() + Duration::from_millis(300));
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
                    Control::SubDelayBy(d) => {
                        run(&["add", "sub-delay", &format!("{d:.3}")]);
                    }
                    Control::ResetSubDelay => {
                        run(&["set", "sub-delay", "0"]);
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
                    Control::Screenshot(path) => {
                        run(&["screenshot-to-file", &path, "subtitles"]);
                    }
                    Control::SubPos(_) => {}
                    Control::AddSubtitle { url, title, lang } => {
                        run(&["sub-add", &url, "select", &title, &lang]);
                        tracks_due = Some(Instant::now() + Duration::from_millis(500));
                    }
                }
            }
            if let Some(p) = sub_pos {
                // Older mpv builds only take whole percentages: the first fractional attempt tells which.
                let text = if sub_float != Some(false) { format!("{p:.2}") } else { format!("{}", p.round() as i64) };
                let args: Vec<CString> = ["set", "sub-pos", &text].iter().map(|a| c(a)).collect();
                let mut argv: Vec<*const c_char> = args.iter().map(|a| a.as_ptr()).collect();
                argv.push(std::ptr::null());
                if sub_float.is_none() {
                    sub_float = Some((api.command)(ctx, argv.as_ptr()) >= 0);
                } else {
                    (api.command_async)(ctx, 0, argv.as_ptr());
                }
            }
            let ev = &*(api.wait)(ctx, if sub_pos.is_some() { 0.008 } else { 0.03 });
            match ev.event_id {
                EV_SHUTDOWN => break,
                EV_FILE_LOADED => {
                    if let Some(a) = opts.audio_pos {
                        run(&["set", "aid", &a.to_string()]);
                    }
                    if opts.decoupled && read_str("current-tracks/sub/external").as_deref() != Some("yes") && read_str("current-tracks/sub/codec").is_some_and(|c| is_text_codec(&c)) {
                        run(&["set", "sid", "no"]);
                    }
                    for sub in &opts.external_subs {
                        run(&["sub-add", &sub.url, if sub.select { "select" } else { "auto" }, &sub.title, &sub.lang]);
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
            // While subtitles are sliding with the controls, don't stall the loop on the property poll: each poll
            // blocks for a moment, which made the slide hitch about four times a second. It runs right after.
            if !forced_poll && (sub_pos.is_some() || last_poll.elapsed() < Duration::from_millis(250)) {
                continue;
            }
            last_poll = Instant::now();
            if let Some(position) = read_str("time-pos").and_then(|s| s.parse::<f64>().ok()) {
                let tracks = match tracks_due {
                    Some(t) if Instant::now() >= t => {
                        tracks_due = None;
                        let t = read_tracks(&read_str, opts.decoupled);
                        sub_ids = t.iter().filter(|t| t.kind == "sub").map(|t| t.id).collect();
                        Some(t)
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
                    aspect: read_str("video-params/aspect").and_then(|s| s.parse().ok()).unwrap_or(0.0),
                    audio: track_label(&read_str, "audio"),
                    subtitle: track_label(&read_str, "sub"),
                    audio_delay: read_str("audio-delay").and_then(|s| s.parse().ok()).unwrap_or(0.0),
                    sub_delay: read_str("sub-delay").and_then(|s| s.parse().ok()).unwrap_or(0.0),
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
        plog("player: playback loop ended, releasing the render context");
        if surface.unpublish() {
            destroy_core(api, ctx);
        } else {
            plog("player: the render context was not released in time, so the mpv core is left alive");
        }
        plog("player: teardown finished");
        if let Some(path) = &opts.debug_log {
            redact_log(path);
        }
        Ok(outcome)
    }
}

/// mpv logs the stream URL, which carries the access token: blank it out of the debug log.
/// Run when playback ends, and by the app at start-up, because a hang or crash skips the first.
pub fn redact_log(path: &str) {
    // Bytes, not read_to_string: a stray invalid byte must not make the redaction silently skip the file.
    let Ok(bytes) = std::fs::read(path) else { return };
    let text = String::from_utf8_lossy(&bytes);
    if !text.contains("api_key=") {
        return;
    }
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

fn is_text_codec(c: &str) -> bool {
    matches!(c, "subrip" | "srt" | "ass" | "ssa" | "mov_text" | "webvtt" | "text" | "subviewer")
}

fn read_tracks(read: &dyn Fn(&str) -> Option<String>, hide_embedded_text: bool) -> Vec<Track> {
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
        if hide_embedded_text && kind == "sub" && !yes("external") && field("codec").is_some_and(|c| is_text_codec(&c)) {
            continue;
        }
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

/// Android: FFmpeg's MediaCodec decoder needs the process's JavaVM. libavcodec ships next to libmpv
/// in the APK; looked up by name so desktop builds don't need it.
#[cfg(target_os = "android")]
pub fn set_java_vm(vm: *mut c_void) {
    // SAFETY: av_jni_set_java_vm(void *vm, void *log_ctx) is FFmpeg's documented entry point.
    unsafe {
        let Ok(lib) = Library::new("libavcodec.so") else { return };
        let lib: &'static Library = Box::leak(Box::new(lib));
        if let Ok(f) = lib.get::<unsafe extern "C" fn(*mut c_void, *mut c_void) -> c_int>(b"av_jni_set_java_vm") {
            f(vm, std::ptr::null_mut());
        }
    }
}

#[cfg(not(target_os = "android"))]
pub fn set_java_vm(_vm: *mut c_void) {}
