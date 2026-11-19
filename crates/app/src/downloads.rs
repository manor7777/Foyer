//! Downloads: "download for offline play" (kept inside Foyer, playable with no server) and
//! "download as file" (saved to the Downloads folder). A series or season downloads every episode.
//! Needs the server's "Allow media downloading" permission for the user.

use super::*;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Entry {
    pub id: String,
    pub title: String,
    pub subtitle: String,
    /// Kept for offline play (true) or saved as a plain file (false).
    pub offline: bool,
    pub file: String,
    pub total: u64,
    pub done: u64,
    /// 0 waiting, 1 downloading, 2 finished, 3 failed
    pub state: u8,
    pub error: String,
}

static LIST: Mutex<Vec<Entry>> = Mutex::new(Vec::new());
/// "id:offline" keys whose download should stop.
static CANCEL: Mutex<Vec<String>> = Mutex::new(Vec::new());
static SLOTS: std::sync::OnceLock<Semaphore> = std::sync::OnceLock::new();

fn key(id: &str, offline: bool) -> String {
    format!("{id}:{offline}")
}

fn dir() -> PathBuf {
    data_dir().join("downloads")
}

fn persist() {
    let list = LIST.lock().unwrap().clone();
    let _ = std::fs::create_dir_all(dir());
    if let Ok(j) = serde_json::to_vec(&list) {
        let _ = std::fs::write(dir().join("index.json"), j);
    }
}

/// Reads the saved list. Anything that was mid-download when Foyer closed is marked failed.
pub fn load() {
    let mut list: Vec<Entry> = std::fs::read(dir().join("index.json")).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or_default();
    for e in list.iter_mut().filter(|e| e.state < 2) {
        e.state = 3;
        e.error = "Interrupted".into();
    }
    *LIST.lock().unwrap() = list;
}

/// The downloaded file for offline play, if there is one.
pub fn offline_path(id: &str) -> Option<PathBuf> {
    let l = LIST.lock().unwrap();
    l.iter().find(|e| e.id == id && e.offline && e.state == 2).map(|e| PathBuf::from(&e.file)).filter(|p| p.exists())
}

/// The item's details saved with an offline copy (so it can play with no server).
pub fn offline_item(id: &str) -> Option<Item> {
    serde_json::from_slice(&std::fs::read(dir().join(format!("{id}.json"))).ok()?).ok()
}

pub fn any_offline() -> bool {
    LIST.lock().unwrap().iter().any(|e| e.state == 2 && e.offline)
}

fn update(id: &str, offline: bool, f: impl FnOnce(&mut Entry)) {
    if let Some(e) = LIST.lock().unwrap().iter_mut().find(|e| e.id == id && e.offline == offline) {
        f(e);
    }
}

