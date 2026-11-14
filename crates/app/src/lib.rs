use emby_api::{Browse, Client, ImageKind, Item, MediaStream, User, TICKS_PER_SEC};
use serde::{Deserialize, Serialize};
use slint::{ComponentHandle, Image, Model, ModelRc, Rgba8Pixel, SharedPixelBuffer, SharedString, VecModel, Weak};
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use tokio::runtime::Handle;
use tokio::sync::Semaphore;

slint::include_modules!();

#[cfg(not(target_os = "android"))]
mod engine;
mod platform;
mod update;
mod seerr;
mod notes;
mod hide;
#[cfg(windows)]
mod media;
/// Windows-only media flyout / media keys; nothing to do elsewhere.
#[cfg(not(windows))]
mod media {
    use crate::AppWindow;
    pub fn sync(_ui: &AppWindow, _tx: &std::sync::mpsc::Sender<player::Control>) {}
    pub fn close() {}
}
use hide::HideTarget;
use notes::NoteRec;
use seerr::SeerrState;
use foyer_core::color::{parse_hex, rgb_to_hsv};
use foyer_core::{
    clock, data_dir, decide, duration_label, fmt_bps, fmt_size, home_order, load_config, quality_label, save_config, skip_step_index, Config, Quality, SavedServer, AUTOPLAY_STREAK_LIMIT,
    CARD_SCALES, HOME_ROWS, PRESETS, SKIP_STEPS,
};

// ---------------------------------------------------------------- config

static DEBUG: AtomicBool = AtomicBool::new(false);

fn logs_dir() -> PathBuf {
    data_dir().join("logs")
}

/// Extensive diagnostics, written only when switched on in Settings. Never receives passwords,
/// tokens or cookies.
fn dlog(msg: &str) {
    use std::io::Write;
    if !DEBUG.load(Ordering::Relaxed) {
        return;
    }
    let dir = logs_dir();
    let _ = std::fs::create_dir_all(&dir);
    let path = dir.join("debug.log");
    if std::fs::metadata(&path).map(|m| m.len() > 20_000_000).unwrap_or(false) {
        let _ = std::fs::rename(&path, dir.join("debug.old.log"));
    }
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(&path) {
        let _ = writeln!(f, "{} {msg}", chrono::Local::now().format("%F %T%.3f"));
    }
}

fn debug_banner(cfg: &Config) {
    let host = |u: &str| u.split("://").last().unwrap_or("").split('/').next().unwrap_or("").to_string();
    dlog("=== Foyer extensive debug logging is on ===");
    dlog(&format!("version {} | exe {:?}", env!("CARGO_PKG_VERSION"), std::env::current_exe().ok()));
    dlog(&format!("os: {}", platform::os_version()));
    dlog(&format!(
        "settings: quality={:?} hwdec={} autoplay={} material={} accent_mode={} nav_pinned={} seerr={} notify={}",
        Quality::from_cfg(&cfg.quality),
        cfg.hw_decode,
        cfg.autoplay_next,
        cfg.material,
        cfg.accent_mode,
        cfg.nav_pinned,
        cfg.seerr_enabled,
        cfg.notify_enabled
    ));
    dlog(&format!("media server host: {} | seerr host: {}", host(&cfg.server), host(&cfg.seerr_url)));
    dlog("(passwords, tokens and cookies are never written to this log)");
}

/// Appends a line to %APPDATA%\EmbyRevived\app.log (for diagnosing server/playback issues).
fn applog(msg: &str) {
    use std::io::Write;
    let _ = std::fs::create_dir_all(data_dir());
    if let Ok(mut f) = std::fs::OpenOptions::new().create(true).append(true).open(data_dir().join("app.log")) {
        let _ = writeln!(f, "{} {msg}", chrono::Local::now().format("%F %T"));
    }
    dlog(msg);
}

/// What is playing now, for subtitle search and external subtitles.
#[derive(Default)]
struct NowPlaying {
    item: String,
    source: String,
    added: std::collections::HashSet<i32>,
}

fn quality_rows(q: Quality, resolved: &str) -> Vec<TrackRow> {
    let mut rows = vec![
        TrackRow { id: 1, label: "Original (Direct)".into(), selected: q == Quality::Direct },
        TrackRow { id: 0, label: quality_label(Quality::Auto, resolved).into(), selected: q == Quality::Auto },
    ];
    for (i, p) in PRESETS.iter().enumerate() {
        rows.push(TrackRow { id: 2 + i as i32, label: p.long.into(), selected: q == Quality::Max(i) });
    }
    rows
}

const SUB_LANGS: [(&str, &str); 15] = [
    ("English", "eng"),
    ("Spanish", "spa"),
    ("French", "fre"),
    ("German", "ger"),
    ("Italian", "ita"),
    ("Portuguese", "por"),
    ("Dutch", "dut"),
    ("Russian", "rus"),
    ("Japanese", "jpn"),
    ("Chinese", "chi"),
    ("Korean", "kor"),
    ("Arabic", "ara"),
    ("Polish", "pol"),
    ("Swedish", "swe"),
    ("Turkish", "tur"),
];

fn posters_dir() -> PathBuf {
    data_dir().join("posters")
}

/// Loads the cached poster files (if any) and hands them to the login backdrop.
fn load_posters(weak: Weak<AppWindow>) {
    std::thread::spawn(move || {
        let Ok(rd) = std::fs::read_dir(posters_dir()) else { return };
        let mut bufs: Vec<SharedPixelBuffer<Rgba8Pixel>> = Vec::new();
        for entry in rd.flatten().take(48) {
            let Ok(bytes) = std::fs::read(entry.path()) else { continue };
            if let Ok(img) = image::load_from_memory(&bytes) {
                let rgba = img.to_rgba8();
                let (w, h) = rgba.dimensions();
                bufs.push(SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(rgba.as_raw(), w, h));
            }
        }
        if bufs.is_empty() {
            return;
        }
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = weak.upgrade() {
                let imgs: Vec<Image> = bufs.into_iter().map(Image::from_rgba8).collect();
                ui.set_posters(ModelRc::new(VecModel::from(imgs)));
            }
        });
    });
}

/// A signed-in server kept for quick switching.
fn login_error(e: &emby_api::Error) -> String {
    match e {
        emby_api::Error::Http(h) if h.status().is_some_and(|s| s.as_u16() == 401 || s.as_u16() == 400) => "Wrong user name or password.".to_string(),
        emby_api::Error::Http(h) if h.is_connect() || h.is_timeout() => "Couldn't reach the server. Check the address.".to_string(),
        emby_api::Error::Other(m) => m.clone(),
        e => format!("Sign-in failed: {e}"),
    }
}

// ---------------------------------------------------------------- view data

#[derive(Clone)]
struct CardData {
    id: String,
    kind: String,
    title: String,
    subtitle: String,
    progress: f32,
    played: bool,
    image: Option<String>,
    current: bool,
    next_season: bool,
    /// Status tag drawn on the card ("Available", "Requested"...) and its colour kind.
    badge: String,
    badge_kind: i32,
    /// Blurb shown when hovering a Seerr card.
    overview: String,
}

const GUIDE_MINUTES: i64 = 240;
const PX_PER_MIN: f32 = 6.0;

struct ProgData {
    id: String,
    title: String,
    sub: String,
    x: f32,
    w: f32,
    now: bool,
}

struct ChanData {
    id: String,
    name: String,
    number: String,
    progs: Vec<ProgData>,
}

fn episode_tag(i: &Item) -> String {
    match (i.parent_index_number, i.index_number) {
        (Some(s), Some(e)) => format!("S{s}:E{e}"),
        (None, Some(e)) => format!("E{e}"),
        _ => String::new(),
    }
}

fn images_dir() -> PathBuf {
    data_dir().join("images")
}

/// Picture bytes from the disk cache, else from the network (and stored). Only pictures are
/// cached, never lists, so content stays fresh; Emby's URLs carry the image tag, so a changed
/// picture is a new URL. shortcut: no size cap, files older than 30 days are pruned at start.
async fn cached_image(client: &Client, url: &str) -> Result<Vec<u8>, emby_api::Error> {
    let key = url.bytes().fold(0xcbf29ce484222325u64, |h, b| (h ^ b as u64).wrapping_mul(0x100000001b3));
    let path = images_dir().join(format!("{key:016x}"));
    if let Ok(b) = std::fs::read(&path) {
        return Ok(b);
    }
    let bytes = client.fetch_bytes(url).await?;
    let _ = std::fs::create_dir_all(images_dir());
    let _ = std::fs::write(&path, &bytes);
    Ok(bytes)
}

fn prune_image_cache() {
    let Ok(rd) = std::fs::read_dir(images_dir()) else { return };
    let cutoff = std::time::SystemTime::now() - Duration::from_secs(30 * 86400);
    for e in rd.flatten() {
        if e.metadata().and_then(|m| m.modified()).is_ok_and(|t| t < cutoff) {
            let _ = std::fs::remove_file(e.path());
        }
    }
}

fn card_for(client: &Client, i: &Item, wide: bool) -> CardData {
    let is_ep = i.kind == "Episode";
    let (title, subtitle) = if is_ep {
        let tag = episode_tag(i);
        let sub = if tag.is_empty() { i.name.clone() } else { format!("{tag} · {}", i.name) };
        (i.series_name.clone().unwrap_or_else(|| i.name.clone()), sub)
    } else {
        (i.name.clone(), i.production_year.map(|y| y.to_string()).unwrap_or_default())
    };
    let image = if wide {
        let order: &[ImageKind] = if is_ep {
            &[ImageKind::Primary, ImageKind::Thumb, ImageKind::Backdrop]
        } else {
            &[ImageKind::Thumb, ImageKind::Backdrop, ImageKind::Primary]
        };
        order.iter().find_map(|k| client.image_url(i, *k, 480))
    } else {
        client.image_url(i, ImageKind::Primary, 300)
    };
    let kind = if i.collection_type.as_deref() == Some("livetv") { "LiveTvView".to_string() } else { i.kind.clone() };
    CardData {
        id: i.id.clone(),
        kind,
        title,
        subtitle,
        progress: i.progress(),
        played: i.user_data.as_ref().is_some_and(|u| u.played),
        image,
        current: false,
                    next_season: false,
                    badge: String::new(),
                    badge_kind: 0,
                    overview: String::new(),
    }
}

fn to_card(c: &CardData) -> Card {
    Card {
        id: c.id.clone().into(),
        kind: c.kind.clone().into(),
        title: c.title.clone().into(),
        subtitle: c.subtitle.clone().into(),
        image: Image::default(),
        progress: c.progress,
        played: c.played,
        current: c.current,
        next_season: c.next_season,
        badge: c.badge.clone().into(),
        badge_kind: c.badge_kind,
        overview: c.overview.clone().into(),
        followed: c.kind == "Series" && notes::is_followed_id(&c.id),
        loading: c.image.is_some(),
    }
}

fn cards_model(cards: &[CardData]) -> ModelRc<Card> {
    ModelRc::new(VecModel::from(cards.iter().map(to_card).collect::<Vec<_>>()))
}

// ---------------------------------------------------------------- images

#[derive(Clone, Copy)]
enum Slot {
    Home(usize, usize),
    Grid(usize),
    Episode(usize),
    Season(usize),
    Backdrop,
    Poster,
    Logo(usize),
    Cast(usize),
    Related(usize),
    DetailLogo,
    SRow(usize, usize),
    SCard(usize),
    SCast(usize),
    SBackdrop,
    SPoster,
    SReq(usize),
    SIssue(usize),
    SIssuePoster,
    SPersonPhoto,
    Session(usize),
}

fn set_card_image(m: &ModelRc<Card>, i: usize, img: Image) {
    if let Some(mut c) = m.row_data(i) {
        c.image = img;
        c.loading = false;
        m.set_row_data(i, c);
    }
}

fn apply_image(ui: &AppWindow, slot: Slot, img: Image) {
    match slot {
        Slot::Home(r, c) => {
            if let Some(row) = ui.get_rows().row_data(r) {
                set_card_image(&row.cards, c, img);
            }
        }
        Slot::Grid(i) => set_card_image(&ui.get_cards(), i, img),
        Slot::Episode(i) => set_card_image(&ui.get_episodes(), i, img),
        Slot::Season(i) => set_card_image(&ui.get_season_cards(), i, img),
        Slot::Cast(i) => set_card_image(&ui.get_detail_cast(), i, img),
        Slot::DetailLogo => {
            let mut d = ui.get_detail();
            d.logo = img;
            ui.set_detail(d);
        }
        Slot::Related(i) => set_card_image(&ui.get_related(), i, img),
        Slot::SRow(r, c) => {
            if let Some(row) = ui.get_sr_rows().row_data(r) {
                set_card_image(&row.cards, c, img);
            }
        }
        Slot::SCard(i) => set_card_image(&ui.get_sr_cards(), i, img),
        Slot::SCast(i) => set_card_image(&ui.get_sr_cast(), i, img),
        Slot::SBackdrop => {
            let mut d = ui.get_sr_detail();
            d.backdrop = img;
            ui.set_sr_detail(d);
        }
        Slot::SPoster => {
            let mut d = ui.get_sr_detail();
            d.poster = img;
            ui.set_sr_detail(d);
        }
        Slot::SReq(i) => {
            let m = ui.get_sr_requests();
            if let Some(mut r) = m.row_data(i) {
                r.poster = img;
                m.set_row_data(i, r);
            }
        }
        Slot::SIssue(i) => {
            let m = ui.get_sr_issues();
            if let Some(mut r) = m.row_data(i) {
                r.poster = img;
                m.set_row_data(i, r);
            }
        }
        Slot::Session(i) => {
            let m = ui.get_sessions();
            if let Some(mut r) = m.row_data(i) {
                r.image = img;
                m.set_row_data(i, r);
            }
        }
        Slot::SPersonPhoto => {
            let mut p = ui.get_sr_person();
            p.photo = img;
            ui.set_sr_person(p);
        }
        Slot::SIssuePoster => {
            let mut d = ui.get_sr_issue();
            d.poster = img;
            ui.set_sr_issue(d);
        }
        Slot::Backdrop => {
            let mut d = ui.get_detail();
            d.backdrop = img;
            ui.set_detail(d);
        }
        Slot::Poster => {
            let mut d = ui.get_detail();
            d.poster = img;
            ui.set_detail(d);
        }
        Slot::Logo(i) => {
            let m = ui.get_channels();
            if let Some(mut c) = m.row_data(i) {
                c.logo = img;
                m.set_row_data(i, c);
            }
        }
    }
}

/// Bumped on every poster enter / leave so a slow banner fetch can't land after the pointer moved on.
static HOVER_GEN: std::sync::atomic::AtomicU64 = std::sync::atomic::AtomicU64::new(0);

// ---------------------------------------------------------------- app context

#[derive(Clone, PartialEq, Debug)]
enum Page {
    Home,
    Library { id: String, title: String },
    Detail { id: String },
    Person { id: String },
    Genre { name: String, kind: String },
    Search(String),
    LiveTv,
    Server,
    Settings,
    SeerrConnect,
    SeerrDiscover,
    SeerrDetail { kind: String, id: i64 },
    SeerrRequests,
    SeerrWatchlist,
    SeerrBrowse { title: String },
    SeerrPerson { id: i64 },
    SeerrIssues,
    SeerrIssue { id: i64 },
    SeerrUsers,
    SeerrUser { id: i64 },
    Notifications,
}

struct State {
    client: Option<Client>,
    user: Option<User>,
    cfg: Config,
    nav: Vec<Page>,
    playing: bool,
    block_back_until: Instant,
    series_id: Option<String>,
    season_ids: Vec<String>,
    hide_target: Option<HideTarget>,
    /// Emby stream indexes behind the detail page's audio dropdown.
    audio_streams: Vec<i32>,
    player_tx: Option<std::sync::mpsc::Sender<player::Control>>,
    remote: Option<tokio::task::JoinHandle<()>>,
    /// Plex sign-in waiting for a server choice: the plex.tv client, the user and the choices.
    plex_pending: Option<(Client, User, Vec<emby_api::PlexServer>)>,
    last_move: Instant,
    last_click: Option<Instant>,
    click_gen: u64,
    /// Window size / position / maximised state to go back to when leaving picture-in-picture.
    pip_saved: Option<(slint::PhysicalSize, slint::PhysicalPosition, bool)>,
    lib: LibView,
    sr: SeerrState,
    /// Log of every notification Foyer has shown, newest first.
    notes: Vec<NoteRec>,
    /// The Mine tab's favourites and which sub-tab (TV / Movies / Other) is showing.
    fav_items: Vec<Item>,
    fav_sub: usize,
    /// Search text on the Mine tab.
    fav_query: String,
}

/// What the library page is currently showing.
#[derive(Default, Clone)]
struct LibView {
    parent: String,
    title: String,
    sort: usize,
    genre: Option<String>,
    letter: Option<String>,
    search: Option<String>,
    loaded: usize,
    total: usize,
    genres: Vec<String>,
}

/// (SortBy, descending), in the same order as the UI's sort menu.
const SORTS: [(&str, bool); 6] = [
    ("SortName", false),
    ("SortName", true),
    ("DateCreated,SortName", true),
    ("PremiereDate,SortName", true),
    ("PremiereDate,SortName", false),
    ("CommunityRating,SortName", true),
];

#[derive(Clone)]
struct Ctx {
    ui: Weak<AppWindow>,
    rt: Handle,
    st: Arc<Mutex<State>>,
    gen: Arc<AtomicU64>,
    detect_gen: Arc<AtomicU64>,
    sem: Arc<Semaphore>,
    surface: player::Surface,
    now: Arc<Mutex<NowPlaying>>,
    bandwidth: Arc<Mutex<Option<(Instant, f64)>>>,
    restart: Arc<Mutex<Option<f64>>>,
}

impl Ctx {
    fn session(&self) -> Option<(Client, String)> {
        let s = self.st.lock().unwrap();
        Some((s.client.clone()?, s.user.as_ref()?.id.clone()))
    }

    /// Starts a new page load; older in-flight loads stop applying their results.
    fn begin(&self) -> u64 {
        if let Some(ui) = self.ui.upgrade() {
            ui.set_busy(true);
        }
        self.gen.fetch_add(1, Ordering::SeqCst) + 1
    }

    fn post(&self, my: u64, f: impl FnOnce(&AppWindow) + Send + 'static) {
        let (ui, gen) = (self.ui.clone(), self.gen.clone());
        let _ = slint::invoke_from_event_loop(move || {
            if gen.load(Ordering::SeqCst) == my {
                if let Some(ui) = ui.upgrade() {
                    f(&ui);
                }
            }
        });
    }

    fn spawn_images(&self, client: &Client, my: u64, jobs: Vec<(Slot, String)>) {
        for (slot, url) in jobs {
            let (ctx, client) = (self.clone(), client.clone());
            self.rt.spawn(async move {
                let Ok(_permit) = ctx.sem.clone().acquire_owned().await else { return };
                if ctx.gen.load(Ordering::SeqCst) != my {
                    return;
                }
                let Ok(bytes) = cached_image(&client, &url).await else {
                    // stop the shimmer; the card falls back to its title
                    ctx.post(my, move |ui| apply_image(ui, slot, Image::default()));
                    return;
                };
                let decoded = tokio::task::spawn_blocking(move || {
                    let rgba = image::load_from_memory(&bytes).ok()?.to_rgba8();
                    let (w, h) = rgba.dimensions();
                    Some(SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(rgba.as_raw(), w, h))
                })
                .await
                .ok()
                .flatten();
                if let Some(buf) = decoded {
                    ctx.post(my, move |ui| apply_image(ui, slot, Image::from_rgba8(buf)));
                }
            });
        }
    }

    fn fail(&self, my: u64, err: emby_api::Error) {
        let msg = match &err {
            emby_api::Error::Http(e) if e.status().is_some_and(|s| s.as_u16() == 401) => {
                "Your session expired. Please sign in again.".to_string()
            }
            e => format!("Couldn't load from the server: {e}"),
        };
        let ctx = self.clone();
        self.post(my, move |ui| {
            ui.set_busy(false);
            ui.set_login_error(msg.into());
            ctx.show_login(ui);
        });
    }

    fn show_login(&self, ui: &AppWindow) {
        let server = self.st.lock().unwrap().cfg.server.clone();
        let kind = self.st.lock().unwrap().cfg.kind;
        ui.set_server_text(server.into());
        ui.set_login_kind(emby_api::Kind::ALL.iter().position(|k| *k == kind).unwrap_or(0) as i32);
        ui.set_detect_note("".into());
        ui.set_login_cancel(false);
        ui.set_plex_picking(false);
        self.sync_servers(ui);
        ui.set_page("login".into());
    }

    /// Pushes the saved-server list and the multi-server switch to the UI.
    fn sync_servers(&self, ui: &AppWindow) {
        let s = self.st.lock().unwrap();
        let label = |x: &SavedServer| {
            let h = x.url.split("://").last().unwrap_or("");
            format!("{} · {} · {}", x.kind.label(), h, x.user_name)
        };
        let names: Vec<SharedString> = s.cfg.servers.iter().map(|x| label(x).into()).collect();
        let active = s.cfg.servers.iter().position(|x| x.url == s.cfg.server && x.user_id == s.cfg.user_id).unwrap_or(0);
        ui.set_multi_server(s.cfg.multi_server);
        ui.set_server_list(ModelRc::new(VecModel::from(names)));
        ui.set_active_server(active as i32);
    }

    /// Switches to another saved server using its stored token.
    fn switch_to(&self, i: i32) {
        {
            let mut s = self.st.lock().unwrap();
            let Some(sv) = s.cfg.servers.get(i as usize).cloned() else { return };
            if sv.url == s.cfg.server && sv.user_id == s.cfg.user_id {
                return;
            }
            s.cfg.server = sv.url;
            s.cfg.kind = sv.kind;
            s.cfg.user_id = sv.user_id;
            s.cfg.user_name = sv.user_name;
            s.cfg.token = sv.token;
            save_config(&s.cfg);
            s.user = None;
            s.client = None; // the old session's token stays valid so you can switch back
            s.nav.clear();
            if let Some(h) = s.remote.take() {
                h.abort();
            }
        }
        self.gen.fetch_add(1, Ordering::SeqCst);
        self.try_saved_login();
    }