fn safe_name(s: &str) -> String {
    let s: String = s.chars().map(|c| if r#"\/:*?"<>|"#.contains(c) || c.is_control() { '_' } else { c }).collect();
    // Windows rejects trailing dots/spaces and device names such as CON or NUL.
    let s = s.trim_end_matches(['.', ' ']).trim_start().to_string();
    if s.is_empty() {
        return "download".into();
    }
    let stem = s.split('.').next().unwrap_or("").trim_end().to_ascii_uppercase();
    let reserved = matches!(stem.as_str(), "CON" | "PRN" | "AUX" | "NUL") || (stem.len() == 4 && (stem.starts_with("COM") || stem.starts_with("LPT")) && stem.as_bytes()[3].is_ascii_digit());
    if reserved { format!("_{s}") } else { s }
}

/// Deletes every file Foyer keeps for `id` (video, details, poster), including partial ones.
fn remove_files(id: &str) {
    let prefix = format!("{id}.");
    for d in [dir(), dir().join("tmp")] {
        for e in std::fs::read_dir(d).into_iter().flatten().flatten() {
            if e.file_name().to_string_lossy().starts_with(&prefix) {
                let _ = std::fs::remove_file(e.path());
            }
        }
    }
}

/// The item itself, or every episode of a series / season, or the playable items of a folder.
async fn expand(client: &Client, uid: &str, id: &str) -> Result<Vec<Item>, emby_api::Error> {
    let item = client.item(uid, id).await?;
    Ok(match item.kind.as_str() {
        "Series" => client.series_episodes(uid, &item.id).await?,
        "Season" => client.episodes(uid, item.series_id.as_deref().unwrap_or(""), &item.id).await?,
        _ if item.is_folder => client.children(uid, &item.id, 300).await?.into_iter().filter(|i| i.is_playable()).collect(),
        _ => vec![item],
    })
}

fn title_of(i: &Item) -> String {
    match i.kind.as_str() {
        "Episode" => format!("{} {} {}", i.series_name.clone().unwrap_or_default(), episode_tag(i), i.name),
        _ => i.name.clone(),
    }
}

async fn fetch_one(client: &Client, uid: &str, id: &str, offline: bool, ctx: &Ctx) -> Result<(), String> {
    let raw = client.item_raw(uid, id).await.map_err(|e| e.to_string())?;
    let item: Item = serde_json::from_value(raw.clone()).map_err(|e| e.to_string())?;
    let d = dir();
    std::fs::create_dir_all(d.join("tmp")).map_err(|e| e.to_string())?;
    let ext = item.media_sources.first().and_then(|m| m.container.clone()).and_then(|c| c.split(',').next().map(str::to_string)).filter(|c| !c.is_empty()).unwrap_or_else(|| "mkv".into());
    if offline {
        let _ = std::fs::write(d.join(format!("{id}.json")), raw.to_string());
        if let Some(u) = client.image_url(&item, ImageKind::Primary, 300) {
            if let Ok(b) = client.fetch_bytes(&u).await {
                let _ = std::fs::write(d.join(format!("{id}.jpg")), b);
            }
        }
    }
    let url = client.download_url(id).ok_or("This server doesn't allow downloads")?;
    let path = if offline { d.join(format!("{id}.{ext}")) } else { d.join("tmp").join(format!("{id}.{ext}")) };
    let k = key(id, offline);
    let last = Mutex::new(Instant::now());
    let (idc, c2) = (id.to_string(), ctx.clone());
    // What the server says the file should weigh. A server that streams from a remote mount (rclone,
    // a NAS) can answer "200 OK" with an empty or short body while it is still fetching the file.
    let expected = item.media_sources.first().and_then(|m| m.size).filter(|s| *s > 0).unwrap_or(0) as u64;
    let mut attempt = 0;
    loop {
        let finished = client
            .download_to(
                &url,
                &path,
                |done, total| {
                    update(&idc, offline, |e| {
                        e.done = done;
                        e.total = if total > 0 { total } else { expected };
                    });
                    let mut l = last.lock().unwrap();
                    if l.elapsed() > Duration::from_millis(400) {
                        *l = Instant::now();
                        let c = c2.clone();
                        let _ = slint::invoke_from_event_loop(move || c.dl_sync_ui());
                    }
                },
                || CANCEL.lock().unwrap().contains(&k),
            )
            .await
            .map_err(|e| e.to_string());
        let finished = match finished {
            Ok(f) => f,
            Err(e) if attempt < 3 => {
                applog(&format!("download {id}: {e}; retrying"));
                attempt += 1;
                tokio::time::sleep(Duration::from_secs(5 * attempt as u64)).await;
                continue;
            }
            Err(e) => {
                let _ = std::fs::remove_file(&path);
                return Err(e);
            }
        };
        if !finished {
            return Err("Cancelled".into());
        }
        let len = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        // Allow a little slack in case the server's size is only approximate.
        if len > 0 && (expected == 0 || len + 1024 >= expected) {
            break;
        }
        let _ = std::fs::remove_file(&path);
        applog(&format!("download {id}: server sent {len} of {expected} bytes (attempt {})", attempt + 1));
        if attempt >= 3 {
            return Err(if len == 0 { "The server sent an empty file (it may still be fetching it from storage). Try again in a minute.".into() } else { format!("The server stopped early ({} of {})", fmt_size(len as i64), fmt_size(expected as i64)) });
        }
        attempt += 1;
        // Give the server (e.g. an rclone-backed NAS) time to pull the file before asking again.
        tokio::time::sleep(Duration::from_secs(10 * attempt as u64)).await;
        if CANCEL.lock().unwrap().contains(&k) {
            return Err("Cancelled".into());
        }
    }
    let dest = ctx.st.lock().unwrap().cfg.download_dir.clone();
    let final_path = if offline {
        path.to_string_lossy().into_owned()
    } else {
        platform::save_to_downloads(&path, &format!("{}.{ext}", safe_name(&title_of(&item))), &dest)?
    };
    update(id, offline, |e| {
        e.file = final_path;
        e.state = 2;
        e.done = e.total.max(e.done);
    });
    Ok(())
}

impl Ctx {
    /// Queues `id` (an item, season or series). Called from the menus and the detail page.
    pub fn dl_start(&self, id: String, offline: bool) {
        let Some((client, uid)) = self.session() else { return };
        let ctx = self.clone();
        self.rt.spawn(async move {
            let items = match expand(&client, &uid, &id).await {
                Ok(i) if !i.is_empty() => i,
                Ok(_) => return ctx.dl_toast("Nothing to download there.".into()),
                Err(e) => return ctx.dl_toast(format!("Couldn't start the download: {e}")),
            };
            let mut queued: Vec<(String, String)> = Vec::new();
            {
                let mut l = LIST.lock().unwrap();
                for i in &items {
                    // Already waiting, downloading or finished: leave it. A failed one tries again.
                    if l.iter().any(|e| e.id == i.id && e.offline == offline && (e.state < 2 || (e.state == 2 && (!offline || std::path::Path::new(&e.file).exists())))) {
                        continue;
                    }
                    l.retain(|e| !(e.id == i.id && e.offline == offline));
                    CANCEL.lock().unwrap().retain(|k| *k != key(&i.id, offline));
                    l.push(Entry { id: i.id.clone(), title: title_of(i), subtitle: i.kind.clone(), offline, ..Default::default() });
                    queued.push((i.id.clone(), title_of(i)));
                }
            }
            persist();
            let c = ctx.clone();
            let _ = slint::invoke_from_event_loop(move || c.dl_sync_ui());
            if queued.is_empty() {
                return ctx.dl_toast("Already downloaded or downloading.".into());
            }
            ctx.dl_toast(format!("Downloading {}…", if queued.len() == 1 { queued[0].1.clone() } else { format!("{} items", queued.len()) }));
            let slots = SLOTS.get_or_init(|| Semaphore::new(2));
            let (mut ok, mut failed) = (0, 0);
            for (qid, title) in &queued {
                let Ok(_permit) = slots.acquire().await else { return };
                // Cancelled or removed while it waited.
                if !LIST.lock().unwrap().iter().any(|e| e.id == *qid && e.offline == offline && e.state == 0) {
                    continue;
                }
                update(qid, offline, |e| e.state = 1);
                let c = ctx.clone();
                let _ = slint::invoke_from_event_loop(move || c.dl_sync_ui());
                match fetch_one(&client, &uid, qid, offline, &ctx).await {
                    Ok(()) => {
                        ok += 1;
                        if queued.len() == 1 {
                            ctx.dl_done("Download complete", title);
                        }
                    }
                    Err(m) if m == "Cancelled" => {
                        LIST.lock().unwrap().retain(|e| !(e.id == *qid && e.offline == offline));
                    }
                    Err(m) => {
                        failed += 1;
                        applog(&format!("download {qid} failed: {m}"));
                        update(qid, offline, |e| {
                            e.state = 3;
                            e.error = m.clone();
                        });
                        if queued.len() == 1 {
                            ctx.dl_done("Download failed", &format!("{title}: {m}"));
                        }
                    }
                }
                CANCEL.lock().unwrap().retain(|k| *k != key(qid, offline));
                persist();
                let c = ctx.clone();
                let _ = slint::invoke_from_event_loop(move || c.dl_sync_ui());
            }
            if queued.len() > 1 {
                let body = if failed == 0 { format!("{ok} items finished downloading.") } else { format!("{ok} finished, {failed} failed.") };
                ctx.dl_done("Downloads complete", &body);
            }
        });
    }

    /// A system notification plus the in-app message.
    fn dl_done(&self, title: &str, body: &str) {
        platform::system_notify(title, body);
        self.dl_toast(format!("{title}: {body}"));
    }

    fn dl_toast(&self, msg: String) {
        let c = self.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = c.ui.upgrade() {
                c.show_toast(&ui, msg);
            }
        });
    }

    /// Pushes the list to the page. Must run on the UI thread.
    pub fn dl_sync_ui(&self) {
        let Some(ui) = self.ui.upgrade() else { return };
        let list = LIST.lock().unwrap().clone();
        let rows: Vec<DownloadRow> = list
            .iter()
            .rev()
            .map(|e| {
                let pct = if e.total > 0 { (e.done as f64 / e.total as f64) as f32 } else { 0.0 };
                let status = match e.state {
                    0 => "Waiting…".to_string(),
                    1 if e.total > 0 => format!("{:.0}% · {} of {}", pct * 100.0, fmt_size(e.done as i64), fmt_size(e.total as i64)),
                    1 => "Starting…".to_string(),
                    2 if e.offline => format!("Ready to watch offline · {}", fmt_size(e.total as i64)),
                    2 => format!("Saved to {}", e.file),
                    _ => format!("Failed: {}", e.error),
                };
                DownloadRow { id: e.id.clone().into(), title: e.title.clone().into(), status: status.into(), progress: pct, state: e.state as i32, offline: e.offline }
            })
            .collect();
        let g = ui.global::<DownloadsUi>();
        g.set_rows(ModelRc::new(VecModel::from(rows)));
        g.set_any(!list.is_empty());
        g.set_active(list.iter().filter(|e| e.state < 2).count() as i32);
    }

    pub fn show_downloads(&self) {
        self.begin();
        if let Some(ui) = self.ui.upgrade() {
            ui.set_busy(false);
            self.dl_sync_ui();
            ui.set_page("downloads".into());
        }
    }

    pub fn dl_cancel(&self, id: String, offline: bool) {
        let state = LIST.lock().unwrap().iter().find(|e| e.id == id && e.offline == offline).map(|e| e.state);
        match state {
            Some(1) => CANCEL.lock().unwrap().push(key(&id, offline)),
            Some(_) => {
                LIST.lock().unwrap().retain(|e| !(e.id == id && e.offline == offline));
                persist();
            }
            None => {}
        }
        self.dl_sync_ui();
    }

    /// Removes the entry and, for offline copies, the files Foyer keeps.
    pub fn dl_remove(&self, id: String, offline: bool) {
        if LIST.lock().unwrap().iter().any(|e| e.id == id && e.offline == offline && e.state == 1) {
            return self.dl_cancel(id, offline);
        }
        let gone = {
            let mut l = LIST.lock().unwrap();
            let before = l.len();
            let file = l.iter().find(|e| e.id == id && e.offline == offline).map(|e| e.file.clone());
            l.retain(|e| !(e.id == id && e.offline == offline));
            (l.len() < before).then_some(file)
        };
        if let Some(file) = gone {
            if offline {
                remove_files(&id);
                if let Some(f) = file.filter(|f| !f.is_empty()) {
                    let _ = std::fs::remove_file(f);
                }
            }
            persist();
        }
        self.dl_sync_ui();
    }

    pub fn dl_open_folder(&self, id: String) {
        let file = LIST.lock().unwrap().iter().find(|e| e.id == id && !e.offline).map(|e| e.file.clone());
        if let Some(p) = file.and_then(|f| PathBuf::from(f).parent().map(|p| p.to_path_buf())) {
            platform::open_folder(&p);
        }
    }

    /// Whether a finished offline copy of this item exists (for the card menu).
    pub fn dl_has(&self, id: &str) -> bool {
        offline_path(id).is_some()
    }
}