    fn remove_server(&self, i: i32) {
        let active = {
            let mut s = self.st.lock().unwrap();
            let Some(sv) = s.cfg.servers.get(i as usize).cloned() else { return };
            if sv.url == s.cfg.server && sv.user_id == s.cfg.user_id {
                true
            } else {
                s.cfg.servers.remove(i as usize);
                save_config(&s.cfg);
                false
            }
        };
        if active {
            self.sign_out();
        } else if let Some(ui) = self.ui.upgrade() {
            self.sync_servers(&ui);
        }
    }

    /// Sign-in page for an extra server, keeping the current session.
    fn add_server(&self) {
        self.gen.fetch_add(1, Ordering::SeqCst);
        if let Some(ui) = self.ui.upgrade() {
            self.show_login(&ui);
            ui.set_server_text("".into());
            ui.set_login_cancel(true);
        }
    }

    /// Asks the typed address what it is (after a short pause) and sets the kind dropdown.
    fn detect_server(&self, text: String) {
        let my = self.detect_gen.fetch_add(1, Ordering::SeqCst) + 1;
        let Some(ui) = self.ui.upgrade() else { return };
        if text.trim().len() < 3 {
            ui.set_detect_note("".into());
            return;
        }
        ui.set_detect_note("Detecting…".into());
        let ctx = self.clone();
        self.rt.spawn(async move {
            tokio::time::sleep(Duration::from_millis(700)).await;
            if ctx.detect_gen.load(Ordering::SeqCst) != my {
                return;
            }
            let found = emby_api::detect(&text).await;
            let _ = slint::invoke_from_event_loop(move || {
                let Some(ui) = ctx.ui.upgrade() else { return };
                if ctx.detect_gen.load(Ordering::SeqCst) != my {
                    return;
                }
                match found {
                    Some((base, kind)) => {
                        ui.set_login_kind(emby_api::Kind::ALL.iter().position(|k| *k == kind).unwrap_or(0) as i32);
                        // A bare host gets the port the server actually answered on.
                        if !text.contains("://") && !text.trim().rsplit_once(':').is_some_and(|(_, p)| p.parse::<u16>().is_ok()) {
                            ui.set_server_text(base.into());
                        }
                        ui.set_detect_note(format!("Detected {}", kind.label()).into());
                    }
                    None => ui.set_detect_note("Couldn't identify a server there. Pick the type yourself.".into()),
                }
            });
        });
    }

    // ---- navigation

    fn navigate(&self, page: Page, push: bool) {
        dlog(&format!("navigate {page:?} push={push}"));
        {
            let mut s = self.st.lock().unwrap();
            if push {
                s.nav.push(page.clone());
            }
            if let Some(ui) = self.ui.upgrade() {
                ui.set_can_go_back(s.nav.len() > 1);
            }
        }
        match page {
            Page::Home => self.show_home(),
            Page::Library { id, title } => self.show_children(id, title),
            Page::Detail { id } => self.show_detail(id, None),
            Page::Person { id } => self.show_person(id),
            Page::Genre { name, kind } => self.show_genre(name, kind),
            Page::Search(t) => self.show_search(t),
            Page::LiveTv => self.show_guide(),
            Page::Server => self.load_server(true),
            Page::Settings => self.show_settings(),
            Page::SeerrConnect => self.show_sr_connect(),
            Page::SeerrDiscover => self.show_sr_discover(),
            Page::SeerrDetail { kind, id } => self.show_sr_detail(kind, id),
            Page::SeerrRequests => self.show_sr_requests(),
            Page::SeerrWatchlist => self.show_sr_watchlist(),
            Page::SeerrBrowse { title } => self.show_sr_browse(title),
            Page::SeerrPerson { id } => self.show_sr_person(id),
            Page::SeerrIssues => self.show_sr_issues(),
            Page::SeerrIssue { id } => self.show_sr_issue(id),
            Page::SeerrUsers => self.show_sr_users(),
            Page::SeerrUser { id } => self.show_sr_user(id),
            Page::Notifications => self.show_notifications(),
        }
    }

    fn go_back(&self) {
        let page = {
            let mut s = self.st.lock().unwrap();
            if s.playing || Instant::now() < s.block_back_until || s.nav.len() < 2 {
                return;
            }
            s.nav.pop();
            s.nav.last().cloned()
        };
        if let Some(p) = page {
            self.navigate(p, false);
        }
    }

    fn go_home(&self) {
        self.st.lock().unwrap().nav.clear();
        self.navigate(Page::Home, true);
    }

    fn reload_current(&self) {
        let page = self.st.lock().unwrap().nav.last().cloned();
        if let Some(p) = page {
            self.navigate(p, false);
        }
    }

    fn open_item(&self, id: String, kind: String) {
        if let Some(rest) = kind.strip_prefix("seerr:") {
            let mut parts = rest.split(':');
            let k = parts.next().unwrap_or("movie").to_string();
            let available = parts.next() == Some("1");
            self.sr_open_target(k, id, available);
            return;
        }
        if kind == "Person" {
            // Prefer the Seerr cast page (TMDB credits, requestable); fall back to the library's own.
            let Some((client, uid)) = self.session() else { return };
            let ctx = self.clone();
            self.rt.spawn(async move {
                let tmdb = client.item(&uid, &id).await.ok().and_then(|p| p.provider_ids.iter().find(|(k, _)| k.eq_ignore_ascii_case("tmdb")).and_then(|(_, v)| v.parse::<i64>().ok()));
                let linked = ctx.st.lock().unwrap().sr.client.is_some();
                let _ = slint::invoke_from_event_loop(move || match tmdb.filter(|_| linked) {
                    Some(id) => ctx.navigate(Page::SeerrPerson { id }, true),
                    None => ctx.navigate(Page::Person { id }, true),
                });
            });
            return;
        }
        if kind == "person" {
            return;
        }
        if kind == "TvChannel" {
            self.play_live(id, String::new());
            return;
        }
        let page = if kind == "LiveTvView" {
            Page::LiveTv
        } else if kind == "CollectionFolder" || kind == "UserView" {
            Page::Library { id, title: String::new() }
        } else {
            Page::Detail { id }
        };
        self.navigate(page, true);
    }

    // ---- pages

    fn show_home(&self) {
        let Some((client, uid)) = self.session() else { return };
        let my = self.begin();
        if let Some(ui) = self.ui.upgrade() {
            ui.set_page("home".into());
            ui.set_rows(ModelRc::new(VecModel::<Row>::default()));
        }
        let ctx = self.clone();
        self.rt.spawn(async move {
            let (resume, next, views) = tokio::join!(client.resume(&uid), client.next_up(&uid), client.views(&uid));
            let views = match views {
                Ok(v) => v,
                Err(e) => return ctx.fail(my, e),
            };
            {
                let libs: Vec<CardData> = views
                    .iter()
                    .map(|v| {
                        let mut c = card_for(&client, v, true);
                        c.subtitle = v.collection_type.clone().unwrap_or_default();
                        c
                    })
                    .collect();
                ctx.post(my, move |ui| ui.set_libraries(cards_model(&libs)));
            }
            let hidden = ctx.st.lock().unwrap().cfg.hidden_rows.clone();
            let shown = |name: &str| !hidden.iter().any(|h| h == name);
            let mut idx = 0usize;
            let mut add_cards = |title: &str, wide: bool, cards: Vec<CardData>, ctx: &Ctx| {
                if cards.is_empty() {
                    return;
                }
                let jobs = cards.iter().enumerate().filter_map(|(c, card)| card.image.clone().map(|u| (Slot::Home(idx, c), u))).collect();
                let title = title.to_string();
                ctx.post(my, move |ui| {
                    ui.set_busy(false);
                    let row = Row { title: title.into(), wide, cards: cards_model(&cards) };
                    if let Some(m) = ui.get_rows().as_any().downcast_ref::<VecModel<Row>>() {
                        m.push(row);
                    }
                });
                ctx.spawn_images(&client, my, jobs);
                idx += 1;
            };
            let cfg = ctx.st.lock().unwrap().cfg.clone();
            let to_cards = |items: &[Item], wide: bool| -> Vec<CardData> { items.iter().filter(|i| !hide::hidden(&cfg, i, false)).map(|i| card_for(&client, i, wide)).collect() };

            let mut resume = resume.unwrap_or_default();
            resume.retain(|i| !hide::hidden(&cfg, i, true));
            let order = home_order(&ctx.st.lock().unwrap().cfg);
            let mut next = Some(next.unwrap_or_default().into_iter().filter(|i| !hide::hidden(&cfg, i, true)).collect::<Vec<_>>());
            for key in order {
                if !shown(&key) {
                    continue;
                }
                match key.as_str() {
                    "media" => add_cards("My Media", true, to_cards(&views, true), &ctx),
                    "continue" => add_cards("Continue Watching", true, to_cards(&resume, true), &ctx),
                    "nextup" => {
                        let n = next_up_items(&client, &uid, &resume, next.take().unwrap_or_default()).await;
                        add_cards("Next Up", true, to_cards(&n, true), &ctx);
                    }
                    "requests" => {
                        let ready = {
                            let s = ctx.st.lock().unwrap();
                            s.cfg.seerr_enabled && s.sr.client.is_some()
                        };
                        if ready {
                            let cards = ctx.sr_home_cards().await;
                            add_cards("My requests", false, cards, &ctx);
                        }
                    }
                    "latest" => {
                        for v in views.iter().take(6) {
                            if matches!(v.collection_type.as_deref(), Some("movies" | "tvshows" | "homevideos" | "boxsets" | "music")) {
                                if let Ok(latest) = ctx.session_client().latest(&uid, &v.id).await {
                                    add_cards(&format!("Recently added to {}", v.name), false, to_cards(&latest, false), &ctx);
                                }
                            }
                        }
                    }
                    "released" => {
                        let today = chrono::Local::now().format("%Y-%m-%d").to_string();
                        if let Ok(list) = client.recently_released(&uid, &today).await {
                            add_cards("Recently released", false, to_cards(&list, false), &ctx);
                        }
                    }
                    _ => {}
                }
            }
            ctx.post(my, |ui| ui.set_busy(false));
        });
    }

    fn session_client(&self) -> Client {
        self.st.lock().unwrap().client.clone().expect("signed in")
    }

    fn show_person(&self, id: String) {
        let Some((client, uid)) = self.session() else { return };
        let my = self.begin();
        if let Some(ui) = self.ui.upgrade() {
            ui.set_cards(ModelRc::new(VecModel::<Card>::default()));
            ui.set_grid_title("".into());
            ui.set_page("grid".into());
        }
        let ctx = self.clone();
        self.rt.spawn(async move {
            let name = client.item(&uid, &id).await.map(|p| p.name).unwrap_or_default();
            match client.person_items(&uid, &id).await {
                Ok(items) => ctx.show_grid(my, &client, name, items, "grid"),
                Err(e) => ctx.fail(my, e),
            }
        });
    }

    fn show_genre(&self, name: String, kind: String) {
        let Some((client, uid)) = self.session() else { return };
        let my = self.begin();
        if let Some(ui) = self.ui.upgrade() {
            ui.set_cards(ModelRc::new(VecModel::<Card>::default()));
            ui.set_grid_title(name.clone().into());
            ui.set_page("grid".into());
        }
        let ctx = self.clone();
        self.rt.spawn(async move {
            match client.by_genre(&uid, &name, &kind).await {
                Ok(items) => ctx.show_grid(my, &client, name, items, "grid"),
                Err(e) => ctx.fail(my, e),
            }
        });
    }

    fn show_grid(&self, my: u64, client: &Client, title: String, items: Vec<Item>, page: &'static str) {
        let items = self.visible(items);
        let cards: Vec<CardData> = items.iter().map(|i| card_for(client, i, false)).collect();
        let jobs = cards.iter().enumerate().filter_map(|(i, c)| c.image.clone().map(|u| (Slot::Grid(i), u))).collect();
        self.post(my, move |ui| {
            ui.set_busy(false);
            ui.set_fav_tabs(ModelRc::new(VecModel::<SharedString>::default()));
            ui.set_lib_controls(false);
            ui.set_lib_can_more(false);
            ui.set_grid_title(title.into());
            ui.set_cards(cards_model(&cards));
            ui.set_page(page.into());
        });
        self.spawn_images(client, my, jobs);
    }

    fn show_guide(&self) {
        let Some((client, uid)) = self.session() else { return };
        let my = self.begin();
        if let Some(ui) = self.ui.upgrade() {
            ui.set_channels(ModelRc::new(VecModel::<Chan>::default()));
            ui.set_page("livetv".into());
        }
        let ctx = self.clone();
        self.rt.spawn(async move {
            let channels = match client.live_channels(&uid, 150).await {
                Ok(c) => c,
                Err(e) => return ctx.fail(my, e),
            };
            let now = chrono::Utc::now().timestamp();
            let from = now - now % 1800;
            let to = from + GUIDE_MINUTES * 60;
            let iso = |t: i64| {
                chrono::DateTime::from_timestamp(t, 0)
                    .unwrap_or_default()
                    .to_rfc3339_opts(chrono::SecondsFormat::Secs, true)
            };
            let ids: Vec<String> = channels.iter().map(|c| c.id.clone()).collect();
            let programs = client.live_programs(&uid, &ids, &iso(from), &iso(to)).await.unwrap_or_default();

            let clock = |t: i64| {
                chrono::DateTime::from_timestamp(t, 0)
                    .map(|d| d.with_timezone(&chrono::Local).format("%-I:%M %p").to_string())
                    .unwrap_or_default()
            };
            let parse = |s: &Option<String>| {
                s.as_deref().and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok()).map(|d| d.timestamp())
            };

            let mut by_channel: std::collections::HashMap<String, Vec<ProgData>> = std::collections::HashMap::new();
            for p in &programs {
                let (Some(cid), Some(st), Some(en)) = (p.channel_id.clone(), parse(&p.start_date), parse(&p.end_date)) else {
                    continue;
                };
                let a = ((st - from).max(0) as f32) / 60.0;
                let b = ((en.min(to) - from) as f32) / 60.0;
                let (x, w) = (a * PX_PER_MIN + 1.0, (b - a) * PX_PER_MIN - 2.0);
                if w < 4.0 {
                    continue;
                }
                by_channel.entry(cid).or_default().push(ProgData {
                    id: p.id.clone(),
                    title: p.name.clone(),
                    sub: p.episode_title.clone().unwrap_or_else(|| format!("{} – {}", clock(st), clock(en))),
                    x,
                    w,
                    now: st <= now && now < en,
                });
            }

            let chans: Vec<ChanData> = channels
                .iter()
                .map(|c| ChanData {
                    id: c.id.clone(),
                    name: c.name.clone(),
                    number: c.number.clone().unwrap_or_default(),
                    progs: by_channel.remove(&c.id).unwrap_or_default(),
                })
                .collect();
            let jobs: Vec<(Slot, String)> = channels
                .iter()
                .enumerate()
                .filter_map(|(i, c)| client.image_url(c, ImageKind::Primary, 120).map(|u| (Slot::Logo(i), u)))
                .collect();
            let marks: Vec<(String, f32)> = (0..GUIDE_MINUTES / 30)
                .map(|i| (clock(from + i * 1800), (i * 30) as f32 * PX_PER_MIN))
                .collect();

            ctx.post(my, move |ui| {
                ui.set_busy(false);
                ui.set_guide_width(GUIDE_MINUTES as f32 * PX_PER_MIN);
                ui.set_time_marks(ModelRc::new(VecModel::from(
                    marks.into_iter().map(|(l, x)| TimeMark { label: l.into(), x }).collect::<Vec<_>>(),
                )));
                let rows: Vec<Chan> = chans
                    .into_iter()
                    .map(|c| Chan {
                        id: c.id.into(),
                        name: c.name.into(),
                        number: c.number.into(),
                        logo: Image::default(),
                        progs: ModelRc::new(VecModel::from(
                            c.progs
                                .into_iter()
                                .map(|p| Prog { id: p.id.into(), title: p.title.into(), sub: p.sub.into(), x: p.x, w: p.w, now: p.now })
                                .collect::<Vec<_>>(),
                        )),
                    })
                    .collect();
                ui.set_channels(ModelRc::new(VecModel::from(rows)));
            });
            ctx.spawn_images(&client, my, jobs);
        });
    }

    fn show_children(&self, id: String, title: String) {
        self.show_server_library(id, title);
    }

    /// Favourites page: everything the user has hearted.
    fn show_favorites(&self) {
        let (id, title) = (String::new(), "Favourites".to_string());
        let Some((client, uid)) = self.session() else { return };
        {
            let mut s = self.st.lock().unwrap();
            s.lib = LibView { parent: id, title: title.clone(), ..Default::default() };
            s.fav_query.clear();
        }
        let my = self.begin();
        if let Some(ui) = self.ui.upgrade() {
            ui.set_fav_query("".into());
            ui.set_cards(ModelRc::new(VecModel::<Card>::default()));
            ui.set_grid_title(title.clone().into());
            ui.set_lib_controls(false);
            ui.set_lib_can_more(false);
            ui.set_page("grid".into());
        }
        let ctx = self.clone();
        self.rt.spawn(async move {
            match client.favorites(&uid, 20000).await {
                Ok(items) => {
                    {
                        // Open on the first sub-tab that has something in it.
                        let mut s = ctx.st.lock().unwrap();
                        s.fav_sub = (0..3).find(|k| items.iter().any(|i| fav_group(i) == *k)).unwrap_or(0);
                        s.fav_items = items;
                    }
                    ctx.favs_view(my, &client);
                }
                Err(e) => ctx.fail(my, e),
            }
        });
    }

    /// Shows the Mine tab's current TV / Movies / Other sub-tab.
    fn favs_view(&self, my: u64, client: &Client) {
        let (items, sub, title, query) = {
            let s = self.st.lock().unwrap();
            (s.fav_items.clone(), s.fav_sub, s.lib.title.clone(), s.fav_query.trim().to_lowercase())
        };
        let mut tabs = vec!["TV", "Movies"];
        if items.iter().any(|i| fav_group(i) == 2) {
            tabs.push("Other");
        }
        let sub = sub.min(tabs.len() - 1);
        // A search looks through everything; otherwise just the picked sub-tab.
        let shown: Vec<Item> = items
            .into_iter()
            .filter(|i| if query.is_empty() { fav_group(i) == sub } else { i.name.to_lowercase().contains(&query) })
            .collect();
        self.show_grid(my, client, title, shown, "grid");
        self.post(my, move |ui| {
            ui.set_fav_tabs(ModelRc::new(VecModel::from(tabs.into_iter().map(SharedString::from).collect::<Vec<_>>())));
            ui.set_fav_tab(sub as i32);
        });
    }

    fn fav_search(&self, q: String) {
        let Some((client, _)) = self.session() else { return };
        self.st.lock().unwrap().fav_query = q;
        let my = self.begin();
        self.favs_view(my, &client);
    }

    fn fav_tab_picked(&self, i: i32) {
        let Some((client, _)) = self.session() else { return };
        self.st.lock().unwrap().fav_sub = i.max(0) as usize;
        let my = self.begin();
        self.favs_view(my, &client);
    }

    fn show_server_library(&self, id: String, title: String) {
        let sort = {
            let mut st = self.st.lock().unwrap();
            let sort = st.cfg.library_sort.min(SORTS.len() - 1);
            st.lib = LibView { parent: id, title: title.clone(), sort, ..Default::default() };
            sort
        };
        if let Some(ui) = self.ui.upgrade() {
            ui.set_cards(ModelRc::new(VecModel::<Card>::default()));
            ui.set_grid_title(title.into());
            ui.set_lib_controls(true);
            ui.set_fav_tabs(ModelRc::new(VecModel::<SharedString>::default()));
            ui.set_lib_can_more(false);
            ui.set_lib_total(0);
            ui.set_lib_sort_index(sort as i32);
            ui.set_lib_genre_index(0);
            ui.set_lib_letter("".into());
            ui.set_lib_query("".into());
            ui.set_lib_genres(ModelRc::new(VecModel::from(vec![SharedString::from("All genres")])));
            ui.set_page("grid".into());
        }
        self.load_library(false);
    }

    /// Loads the first page (after a sort/filter change) or the next one of the current library.
    fn load_library(&self, append: bool) {
        let Some((client, uid)) = self.session() else { return };
        let my = if append { self.gen.load(Ordering::SeqCst) } else { self.begin() };
        let lv = self.st.lock().unwrap().lib.clone();
        if lv.parent.is_empty() {
            return;
        }
        let (sort_by, descending) = SORTS[lv.sort.min(SORTS.len() - 1)];
        let offset = if append { lv.loaded } else { 0 };
        let query = Browse {
            sort_by: sort_by.into(),
            descending,
            genre: lv.genre.clone(),
            letter: lv.letter.clone(),
            search: lv.search.clone(),
            start: offset,
            limit: 120,
        };
        let ctx = self.clone();
        self.rt.spawn(async move {
            let title = if !append && lv.title.is_empty() {
                client.item(&uid, &lv.parent).await.map(|i| i.name).unwrap_or_default()
            } else {
                lv.title.clone()
            };
            let genres = if !append && lv.genres.is_empty() { client.genres(&uid, &lv.parent).await.unwrap_or_default() } else { Vec::new() };
            match client.browse(&uid, &lv.parent, &query).await {
                Ok((items, total)) => {
                    {
                        let mut st = ctx.st.lock().unwrap();
                        if st.lib.parent == lv.parent {
                            st.lib.loaded = offset + items.len();
                            st.lib.total = total;
                            if !append {
                                st.lib.title = title.clone();
                            }
                            if !genres.is_empty() {
                                st.lib.genres = genres.clone();
                            }
                        }
                    }
                    ctx.show_library_page(my, &client, title, items, total, genres, append, offset);
                }
                Err(e) => ctx.fail(my, e),
            }
        });
    }

    #[allow(clippy::too_many_arguments)]
    fn show_library_page(&self, my: u64, client: &Client, title: String, items: Vec<Item>, total: usize, genres: Vec<String>, append: bool, offset: usize) {
        let items = self.visible(items);
        let cards: Vec<CardData> = items.iter().map(|i| card_for(client, i, false)).collect();
        let jobs = cards.iter().enumerate().filter_map(|(i, c)| c.image.clone().map(|u| (Slot::Grid(offset + i), u))).collect();
        let loaded = offset + cards.len();
        self.post(my, move |ui| {
            ui.set_busy(false);
            ui.set_lib_controls(true);
            ui.set_lib_total(total as i32);
            ui.set_lib_can_more(loaded < total);
            if !append {
                ui.set_grid_title(title.into());
                ui.set_cards(cards_model(&cards));
            } else if let Some(m) = ui.get_cards().as_any().downcast_ref::<VecModel<Card>>() {
                for c in &cards {
                    m.push(to_card(c));
                }
            }
            if !genres.is_empty() {
                let mut g = vec![SharedString::from("All genres")];
                g.extend(genres.into_iter().map(SharedString::from));
                ui.set_lib_genres(ModelRc::new(VecModel::from(g)));
            }
            ui.set_page("grid".into());
        });
        self.spawn_images(client, my, jobs);
    }

    /// Watched / favourite toggle on the detail page: updates at once, reverts if the server refuses.
    fn toggle_detail_flag(&self, favorite: bool) {
        let Some((client, uid)) = self.session() else { return };
        let Some(ui) = self.ui.upgrade() else { return };
        let mut d = ui.get_detail();
        let id = d.id.to_string();
        let now = if favorite { !d.favorite } else { !d.played };
        if favorite {
            d.favorite = now;
        } else {
            d.played = now;
            if now {
                d.can_resume = false;
                d.progress = 0.0;
                d.remaining = "".into();
            }
        }
        ui.set_detail(d);
        let weak = self.ui.clone();
        self.rt.spawn(async move {
            let res = if favorite { client.set_favorite(&uid, &id, now).await } else { client.set_played(&uid, &id, now).await };
            if let Err(e) = res {
                applog(&format!("toggle {} failed: {e}", if favorite { "favourite" } else { "watched" }));
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(ui) = weak.upgrade() {
                        let mut d = ui.get_detail();
                        if d.id.as_str() == id {
                            if favorite {
                                d.favorite = !now;
                            } else {
                                d.played = !now;
                            }
                            ui.set_detail(d);
                        }
                    }
                });
            }
        });
    }

    fn lib_set(&self, f: impl FnOnce(&mut LibView)) {
        {
            let mut st = self.st.lock().unwrap();
            f(&mut st.lib);
            st.lib.loaded = 0;
        }
        self.load_library(false);
    }

    fn set_history_ui(&self, ui: &AppWindow) {
        let h = self.st.lock().unwrap().cfg.search_history.clone();
        ui.set_history(ModelRc::new(VecModel::from(h.into_iter().map(SharedString::from).collect::<Vec<_>>())));
    }

    fn add_history(&self, term: &str) {
        let mut s = self.st.lock().unwrap();
        let h = &mut s.cfg.search_history;
        h.retain(|x| !x.eq_ignore_ascii_case(term));
        h.insert(0, term.to_string());
        h.truncate(12);
        save_config(&s.cfg);
    }

    fn clear_history(&self) {
        {
            let mut s = self.st.lock().unwrap();
            s.cfg.search_history.clear();
            save_config(&s.cfg);
        }
        if let Some(ui) = self.ui.upgrade() {
            self.set_history_ui(&ui);
        }
    }

    /// Search from the search page replaces the current entry instead of stacking.
    fn do_search(&self, term: String) {
        {
            let mut s = self.st.lock().unwrap();
            if matches!(s.nav.last(), Some(Page::Search(_))) {
                s.nav.pop();
            }
        }
        self.navigate(Page::Search(term), true);
    }

    fn show_search(&self, term: String) {
        let Some((client, uid)) = self.session() else { return };
        let my = self.begin();
        if let Some(ui) = self.ui.upgrade() {
            ui.set_cards(ModelRc::new(VecModel::<Card>::default()));
            ui.set_search_text(term.clone().into());
            ui.set_page("search".into());
        }
        if term.is_empty() {
            if let Some(ui) = self.ui.upgrade() {
                ui.set_search_active(false);
                ui.set_busy(false);
                self.set_history_ui(&ui);
            }
            return;
        }
        self.add_history(&term);
        if let Some(ui) = self.ui.upgrade() {
            ui.set_search_active(true);
            ui.set_grid_title(format!("Results for “{term}”").into());
            self.set_history_ui(&ui);
        }
        let ctx = self.clone();
        self.rt.spawn(async move {
            match client.search(&uid, &term).await {
                Ok(items) => ctx.show_grid(my, &client, format!("Results for “{term}”"), items, "search"),
                Err(e) => ctx.fail(my, e),
            }
        });
    }

    // ---- settings

    fn sync_settings(&self, ui: &AppWindow) {
        self.sync_hidden(ui);
        self.sync_servers(ui);
        let s = self.st.lock().unwrap();
        let q = Quality::from_cfg(&s.cfg.quality);
        ui.set_quality_index(match q {
            Quality::Direct => 0,
            Quality::Auto => 1,
            Quality::Max(i) => 2 + i as i32,
        });
        ui.set_autoplay_next(s.cfg.autoplay_next);
        ui.set_auto_skip_intro(s.cfg.auto_skip_intro);
        ui.set_rewind_secs(s.cfg.rewind_secs);
        ui.set_forward_secs(s.cfg.forward_secs);
        ui.set_rewind_index(skip_step_index(s.cfg.rewind_secs));
        ui.set_forward_index(skip_step_index(s.cfg.forward_secs));
        ui.set_hw_decode(s.cfg.hw_decode);
        ui.set_hw_mode(s.cfg.hw_mode as i32);
        ui.set_on_top(s.cfg.always_on_top);
        ui.set_sub_lang_index(s.cfg.sub_lang as i32);
        ui.set_material_index(s.cfg.material as i32);
        ui.set_nav_pinned(s.cfg.nav_pinned);
        ui.set_hover_backdrop(s.cfg.hover_backdrop);
        let shown = |n: &str| !s.cfg.hidden_rows.iter().any(|h| h == n);
        let _ = shown;
        ui.set_home_rows(home_rows_model(&s.cfg));
        ui.set_home_continue(shown("continue"));
        ui.set_home_nextup(shown("nextup"));
        ui.set_home_requests(shown("requests"));
        ui.set_home_media(shown("media"));
        ui.set_home_latest(shown("latest"));
        ui.set_seerr_on(s.cfg.seerr_enabled);
        ui.set_notify_enabled(s.cfg.notify_enabled);
        ui.set_notify_movies(s.cfg.notify_movies);
        ui.set_notify_episodes(s.cfg.notify_episodes);
        ui.set_notify_windows(s.cfg.notify_windows);
        ui.set_notify_issues(s.cfg.notify_issues);
        ui.set_notify_my_issues(s.cfg.notify_my_issues);
        ui.set_update_check_on(s.cfg.update_check);
        ui.set_update_install_on(s.cfg.update_install);
        ui.set_app_version(update::current_version().into());
        ui.set_notify_mode(s.cfg.notify_mode as i32);
        ui.set_notify_interval(s.cfg.notify_interval as i32);
        ui.set_debug_logging(s.cfg.debug_logging);
        ui.set_close_to_tray(s.cfg.close_to_tray);
        ui.set_hide_spoilers(s.cfg.hide_spoilers);
        ui.set_watchlist_auto(s.cfg.watchlist_autoremove);
        ui.set_card_scale_index(s.cfg.card_scale.min(CARD_SCALES.len() - 1) as i32);
        ui.global::<Scale>().set_base(CARD_SCALES[s.cfg.card_scale.min(CARD_SCALES.len() - 1)]);
        ui.global::<Scale>().set_auto(s.cfg.card_auto);
        ui.set_card_auto(s.cfg.card_auto);
        ui.set_sr_url(s.cfg.seerr_url.clone().into());
    }

    /// Applies the configured accent: the Windows one, or the picked colour.
    fn refresh_accent(&self, sync_picker: bool) {
        let (mode, picked) = {
            let s = self.st.lock().unwrap();
            (s.cfg.accent_mode, s.cfg.accent_rgb)
        };
        let rgb = if mode == 0 { platform::system_accent().unwrap_or(picked) } else { picked };
        if let Some(ui) = self.ui.upgrade() {
            ui.set_accent_mode(mode as i32);
            apply_accent(&ui, rgb, sync_picker);
        }
    }

    fn show_settings(&self) {
        self.begin();
        if let Some(ui) = self.ui.upgrade() {
            ui.set_busy(false);
            self.sync_settings(&ui);
            ui.set_page("settings".into());
        }
    }

    fn update_cfg(&self, f: impl FnOnce(&mut Config)) {
        let mut s = self.st.lock().unwrap();
        f(&mut s.cfg);
        save_config(&s.cfg);
        notes::sync_followed(&s.cfg.followed);
    }

    // ---- server administration

    fn load_server(&self, first: bool) {
        let Some((client, _)) = self.session() else { return };
        let my = if first {
            let my = self.begin();
            if let Some(ui) = self.ui.upgrade() {
                ui.set_page("server".into());
            }
            my
        } else {
            self.gen.load(Ordering::SeqCst)
        };
        let ctx = self.clone();
        self.rt.spawn(async move {
            let (info, sessions, tasks, folders, activity) = tokio::join!(
                client.system_info(),
                client.sessions(),
                client.scheduled_tasks(),
                client.virtual_folders(),
                client.activity(15)
            );
            let info = match info {
                Ok(i) => i,
                Err(e) => {
                    if first {
                        ctx.fail(my, e);
                    }
                    return;
                }
            };
            let local = |s: &Option<String>| {
                s.as_deref()
                    .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                    .map(|d| d.with_timezone(&chrono::Local).format("%b %-d, %-I:%M %p").to_string())
                    .unwrap_or_default()
            };

            let detail = [
                Some(format!("{} Server {}", client.kind().label(), info.version)),
                info.operating_system.clone(),
                info.local_address.clone(),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<_>>()
            .join("  ·  ");
            let info_row = ServerInfo {
                name: info.server_name.clone().into(),
                detail: detail.into(),
                update: info.has_update_available,
                restart_pending: info.has_pending_restart,
            };

            // Only people who are actually watching something.
            let playing: Vec<_> = sessions.unwrap_or_default().into_iter().filter(|s| s.user_name.is_some() && s.now_playing_item.is_some()).collect();
            let session_images: Vec<Option<String>> = playing
                .iter()
                .map(|s| s.now_playing_item.as_ref().and_then(|i| client.image_url(i, ImageKind::Backdrop, 400).or_else(|| client.image_url(i, ImageKind::Primary, 400))))
                .collect();
            let session_data: Vec<(String, String, String, bool, f32, bool, String, f32)> = playing
                .into_iter()
                .map(|s| {
                    let who = [s.user_name.clone(), s.client.clone().zip(s.device_name.clone()).map(|(c, d)| format!("{c} on {d}"))]
                        .into_iter()
                        .flatten()
                        .collect::<Vec<_>>()
                        .join(" · ");
                    let i = s.now_playing_item.as_ref().expect("filtered");
                    let now = if i.kind == "Episode" {
                        format!("{} {} {}", i.series_name.clone().unwrap_or_default(), episode_tag(i), i.name)
                    } else {
                        i.name.clone()
                    };
                    let pos = s.play_state.as_ref().and_then(|p| p.position_ticks).unwrap_or(0);
                    let total = i.run_time_ticks.unwrap_or(0);
                    (
                        s.id.clone(),
                        who,
                        now,
                        s.play_state.as_ref().is_some_and(|p| p.is_paused),
                        if total > 0 { (pos as f32 / total as f32).clamp(0.0, 1.0) } else { 0.0 },
                        s.supports_remote_control,
                        format!("{} / {}", clock(pos / TICKS_PER_SEC), clock(total / TICKS_PER_SEC)),
                        (total / TICKS_PER_SEC) as f32,
                    )
                })
                .collect();

            let task_rows: Vec<TaskRow> = tasks
                .unwrap_or_default()
                .into_iter()
                .map(|t| {
                    let running = t.state == "Running" || t.state == "Cancelling";
                    let pct = t.current_progress_percentage.unwrap_or(0.0);
                    TaskRow {
                        id: t.id.clone().into(),
                        name: t.name.clone().into(),
                        status: if running {
                            format!("{} · {} {pct:.0}%", t.category.clone().unwrap_or_default(), t.state)
                        } else {
                            format!("{} · {}", t.category.clone().unwrap_or_default(), t.state)
                        }
                        .into(),
                        progress: (pct / 100.0) as f32,
                        running,
                    }
                })
                .collect();

            let folder_rows: Vec<FolderRow> = folders
                .unwrap_or_default()
                .into_iter()
                .map(|f| FolderRow { name: f.name.into(), paths: f.locations.join("   ·   ").into() })
                .collect();

            let activity_rows: Vec<ActivityRow> = activity
                .unwrap_or_default()
                .into_iter()
                .map(|a| ActivityRow { text: a.name.into(), date: local(&a.date).into() })
                .collect();

            let (ctx2, client2) = (ctx.clone(), client.clone());
            ctx.post(my, move |ui| {
                ui.set_busy(false);
                ui.set_server_info(info_row);
                // Update sessions in place when the set is unchanged, so a message being typed survives
                // refreshes; pictures are only fetched again when the title changes.
                let session_rows: Vec<SessionRow> = session_data
                    .into_iter()
                    .map(|(id, title, now, paused, progress, remote, clock, total_secs)| SessionRow {
                        id: id.into(),
                        title: title.into(),
                        now: now.into(),
                        paused,
                        progress,
                        remote,
                        image: Image::default(),
                        clock: clock.into(),
                        total_secs,
                    })
                    .collect();
                let m = ui.get_sessions();
                let mut fetch = Vec::new();
                let same = m.row_count() == session_rows.len() && session_rows.iter().enumerate().all(|(i, r)| m.row_data(i).is_some_and(|o| o.id == r.id));
                if same {
                    for (i, mut r) in session_rows.into_iter().enumerate() {
                        let old = m.row_data(i).expect("same");
                        if old.now == r.now {
                            r.image = old.image;
                        } else {
                            fetch.push(i);
                        }
                        m.set_row_data(i, r);
                    }
                } else {
                    fetch = (0..session_rows.len()).collect();
                    ui.set_sessions(ModelRc::new(VecModel::from(session_rows)));
                }
                let jobs = fetch.into_iter().filter_map(|i| session_images[i].clone().map(|u| (Slot::Session(i), u))).collect();
                ctx2.spawn_images(&client2, my, jobs);
                ui.set_tasks(ModelRc::new(VecModel::from(task_rows)));
                ui.set_folders(ModelRc::new(VecModel::from(folder_rows)));
                ui.set_activity(ModelRc::new(VecModel::from(activity_rows)));
            });
        });
    }

    /// Runs an admin action in the background, then refreshes the server page.
    fn server_action<F, Fut>(&self, f: F)
    where
        F: FnOnce(Client) -> Fut + Send + 'static,
        Fut: std::future::Future<Output = emby_api::Result<()>> + Send + 'static,
    {
        let Some((client, _)) = self.session() else { return };
        let ctx = self.clone();
        self.rt.spawn(async move {
            let _ = f(client).await;
            tokio::time::sleep(Duration::from_millis(600)).await;
            ctx.load_server(false);
        });
    }

    fn show_detail(&self, id: String, preselect_season: Option<String>) {
        let Some((client, uid)) = self.session() else { return };
        let my = self.begin();
        if let Some(ui) = self.ui.upgrade() {
            ui.set_detail(Detail::default());
            ui.set_seasons(ModelRc::new(VecModel::<SharedString>::default()));
            ui.set_episodes(ModelRc::new(VecModel::<Card>::default()));
            ui.set_season_cards(ModelRc::new(VecModel::<Card>::default()));
            ui.set_season_play(-1);
            ui.set_series_play_id("".into());
            ui.set_detail_genres(ModelRc::new(VecModel::<SharedString>::default()));
            ui.set_detail_cast(ModelRc::new(VecModel::<Card>::default()));
            ui.set_related(ModelRc::new(VecModel::<Card>::default()));
            ui.set_info_rows(ModelRc::new(VecModel::<InfoRow>::default()));
            ui.set_info_open(false);
            ui.set_page("detail".into());
        }
        let ctx = self.clone();
        self.rt.spawn(async move {
            let item = match client.item(&uid, &id).await {
                Ok(i) => i,
                Err(e) => return ctx.fail(my, e),
            };
            let season = preselect_season;
            // Folders other than series (collections, playlists, albums...) show their contents.
            if item.is_folder && item.kind != "Series" && item.kind != "Season" {
                return match client.children(&uid, &item.id, 300).await {
                    Ok(items) => ctx.show_grid(my, &client, item.name.clone(), items, "grid"),
                    Err(e) => ctx.fail(my, e),
                };
            }

            let mut meta: Vec<String> = Vec::new();
            if let Some(y) = item.production_year {
                meta.push(y.to_string());
            }
            if let Some(r) = item.run_time_ticks.filter(|t| *t > 0) {
                meta.push(duration_label(r));
            }
            if let Some(r) = &item.official_rating {
                meta.push(r.clone());
            }
            if let Some(r) = item.community_rating {
                meta.push(format!("★ {r:.1}"));
            }
            if !item.genres.is_empty() {
                meta.push(item.genres.join(", "));
            }
            let resume_ticks = client.saved_position(&uid, &item).await;
            let resume_secs = resume_ticks / TICKS_PER_SEC;
            let played = item.user_data.as_ref().is_some_and(|u| u.played);
            applog(&format!(
                "detail {} ({}): item position={} saved position={} played={} runtime={:?}",
                item.id,
                item.name,
                item.resume_ticks(),
                resume_ticks,
                played,
                item.run_time_ticks
            ));
            // Header facts: rating, date, runtime, when it would end if started now.
            let remaining_ticks = item.run_time_ticks.map(|t| (t - if resume_secs >= 5 { resume_ticks } else { 0 }).max(0));
            let mut meta: Vec<String> = Vec::new();
            if let Some(r) = item.community_rating {
                meta.push(format!("★ {r:.1}"));
            }
            let date = item
                .premiere_date
                .as_deref()
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .map(|d| d.format("%-d %b %Y").to_string());
            if let Some(d) = date {
                meta.push(d);
            } else if let Some(y) = item.production_year {
                meta.push(y.to_string());
            }
            if let Some(rt) = item.run_time_ticks.filter(|t| *t > 0) {
                meta.push(duration_label(rt));
            }
            if let Some(r) = &item.official_rating {
                meta.push(r.clone());
            }
            if item.is_playable() {
                if let Some(rem) = remaining_ticks.filter(|t| *t > 0) {
                    let end = chrono::Local::now() + chrono::Duration::seconds(rem / TICKS_PER_SEC);
                    meta.push(format!("Ends at {}", end.format("%H:%M")));
                }
            }
            if item.kind == "Season" && !item.genres.is_empty() {
                meta.push(item.genres.join(", "));
            }
            let d_progress = match (item.run_time_ticks, resume_ticks) {
                (Some(rt), p) if rt > 0 && p > 0 => (p as f32 / rt as f32).clamp(0.0, 1.0),
                _ => 0.0,
            };
            let d_remaining = if resume_secs >= 5 { remaining_ticks.map(|t| format!("{} remaining", duration_label(t))).unwrap_or_default() } else { String::new() };
            let d_favorite = item.user_data.as_ref().is_some_and(|u| u.is_favorite);
            let (d_summary, info_rows) = media_info(&item);
            let subtitle = if item.kind == "Episode" {
                let tag = episode_tag(&item);
                format!("{} {}", item.series_name.clone().unwrap_or_default(), tag).trim().to_string()
            } else {
                String::new()
            };
            let detail = (
                item.id.clone(),
                item.kind.clone(),
                item.name.clone(),
                subtitle,
                meta.join("  ·  "),
                if item.kind == "Episode" && !item.user_data.as_ref().is_some_and(|u| u.played) && ctx.st.lock().unwrap().cfg.hide_spoilers { String::new() } else { item.overview.clone().unwrap_or_default() },
                item.is_playable(),
                format!("Resume from {}", clock(resume_secs)),
                resume_secs >= 5,
            );
            let mut jobs = Vec::new();
            // Backdrop, else the series' (episodes), else the item's own still.
            if let Some(u) = client.image_url(&item, ImageKind::Backdrop, 1920).or_else(|| client.image_url(&item, ImageKind::Primary, 1920)) {
                jobs.push((Slot::Backdrop, u));
            }
            if let Some(u) = client.image_url(&item, ImageKind::Primary, 400) {
                jobs.push((Slot::Poster, u));
            }
            if let Some(u) = client.image_url(&item, ImageKind::Logo, 600) {
                jobs.push((Slot::DetailLogo, u));
            }

            // Series: seasons + episodes of the season you are most likely on.
            let mut season_names: Vec<String> = Vec::new();
            let mut season_ids: Vec<String> = Vec::new();
            let mut season_index = 0usize;
            let mut episodes: Vec<Item> = Vec::new();
            let mut season_cards: Vec<CardData> = Vec::new();
            if item.kind == "Series" {
                if let Ok(mut seasons) = client.seasons(&uid, &item.id).await {
                    // Specials (season 0) go last and are never the default.
                    seasons.sort_by_key(|s| match s.index_number {
                        Some(0) | None => i32::MAX,
                        Some(n) => n,
                    });
                    let is_special = |s: &Item| s.index_number.unwrap_or(0) == 0;
                    season_index = seasons
                        .iter()
                        .position(|s| Some(&s.id) == season.as_ref())
                        .or_else(|| seasons.iter().position(|s| !is_special(s) && s.user_data.as_ref().and_then(|u| u.unplayed_item_count).unwrap_or(0) > 0))
                        .or_else(|| seasons.iter().position(|s| !is_special(s)))
                        .unwrap_or(0);
                    season_names = seasons.iter().map(|s| s.name.clone()).collect();
                    season_ids = seasons.iter().map(|s| s.id.clone()).collect();
                    // One season needs no folder: its episodes go straight on the page.
                    if seasons.len() == 1 {
                        episodes = client.episodes(&uid, &item.id, &seasons[0].id).await.unwrap_or_default();
                    }
                    season_cards = seasons
                        .iter()
                        .filter(|_| seasons.len() > 1)
                        .map(|s| {
                            let mut c = card_for(&client, s, false);
                            // no season poster: use the series'
                            if c.image.is_none() {
                                c.image = client.image_url(&item, ImageKind::Primary, 300);
                            }
                            c.subtitle = match s.user_data.as_ref().and_then(|u| u.unplayed_item_count) {
                                Some(n) if n > 0 => format!("{n} unwatched"),
                                _ => String::new(),
                            };
                            c
                        })
                        .collect();
                }
            } else if item.kind == "Season" {
                if let Some(sid) = &item.series_id {
                    episodes = client.episodes(&uid, sid, &item.id).await.unwrap_or_default();
                }
            }
            let spoil = ctx.st.lock().unwrap().cfg.hide_spoilers;
            let ep_cards: Vec<CardData> = episodes.iter().map(|e| episode_row(&client, e, spoil)).collect();
            // Series Play button: the episode in progress, else the first unwatched, else the first.
            let mut series_play: (String, bool, String) = Default::default();
            if item.kind == "Series" {
                if let Ok(all) = client.series_episodes(&uid, &item.id).await {
                    let main: Vec<&Item> = all.iter().filter(|e| e.parent_index_number != Some(0)).collect();
                    let pool = if main.is_empty() { all.iter().collect() } else { main };
                    let pick = pool
                        .iter()
                        .find(|e| e.resume_ticks() > 0)
                        .or_else(|| pool.iter().find(|e| !e.user_data.as_ref().is_some_and(|u| u.played)))
                        .or(pool.first());
                    if let Some(e) = pick {
                        let resume = e.resume_ticks() > 0;
                        series_play = (e.id.clone(), resume, format!("{} {}", if resume { "Resume" } else { "Play" }, episode_tag(e)).trim().to_string());
                    }
                }
            }
            let d_genres: Vec<SharedString> = if matches!(item.kind.as_str(), "Movie" | "Series") { item.genres.iter().map(|g| g.as_str().into()).collect() } else { Vec::new() };
            // Season Play button: the episode in progress, else the first unwatched, else the first.
            let play_ep = if item.kind == "Season" {
                episodes
                    .iter()
                    .position(|e| e.resume_ticks() > 0)
                    .or_else(|| episodes.iter().position(|e| !e.user_data.as_ref().is_some_and(|u| u.played)))
                    .or(if episodes.is_empty() { None } else { Some(0) })
            } else {
                None
            };
            let play_label = play_ep
                .map(|i| format!("{} {}", if episodes[i].resume_ticks() > 0 { "Resume" } else { "Play" }, episode_tag(&episodes[i])).trim().to_string())
                .unwrap_or_default();
            jobs.extend(season_cards.iter().enumerate().filter_map(|(i, c)| c.image.clone().map(|u| (Slot::Season(i), u))));
            jobs.extend(ep_cards.iter().enumerate().filter_map(|(i, c)| c.image.clone().map(|u| (Slot::Episode(i), u))));

            // Cast & crew
            let cast_cards: Vec<CardData> = item
                .people
                .iter()
                .filter(|p| matches!(p.kind.as_deref(), Some("Actor" | "GuestStar" | "Director")))
                .take(24)
                .map(|p| CardData {
                    id: p.id.clone(),
                    kind: "Person".into(),
                    title: p.name.clone(),
                    subtitle: p.role.clone().filter(|r| !r.is_empty()).or(p.kind.clone()).unwrap_or_default(),
                    progress: 0.0,
                    played: false,
                    image: client.person_image_url(p, 300),
                    current: false,
                    next_season: false,
                    badge: String::new(),
                    badge_kind: 0,
                    overview: String::new(),
                })
                .collect();
            jobs.extend(cast_cards.iter().enumerate().filter_map(|(i, c)| c.image.clone().map(|u| (Slot::Cast(i), u))));

            // The row under the details: the rest of this season (scrolled to this episode and
            // ending with a card for the next season), or similar titles for movies and series.
            let mut related_cards: Vec<CardData> = Vec::new();
            let mut related_index = 0usize;
            let mut related_title = String::new();
            let mut related_wide = true;
            if item.kind == "Episode" {
                related_title = "More from this season".to_string();
                if let (Some(series), Some(season)) = (&item.series_id, &item.season_id) {
                    if let Ok(eps) = client.episodes(&uid, series, season).await {
                        related_index = eps.iter().position(|e| e.id == item.id).unwrap_or(0);
                        related_cards = eps
                            .iter()
                            .map(|e| {
                                let tag = episode_tag(e);
                                CardData {
                                    id: e.id.clone(),
                                    kind: e.kind.clone(),
                                    title: if tag.is_empty() { e.name.clone() } else { format!("{tag}  {}", e.name) },
                                    subtitle: e.run_time_ticks.filter(|t| *t > 0).map(duration_label).unwrap_or_default(),
                                    progress: e.progress(),
                                    played: e.user_data.as_ref().is_some_and(|u| u.played),
                                    image: client.image_url(e, ImageKind::Primary, 480),
                                    current: e.id == item.id,
                                    next_season: false,
                                    badge: String::new(),
                                    badge_kind: 0,
                                    overview: String::new(),
                                }
                            })
                            .collect();
                    }
                    // A closing card that jumps to episode 1 of the next season, if there is one.
                    if let Ok(mut seasons) = client.seasons(&uid, series).await {
                        seasons.sort_by_key(|s| match s.index_number {
                            Some(0) | None => i32::MAX,
                            Some(n) => n,
                        });
                        if let Some(pos) = seasons.iter().position(|s| &s.id == season) {
                            if let Some(next) = seasons.get(pos + 1).filter(|s| s.index_number.unwrap_or(0) != 0) {
                                if let Ok(first) = client.episodes(&uid, series, &next.id).await {
                                    if let Some(f) = first.first() {
                                        related_cards.push(CardData {
                                            id: f.id.clone(),
                                            kind: f.kind.clone(),
                                            title: next.name.clone(),
                                            subtitle: format!("Starts with {} {}", episode_tag(f), f.name),
                                            progress: 0.0,
                                            played: false,
                                            image: None,
                                            current: false,
                                            next_season: true,
                                            badge: String::new(),
                                            badge_kind: 0,
                                            overview: String::new(),
                                        });
                                    }
                                }
                            }
                        }
                    }
                }
            } else if matches!(item.kind.as_str(), "Movie" | "Series") {
                related_title = "More like this".to_string();
                related_wide = false;
                if let Ok(list) = client.similar(&uid, &item.id, 20).await {
                    related_cards = ctx.visible(list).iter().map(|i| card_for(&client, i, false)).collect();
                }
            }
            jobs.extend(related_cards.iter().enumerate().filter_map(|(i, c)| c.image.clone().map(|u| (Slot::Related(i), u))));
            let related_offset = related_index as f32 * 254.0 * CARD_SCALES[ctx.st.lock().unwrap().cfg.card_scale.min(CARD_SCALES.len() - 1)];

            let (audio_labels, audio_default, audio_idx) = {
                let streams = if item.is_playable() { audio_streams(&item) } else { Vec::new() };
                let labels: Vec<SharedString> = streams
                    .iter()
                    .enumerate()
                    .map(|(n, a)| a.display_title.clone().or(a.title.clone()).or(a.language.clone()).unwrap_or_else(|| format!("Track {}", n + 1)).into())
                    .collect();
                (labels, default_audio(&streams), streams.iter().map(|a| a.index).collect::<Vec<_>>())
            };
            *AUDIO_PICK.lock().unwrap() = None;
            {
                let mut s = ctx.st.lock().unwrap();
                s.series_id = (item.kind == "Series").then(|| item.id.clone());
                s.season_ids = season_ids;
                s.hide_target = Some(HideTarget {
                    kind: item.kind.clone(),
                    id: item.id.clone(),
                    name: item.name.clone(),
                    series_id: item.series_id.clone().unwrap_or_default(),
                    series_name: item.series_name.clone().unwrap_or_default(),
                    season_id: item.season_id.clone().unwrap_or_default(),
                    season_name: item.parent_index_number.map(|n| format!("Season {n}")).unwrap_or_default(),
                });
                s.audio_streams = audio_idx;
            }
            let following = item.kind == "Series" && ctx.is_following(&item.id);
            let d_series_id = if item.kind == "Episode" || item.kind == "Season" { item.series_id.clone().unwrap_or_default() } else { String::new() };
            ctx.post(my, move |ui| {
                ui.set_busy(false);
                ui.set_detail(Detail {
                    id: detail.0.into(),
                    kind: detail.1.into(),
                    title: detail.2.into(),
                    subtitle: detail.3.into(),
                    meta: detail.4.into(),
                    overview: detail.5.into(),
                    backdrop: Image::default(),
                    poster: Image::default(),
                    playable: detail.6,
                    resume_label: detail.7.into(),
                    can_resume: detail.8,
                    logo: Image::default(),
                    played,
                    favorite: d_favorite,
                    following: false,
                    progress: d_progress,
                    remaining: d_remaining.into(),
                    summary: d_summary.into(),
                    series_id: d_series_id.into(),
                });
                ui.set_following(following);
                ui.set_audio_options(ModelRc::new(VecModel::from(audio_labels)));
                ui.set_audio_index(audio_default as i32);
                ui.set_info_rows(ModelRc::new(VecModel::from(info_rows)));
                ui.set_seasons(ModelRc::new(VecModel::from(season_names.into_iter().map(SharedString::from).collect::<Vec<_>>())));
                ui.set_season_index(season_index as i32);
                ui.set_episodes(cards_model(&ep_cards));
                ui.set_season_cards(cards_model(&season_cards));
                ui.set_season_play(play_ep.map_or(-1, |i| i as i32));
                ui.set_series_play_id(series_play.0.into());
                ui.set_series_play_resume(series_play.1);
                ui.set_series_play_label(series_play.2.into());
                ui.set_detail_genres(ModelRc::new(VecModel::from(d_genres)));
                ui.set_season_play_label(play_label.into());
                ui.set_detail_cast(cards_model(&cast_cards));
                ui.set_related(cards_model(&related_cards));
                ui.set_related_offset(related_offset);
                ui.set_related_title(related_title.into());
                ui.set_related_wide(related_wide);
            });
            ctx.spawn_images(&client, my, jobs);
        });
    }

    fn select_audio(&self, index: i32) {
        let Some(ui) = self.ui.upgrade() else { return };
        let idx = self.st.lock().unwrap().audio_streams.get(index.max(0) as usize).copied();
        *AUDIO_PICK.lock().unwrap() = idx.map(|i| (ui.get_detail().id.to_string(), i));
    }

    fn select_season(&self, index: i32) {
        let (series, season) = {
            let s = self.st.lock().unwrap();
            (s.series_id.clone(), s.season_ids.get(index.max(0) as usize).cloned())
        };
        let (Some(series), Some(season), Some((client, uid))) = (series, season, self.session()) else { return };
        let my = self.gen.load(Ordering::SeqCst);
        let ctx = self.clone();
        self.rt.spawn(async move {
            let Ok(eps) = client.episodes(&uid, &series, &season).await else { return };
            let spoil = ctx.st.lock().unwrap().cfg.hide_spoilers;
            let cards: Vec<CardData> = eps.iter().map(|e| episode_row(&client, e, spoil)).collect();
            let jobs = cards.iter().enumerate().filter_map(|(i, c)| c.image.clone().map(|u| (Slot::Episode(i), u))).collect();
            ctx.post(my, move |ui| ui.set_episodes(cards_model(&cards)));
            ctx.spawn_images(&client, my, jobs);
        });
    }

    // ---- sign in / out

    fn after_login(&self, ui: &AppWindow, client: Client, user: User, server: &str) {
        ui.set_is_plex(client.kind() == emby_api::Kind::Plex);
        {
            let mut s = self.st.lock().unwrap();
            s.cfg.server = client.base().to_string();
            s.cfg.kind = client.kind();
            s.cfg.user_id = user.id.clone();
            s.cfg.user_name = user.name.clone();
            s.cfg.token = client.token().unwrap_or_default().to_string();
            let entry = SavedServer { kind: s.cfg.kind, url: s.cfg.server.clone(), user_id: user.id.clone(), user_name: user.name.clone(), token: s.cfg.token.clone() };
            match s.cfg.servers.iter_mut().find(|x| x.url == entry.url && x.user_id == entry.user_id) {
                Some(x) => *x = entry,
                None => s.cfg.servers.push(entry),
            }
            save_config(&s.cfg);
            s.client = Some(client);
            s.user = Some(user.clone());
        }
        let _ = server;
        self.start_remote();
        self.refresh_posters();
        ui.set_is_admin(user.is_admin());
        ui.set_user_name(user.name.into());
        ui.set_login_error("".into());
        self.sync_servers(ui);
        self.go_home();
    }

    fn login(&self, server: String, user: String, pass: String, kind: i32) {
        let kind = emby_api::Kind::ALL.get(kind as usize).copied().unwrap_or_default();
        let plex = kind == emby_api::Kind::Plex;
        if (!plex && server.trim().is_empty()) || user.trim().is_empty() {
            if let Some(ui) = self.ui.upgrade() {
                ui.set_login_error(if plex { "Enter your Plex email or user name." } else { "Enter the server address and a user name." }.into());
            }
            return;
        }
        if let Some(ui) = self.ui.upgrade() {
            ui.set_busy(true);
            ui.set_login_error("".into());
        }
        let device = self.st.lock().unwrap().cfg.device_id.clone();
        let ctx = self.clone();
        self.rt.spawn(async move {
            if plex {
                // Plex signs in to plex.tv, then you pick one of the servers your account can use.
                let mut acct = Client::new("https://plex.tv", device).with_kind(kind);
                let res = async {
                    let u = acct.login(user.trim(), &pass).await?;
                    let servers = acct.plex_servers().await?;
                    Ok::<_, emby_api::Error>((u, servers))
                }
                .await;
                let _ = slint::invoke_from_event_loop(move || {
                    let Some(ui) = ctx.ui.upgrade() else { return };
                    ui.set_busy(false);
                    match res {
                        Ok((u, servers)) => ctx.plex_choose(&ui, acct, u, servers, &server),
                        Err(e) => ui.set_login_error(login_error(&e).into()),
                    }
                });
                return;
            }
            let mut client = Client::new(&server, device).with_kind(kind);
            let res = client.login(user.trim(), &pass).await;
            let _ = slint::invoke_from_event_loop(move || {
                let Some(ui) = ctx.ui.upgrade() else { return };
                ui.set_busy(false);
                match res {
                    Ok(u) => ctx.after_login(&ui, client, u, &server),
                    Err(e) => ui.set_login_error(login_error(&e).into()),
                }
            });
        });
    }

    /// After plex.tv sign-in: connect straight away when the typed address matches a server (or
    /// there is only one), otherwise show the picker.
    fn plex_choose(&self, ui: &AppWindow, acct: Client, user: User, servers: Vec<emby_api::PlexServer>, typed: &str) {
        let pre = servers.iter().find(|s| !typed.trim().is_empty() && s.matches(typed)).or(if servers.len() == 1 { servers.first() } else { None }).cloned();
        match pre {
            Some(srv) => self.plex_connect_to(acct, user, srv),
            None if servers.is_empty() => ui.set_login_error("No Plex servers are linked to that account.".into()),
            None => {
                let names: Vec<SharedString> = servers.iter().map(|s| format!("{}{}", s.name, if s.owned { "" } else { "  (shared with you)" }).into()).collect();
                self.st.lock().unwrap().plex_pending = Some((acct, user, servers));
                ui.set_plex_servers(ModelRc::new(VecModel::from(names)));
                ui.set_plex_picking(true);
            }
        }
    }

    fn plex_connect_to(&self, acct: Client, mut user: User, srv: emby_api::PlexServer) {
        if let Some(ui) = self.ui.upgrade() {
            ui.set_busy(true);
            ui.set_login_error("".into());
        }
        let ctx = self.clone();
        self.rt.spawn(async move {
            let res = match acct.plex_connect(&srv).await {
                Ok(c) => {
                    let owner = c.plex_is_owner().await;
                    Ok((c, owner))
                }
                Err(e) => Err(e),
            };
            let _ = slint::invoke_from_event_loop(move || {
                let Some(ui) = ctx.ui.upgrade() else { return };
                ui.set_busy(false);
                match res {
                    Ok((client, owner)) => {
                        user.policy = Some(emby_api::Policy { is_administrator: owner });
                        ui.set_plex_picking(false);
                        ctx.st.lock().unwrap().plex_pending = None;
                        ctx.after_login(&ui, client, user, "");
                    }
                    Err(e) => ui.set_login_error(login_error(&e).into()),
                }
            });
        });
    }

    fn pick_plex_server(&self, i: i32) {
        let pending = self.st.lock().unwrap().plex_pending.clone();
        if let Some((acct, user, servers)) = pending {
            if let Some(srv) = servers.get(i as usize).cloned() {
                self.plex_connect_to(acct, user, srv);
            }
        }
    }

    fn cancel_plex_pick(&self) {
        self.st.lock().unwrap().plex_pending = None;
        if let Some(ui) = self.ui.upgrade() {
            ui.set_plex_picking(false);
            ui.set_login_error("".into());
        }
    }

    fn try_saved_login(&self) {
        let cfg = self.st.lock().unwrap().cfg.clone();
        if let Some(ui) = self.ui.upgrade() {
            ui.set_server_text(cfg.server.clone().into());
        }
        if cfg.token.is_empty() || cfg.user_id.is_empty() || cfg.server.is_empty() {
            return;
        }
        if let Some(ui) = self.ui.upgrade() {
            ui.set_busy(true);
        }
        let ctx = self.clone();
        self.rt.spawn(async move {
            let client = Client::new(&cfg.server, cfg.device_id.clone()).with_kind(cfg.kind).with_token(cfg.token.clone());
            let res = client.validate(&cfg.user_id).await;
            let _ = slint::invoke_from_event_loop(move || {
                let Some(ui) = ctx.ui.upgrade() else { return };
                ui.set_busy(false);
                // A network error keeps you on the login page without discarding the token.
                match res {
                    Ok(mut u) => {
                        if cfg.kind == emby_api::Kind::Plex {
                            u.name = cfg.user_name.clone();
                        }
                        ctx.after_login(&ui, client, u, &cfg.server)
                    }
                    Err(_) => {
                        ctx.show_login(&ui);
                        ui.set_login_error("Couldn't sign in to that server. Check it's online, or sign in again.".into());
                    }
                }
            });
        });
    }

    fn sign_out(&self) {
        let client = {
            let mut s = self.st.lock().unwrap();
            s.cfg.token.clear();
            let (url, uid) = (s.cfg.server.clone(), s.cfg.user_id.clone());
            s.cfg.servers.retain(|x| !(x.url == url && x.user_id == uid));
            save_config(&s.cfg);
            s.user = None;
            s.nav.clear();
            if let Some(h) = s.remote.take() {
                h.abort();
            }
            s.client.take()
        };
        if let Some(c) = client {
            self.rt.spawn(async move { c.logout().await });
        }
        self.gen.fetch_add(1, Ordering::SeqCst);
        if let Some(ui) = self.ui.upgrade() {
            ui.set_busy(false);
            ui.set_is_admin(false);
            self.show_login(&ui);
        }
    }

    /// Registers this device as remotely controllable and listens for dashboard commands.
    /// Caches a fresh random set of film and show posters from the user's own library, for the
    /// login backdrop. At most once every few days.
    fn refresh_posters(&self) {
        let Some((client, uid)) = self.session() else { return };
        let dir = posters_dir();
        let fresh = std::fs::read_dir(&dir)
            .ok()
            .map(|rd| {
                let files: Vec<_> = rd.flatten().collect();
                files.len() >= 24
                    && files
                        .iter()
                        .filter_map(|f| f.metadata().ok()?.modified().ok())
                        .max()
                        .is_some_and(|t| t.elapsed().map(|e| e < Duration::from_secs(3 * 86400)).unwrap_or(false))
            })
            .unwrap_or(false);
        if fresh {
            return;
        }
        let weak = self.ui.clone();
        self.rt.spawn(async move {
            let Ok(items) = client.random_items(&uid, 48).await else { return };
            let mut saved: Vec<(usize, Vec<u8>)> = Vec::new();
            for (i, it) in items.iter().enumerate() {
                if let Some(url) = client.image_url(it, ImageKind::Primary, 300) {
                    if let Ok(bytes) = client.fetch_bytes(&url).await {
                        saved.push((i, bytes));
                    }
                }
            }
            if saved.len() < 8 {
                return;
            }
            let _ = std::fs::remove_dir_all(&dir);
            let _ = std::fs::create_dir_all(&dir);
            for (i, bytes) in saved {
                let _ = std::fs::write(dir.join(format!("p{i:02}.img")), bytes);
            }
            applog("login poster wall refreshed");
            load_posters(weak);
        });
    }

    fn start_remote(&self) {
        let Some((client, _)) = self.session() else { return };
        if client.kind() == emby_api::Kind::Plex {
            return; // Plex has no equivalent of Emby's dashboard remote control
        }
        let ctx = self.clone();
        let handler: Arc<dyn Fn(emby_api::remote::RemoteEvent) + Send + Sync> = Arc::new(move |ev| ctx.handle_remote(ev));
        let handle = self.rt.spawn(emby_api::remote::run(client, handler));
        if let Some(old) = self.st.lock().unwrap().remote.replace(handle) {
            old.abort();
        }
    }

    fn handle_remote(&self, ev: emby_api::remote::RemoteEvent) {
        use emby_api::remote::RemoteEvent as E;
        use player::Control as K;
        let send = |k: K| {
            if let Some(tx) = &self.st.lock().unwrap().player_tx {
                let _ = tx.send(k);
            }
        };
        match ev {
            E::Pause => send(K::SetPause(true)),
            E::Unpause => send(K::SetPause(false)),
            E::PlayPause => send(K::TogglePause),
            E::Stop => send(K::Quit),
            E::SeekTicks(t) => send(K::SeekTo(t as f64 / TICKS_PER_SEC as f64)),
            // read the setting first: `send` locks `st` too, and the guard would still be held
            E::Rewind => {
                let secs = self.st.lock().unwrap().cfg.rewind_secs.max(1);
                send(K::SeekBy(-secs))
            }
            E::FastForward => {
                let secs = self.st.lock().unwrap().cfg.forward_secs.max(1);
                send(K::SeekBy(secs))
            }
            E::SetVolume(v) => send(K::SetVolume(v)),
            E::Mute => send(K::SetMute(true)),
            E::Unmute => send(K::SetMute(false)),
            E::ToggleMute => send(K::ToggleMute),
            E::Next => goto_episode(self, false),
            E::Previous => goto_episode(self, true),
            E::Message { header, text } => {
                if self.st.lock().unwrap().playing {
                    send(K::SetPause(true));
                }
                let ctx = self.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(ui) = ctx.ui.upgrade() {
                        ui.set_message_header(header.into());
                        ui.set_message_text(text.into());
                        ui.set_message_open(true);
                    }
                });
            }
            E::Play { item_id, start_ticks } => {
                let ctx = self.clone();
                let _ = slint::invoke_from_event_loop(move || ctx.play(item_id, start_ticks > 0, false));
            }
        }
    }

    /// Back to the sign-in page (server field editable) without signing out.
    fn switch_server(&self) {
        self.gen.fetch_add(1, Ordering::SeqCst);
        if let Some(ui) = self.ui.upgrade() {
            ui.set_busy(false);
            self.show_login(&ui);
        }
    }

    fn discover(&self) {
        if let Some(ui) = self.ui.upgrade() {
            ui.set_discovering(true);
            ui.set_discover_note("Looking for servers on your network…".into());
            ui.set_discovered(ModelRc::new(VecModel::<SharedString>::default()));
        }
        let ctx = self.clone();
        self.rt.spawn(async move {
            let found = emby_api::discover(Duration::from_secs(3)).await.unwrap_or_default();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = ctx.ui.upgrade() {
                    let n = found.len();
                    let list: Vec<SharedString> = found.into_iter().map(|s| SharedString::from(s.address)).collect();
                    ui.set_discovered(ModelRc::new(VecModel::from(list)));
                    ui.set_discovering(false);
                    ui.set_discover_note(
                        if n == 0 {
                            "No servers found. Servers on another network or behind a proxy don't show up here, so type the address instead.".to_string()
                        } else {
                            format!("Found {n} server{}. Tap one to use it.", if n == 1 { "" } else { "s" })
                        }
                        .into(),
                    );
                }
            });
        });
    }

    // ---- player UI actions

    fn send_control(&self, c: player::Control) {
        dlog(&format!("control {c:?}"));
        if let Some(tx) = &self.st.lock().unwrap().player_tx {
            let _ = tx.send(c);
        }
    }

    /// Poster hover: after a short pause, fetch the title's banner art and fade it in behind the page.
    fn hover_entered(&self, id: String, kind: String) {
        let my = HOVER_GEN.fetch_add(1, Ordering::SeqCst) + 1;
        if !self.st.lock().unwrap().cfg.hover_backdrop
            || !matches!(kind.as_str(), "Movie" | "Series" | "Season" | "Episode" | "BoxSet" | "Video" | "MusicVideo")
        {
            return;
        }
        let Some((client, uid)) = self.session() else { return };
        let ctx = self.clone();
        self.rt.spawn(async move {
            tokio::time::sleep(Duration::from_millis(150)).await;
            if HOVER_GEN.load(Ordering::SeqCst) != my {
                return;
            }
            // Series / movie banner, never the episode's own thumbnail.
            let Ok(item) = client.item(&uid, &id).await else { return };
            let Some(url) = client.image_url(&item, ImageKind::Backdrop, 1280) else { return };
            let Ok(bytes) = client.fetch_bytes(&url).await else { return };
            let decoded = tokio::task::spawn_blocking(move || {
                let rgba = image::load_from_memory(&bytes).ok()?.to_rgba8();
                let (w, h) = rgba.dimensions();
                Some(SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(rgba.as_raw(), w, h))
            })
            .await
            .ok()
            .flatten();
            let Some(buf) = decoded else { return };
            let weak = ctx.ui.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if HOVER_GEN.load(Ordering::SeqCst) != my {
                    return;
                }
                if let Some(ui) = weak.upgrade() {
                    let h = ui.global::<Hover>();
                    h.set_image(Image::from_rgba8(buf));
                    h.set_active(true);
                }
            });
        });
    }

    fn hover_left(&self) {
        HOVER_GEN.fetch_add(1, Ordering::SeqCst);
        if let Some(ui) = self.ui.upgrade() {
            ui.global::<Hover>().set_active(false);
        }
    }

    /// Picture-in-picture: shrink this window to a small borderless always-on-top video in the
    /// screen corner; leaving puts it back as it was.
    fn apply_pip(&self, on: bool) {
        let Some(ui) = self.ui.upgrade() else { return };
        let w = ui.window();
        if on {
            if self.st.lock().unwrap().pip_saved.is_some() {
                return;
            }
            ui.set_mini(false);
            if w.is_fullscreen() {
                w.set_fullscreen(false);
            }
            self.st.lock().unwrap().pip_saved = Some((w.size(), w.position(), w.is_maximized()));
            if w.is_maximized() {
                w.set_maximized(false);
            }
            let sf = w.scale_factor();
            let (pw, ph) = ((480.0 * sf) as u32, (480.0 * sf / ui.get_video_aspect().max(0.2)) as u32);
            platform::set_decorations(w, false);
            let area = platform::monitor_area(w);
            // Let the lowered minimum size take effect before resizing.
            let weak = self.ui.clone();
            slint::Timer::single_shot(Duration::from_millis(60), move || {
                let Some(ui) = weak.upgrade() else { return };
                ui.window().set_size(slint::PhysicalSize::new(pw, ph));
                if let Some((mx, my, mw, mh)) = area {
                    let (margin, bottom) = ((24.0 * sf) as i32, (72.0 * sf) as i32);
                    ui.window().set_position(slint::PhysicalPosition::new(mx + mw as i32 - pw as i32 - margin, my + mh as i32 - ph as i32 - bottom));
                }
            });
        } else {
            let Some((size, pos, max)) = self.st.lock().unwrap().pip_saved.take() else { return };
            platform::set_decorations(w, true);
            w.set_size(size);
            w.set_position(pos);
            if max {
                w.set_maximized(true);
            }
        }
    }

    fn mouse_moved(&self) {
        self.st.lock().unwrap().last_move = Instant::now();
    }

    /// Single click = pause (after a short wait), double click = full screen.
    fn video_click(&self) {
        if let Some(ui) = self.ui.upgrade() {
            ui.set_menu_hold(false);
        }
        let now = Instant::now();
        let (double, gen) = {
            let mut s = self.st.lock().unwrap();
            let double = s.last_click.is_some_and(|t| now.duration_since(t) < Duration::from_millis(350));
            s.click_gen += 1;
            s.last_click = if double { None } else { Some(now) };
            (double, s.click_gen)
        };
        // Pause/resume immediately so a click feels instant; a quick second click undoes
        // that toggle and goes full screen instead.
        let _ = gen;
        self.send_control(player::Control::TogglePause);
        if double {
            if let Some(ui) = self.ui.upgrade() {
                ui.window().set_fullscreen(!ui.window().is_fullscreen());
            }
        }
    }

    fn open_sub_search(&self) {
        self.send_control(player::Control::SetPause(true));
        if let Some(ui) = self.ui.upgrade() {
            ui.set_sub_results(ModelRc::new(VecModel::<SubResult>::default()));
            ui.set_sub_dialog_open(true);
        }
        let idx = self.st.lock().unwrap().cfg.sub_lang as i32;
        if let Some(ui) = self.ui.upgrade() {
            ui.set_sub_lang_index(idx);
        }
        self.sub_search(idx);
    }

    fn close_sub_search(&self) {
        if let Some(ui) = self.ui.upgrade() {
            ui.set_sub_dialog_open(false);
            ui.set_menu_hold(false);
        }
        self.send_control(player::Control::SetPause(false));
    }

    fn sub_search(&self, lang_index: i32) {
        let Some((client, _)) = self.session() else { return };
        let item = self.now.lock().unwrap().item.clone();
        let Some(ui) = self.ui.upgrade() else { return };
        if item.is_empty() {
            ui.set_sub_status("Subtitle search isn't available for live TV.".into());
            return;
        }
        let code = SUB_LANGS.get(lang_index.max(0) as usize).map(|l| l.1).unwrap_or("eng");
        ui.set_sub_busy(true);
        ui.set_sub_status("Searching…".into());
        ui.set_sub_results(ModelRc::new(VecModel::<SubResult>::default()));
        let weak = self.ui.clone();
        self.rt.spawn(async move {
            let res = client.search_subtitles(&item, code).await;
            applog(&format!("subtitle search {code} for {item}: {}", match &res { Ok(l) => format!("{} results", l.len()), Err(e) => e.to_string() }));
            let _ = slint::invoke_from_event_loop(move || {
                let Some(ui) = weak.upgrade() else { return };
                ui.set_sub_busy(false);
                match res {
                    Ok(list) => {
                        ui.set_sub_status(if list.is_empty() { "No subtitles found.".into() } else { format!("{} results. Click one to use it.", list.len()).into() });
                        let rows: Vec<SubResult> = list
                            .into_iter()
                            .map(|r| {
                                let detail = [
                                    r.provider_name.clone(),
                                    r.format.clone(),
                                    r.author.clone(),
                                    r.download_count.map(|d| format!("{d} downloads")),
                                    r.is_hash_match.filter(|m| *m).map(|_| "exact match".to_string()),
                                ]
                                .into_iter()
                                .flatten()
                                .collect::<Vec<_>>()
                                .join("  ·  ");
                                SubResult { id: r.id.into(), title: r.name.into(), detail: detail.into() }
                            })
                            .collect();
                        ui.set_sub_results(ModelRc::new(VecModel::from(rows)));
                    }
                    Err(e) => ui.set_sub_status(format!("Search failed: {e}. Is a subtitle provider set up on the server?").into()),
                }
            });
        });
    }

    fn sub_download(&self, id: String) {
        let Some((client, uid)) = self.session() else { return };
        let now = self.now.clone();
        let item = now.lock().unwrap().item.clone();
        if let Some(ui) = self.ui.upgrade() {
            ui.set_sub_busy(true);
            ui.set_sub_status("Downloading…".into());
        }
        let ctx = self.clone();
        self.rt.spawn(async move {
            let weak = ctx.ui.clone();
            let fail = move |msg: String| {
                let weak = weak.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if let Some(ui) = weak.upgrade() {
                        ui.set_sub_busy(false);
                        ui.set_sub_status(msg.into());
                    }
                });
            };
            if let Err(e) = client.download_subtitle(&item, &id).await {
                applog(&format!("subtitle download failed: {e}"));
                return fail(format!("Download failed: {e}"));
            }
            // The server saves the file asynchronously: poll until the new subtitle shows up.
            let mut added = 0;
            for attempt in 0..10 {
                if attempt > 0 {
                    let weak = ctx.ui.clone();
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(ui) = weak.upgrade() {
                            ui.set_sub_status(format!("Downloaded. Waiting for the server to list it… ({attempt}/10)").into());
                        }
                    });
                    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
                }
                let full = match client.item(&uid, &item).await {
                    Ok(i) => i,
                    Err(e) => return fail(format!("Downloaded, but couldn't reload the item: {e}")),
                };
                let mut n = now.lock().unwrap();
                let source = n.source.clone();
                let src = full.media_sources.iter().find(|m| m.id == source).or(full.media_sources.first());
                if let Some(src) = src {
                    for st in src.media_streams.iter().filter(|m| m.kind == "Subtitle" && m.is_external) {
                        if n.added.insert(st.index) {
                            if let Some(url) = client.subtitle_url(&full.id, &src.id, st) {
                                ctx.send_control(player::Control::AddSubtitle {
                                    url,
                                    title: st.display_title.clone().or(st.title.clone()).unwrap_or_else(|| "Downloaded".into()),
                                    lang: st.language.clone().unwrap_or_default(),
                                });
                                added += 1;
                            }
                        }
                    }
                }
                if added > 0 {
                    break;
                }
            }
            if added == 0 {
                return fail("Downloaded, but the server didn't list a new subtitle yet. Try again in a moment.".into());
            }
            let ctx2 = ctx.clone();
            let _ = slint::invoke_from_event_loop(move || ctx2.close_sub_search());
        });
    }

    // ---- playback

    /// Plays an item, then auto-continues to the next episode only after a *clean* finish.
    /// Errors never advance and never touch the saved position unless real playback happened.
    fn quality(&self) -> Quality {
        Quality::from_cfg(&self.st.lock().unwrap().cfg.quality)
    }

    /// `transcode` is the error dialog's "Try transcode": a moderate fixed quality.
    fn play(&self, id: String, resume: bool, transcode: bool) {
        let quality = if transcode { Quality::Max(3) } else { self.quality() };
        self.start_playback(Source::Item { id, resume, quality, from: None }, String::new());
    }

    /// Changes quality from the player menu; restarts at the current position if playing.
    fn pick_quality(&self, idx: i32) {
        let q = match idx {
            0 => Quality::Auto,
            1 => Quality::Direct,
            n => Quality::Max(((n - 2).max(0) as usize).min(PRESETS.len() - 1)),
        };
        {
            let mut s = self.st.lock().unwrap();
            s.cfg.quality = q.to_cfg();
            save_config(&s.cfg);
        }
        let Some(ui) = self.ui.upgrade() else { return };
        ui.set_menu_hold(false);
        ui.set_quality_label(quality_label(q, "").into());
        ui.set_quality_rows(ModelRc::new(VecModel::from(quality_rows(q, ""))));
        if ui.get_playing() && !ui.get_player_live() {
            *self.restart.lock().unwrap() = Some((ui.get_player_pos() as f64).max(0.0));
            self.send_control(player::Control::Quit);
        }
    }

    fn play_live(&self, channel_id: String, name: String) {
        self.start_playback(Source::Live { channel_id }, name);
    }

    fn start_playback(&self, source: Source, title_hint: String) {
        let Some((client, uid)) = self.session() else { return };
        {
            let mut s = self.st.lock().unwrap();
            if s.playing {
                // Playing in the mini player while browsing: stop it, then start this one.
                let mini = self.ui.upgrade().is_some_and(|ui| ui.get_mini());
                if mini {
                    if let Some(tx) = &s.player_tx {
                        let _ = tx.send(player::Control::Quit);
                    }
                    drop(s);
                    let (c, title) = (self.clone(), title_hint);
                    std::thread::spawn(move || {
                        for _ in 0..100 {
                            std::thread::sleep(Duration::from_millis(50));
                            if !c.st.lock().unwrap().playing {
                                break;
                            }
                        }
                        let _ = slint::invoke_from_event_loop(move || c.start_playback(source, title));
                    });
                }
                return;
            }
            s.playing = true;
        }
        if let Some(ui) = self.ui.upgrade() {
            ui.set_play_error("".into());
            ui.set_player_title(title_hint.into());
            ui.set_player_pos(0.0);
            ui.set_player_dur(0.0);
            ui.set_player_paused(false);
            ui.set_player_started(false);
            ui.set_controls_shown(true);
            ui.set_player_live(matches!(source, Source::Live { .. }));
            ui.set_mini(false);
            ui.set_playing(true);
        }
        let (tx, rx) = std::sync::mpsc::channel();
        self.st.lock().unwrap().player_tx = Some(tx.clone());
        let (autoplay, hw_decode, hw_mode, auto_skip_intro) = {
            let s = self.st.lock().unwrap();
            (s.cfg.autoplay_next, s.cfg.hw_decode, if s.cfg.hw_mode == 0 { auto_hw_mode() } else { s.cfg.hw_mode }, s.cfg.auto_skip_intro)
        };
        let ctx = self.clone();
        std::thread::spawn(move || {
            let env = PlayEnv { controls: rx, surface: ctx.surface.clone(), ui: ctx.ui.clone(), now: ctx.now.clone(), bandwidth: ctx.bandwidth.clone(), restart: ctx.restart.clone(), hw_decode, hw_mode, auto_skip_intro, autoplay, tx };
            let mut source = source;
            let mut error: Option<(String, String)> = None; // (item id, message)
            // Episodes that rolled on by themselves since you last pressed something.
            let mut auto_streak = 0;
            let mut still_watching = false;
            loop {
                let result = play_one(&ctx.rt, &client, &uid, &source, &env);
                ctx.watchlist_prune(source.id());
                match result {
                    PlayResult::Next(next) => {
                        // "Play next" pressed on the prompt works even with autoplay off.
                        let forced = std::mem::take(&mut SKIP.lock().unwrap().force_next);
                        if !autoplay && !forced {
                            break;
                        }
                        if forced {
                            auto_streak = 0;
                        } else {
                            auto_streak += 1;
                            if auto_streak >= AUTOPLAY_STREAK_LIMIT {
                                still_watching = true;
                                break;
                            }
                        }
                        if let Source::Item { quality, .. } = source {
                            source = Source::Item { id: next, resume: false, quality, from: None };
                        }
                    }
                    PlayResult::Restart(pos) => {
                        // Quality changed: reopen the same item at the same position.
                        if let Source::Item { id, .. } = &source {
                            source = Source::Item { id: id.clone(), resume: false, quality: ctx.quality(), from: Some(pos) };
                        }
                        let weak = ctx.ui.clone();
                        let _ = slint::invoke_from_event_loop(move || {
                            if let Some(ui) = weak.upgrade() {
                                ui.set_player_started(false);
                                ui.set_controls_shown(true);
                            }
                        });
                    }
                    PlayResult::Done => break,
                    PlayResult::Failed(m) => {
                        error = Some((source.id().to_string(), m));
                        break;
                    }
                }
            }
            let ctx2 = ctx.clone();
            let _ = slint::invoke_from_event_loop(move || {
                {
                    let mut s = ctx2.st.lock().unwrap();
                    s.playing = false;
                    s.player_tx = None;
                    // The UI is still leaving the player: ignore back input briefly.
                    s.block_back_until = Instant::now() + Duration::from_millis(1500);
                }
                let Some(ui) = ctx2.ui.upgrade() else { return };
                ui.set_mini(false);
                ctx2.apply_pip(false);
                ui.set_pip(false);
                ui.set_playing(false);
                media::close();
                // Leave full screen with the player, otherwise the browse pages have no window controls.
                if ui.window().is_fullscreen() {
                    ui.window().set_fullscreen(false);
                }
                if let Some((id, m)) = error {
                    ui.set_failed_id(id.into());
                    ui.set_play_error(m.into());
                } else {
                    ctx2.reload_current();
                }
                if still_watching {
                    ui.set_message_header("Still watching?".into());
                    ui.set_message_text(
                        format!("{AUTOPLAY_STREAK_LIMIT} episodes have played in a row, so autoplay has stopped. Pick the next episode to keep going.").into(),
                    );
                    ui.set_message_open(true);
                }
            });
        });
    }
}

fn rgb_of(c: slint::Color) -> u32 {
    ((c.red() as u32) << 16) | ((c.green() as u32) << 8) | c.blue() as u32
}

/// Pushes the accent seed into the theme. `sync_picker` also moves the picker controls.
fn apply_accent(ui: &AppWindow, rgb: u32, sync_picker: bool) {
    ui.global::<Palette>().set_seed(slint::Color::from_rgb_u8((rgb >> 16) as u8, (rgb >> 8) as u8, rgb as u8));
    ui.set_accent_hex(format!("#{rgb:06X}").into());
    if sync_picker {
        let (h, s, v) = rgb_to_hsv(rgb);
        if s > 0.01 {
            ui.set_pick_h(h);
        }
        ui.set_pick_s(s);
        ui.set_pick_v(v);
    }
}

/// Window material: 0 solid, 1 Mica, 2 Acrylic (Windows 11 only; solid elsewhere).
fn apply_material(ui: &AppWindow, idx: usize) {
    let idx = if platform::SUPPORTS_MATERIAL { idx } else { 0 };
    platform::set_material(ui.window(), idx);
    ui.global::<Palette>().set_material(idx as i32);
}

/// The "Next Up" row: the episode *after* the one you're partway through for each series, plus
/// the server's own suggestions for series you've finished an episode of.
async fn next_up_items(client: &Client, uid: &str, resume: &[Item], server: Vec<Item>) -> Vec<Item> {
    let mut lookups = Vec::new();
    for r in resume.iter().filter(|r| r.kind == "Episode") {
        if let Some(series) = r.series_id.clone() {
            let (c, u, id, season) = (client.clone(), uid.to_string(), r.id.clone(), r.season_id.clone());
            lookups.push(tokio::spawn(async move { c.episode_after(&u, &series, season.as_deref(), &id).await }));
        }
    }
    let mut out: Vec<Item> = Vec::new();
    for l in lookups {
        if let Ok(Some(item)) = l.await {
            if !out.iter().any(|o| o.id == item.id) {
                out.push(item);
            }
        }
    }
    for n in server {
        let in_progress = resume.iter().any(|r| r.id == n.id || (r.series_id.is_some() && r.series_id == n.series_id));
        let represented = out.iter().any(|o| o.id == n.id || (o.series_id.is_some() && o.series_id == n.series_id));
        if !in_progress && !represented {
            out.push(n);
        }
    }
    out.retain(|i| !resume.iter().any(|r| r.id == i.id));
    out
}

/// A one-line summary (like "1080p HEVC · English EAC3 5.1 · 3 subtitle tracks") and the
/// expandable file/codec details for the detail page.
fn media_info(item: &Item) -> (String, Vec<InfoRow>) {
    let mut rows: Vec<InfoRow> = Vec::new();
    let Some(src) = item.media_sources.first() else { return (String::new(), rows) };
    let head = |t: &str| InfoRow { head: true, label: t.into(), value: "".into() };
    let row = |l: &str, v: String| InfoRow { head: false, label: l.into(), value: v.into() };

    rows.push(head("File"));
    if let Some(p) = &src.path {
        rows.push(row("Path", p.clone()));
    }
    if let Some(c) = &src.container {
        rows.push(row("Container", c.to_uppercase()));
    }
    if let Some(sz) = src.size {
        rows.push(row("Size", fmt_size(sz)));
    }
    if let Some(b) = src.bitrate {
        rows.push(row("Total bitrate", fmt_bps(b)));
    }

    let video: Option<&MediaStream> = src.media_streams.iter().find(|m| m.kind == "Video");
    let audios: Vec<&MediaStream> = src.media_streams.iter().filter(|m| m.kind == "Audio").collect();
    let subs: Vec<&MediaStream> = src.media_streams.iter().filter(|m| m.kind == "Subtitle").collect();

    if let Some(v) = video {
        rows.push(head("Video"));
        let codec = [v.codec.clone().map(|c| c.to_uppercase()), v.profile.clone()].into_iter().flatten().collect::<Vec<_>>().join(" · ");
        if !codec.is_empty() {
            rows.push(row("Codec", codec));
        }
        if let (Some(w), Some(h)) = (v.width, v.height) {
            rows.push(row("Resolution", format!("{w} × {h}")));
        }
        if let Some(f) = v.average_frame_rate.filter(|f| *f > 0.0) {
            rows.push(row("Frame rate", format!("{f:.3} fps")));
        }
        if let Some(b) = v.bit_rate {
            rows.push(row("Bitrate", fmt_bps(b)));
        }
        if let Some(d) = v.bit_depth {
            rows.push(row("Bit depth", format!("{d}-bit")));
        }
        if let Some(r) = v.video_range.clone().filter(|r| !r.is_empty()) {
            rows.push(row("Dynamic range", r));
        }
    }
    for (i, a) in audios.iter().enumerate() {
        rows.push(head(&format!("Audio {}", i + 1)));
        if let Some(t) = a.display_title.clone().or(a.title.clone()) {
            rows.push(row("Track", t));
        }
        if let Some(c) = &a.codec {
            rows.push(row("Codec", c.to_uppercase()));
        }
        if let Some(l) = a.channel_layout.clone().or(a.channels.map(|c| format!("{c} channels"))) {
            rows.push(row("Channels", l));
        }
        if let Some(b) = a.bit_rate {
            rows.push(row("Bitrate", fmt_bps(b)));
        }
        if let Some(r) = a.sample_rate {
            rows.push(row("Sample rate", format!("{:.1} kHz", r as f64 / 1000.0)));
        }
    }
    if !subs.is_empty() {
        rows.push(head("Subtitles"));
        for st in &subs {
            let label = st.display_title.clone().or(st.title.clone()).or(st.language.clone()).unwrap_or_else(|| "Subtitle".into());
            let kind = if st.is_external { "External" } else { "Embedded" };
            rows.push(row(&label, format!("{} · {kind}", st.codec.clone().unwrap_or_default().to_uppercase())));
        }
    }

    let mut parts: Vec<String> = Vec::new();
    if let Some(v) = video {
        if let Some(t) = v.display_title.clone() {
            parts.push(t);
        }
    }
    if let Some(a) = audios.iter().find(|a| a.is_default).or(audios.first()) {
        if let Some(t) = a.display_title.clone().or(a.title.clone()) {
            parts.push(t);
        }
    }
    parts.push(if subs.is_empty() { "No subtitles".into() } else { format!("{} subtitle track{}", subs.len(), if subs.len() == 1 { "" } else { "s" }) });
    (parts.join("  ·  "), rows)
}

fn episode_row(client: &Client, e: &Item, hide_spoilers: bool) -> CardData {
    let played = e.user_data.as_ref().is_some_and(|u| u.played);
    let rt = e.run_time_ticks.filter(|t| *t > 0).map(duration_label).unwrap_or_default();
    let title = match (e.parent_index_number, e.index_number) {
        (Some(s), Some(n)) => format!("S{s:02}E{n:02} - {}", e.name),
        (None, Some(n)) => format!("E{n:02} - {}", e.name),
        _ => e.name.clone(),
    };
    CardData {
        id: e.id.clone(),
        kind: e.kind.clone(),
        title,
        subtitle: rt,
        progress: e.progress(),
        played,
        image: client.image_url(e, ImageKind::Primary, 320),
        current: false,
                    next_season: false,
                    badge: String::new(),
                    badge_kind: 0,
                    overview: if hide_spoilers && !played { String::new() } else { e.overview.clone().unwrap_or_default() },
    }
}

enum Source {
    Item { id: String, resume: bool, quality: Quality, from: Option<f64> },
    Live { channel_id: String },
}

impl Source {
    fn id(&self) -> &str {
        match self {
            Source::Item { id, .. } => id,
            Source::Live { channel_id } => channel_id,
        }
    }
}

struct PlayEnv {
    controls: std::sync::mpsc::Receiver<player::Control>,
    surface: player::Surface,
    ui: Weak<AppWindow>,
    now: Arc<Mutex<NowPlaying>>,
    bandwidth: Arc<Mutex<Option<(Instant, f64)>>>,
    restart: Arc<Mutex<Option<f64>>>,
    hw_decode: bool,
    hw_mode: usize,
    auto_skip_intro: bool,
    autoplay: bool,
    tx: std::sync::mpsc::Sender<player::Control>,
}

impl PlayEnv {
    fn push_quality(&self, q: Quality, resolved: &str) {
        let (weak, resolved) = (self.ui.clone(), resolved.to_string());
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = weak.upgrade() {
                ui.set_quality_label(quality_label(q, &resolved).into());
                ui.set_quality_rows(ModelRc::new(VecModel::from(quality_rows(q, &resolved))));
            }
        });
    }

    /// Link speed, cached for a few minutes. None if it can't be measured.
    fn bandwidth(&self, rt: &Handle, client: &Client) -> Option<f64> {
        if let Some((at, v)) = *self.bandwidth.lock().unwrap() {
            if at.elapsed() < Duration::from_secs(300) {
                return Some(v);
            }
        }
        let v = rt.block_on(client.measure_bandwidth(6_000_000, Duration::from_secs(4)));
        applog(&format!("bandwidth test: {}", v.map_or("failed".to_string(), |b| format!("{:.1} Mbps", b / 1e6))));
        if let Some(v) = v {
            *self.bandwidth.lock().unwrap() = Some((Instant::now(), v));
        }
        v
    }

    fn push_state(&self, t: &player::Tick) {
        let (weak, t, tx) = (self.ui.clone(), t.clone(), self.tx.clone());
        let _ = slint::invoke_from_event_loop(move || {
            let Some(ui) = weak.upgrade() else { return };
            ui.set_player_started(true);
            ui.set_player_pos(t.position as f32);
            ui.set_player_dur(t.duration as f32);
            ui.set_player_paused(t.paused);
            ui.set_player_volume(t.volume as f32);
            ui.set_player_muted(t.muted);
            if t.aspect > 0.2 {
                ui.set_video_aspect(t.aspect as f32);
            }
            ui.set_player_audio(t.audio.into());
            ui.set_player_sub(t.subtitle.into());
            let delay_changed = (ui.get_player_audio_delay() - t.audio_delay as f32).abs() > 0.001;
            ui.set_player_audio_delay(t.audio_delay as f32);
            ui.set_player_sub_delay(t.sub_delay as f32);
            let delay_rows = |d: f64| {
                vec![
                    TrackRow { id: -10, label: "Audio later  (+50 ms)".into(), selected: false },
                    TrackRow { id: -11, label: "Audio earlier  (−50 ms)".into(), selected: false },
                    TrackRow { id: -12, label: format!("Reset audio delay  (now {d:+.2} s)").into(), selected: false },
                ]
            };
            if t.tracks.is_none() && delay_changed {
                let mut rows: Vec<TrackRow> = ui.get_audio_tracks().iter().filter(|r| r.id >= 0).collect();
                rows.extend(delay_rows(t.audio_delay));
                ui.set_audio_tracks(ModelRc::new(VecModel::from(rows)));
            }
            if let Some(tracks) = &t.tracks {
                let rows = |kind: &str| -> Vec<TrackRow> {
                    tracks
                        .iter()
                        .filter(|x| x.kind == kind)
                        .map(|x| TrackRow { id: x.id as i32, label: x.label.clone().into(), selected: x.selected })
                        .collect()
                };
                let mut audio = rows("audio");
                audio.extend(delay_rows(t.audio_delay));
                ui.set_audio_tracks(ModelRc::new(VecModel::from(audio)));
                let mut subs = rows("sub");
                let none_selected = !subs.iter().any(|r| r.selected);
                subs.insert(0, TrackRow { id: -1, label: "Off".into(), selected: none_selected });
                ui.set_sub_tracks(ModelRc::new(VecModel::from(subs)));
            }
            media::sync(&ui, &tx);
        });
    }
}

enum Rep {
    Start(i64),
    Progress(i64, bool),
    Stop(i64),
}

/// Reports go through one background task, in order, so the player loop never waits on the
/// (possibly remote) server and remote-control commands stay instant.
fn spawn_reporter(
    rt: &Handle,
    client: Client,
    item: String,
    source: String,
    session: String,
    method: &'static str,
) -> (tokio::sync::mpsc::UnboundedSender<Rep>, tokio::task::JoinHandle<()>) {
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Rep>();
    let h = rt.spawn(async move {
        while let Some(m) = rx.recv().await {
            let _ = tokio::time::timeout(Duration::from_secs(8), async {
                let (label, res) = match m {
                    Rep::Start(t) => (format!("start pos={t}"), client.report_start(&item, &source, &session, t, method).await),
                    Rep::Progress(t, p) => (format!("progress pos={t} paused={p}"), client.report_progress(&item, &source, &session, t, p, method).await),
                    Rep::Stop(t) => (format!("stop pos={t}"), client.report_stopped(&item, &source, &session, t, method).await),
                };
                let outcome = match &res {
                    Ok(_) => "ok".to_string(),
                    Err(e) => e.to_string(),
                };
                applog(&format!("report {label} item={item}: {outcome}"));
            })
            .await;
        }
    });
    (tx, h)
}

/// What the "Skip intro" / "Play next" prompts need, shared between the player thread and the UI.
struct SkipState {
    intro: Option<(f64, f64)>,
    duration: f64,
    /// 0 nothing shown, 1 skip intro, 2 play next / keep watching.
    kind: i32,
    credits_dismissed: bool,
    force_next: bool,
    /// Neighbouring episodes in the series (across seasons), for the previous / next buttons.
    prev_ep: Option<String>,
    next_ep: Option<String>,
    /// Set by the previous / next buttons: the episode to switch to once mpv has stopped.
    goto: Option<String>,
}

/// Previous / next episode button: stop this one and let the play loop open the other.
fn goto_episode(c: &Ctx, prev: bool) {
    let tx = c.st.lock().unwrap().player_tx.clone();
    if let Some(tx) = tx {
        goto_with(&tx, prev);
    }
}

fn goto_with(tx: &std::sync::mpsc::Sender<player::Control>, prev: bool) {
    let target = {
        let mut k = SKIP.lock().unwrap();
        let t = if prev { k.prev_ep.clone() } else { k.next_ep.clone() };
        if t.is_some() {
            k.goto = t.clone();
            k.force_next = true;
        }
        t
    };
    if target.is_some() {
        let _ = tx.send(player::Control::Quit);
    }
}

/// The audio track picked on the detail page: (item id, Emby stream index). Anything else
/// plays English when the file has it.
static AUDIO_PICK: Mutex<Option<(String, i32)>> = Mutex::new(None);

fn audio_streams(item: &Item) -> Vec<&MediaStream> {
    item.media_sources.first().map(|s| s.media_streams.iter().filter(|m| m.kind == "Audio").collect()).unwrap_or_default()
}

fn is_english(s: &MediaStream) -> bool {
    s.language.as_deref().is_some_and(|l| matches!(l.to_ascii_lowercase().as_str(), "eng" | "en" | "english"))
}

/// Position of the English track (else the file's default, else the first).
fn default_audio(streams: &[&MediaStream]) -> usize {
    streams.iter().position(|s| is_english(s)).or_else(|| streams.iter().position(|s| s.is_default)).unwrap_or(0)
}

static SKIP: Mutex<SkipState> = Mutex::new(SkipState { intro: None, duration: 0.0, kind: 0, credits_dismissed: false, force_next: false, prev_ep: None, next_ep: None, goto: None });

enum PlayResult {
    Done,
    Next(String),
    Restart(f64),
    Failed(String),
}

const MIN_REAL_PLAY_SECS: f64 = 5.0;

/// Stops the monitor sleeping while a video plays. The flag belongs to the calling (player) thread.
#[cfg(windows)]
fn keep_display_on(on: bool) {
    use windows_sys::Win32::System::Power::{SetThreadExecutionState, ES_CONTINUOUS, ES_DISPLAY_REQUIRED, ES_SYSTEM_REQUIRED};
    // SAFETY: plain flag call with no pointers.
    unsafe { SetThreadExecutionState(if on { ES_CONTINUOUS | ES_DISPLAY_REQUIRED | ES_SYSTEM_REQUIRED } else { ES_CONTINUOUS }) };
}

#[cfg(not(windows))]
fn keep_display_on(_: bool) {}

fn ticks(secs: f64) -> i64 {
    (secs * TICKS_PER_SEC as f64) as i64
}

fn play_one(rt: &Handle, client: &Client, uid: &str, src: &Source, env: &PlayEnv) -> PlayResult {
    // Resolve what to play.
    let session = uuid::Uuid::new_v4().simple().to_string();
    let mut audio_pos: Option<i64> = None;
    let (item_id, source_id, url, title, start, live_stream, item, method) = match src {
        Source::Item { id, resume, quality, from } => {
            let item = match rt.block_on(client.item(uid, id)) {
                Ok(i) => i,
                Err(e) => return PlayResult::Failed(format!("Couldn't load the item: {e}")),
            };
            let source = item.media_sources.first().map(|s| s.id.clone()).unwrap_or_else(|| item.id.clone());
            // Auto = direct play unless a speed test shows the file won't fit through the link.
            let src_bps = item.media_sources.iter().find(|m| m.id == source).or(item.media_sources.first()).and_then(|m| m.bitrate);
            let measured = if *quality == Quality::Auto && src_bps.is_some() { env.bandwidth(rt, client) } else { None };
            let (preset, resolved) = decide(*quality, src_bps, measured);
            // The dropdown's pick for this item, else English, else whatever the file defaults to.
            let streams = audio_streams(&item);
            let pos = match &*AUDIO_PICK.lock().unwrap() {
                Some((pid, idx)) if *pid == item.id => streams.iter().position(|s| s.index == *idx),
                _ => None,
            }
            .unwrap_or_else(|| default_audio(&streams));
            let audio_index = streams.get(pos).map(|s| s.index);
            audio_pos = (!streams.is_empty()).then_some(pos as i64 + 1);
            let url = match client.stream_url(&item.id, &source, &session, preset.map(|i| (PRESETS[i].bps, PRESETS[i].height)), audio_index) {
                Ok(u) => u,
                Err(e) => return PlayResult::Failed(e.to_string()),
            };
            if preset.is_some() {
                audio_pos = None;
            }
            env.push_quality(*quality, &resolved);
            let start = match from {
                Some(p) => *p,
                None if *resume => rt.block_on(client.saved_position(uid, &item)) as f64 / TICKS_PER_SEC as f64,
                None => 0.0,
            };
            applog(&format!(
                "play {} resume={resume} start={start:.1}s quality={quality:?} source={:?} bps measured={:?} -> {}",
                item.id,
                src_bps,
                measured.map(|m| m as i64),
                preset.map_or("direct".to_string(), |i| PRESETS[i].long.to_string())
            ));
            let method: &'static str = if preset.is_some() { "Transcode" } else { "DirectPlay" };
            let title = match item.kind.as_str() {
                "Episode" => format!("{} {} {}", item.series_name.clone().unwrap_or_default(), episode_tag(&item), item.name),
                _ => item.name.clone(),
            };
            (item.id.clone(), source, url, title, start, None, Some(item), method)
        }
        Source::Live { channel_id } => match rt.block_on(client.open_live(uid, channel_id)) {
            Ok(ls) => (channel_id.clone(), ls.source_id.clone(), ls.url.clone(), String::new(), 0.0, ls.live_stream_id.clone(), None, "DirectStream"),
            Err(e) => return PlayResult::Failed(format!("Couldn't open the channel: {e}")),
        },
    };
    let is_live = item.is_none();
    // The episode after this one (for the "Play next" prompt and for autoplay).
    let (prev_ep, next_ep): (Option<String>, Option<String>) = item
        .as_ref()
        .filter(|i| i.kind == "Episode")
        .and_then(|item| {
            let series = item.series_id.as_ref()?;
            // Whole series, so the last episode of a season links on to the next season.
            let eps = rt.block_on(client.series_episodes(uid, series)).ok()?;
            let pos = eps.iter().position(|e| e.id == item.id)?;
            let prev = pos.checked_sub(1).and_then(|p| eps.get(p)).map(|n| n.id.clone());
            Some((prev, eps.get(pos + 1).map(|n| n.id.clone())))
        })
        .unwrap_or((None, None));
    {
        let (weak, p, n) = (env.ui.clone(), prev_ep.is_some(), next_ep.is_some());
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = weak.upgrade() {
                ui.set_player_has_prev(p);
                ui.set_player_has_next(n);
            }
        });
    }
    {
        // Scene markers: the server's chapters (not the intro / credits markers) as ticks on the seek bar.
        let ticks: Vec<f32> = item.as_ref().map(|i| i.chapters.iter().filter(|c| c.marker_type.as_deref().is_none_or(|m| m.eq_ignore_ascii_case("chapter")) && c.start_position_ticks > 0).map(|c| (c.start_position_ticks as f64 / 10_000_000.0) as f32).collect()).unwrap_or_default();
        let weak = env.ui.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = weak.upgrade() {
                ui.set_player_chapters(ModelRc::new(VecModel::from(ticks)));
            }
        });
    }
    let credits_marker = item.as_ref().and_then(|i| i.credits_start());
    {
        let mut k = SKIP.lock().unwrap();
        *k = SkipState { intro: item.as_ref().and_then(|i| i.intro()), duration: 0.0, kind: 0, credits_dismissed: false, force_next: k.force_next, prev_ep: prev_ep.clone(), next_ep: next_ep.clone(), goto: None };
    }
    let mut intro_auto_done = false;
    let mut ext_subs: Vec<player::ExtSub> = Vec::new();
    let mut decoupled = false;
    {
        let mut now = env.now.lock().unwrap();
        *now = NowPlaying::default();
        if let Some(item) = &item {
            now.item = item.id.clone();
            now.source = source_id.clone();
            let src = item.media_sources.iter().find(|m| m.id == source_id).or(item.media_sources.first());
            if let Some(src) = src {
                // Embedded text subtitles are fetched from the server too, so mpv never demuxes them out of the stream.
                for st in src.media_streams.iter().filter(|m| m.kind == "Subtitle" && (m.is_external || m.is_text_subtitle_stream)) {
                    if let Some(url) = client.subtitle_url(&item.id, &src.id, st) {
                        decoupled |= !st.is_external;
                        ext_subs.push(player::ExtSub {
                            select: st.is_default || st.is_forced,
                            url,
                            title: st.display_title.clone().or(st.title.clone()).unwrap_or_else(|| "External".into()),
                            lang: st.language.clone().unwrap_or_default(),
                        });
                        now.added.insert(st.index);
                    }
                }
            }
        }
    }
    let (rtx, rh) = spawn_reporter(rt, client.clone(), item_id.clone(), source_id, session, method);

    // Show the title in the control strip.
    if !title.is_empty() {
        let (weak, t) = (env.ui.clone(), title.clone());
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = weak.upgrade() {
                ui.set_player_title(t.into());
            }
        });
    }

    let mut started = false; // "now playing" has been reported
    let mut real = false; // >= 5 s of genuine playback: only then may the saved position move
    let mut last_pos = start;
    let mut last_paused = false;
    let mut duration = 0.0;
    let mut last_tick = Instant::now();
    let mut last_report = Instant::now();
    let outcome = player::play(
        &url,
        &player::Options { title, start_secs: start, external_subs: ext_subs, decoupled, audio_pos, hw_decode: env.hw_decode, hw_mode: env.hw_mode as u8, keep_open: !env.autoplay && next_ep.is_some(), debug_log: DEBUG.load(Ordering::Relaxed).then(|| logs_dir().join("mpv.log").to_string_lossy().into_owned()) },
        &env.controls,
        &env.surface,
        &mut |t| {
            let dt = last_tick.elapsed().as_secs_f64();
            last_tick = Instant::now();
            let expected = last_pos + if last_paused { 0.0 } else { dt };
            let jumped = started && (t.position - expected).abs() > 3.0;
            let pause_changed = started && t.paused != last_paused;
            last_pos = t.position;
            last_paused = t.paused;
            duration = t.duration;
            if !is_live && t.position - start >= MIN_REAL_PLAY_SECS {
                real = true;
            }
            if let Some(d) = &t.diag {
                applog(&format!("mpv {d}"));
            }
            env.push_state(&t);
            keep_display_on(!t.paused && !t.eof);
            if !is_live {
                let mut k = SKIP.lock().unwrap();
                k.duration = t.duration;
                let pos = t.position;
                let in_intro = k.intro.filter(|(a, b)| pos >= a - 0.5 && pos < b - 1.0);
                if let (Some((_, end)), true) = (in_intro, env.auto_skip_intro && !intro_auto_done) {
                    intro_auto_done = true;
                    let _ = env.tx.send(player::Control::SeekTo(end));
                }
                let credits_at = credits_marker.unwrap_or(if t.duration > 300.0 { t.duration - 40.0 } else { f64::MAX });
                let kind = if in_intro.is_some() && !(env.auto_skip_intro && intro_auto_done) {
                    1
                } else if next_ep.is_some() && (t.eof || (!k.credits_dismissed && t.duration > 0.0 && pos >= credits_at)) {
                    2
                } else {
                    0
                };
                if kind != k.kind {
                    k.kind = kind;
                    let weak = env.ui.clone();
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(ui) = weak.upgrade() {
                            ui.set_skip_kind(kind);
                        }
                    });
                }
            }
            // Position reported to the server: the real one only after genuine playback.
            let reported = if real || is_live { t.position } else { start };
            if !started {
                // Report immediately so the server shows "now playing" with its remote-control
                // buttons right away. Start doesn't move the resume position.
                started = true;
                last_report = Instant::now();
                let _ = rtx.send(Rep::Start(ticks(reported)));
            } else if jumped || pause_changed || last_report.elapsed() >= Duration::from_secs(10) {
                last_report = Instant::now();
                let _ = rtx.send(Rep::Progress(ticks(reported), t.paused));
            }
        },
    );

    keep_display_on(false);
    let stop_pos = if real || is_live { last_pos } else { start };
    let mut next = None;
    match &outcome {
        player::Outcome::Finished => {
            let end = if duration > 0.0 && !is_live { duration } else { last_pos };
            if !started {
                let _ = rtx.send(Rep::Start(0));
            }
            let _ = rtx.send(Rep::Stop(ticks(end)));
            next = next_ep.clone();
        }
        // A failed start (nothing played) writes nothing, so Continue Watching is untouched.
        player::Outcome::Stopped | player::Outcome::Failed(_) => {
            if started {
                let _ = rtx.send(Rep::Stop(ticks(stop_pos)));
            }
        }
    }
    drop(rtx);
    let _ = rt.block_on(async { tokio::time::timeout(Duration::from_secs(8), rh).await });
    if let Some(ls) = live_stream {
        rt.block_on(client.close_live(&ls));
    }

    {
        let mut k = SKIP.lock().unwrap();
        k.kind = 0;
    }
    let weak = env.ui.clone();
    let _ = slint::invoke_from_event_loop(move || {
        if let Some(ui) = weak.upgrade() {
            ui.set_skip_kind(0);
        }
    });
    let restart_pos = env.restart.lock().unwrap().take();
    match outcome {
        player::Outcome::Finished => match next {
            Some(n) => PlayResult::Next(n),
            None => PlayResult::Done,
        },
        player::Outcome::Stopped => match (SKIP.lock().unwrap().goto.take(), restart_pos) {
            (Some(id), _) => PlayResult::Next(id),
            (None, Some(p)) => PlayResult::Restart(p),
            (None, None) => PlayResult::Done,
        },
        player::Outcome::Failed(m) => PlayResult::Failed(m),
    }
}

// ---------------------------------------------------------------- main

/// Starts the app. Shared by the desktop `main` and the Android activity entry point.
pub fn run() -> Result<(), slint::PlatformError> {
    foyer_core::set_data_base(platform::data_base());
    foyer_core::set_log(applog);
    std::panic::set_hook(Box::new(|info| {
        let _ = std::fs::create_dir_all(data_dir());
        let _ = std::fs::write(data_dir().join("crash.log"), format!("{info}\n"));
    }));

    let Some(instance_lock) = notes::single_instance(std::env::args().any(|a| a == "--restarted")) else { return Ok(()) };
    let rt = tokio::runtime::Runtime::new().expect("tokio runtime");
    std::thread::spawn(prune_image_cache);
    let ui = AppWindow::new()?;
    notes::serve_instance(instance_lock, ui.as_weak());

    // Video: mpv draws into this window's OpenGL surface before Slint draws the UI over it.
    let surface = {
        let weak = Mutex::new(ui.as_weak());
        player::Surface::new(move || {
            let w = weak.lock().unwrap().clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = w.upgrade() {
                    ui.window().request_redraw();
                }
            });
        })
    };
    {
        let (s, weak) = (surface.clone(), ui.as_weak());
        let last_tex = std::cell::Cell::new((0u32, 0i32, 0i32));
        if let Err(e) = ui.window().set_rendering_notifier(move |state, api| match state {
            slint::RenderingState::BeforeRendering => {
                if let (slint::GraphicsAPI::NativeOpenGL { get_proc_address }, Some(ui)) = (api, weak.upgrade()) {
                    let size = ui.window().size();
                    // In the in-app mini player the video goes into a texture the UI draws as an image.
                    let mini = ui.get_video_tex().then(|| {
                        let sf = ui.window().scale_factor();
                        ((ui.get_mini_w() * sf) as i32, (ui.get_mini_h() * sf) as i32)
                    });
                    if let Some(tex) = s.render(*get_proc_address, size.width as i32, size.height as i32, mini) {
                        let key = (tex, mini.map_or(0, |m| m.0), mini.map_or(0, |m| m.1));
                        if let (Some(id), true) = (std::num::NonZeroU32::new(tex), last_tex.replace(key) != key) {
                            // SAFETY: the texture belongs to this window's GL context and outlives the image
                            // (it is only freed once the UI stops showing it).
                            let img = unsafe {
                                slint::BorrowedOpenGLTextureBuilder::new_gl_2d_rgba_texture(id, [key.1 as u32, key.2 as u32].into())
                                    .origin(slint::BorrowedOpenGLTextureOrigin::TopLeft)
                                    .build()
                            };
                            ui.set_mini_frame(img);
                        }
                    }
                }
            }
            slint::RenderingState::AfterRendering => s.report_swap(),
            slint::RenderingState::RenderingTeardown => s.teardown(),
            _ => {}
        }) {
            eprintln!("video unavailable: {e}");
        }
    }
    let ctx = Ctx {
        ui: ui.as_weak(),
        rt: rt.handle().clone(),
        st: Arc::new(Mutex::new(State {
            client: None,
            user: None,
            cfg: load_config(),
            nav: Vec::new(),
            playing: false,
            block_back_until: Instant::now(),
            series_id: None,
            season_ids: Vec::new(),
            hide_target: None,
            audio_streams: Vec::new(),
            player_tx: None,
            remote: None,
            plex_pending: None,
            last_move: Instant::now(),
            last_click: None,
            click_gen: 0,
            pip_saved: None,
            lib: LibView::default(),
            sr: SeerrState::default(),
            notes: notes::load_notes(),
            fav_items: Vec::new(),
            fav_sub: 0,
            fav_query: String::new(),
        })),
        gen: Arc::new(AtomicU64::new(0)),
        detect_gen: Arc::new(AtomicU64::new(0)),
        sem: Arc::new(Semaphore::new(6)),
        surface,
        now: Arc::new(Mutex::new(NowPlaying::default())),
        bandwidth: Arc::new(Mutex::new(None)),
        restart: Arc::new(Mutex::new(None)),
    };

    ui.set_rows(ModelRc::new(VecModel::<Row>::default()));

    let c = ctx.clone();
    ui.on_login(move |s, u, p, k| c.login(s.to_string(), u.to_string(), p.to_string(), k));
    let c = ctx.clone();
    ui.on_detect_server(move |t| c.detect_server(t.to_string()));
    let c = ctx.clone();
    ui.on_pick_plex_server(move |i| c.pick_plex_server(i));
    let c = ctx.clone();
    ui.on_cancel_plex_pick(move || c.cancel_plex_pick());
    let c = ctx.clone();
    ui.on_switch_to_server(move |i| c.switch_to(i));
    let c = ctx.clone();
    ui.on_remove_server(move |i| c.remove_server(i));
    let c = ctx.clone();
    ui.on_add_server(move || c.add_server());
    let c = ctx.clone();
    ui.on_set_multi_server(move |v| {
        c.update_cfg(|cfg| cfg.multi_server = v);
        if let Some(ui) = c.ui.upgrade() {
            c.sync_servers(&ui);
        }
    });
    let c = ctx.clone();
    ui.on_discover(move || c.discover());
    let c = ctx.clone();
    ui.on_open_item(move |id, kind| c.open_item(id.to_string(), kind.to_string()));
    let c = ctx.clone();
    ui.on_open_library(move |id, t| c.navigate(Page::Library { id: id.to_string(), title: t.to_string() }, true));
    let c = ctx.clone();
    ui.on_go_back(move || c.go_back());
    let c = ctx.clone();
    ui.on_go_home(move || c.go_home());
    let c = ctx.clone();
    ui.on_search(move |t| {
        let t = t.trim().to_string();
        if !t.is_empty() {
            c.do_search(t);
        }
    });
    let c = ctx.clone();
    ui.on_open_search(move || c.navigate(Page::Search(String::new()), true));
    let c = ctx.clone();
    ui.on_clear_history(move || c.clear_history());
    let c = ctx.clone();
    ui.on_open_server(move || c.navigate(Page::Server, true));
    let c = ctx.clone();
    ui.on_refresh_server(move || c.load_server(false));
    let c = ctx.clone();
    ui.on_session_cmd(move |id, cmd| {
        let (id, cmd) = (id.to_string(), cmd.to_string());
        c.server_action(move |cl| async move { cl.session_playstate(&id, &cmd).await });
    });
    let c = ctx.clone();
    ui.on_session_seek(move |id, secs| {
        let (id, ticks) = (id.to_string(), secs.max(0.0) as i64 * TICKS_PER_SEC);
        c.server_action(move |cl| async move { cl.session_seek(&id, ticks).await });
    });
    let c = ctx.clone();
    ui.on_session_message(move |id, text| {
        let (id, text) = (id.to_string(), text.trim().to_string());
        if !text.is_empty() {
            c.server_action(move |cl| async move { cl.session_message(&id, "Message", &text).await });
        }
    });
    let c = ctx.clone();
    ui.on_task_run(move |id| {
        let id = id.to_string();
        c.server_action(move |cl| async move { cl.start_task(&id).await });
    });
    let c = ctx.clone();
    ui.on_task_stop(move |id| {
        let id = id.to_string();
        c.server_action(move |cl| async move { cl.stop_task(&id).await });
    });
    let c = ctx.clone();
    ui.on_library_scan(move || c.server_action(|cl| async move { cl.library_refresh().await }));
    let c = ctx.clone();
    ui.on_server_power(move |kind| {
        let kind = kind.to_string();
        c.server_action(move |cl| async move { cl.server_power(&kind).await });
    });
    let c = ctx.clone();
    ui.on_play(move |id, resume, transcode| c.play(id.to_string(), resume, transcode));
    let c = ctx.clone();
    ui.on_select_season(move |i| c.select_season(i));
    let c = ctx.clone();
    ui.on_select_audio(move |i| c.select_audio(i));
    let c = ctx.clone();
    ui.on_sign_out(move || c.sign_out());
    let c = ctx.clone();
    ui.on_switch_server(move || c.switch_server());
    let c = ctx.clone();
    let c2 = ctx.clone();
    ui.on_player_key(move |name| {
        use player::Control as K;
        // The prompt buttons: "skip" = skip intro / play next, "keep" = dismiss the credits prompt.
        if matches!(name.as_str(), "skip" | "keep") {
            let mut k = SKIP.lock().unwrap();
            let send = |c: K| {
                if let Some(tx) = &c2.st.lock().unwrap().player_tx {
                    let _ = tx.send(c);
                }
            };
            match (name.as_str(), k.kind) {
                ("skip", 1) => {
                    if let Some((_, end)) = k.intro {
                        send(K::SeekTo(end));
                    }
                }
                ("skip", 2) => {
                    k.force_next = true;
                    send(K::Finish);
                }
                ("keep", 2) => {
                    k.credits_dismissed = true;
                    k.kind = 0;
                    if let Some(ui) = c2.ui.upgrade() {
                        ui.set_skip_kind(0);
                    }
                }
                _ => {}
            }
            return;
        }
        if matches!(name.as_str(), "prev" | "next") {
            goto_episode(&c, name.as_str() == "prev");
            return;
        }
        if name.as_str() == "screenshot" {
            // Pictures\Foyer\Foyer-<unix secs>.png
            let dir = std::path::PathBuf::from(std::env::var("USERPROFILE").unwrap_or_default()).join("Pictures").join("Foyer");
            if std::fs::create_dir_all(&dir).is_ok() {
                let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map_or(0, |d| d.as_secs());
                let path = dir.join(format!("Foyer-{t}.png")).to_string_lossy().into_owned();
                dlog(&format!("screenshot {path}"));
                if let Some(tx) = &c.st.lock().unwrap().player_tx {
                    let _ = tx.send(K::Screenshot(path.clone()));
                }
                // mpv writes the file asynchronously; confirm once it has landed.
                let c = c.clone();
                std::thread::spawn(move || {
                    std::thread::sleep(Duration::from_millis(800));
                    if std::path::Path::new(&path).exists() {
                        c.notify_user("Screenshot saved".into(), path, None);
                    } else {
                        c.notify_user("Screenshot failed".into(), "mpv didn't write the file.".into(), None);
                    }
                });
            }
            return;
        }
        let (rewind, forward) = {
            let s = c.st.lock().unwrap();
            (s.cfg.rewind_secs.max(1), s.cfg.forward_secs.max(1))
        };
        let k = match name.as_str() {
            "quit" => K::Quit,
            "pause" => K::TogglePause,
            "back10" => K::SeekBy(-rewind),
            "fwd30" => K::SeekBy(forward),
            "volup" => K::VolumeBy(5),
            "voldown" => K::VolumeBy(-5),
            "mute" => K::ToggleMute,
            "subs" => K::CycleSubtitles,
            "audio" => K::CycleAudio,
            "audio-later" => K::AudioDelayBy(0.05),
            "audio-earlier" => K::AudioDelayBy(-0.05),
            _ => return,
        };
        if let Some(tx) = &c.st.lock().unwrap().player_tx {
            let _ = tx.send(k);
        }
    });
    let c = ctx.clone();
    ui.on_tune(move |id, name| c.play_live(id.to_string(), name.to_string()));
    let c = ctx.clone();
    ui.on_player_seek(move |secs| {
        if let Some(tx) = &c.st.lock().unwrap().player_tx {
            let _ = tx.send(player::Control::SeekTo(secs as f64));
        }
    });
    let c = ctx.clone();
    ui.on_player_set_volume(move |v| {
        if let Some(tx) = &c.st.lock().unwrap().player_tx {
            let _ = tx.send(player::Control::SetVolume(v.round() as i32));
        }
    });
    let c = ctx.clone();
    ui.on_dismiss_message(move || {
        if let Some(ui) = c.ui.upgrade() {
            ui.set_message_open(false);
        }
        // Resume playback that the message paused.
        let tx = {
            let s = c.st.lock().unwrap();
            if s.playing { s.player_tx.clone() } else { None }
        };
        if let Some(tx) = tx {
            let _ = tx.send(player::Control::SetPause(false));
        }
    });
    let weak = ui.as_weak();
    ui.on_dismiss_play_error(move || {
        if let Some(ui) = weak.upgrade() {
            ui.set_play_error("".into());
        }
    });

    let c = ctx.clone();
    ui.on_pick_quality(move |idx| c.pick_quality(idx));

    // ---- Seerr
    let c = ctx.clone();
    ui.on_open_seerr(move || c.sr_enter());
    let c = ctx.clone();
    ui.on_sr_connect(move |url, kind, login, pass| c.sr_connect(url.to_string(), kind, login.to_string(), pass.to_string()));
    let c = ctx.clone();
    ui.on_sr_disconnect(move || c.sr_disconnect());
    let c = ctx.clone();
    ui.on_sr_tab(move |i| {
        let page = match i {
            0 => Page::SeerrDiscover,
            1 => Page::SeerrRequests,
            2 => Page::SeerrWatchlist,
            3 => Page::SeerrIssues,
            _ => Page::SeerrUsers,
        };
        c.navigate(page, true);
    });
    let c = ctx.clone();
    ui.on_sr_search(move |q| c.sr_search(q.to_string()));
    let c = ctx.clone();
    ui.on_sr_browse(move |title| c.navigate(Page::SeerrBrowse { title: title.to_string() }, true));
    let c = ctx.clone();
    ui.on_sr_browse_load_more(move || c.load_sr_browse(true));
    let c = ctx.clone();
    ui.on_sr_open_person(move |id| {
        if let Ok(id) = id.parse::<i64>() {
            c.navigate(Page::SeerrPerson { id }, true);
        }
    });
    let c = ctx.clone();
    ui.on_sr_person_refilter(move || c.sr_person_refilter());
    let c = ctx.clone();
    ui.on_sr_watchlist_toggle(move || c.sr_watchlist_toggle());
    let c = ctx.clone();
    ui.on_follow_card(move |id, name| c.follow_card(id.to_string(), name.to_string()));
    let c = ctx.clone();
    ui.on_sr_open(move |kind, id| {
        if let Ok(id) = id.parse::<i64>() {
            c.navigate(Page::SeerrDetail { kind: kind.to_string(), id }, true);
        }
    });
    let c = ctx.clone();
    ui.on_sr_open_library(move |kind, id| {
        if let Ok(id) = id.parse::<i64>() {
            c.open_in_library(kind.to_string(), id);
        }
    });
    let c = ctx.clone();
    ui.on_sr_open_target(move |kind, tmdb, available| c.sr_open_target(kind.to_string(), tmdb.to_string(), available));
    let c = ctx.clone();
    ui.on_sr_request_start(move || c.sr_request_start());
    let c = ctx.clone();
    ui.on_sr_request_quick(move |kind, id| {
        if let Ok(id) = id.parse::<i64>() {
            c.sr_request_quick(kind.to_string(), id);
        }
    });
    let c = ctx.clone();
    ui.on_sr_request_submit(move || c.sr_request_submit());
    let c = ctx.clone();
    ui.on_sr_season_toggled(move |i, v| c.sr_season_toggled(i, v));
    let c = ctx.clone();
    ui.on_sr_report_start(move || c.sr_report_start());
    let c = ctx.clone();
    ui.on_sr_report_submit(move || c.sr_report_submit());
    let c = ctx.clone();
    ui.on_lib_report_issue(move || c.report_from_library());
    let c = ctx.clone();
    ui.on_sr_req_filter_changed(move |i| c.sr_req_filter(i));
    let c = ctx.clone();
    ui.on_sr_req_load_more(move || c.load_sr_requests(true));
    let c = ctx.clone();
    ui.on_sr_req_action(move |id, action| c.sr_req_action(id.to_string(), action.to_string()));
    let c = ctx.clone();
    ui.on_sr_issue_filter_changed(move |i| c.sr_issue_filter(i));
    let c = ctx.clone();
    ui.on_sr_issue_load_more(move || c.load_sr_issues(true));
    let c = ctx.clone();
    ui.on_sr_open_issue(move |id| {
        if let Ok(id) = id.parse::<i64>() {
            c.navigate(Page::SeerrIssue { id }, true);
        }
    });
    let c = ctx.clone();
    ui.on_sr_open_user(move |id| {
        if let Ok(id) = id.parse::<i64>() {
            c.navigate(Page::SeerrUser { id }, true);
        }
    });
    let c = ctx.clone();
    ui.on_sr_user_save(move || c.sr_user_save());
    let c = ctx.clone();
    ui.on_sr_req_scope_changed(move |i| c.sr_req_scope(i));
    let c = ctx.clone();
    ui.on_set_notify_issues(move |v| c.update_cfg(|cfg| cfg.notify_issues = v));
    let c = ctx.clone();
    ui.on_set_update_check_on(move |v| c.update_cfg(|cfg| cfg.update_check = v));
    let c = ctx.clone();
    ui.on_set_update_install_on(move |v| c.update_cfg(|cfg| cfg.update_install = v));
    let c = ctx.clone();
    ui.on_check_updates(move || c.update_check());
    let c = ctx.clone();
    ui.on_install_update(move || c.update_install());
    let c = ctx.clone();
    ui.on_restart_app(move || c.update_restart());
    let c = ctx.clone();
    ui.on_set_notify_my_issues(move |v| c.update_cfg(|cfg| cfg.notify_my_issues = v));
    let c = ctx.clone();
    ui.on_sr_issue_comment(move |t| c.sr_issue_comment(t.to_string()));
    let c = ctx.clone();
    ui.on_sr_issue_resolve(move |r| c.sr_issue_resolve(r));
    let c = ctx.clone();
    ui.on_sr_issue_delete(move || c.sr_issue_delete());
    let c = ctx.clone();
    ui.on_set_seerr_on(move |v| {
        c.update_cfg(|cfg| cfg.seerr_enabled = v);
        if let Some(ui) = c.ui.upgrade() {
            ui.set_seerr_on(v);
        }
        if v {
            c.sr_restore();
        }
    });

    // ---- notifications
    let c = ctx.clone();
    ui.on_set_notify_enabled(move |v| c.update_cfg(|cfg| cfg.notify_enabled = v));
    let c = ctx.clone();
    ui.on_set_notify_movies(move |v| c.update_cfg(|cfg| cfg.notify_movies = v));
    let c = ctx.clone();
    ui.on_set_notify_episodes(move |v| c.update_cfg(|cfg| cfg.notify_episodes = v));
    let c = ctx.clone();
    ui.on_set_notify_windows(move |v| c.update_cfg(|cfg| cfg.notify_windows = v));
    let c = ctx.clone();
    ui.on_set_notify_mode(move |i| c.update_cfg(|cfg| cfg.notify_mode = i.max(0) as usize));
    let c = ctx.clone();
    ui.on_set_notify_interval(move |i| c.update_cfg(|cfg| cfg.notify_interval = i.max(0) as usize));
    let c = ctx.clone();
    ui.on_note_open_item(move |id| {
        if let Some(ui) = c.ui.upgrade() {
            ui.set_note_open(false);
        }
        c.open_note_item(id.as_str());
    });
    let c = ctx.clone();
    ui.on_open_notifications(move || c.navigate(Page::Notifications, true));
    let c = ctx.clone();
    ui.on_note_click(move |id, item| {
        c.note_mark_read(id.as_str());
        c.open_note_item(item.as_str());
    });
    let c = ctx.clone();
    ui.on_note_mark_read(move |id| c.note_mark_read(id.as_str()));
    let c = ctx.clone();
    ui.on_notes_mark_all_read(move || c.notes_mark_all_read());
    let c = ctx.clone();
    ui.on_notes_clear(move || c.notes_clear());
    let c = ctx.clone();
    ui.on_hide_item(move || c.hide_item());
    let c = ctx.clone();
    ui.on_cw_hide(move |season| c.cw_hide(season));
    let c = ctx.clone();
    ui.on_unhide(move |id, cw| c.unhide(id.as_str(), cw));
    let c = ctx.clone();
    ui.on_toggle_follow(move || c.toggle_follow());
    let c = ctx.clone();
    ui.on_fav_tab_picked(move |i| c.fav_tab_picked(i));
    let c = ctx.clone();
    ui.on_fav_search(move |q| c.fav_search(q.to_string()));
    let c = ctx.clone();
    ui.on_set_card_scale(move |i| {
        let i = (i.max(0) as usize).min(CARD_SCALES.len() - 1);
        c.update_cfg(|cfg| cfg.card_scale = i);
        if let Some(ui) = c.ui.upgrade() {
            ui.global::<Scale>().set_base(CARD_SCALES[i]);
        }
    });
    let c = ctx.clone();
    ui.on_set_card_auto(move |v| {
        c.update_cfg(|cfg| cfg.card_auto = v);
        if let Some(ui) = c.ui.upgrade() {
            ui.global::<Scale>().set_auto(v);
        }
    });
    let c = ctx.clone();
    ui.on_set_hide_spoilers(move |v| c.update_cfg(|cfg| cfg.hide_spoilers = v));
    let c = ctx.clone();
    ui.on_set_watchlist_auto(move |v| c.update_cfg(|cfg| cfg.watchlist_autoremove = v));
    let c = ctx.clone();
    ui.on_lib_search(move |t| {
        let t = t.trim().to_string();
        c.lib_set(|lv| lv.search = Some(t).filter(|t| !t.is_empty()));
    });
    let c = ctx.clone();
    ui.on_genre_click(move |g| {
        let kind = if c.ui.upgrade().is_some_and(|u| u.get_detail().kind == "Movie") { "Movie" } else { "Series" };
        c.navigate(Page::Genre { name: g.to_string(), kind: kind.into() }, true);
    });
    let c = ctx.clone();
    ui.on_set_close_to_tray(move |v| {
        c.update_cfg(|cfg| cfg.close_to_tray = v);
        notes::set_tray(v);
    });
    let weak = ui.as_weak();
    ui.on_note_dismiss(move || {
        if let Some(ui) = weak.upgrade() {
            ui.set_note_open(false);
        }
    });

    // ---- home screen rows
    let c = ctx.clone();
    ui.on_set_home_row(move |name, shown| {
        let name = name.to_string();
        c.update_cfg(|cfg| {
            cfg.hidden_rows.retain(|r| r != &name);
            if !shown {
                cfg.hidden_rows.push(name.clone());
            }
        });
    });

    let c = ctx.clone();
    ui.on_move_home_row(move |key, dir| {
        let key = key.to_string();
        c.update_cfg(|cfg| {
            let mut o = home_order(cfg);
            if let Some(i) = o.iter().position(|k| *k == key) {
                let j = (i as i32 + dir).clamp(0, o.len() as i32 - 1) as usize;
                o.swap(i, j);
            }
            cfg.row_order = o;
        });
        if let Some(ui) = c.ui.upgrade() {
            ui.set_home_rows(home_rows_model(&c.st.lock().unwrap().cfg));
        }
    });

    // ---- debug logging
    let c = ctx.clone();
    ui.on_set_debug_logging(move |v| {
        c.update_cfg(|cfg| cfg.debug_logging = v);
        DEBUG.store(v, Ordering::Relaxed);
        if v {
            let _ = std::fs::create_dir_all(logs_dir());
            let cfg = c.st.lock().unwrap().cfg.clone();
            debug_banner(&cfg);
        }
    });
    ui.on_open_logs(move || {
        let _ = std::fs::create_dir_all(logs_dir());
        platform::open_folder(&logs_dir());
    });
    ui.on_clear_logs(move || {
        if let Ok(rd) = std::fs::read_dir(logs_dir()) {
            for e in rd.flatten() {
                let _ = std::fs::remove_file(e.path());
            }
        }
    });
    let c = ctx.clone();
    ui.on_lib_sort_changed(move |i| {
        let i = i.max(0) as usize;
        c.update_cfg(|cfg| cfg.library_sort = i);
        c.lib_set(|lv| lv.sort = i);
    });
    let c = ctx.clone();
    ui.on_open_favorites(move || c.show_favorites());
    let c = ctx.clone();
    ui.on_lib_genre_changed(move |i| {
        c.lib_set(|lv| lv.genre = if i <= 0 { None } else { lv.genres.get(i as usize - 1).cloned() });
    });
    let c = ctx.clone();
    ui.on_lib_letter_picked(move |l| {
        let l = l.to_string();
        let mut chosen: Option<String> = None;
        c.lib_set(|lv| {
            // picking the active letter (or "All") clears it
            lv.letter = if l == "All" || lv.letter.as_deref() == Some(l.as_str()) { None } else { Some(l.clone()) };
            chosen = lv.letter.clone();
        });
        if let Some(ui) = c.ui.upgrade() {
            ui.set_lib_letter(chosen.unwrap_or_default().into());
        }
    });
    let c = ctx.clone();
    ui.on_lib_more(move || c.load_library(true));
    let c = ctx.clone();
    ui.on_toggle_played(move || c.toggle_detail_flag(false));
    let c = ctx.clone();
    ui.on_toggle_favorite(move || c.toggle_detail_flag(true));
    let c = ctx.clone();
    ui.on_open_settings(move || c.navigate(Page::Settings, true));
    let c = ctx.clone();
    // The settings list is Original, Auto, then the presets; the player menu ids are Auto=0, Original=1, presets 2+.
    ui.on_set_quality_index(move |idx| c.pick_quality(match idx { 0 => 1, 1 => 0, n => n }));
    let c = ctx.clone();
    ui.on_set_autoplay(move |v| c.update_cfg(|cfg| cfg.autoplay_next = v));
    let c = ctx.clone();
    ui.on_set_auto_skip_intro(move |v| c.update_cfg(|cfg| cfg.auto_skip_intro = v));
    let c = ctx.clone();
    ui.on_set_rewind_index(move |i| {
        let v = SKIP_STEPS[(i.max(0) as usize).min(SKIP_STEPS.len() - 1)];
        c.update_cfg(|cfg| cfg.rewind_secs = v);
        if let Some(ui) = c.ui.upgrade() {
            ui.set_rewind_secs(v);
        }
    });
    let c = ctx.clone();
    ui.on_set_forward_index(move |i| {
        let v = SKIP_STEPS[(i.max(0) as usize).min(SKIP_STEPS.len() - 1)];
        c.update_cfg(|cfg| cfg.forward_secs = v);
        if let Some(ui) = c.ui.upgrade() {
            ui.set_forward_secs(v);
        }
    });
    let c = ctx.clone();
    ui.on_set_hw_mode(move |i| c.update_cfg(|cfg| cfg.hw_mode = i.max(0) as usize));
    let c = ctx.clone();
    ui.on_set_hwdec(move |v| c.update_cfg(|cfg| cfg.hw_decode = v));
    let c = ctx.clone();
    ui.on_set_sub_lang(move |i| c.update_cfg(|cfg| cfg.sub_lang = i.max(0) as usize));
    let c = ctx.clone();
    ui.on_set_on_top(move |v| {
        c.update_cfg(|cfg| cfg.always_on_top = v);
        if let Some(ui) = c.ui.upgrade() {
            // Only takes effect while a video is playing (see always-on-top in main.slint).
            ui.set_on_top(v);
        }
    });
    let c = ctx.clone();
    ui.on_set_accent_mode(move |i| {
        c.update_cfg(|cfg| cfg.accent_mode = i.max(0) as usize);
        c.refresh_accent(true);
    });
    let c = ctx.clone();
    ui.on_set_accent_rgb(move |col| {
        let rgb = rgb_of(col);
        c.update_cfg(|cfg| cfg.accent_rgb = rgb);
        c.refresh_accent(true);
    });
    let c = ctx.clone();
    ui.on_accent_picked(move |col| {
        let rgb = rgb_of(col);
        c.update_cfg(|cfg| cfg.accent_rgb = rgb);
        c.refresh_accent(false);
    });
    let c = ctx.clone();
    ui.on_set_accent_hex(move |t| {
        if let Some(rgb) = parse_hex(t.as_str()) {
            c.update_cfg(|cfg| cfg.accent_rgb = rgb);
        }
        c.refresh_accent(true);
    });
    let c = ctx.clone();
    ui.on_set_hover_backdrop(move |v| {
        c.update_cfg(|cfg| cfg.hover_backdrop = v);
        if let Some(ui) = c.ui.upgrade() {
            ui.set_hover_backdrop(v);
            if !v {
                ui.global::<Hover>().set_active(false);
            }
        }
    });
    let c = ctx.clone();
    {
        let c = c.clone();
        ui.global::<Hover>().on_entered(move |id, kind| c.hover_entered(id.to_string(), kind.to_string()));
    }
    {
        let c = c.clone();
        ui.global::<Hover>().on_left(move || c.hover_left());
    }
    ui.set_hover_backdrop(ctx.st.lock().unwrap().cfg.hover_backdrop);
    let c = ctx.clone();
    ui.on_set_nav_pinned(move |v| {
        c.update_cfg(|cfg| cfg.nav_pinned = v);
        if let Some(ui) = c.ui.upgrade() {
            ui.set_nav_pinned(v);
        }
    });
    let c = ctx.clone();
    ui.on_set_material(move |i| {
        c.update_cfg(|cfg| cfg.material = i.max(0) as usize);
        if let Some(ui) = c.ui.upgrade() {
            apply_material(&ui, i.max(0) as usize);
        }
    });

    // Controls vanish the instant the cursor leaves the window; the mouse's Back thumb button and the
    // keyboard's Browser Back key go back a page. (Desktop only; Android's Back key is handled by the UI.)
    {
        let (weak, c) = (ui.as_weak(), ctx.clone());
        platform::watch_window_events(
            ui.window(),
            move || c.go_back(),
            move || {
                if let Some(ui) = weak.upgrade() {
                    if ui.get_playing() && !ui.get_menu_hold() && !ui.get_sub_dialog_open() {
                        ui.set_controls_shown(false);
                    }
                }
            },
        );
    }
    // Debug logging, hooks and the Seerr session.
    {
        let cfg = ctx.st.lock().unwrap().cfg.clone();
        DEBUG.store(cfg.debug_logging, Ordering::Relaxed);
        emby_api::set_log_hook(dlog);
        seerr_api::set_log_hook(dlog);
        if cfg.debug_logging {
            debug_banner(&cfg);
        }
    }
    // On Android libmpv ships inside the APK, so there is nothing to download.
    #[cfg(not(target_os = "android"))]
    if !player::available() {
        let weak = ui.as_weak();
        let say = move |header: &str, text: String| {
            let (h, w) = (header.to_string(), weak.clone());
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = w.upgrade() {
                    ui.set_message_header(h.into());
                    ui.set_message_text(text.into());
                    ui.set_message_open(true);
                }
            });
        };
        ctx.rt.spawn(async move {
            say("Setting up video playback", "Foyer needs the mpv video engine and will download it once (about 30 MB).".into());
            match engine::install(|t| say("Setting up video playback", t)).await {
                Ok(p) => {
                    applog(&format!("video engine installed: {}", p.display()));
                    say("Video engine ready", "Playback is ready to use.".into());
                }
                Err(e) => {
                    applog(&format!("video engine download failed: {e}"));
                    say("Couldn't download the video engine", format!("{e}

Place libmpv-2.dll (or mpv-1.dll) next to foyer.exe and restart."));
                }
            }
        });
    }
    ctx.update_startup();
    ctx.sr_restore();
    ctx.sr_start_notifier();
    ctx.start_follow_checker();
    ctx.sync_settings(&ui);
    ctx.apply_notes(&ui);
    ctx.refresh_accent(true);
    // Follow changes made in Windows Personalization while "Match Windows" is selected.
    let accent_timer = slint::Timer::default();
    {
        let c = ctx.clone();
        let mut last = 0u32;
        accent_timer.start(slint::TimerMode::Repeated, Duration::from_secs(3), move || {
            if c.st.lock().unwrap().cfg.accent_mode != 0 {
                return;
            }
            if let Some(rgb) = platform::system_accent() {
                if rgb != last {
                    last = rgb;
                    c.refresh_accent(false);
                }
            }
        });
    }
    {
        // The window has to exist before a backdrop can be applied.
        let weak = ui.as_weak();
        let material = ctx.st.lock().unwrap().cfg.material;
        slint::Timer::single_shot(Duration::from_millis(400), move || {
            if let Some(ui) = weak.upgrade() {
                apply_material(&ui, material);
            }
        });
    }
    {
        let q = ctx.quality();
        ui.set_quality_label(quality_label(q, "").into());
        ui.set_quality_rows(ModelRc::new(VecModel::from(quality_rows(q, ""))));
    }
    let c = ctx.clone();
    ui.on_pick_audio(move |id| {
        if let Some(ui) = c.ui.upgrade() {
            ui.set_menu_hold(false);
        }
        c.send_control(match id {
            -10 => player::Control::AudioDelayBy(0.05),
            -11 => player::Control::AudioDelayBy(-0.05),
            -12 => player::Control::ResetAudioDelay,
            _ => player::Control::SelectAudio(id as i64),
        });
    });
    let c = ctx.clone();
    ui.on_sub_sync(move |d| {
        c.send_control(if d == 0.0 { player::Control::ResetSubDelay } else { player::Control::SubDelayBy(d as f64) });
    });
    let c = ctx.clone();
    ui.on_pick_sub(move |id| {
        if let Some(ui) = c.ui.upgrade() {
            ui.set_menu_hold(false);
        }
        c.send_control(player::Control::SelectSub(id as i64));
    });
    let c = ctx.clone();
    ui.on_open_sub_search(move || c.open_sub_search());
    let c = ctx.clone();
    ui.on_close_sub_search(move || c.close_sub_search());
    let c = ctx.clone();
    ui.on_sub_search(move |i| c.sub_search(i));
    let c = ctx.clone();
    ui.on_sub_download(move |id| c.sub_download(id.to_string()));
    let c = ctx.clone();
    ui.on_video_click(move || c.video_click());
    let c = ctx.clone();
    ui.on_mouse_moved(move || c.mouse_moved());
    // Like Chrome's PiP, resizing keeps the video's shape.
    let weak = ui.as_weak();
    ui.on_pip_resized(move || {
        let Some(ui) = weak.upgrade() else { return };
        let s = ui.window().size();
        let h = (s.width as f32 / ui.get_video_aspect().max(0.2)) as u32;
        if ui.get_pip() && h.abs_diff(s.height) > 2 {
            ui.window().set_size(slint::PhysicalSize::new(s.width, h));
        }
    });
    let c = ctx.clone();
    ui.on_pip_changed(move |on| c.apply_pip(on));
    let weak = ui.as_weak();
    ui.on_pip_drag(move || {
        if let Some(ui) = weak.upgrade() {
            platform::drag_window(ui.window());
        }
    });
    let c = ctx.clone();
    let last_pos = std::cell::Cell::new(100i64);
    ui.on_sub_lift(move |ratio| {
        let pos = (100.0 - ratio * 100.0).round() as i64;
        if pos != last_pos.replace(pos) {
            if let Some(tx) = &c.st.lock().unwrap().player_tx {
                let _ = tx.send(player::Control::SubPos(pos));
            }
        }
    });

    // Hide the player controls after 3 s without mouse movement (unless paused or a menu is open).
    let hide_timer = slint::Timer::default();
    {
        let c = ctx.clone();
        let cursor_hidden = std::cell::Cell::new(false);
        hide_timer.start(slint::TimerMode::Repeated, Duration::from_millis(400), move || {
            let Some(ui) = c.ui.upgrade() else { return };
            if ui.get_playing()
                && ui.get_controls_shown()
                && !ui.get_menu_hold()
                && !ui.get_sub_dialog_open()
                && !ui.get_message_open()
                && ui.get_player_started()
                && c.st.lock().unwrap().last_move.elapsed() > Duration::from_secs(3)
            {
                ui.set_controls_shown(false);
            }
            // Slint only re-applies its cursor rule on the next mouse event, so a still cursor would
            // stay visible after the controls fade. Drive the OS cursor directly as well.
            let hidden = ui.get_full_player() && !ui.get_pip() && ui.get_player_started() && !ui.get_controls_shown() && !ui.get_player_paused();
            if hidden != cursor_hidden.replace(hidden) {
                platform::set_cursor_visible(ui.window(), !hidden);
            }
        });
    }

    // Window size / maximised state: restore it, and keep it saved as it changes.
    notes::init_tray_events(ui.as_weak());
    {
        let cfg = ctx.st.lock().unwrap().cfg.clone();
        if cfg.win_w >= 600 && cfg.win_h >= 380 {
            ui.window().set_size(slint::PhysicalSize::new(cfg.win_w, cfg.win_h));
        }
        notes::set_tray(cfg.close_to_tray);
    }
    let _ = ui.show();
    if ctx.st.lock().unwrap().cfg.win_max {
        ui.window().set_maximized(true);
    }
    let save_window = {
        let c = ctx.clone();
        move || {
            let Some(ui) = c.ui.upgrade() else { return };
            let w = ui.window();
            if w.is_fullscreen() || w.is_minimized() || ui.get_pip() {
                return;
            }
            let (max, size) = (w.is_maximized(), w.size());
            let (cw, ch, cm) = {
                let s = c.st.lock().unwrap();
                (s.cfg.win_w, s.cfg.win_h, s.cfg.win_max)
            };
            // A maximised window keeps the last normal size so un-maximising goes back to it.
            let resized = !max && size.width >= 600 && size.height >= 380 && (size.width != cw || size.height != ch);
            if resized || max != cm {
                c.update_cfg(|cfg| {
                    if resized {
                        cfg.win_w = size.width;
                        cfg.win_h = size.height;
                    }
                    cfg.win_max = max;
                });
            }
        }
    };
    let window_timer = slint::Timer::default();
    window_timer.start(slint::TimerMode::Repeated, Duration::from_secs(2), save_window.clone());
    {
        let c = ctx.clone();
        ui.window().on_close_requested(move || {
            save_window();
            if c.st.lock().unwrap().cfg.close_to_tray {
                // Don't leave a video playing in a hidden window.
                if let Some(tx) = &c.st.lock().unwrap().player_tx {
                    let _ = tx.send(player::Control::Quit);
                }
            } else {
                let _ = slint::quit_event_loop();
            }
            slint::CloseRequestResponse::HideWindow
        });
    }

    load_posters(ui.as_weak());
    ctx.try_saved_login();
    let result = slint::run_event_loop_until_quit();
    // Remove the tray icon explicitly; otherwise Windows can leave a dead one behind.
    notes::set_tray(false);
    drop(window_timer);
    drop(hide_timer);
    drop(accent_timer);
    result
}




fn home_rows_model(cfg: &Config) -> ModelRc<HomeRowCfg> {
    let rows: Vec<HomeRowCfg> = home_order(cfg)
        .iter()
        .filter_map(|k| HOME_ROWS.iter().find(|r| r.0 == k.as_str()))
        .map(|(k, t, d)| HomeRowCfg {
            key: (*k).into(),
            title: (*t).into(),
            description: (*d).into(),
            shown: !cfg.hidden_rows.iter().any(|h| h == k),
        })
        .collect();
    ModelRc::new(VecModel::from(rows))
}


/// Which Mine sub-tab a favourite belongs to: 0 TV, 1 Movies, 2 anything else.
fn fav_group(i: &Item) -> usize {
    match i.kind.as_str() {
        "Series" | "Season" | "Episode" => 0,
        "Movie" => 1,
        _ => 2,
    }
}

/// Integrated graphics (Intel, or an AMD APU) rather than a dedicated card.
fn is_integrated(name: &str) -> bool {
    let n = name.to_lowercase();
    n.contains("intel") || n.contains("uhd") || n.contains("iris") || n.contains("vega") && !n.contains("frontier") || (n.contains("radeon") && n.contains("graphics") && !n.contains("rx "))
}

/// The decoding method used when the setting is "Automatic". With two GPUs the integrated one
/// is preferred, since it is the one that idles for video and shares memory with the CPU.
/// Currently every GPU gets the conversion path (copy-back decode into a plain 10-bit layout),
/// which is the combination that draws cleanly with the embedded player; per-vendor choices
/// belong here once other methods are verified.
fn auto_hw_mode() -> usize {
    static MODE: std::sync::OnceLock<usize> = std::sync::OnceLock::new();
    *MODE.get_or_init(|| {
        let gpus = platform::gpu_names();
        let preferred = gpus.iter().find(|g| is_integrated(g)).or(gpus.first()).cloned().unwrap_or_default();
        dlog(&format!("graphics adapters: {gpus:?}; preferred for decoding: {preferred:?}"));
        1
    })
}


/// Android entry point (called by the NativeActivity glue in the APK).
#[cfg(target_os = "android")]
#[no_mangle]
fn android_main(app: slint::android::AndroidApp) {
    if let Some(dir) = app.internal_data_path() {
        std::env::set_var("FOYER_DATA_DIR", dir);
    }
    let (vm, activity) = (app.vm_as_ptr(), app.activity_as_ptr());
    platform::init(vm, activity);
    player::set_java_vm(vm);
    slint::android::init(app).expect("slint android init");
    if let Err(e) = run() {
        eprintln!("foyer exited with an error: {e}");
    }
}
