//! Casting: send what you are watching to another signed-in app on the server (Emby / Jellyfin
//! "remote control" sessions) or to a Chromecast, from a title's page or from the player.

use super::*;
use foyer_core::gcast;

#[derive(Clone)]
enum Target {
    /// Another app signed in to the same server.
    Session { id: String, name: String, who: String },
    Chromecast(gcast::Device),
}

impl Target {
    fn name(&self) -> &str {
        match self {
            Target::Session { name, .. } => name,
            Target::Chromecast(d) => &d.name,
        }
    }

    /// The second line in the picker: which app, and whose it is.
    fn detail(&self) -> String {
        match self {
            Target::Session { who, .. } => who.clone(),
            Target::Chromecast(_) => "Chromecast".into(),
        }
    }
}

/// What the picker is showing, and what it will cast.
static PENDING: Mutex<Option<(String, f64)>> = Mutex::new(None);
static TARGETS: Mutex<Vec<Target>> = Mutex::new(Vec::new());
/// Which opening of the picker the results belong to.
static CAST_GEN: AtomicU64 = AtomicU64::new(0);

enum Active {
    Session { client: Client, id: String },
    Chromecast(gcast::Cast),
}

static ACTIVE: Mutex<Option<(Active, bool)>> = Mutex::new(None); // (target, paused)
/// Which cast the status poller belongs to (a new cast, or stopping, retires the old poller).
static RUN: AtomicU64 = AtomicU64::new(0);

/// What a Chromecast is playing, so the stream can be restarted with another audio track or subtitles.
struct Info {
    client: Client,
    item: String,
    source: String,
    audio: Vec<i32>,
    subs: Vec<i32>,
    audio_pick: usize,
    /// 0 = off, otherwise 1 + position in `subs`.
    sub_pick: usize,
    /// The episodes either side of this one.
    prev: Option<String>,
    next: Option<String>,
    /// 0 = the original file (no transcoding), otherwise 1 + index into PRESETS.
    quality: usize,
    /// The file's container, for the content type when it is sent as it is.
    container: String,
    /// The audio track the file plays by itself (the original can't switch tracks).
    file_audio: usize,
    /// The play session of the stream last handed to the device.
    session: String,
}

/// The quality a transcode falls back to (12 Mbps, 1080p) when the original can't be used.
const FALLBACK_QUALITY: usize = 3;

/// Everything needed to start one title on a Chromecast.
struct Prep {
    info: Info,
    audio_names: Vec<String>,
    sub_names: Vec<String>,
    duration: f64,
    url: String,
    content_type: String,
    title: String,
    image: Option<String>,
}

/// (item, media source, play session) the server is being told about.
/// The quality picked in the controls (kept for the next episode): 0 = original.
static QUALITY: Mutex<usize> = Mutex::new(0);

static IDS: Mutex<(String, String, String)> = Mutex::new((String::new(), String::new(), String::new()));

async fn prepare(client: &Client, uid: &str, item_id: &str) -> Result<Prep, String> {
    let item = client.item(uid, item_id).await.map_err(|e| e.to_string())?;
    let source = item.media_sources.first().map(|s| s.id.clone()).unwrap_or_else(|| item.id.clone());
    // Same audio choice as playing here: the detail page's pick, else English, else the file's default.
    let streams = audio_streams(&item);
    let pos = match &*AUDIO_PICK.lock().unwrap() {
        Some((pid, idx)) if *pid == item.id => streams.iter().position(|s| s.index == *idx),
        _ => None,
    }
    .unwrap_or_else(|| default_audio(&streams));
    // Subtitles: the detail page's pick, else the remembered language / the file's default. They are burned
    // into the picture by the server, which works for every subtitle type on any Cast device.
    let subs_all: Vec<&emby_api::MediaStream> = item.media_sources.first().map(|s| s.media_streams.iter().filter(|m| m.kind == "Subtitle").collect()).unwrap_or_default();
    let sub_pos = match &*SUB_PICK.lock().unwrap() {
        Some((pid, pick)) if *pid == item.id => pick.and_then(|i| subs_all.iter().position(|s| s.index == i)).map_or(0, |p| p + 1),
        _ => default_sub(&subs_all, &SUB_PREF.lock().unwrap()),
    };
    let (prev, next) = match (item.kind.as_str(), item.series_id.as_ref()) {
        ("Episode", Some(series)) => match client.series_episodes(uid, series).await {
            Ok(eps) => match eps.iter().position(|e| e.id == item.id) {
                Some(p) => (p.checked_sub(1).and_then(|p| eps.get(p)).map(|e| e.id.clone()), eps.get(p + 1).map(|e| e.id.clone())),
                None => (None, None),
            },
            Err(_) => (None, None),
        },
        _ => (None, None),
    };
    let file_audio = streams.iter().position(|s| s.is_default).unwrap_or(0);
    let mut info = Info {
        client: client.clone(),
        item: item.id.clone(),
        source: source.clone(),
        audio: streams.iter().map(|s| s.index).collect(),
        subs: subs_all.iter().map(|s| s.index).collect(),
        audio_pick: pos,
        sub_pick: sub_pos,
        prev,
        next,
        quality: *QUALITY.lock().unwrap(),
        container: item.media_sources.first().and_then(|m| m.container.clone()).and_then(|c| c.split(',').next().map(str::to_ascii_lowercase)).unwrap_or_default(),
        file_audio,
        session: String::new(),
    };
    let (url, content_type) = info.url()?;
    Ok(Prep {
        info,
        audio_names: streams.iter().map(|s| stream_label(s)).collect(),
        sub_names: std::iter::once("Off".to_string()).chain(subs_all.iter().map(|s| stream_label(s))).collect(),
        duration: item.run_time_ticks.unwrap_or(0) as f64 / TICKS_PER_SEC as f64,
        url,
        content_type,
        title: match item.kind.as_str() {
            "Episode" => format!("{} {} {}", item.series_name.clone().unwrap_or_default(), episode_tag(&item), item.name),
            _ => item.name.clone(),
        },
        image: client.image_url(&item, emby_api::ImageKind::Backdrop, 1280),
    })
}

static INFO: Mutex<Option<Info>> = Mutex::new(None);

impl Info {
    /// Whether the server has to transcode: subtitles are burned into the picture, and only a transcode can pick
    /// another audio track. Otherwise the original file goes straight through.
    fn must_transcode(&self) -> bool {
        self.sub_pick != 0 || self.audio_pick != self.file_audio
    }

    /// The stream to hand to the device now, and its content type.
    fn url(&mut self) -> Result<(String, String), String> {
        self.session = uuid::Uuid::new_v4().simple().to_string();
        let sub = self.sub_pick.checked_sub(1).and_then(|p| self.subs.get(p)).copied();
        let audio = self.audio.get(self.audio_pick).copied();
        if self.quality == 0 && !self.must_transcode() {
            let url = self.client.stream_url(&self.item, &self.source, &self.session, None, None, None).map_err(|e| e.to_string())?;
            let ct = match self.container.as_str() {
                "mp4" | "m4v" | "mov" => "video/mp4",
                "webm" => "video/webm",
                _ => "video/x-matroska",
            };
            return Ok((url, ct.into()));
        }
        let p = &PRESETS[if self.quality == 0 { FALLBACK_QUALITY } else { self.quality - 1 }.min(PRESETS.len() - 1)];
        let url = self.client.stream_url(&self.item, &self.source, &self.session, Some((p.bps, p.height)), audio, sub).map_err(|e| e.to_string())?;
        Ok((url, "application/x-mpegURL".into()))
    }

    /// What the quality list shows, and a note on why the original isn't being used when it isn't.
    fn quality_names() -> Vec<String> {
        std::iter::once("Original (no transcoding)".to_string()).chain(PRESETS.iter().map(|p| p.long.to_string())).collect()
    }
}

fn stream_label(s: &emby_api::MediaStream) -> String {
    s.display_title.clone().or_else(|| s.title.clone()).or_else(|| s.language.clone()).unwrap_or_else(|| format!("Track {}", s.index))
}

impl Ctx {
    /// Opens the device picker for `item` (from `start` seconds) and looks for devices.
    pub fn cast_open(&self, item: String, start: f64) {
        if item.is_empty() {
            return;
        }
        *PENDING.lock().unwrap() = Some((item, start));
        TARGETS.lock().unwrap().clear();
        if let Some(ui) = self.ui.upgrade() {
            let g = ui.global::<CastUi>();
            g.set_targets(ModelRc::new(VecModel::<CastTarget>::default()));
            g.set_searching(true);
            g.set_open(true);
        }
        let gen = CAST_GEN.fetch_add(1, Ordering::SeqCst) + 1;
        let session = self.session();
        // Two sources, shown together: apps signed in to the server (fast), and standard Cast devices on the network
        // (Chromecasts that have no apps of their own). Each adds its entries as soon as it has them.
        {
            let ctx = self.clone();
            self.rt.spawn(async move {
                let own = session.as_ref().map(|(c, _)| c.device_id().to_string()).unwrap_or_default();
                let sessions = match &session {
                    Some((c, _)) if c.kind() != emby_api::Kind::Plex => c.sessions().await.unwrap_or_default(),
                    _ => Vec::new(),
                };
                let mut found: Vec<Target> = Vec::new();
                for s in sessions.into_iter().filter(|s| s.supports_remote_control && s.device_id.as_deref() != Some(own.as_str())) {
                    let name = s.device_name.clone().unwrap_or_else(|| "Device".into());
                    // "Emby for Android · Mandy": the app, and the user signed in on it.
                    let who = [s.client.clone(), s.user_name.clone()].into_iter().flatten().filter(|x| !x.is_empty()).collect::<Vec<_>>().join(" · ");
                    found.push(Target::Session { id: s.id, name, who });
                }
                ctx.cast_add(gen, found, false);
            });
        }
        let ctx = self.clone();
        self.rt.spawn(async move {
            platform::multicast_lock(true);
            let devices = gcast::discover(Duration::from_millis(3500)).await;
            platform::multicast_lock(false);
            ctx.cast_add(gen, devices.into_iter().map(Target::Chromecast).collect(), true);
        });
    }

    /// Adds entries to the picker (from either source), unless it has been reopened since. `last` ends "Looking…".
    fn cast_add(&self, gen: u64, add: Vec<Target>, last: bool) {
        if CAST_GEN.load(Ordering::SeqCst) != gen {
            return;
        }
        let rows: Vec<CastTarget> = {
            let mut t = TARGETS.lock().unwrap();
            t.extend(add);
            t.iter().map(|t| CastTarget { name: t.name().into(), kind: t.detail().into() }).collect()
        };
        let ctx = self.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = ctx.ui.upgrade() {
                let g = ui.global::<CastUi>();
                g.set_targets(ModelRc::new(VecModel::from(rows)));
                if last {
                    g.set_searching(false);
                }
            }
        });
    }

    /// The cast button in the player: cast what is playing, from where it is.
    pub fn cast_current(&self) {
        let item = self.now.lock().unwrap().item.clone();
        let pos = self.ui.upgrade().map_or(0.0, |ui| ui.get_player_pos() as f64);
        self.cast_open(item, pos.max(0.0));
    }

    pub fn cast_pick(&self, i: i32) {
        let target = TARGETS.lock().unwrap().get(i.max(0) as usize).cloned();
        let pending = PENDING.lock().unwrap().take();
        let (Some(target), Some((item_id, start))) = (target, pending) else { return };
        let Some((client, uid)) = self.session() else { return };
        if let Some(ui) = self.ui.upgrade() {
            ui.global::<CastUi>().set_open(false);
        }
        let ctx = self.clone();
        self.rt.spawn(async move {
            let name = target.name().to_string();
            let res: Result<(Active, Option<(Info, Vec<String>, Vec<String>, f64, String)>), String> = async {
                match &target {
                    Target::Session { id, .. } => {
                        client.session_play(id, &item_id, ticks(start)).await.map_err(|e| e.to_string())?;
                        Ok((Active::Session { client: client.clone(), id: id.clone() }, None))
                    }
                    Target::Chromecast(dev) => {
                        let prep = prepare(&client, &uid, &item_id).await?;
                        let cast = gcast::start(dev, prep.url.clone(), &prep.content_type, &prep.title, "Casting from Foyer", prep.image.clone(), start).await?;
                        // The device streams straight from the server, so tell the server what is playing: it then shows
                        // the session (and its progress) like any other player, and remembers where you got to.
                        let (c, st) = (client.clone(), cast.status_handle());
                        *IDS.lock().unwrap() = (prep.info.item.clone(), prep.info.source.clone(), prep.info.session.clone());
                        tokio::spawn(async move {
                            let t = |s: f64| (s.max(0.0) * 10_000_000.0) as i64;
                            let (i, so, se) = IDS.lock().unwrap().clone();
                            let _ = c.report_start(&i, &so, &se, t(start), "Transcode").await;
                            loop {
                                tokio::time::sleep(Duration::from_secs(5)).await;
                                let s = *st.lock().unwrap();
                                let (i, so, se) = IDS.lock().unwrap().clone();
                                if s.ended {
                                    let _ = c.report_stopped(&i, &so, &se, t(s.pos), "Transcode").await;
                                    break;
                                }
                                let _ = c.report_progress(&i, &so, &se, t(s.pos), s.paused, "Transcode").await;
                            }
                        });
                        let (info, audio_names, sub_names, duration, title) = (prep.info, prep.audio_names, prep.sub_names, prep.duration, prep.title);
                        Ok((Active::Chromecast(cast), Some((info, audio_names, sub_names, duration, title))))
                    }
                }
            }
            .await;
            let _ = slint::invoke_from_event_loop(move || {
                let Some(ui) = ctx.ui.upgrade() else { return };
                match res {
                    Ok((active, info)) => {
                        *ACTIVE.lock().unwrap() = Some((active, false));
                        let run = RUN.fetch_add(1, Ordering::SeqCst) + 1;
                        let g = ui.global::<CastUi>();
                        g.set_active(name.clone().into());
                        g.set_paused(false);
                        g.set_sheet(false);
                        g.set_position(start as f32);
                        g.set_duration(0.0);
                        g.set_can_control(info.is_some());
                        if let Some((info, an, sn, dur, title)) = info {
                            g.set_duration(dur as f32);
                            g.set_has_prev(info.prev.is_some());
                            g.set_has_next(info.next.is_some());
                            g.set_title(title.clone().into());
                            g.set_quality_names(ModelRc::new(VecModel::from(Info::quality_names().into_iter().map(SharedString::from).collect::<Vec<_>>())));
                            g.set_quality_index(info.quality as i32);
                            g.set_transcode_note(info.must_transcode().then_some("The server is transcoding because of the audio or subtitles chosen.").unwrap_or("").into());
                            g.set_audio_index(info.audio_pick as i32);
                            g.set_sub_index(info.sub_pick as i32);
                            g.set_audio_names(ModelRc::new(VecModel::from(an.into_iter().map(SharedString::from).collect::<Vec<_>>())));
                            g.set_sub_names(ModelRc::new(VecModel::from(sn.into_iter().map(SharedString::from).collect::<Vec<_>>())));
                            *INFO.lock().unwrap() = Some(info);
                            ctx.cast_poll(run);
                        }
                        // Playing here as well would be two copies at once: stop this one.
                        if ui.get_playing() {
                            ctx.send_control(player::Control::Quit);
                        }
                        ctx.show_toast(&ui, format!("Casting to {name}"));
                    }
                    Err(e) => ctx.show_toast(&ui, format!("Couldn't cast to {name}: {e}")),
                }
            });
        });
    }

    /// Keeps the controls in step with the device: where it is, whether it paused, and when it finished.
    fn cast_poll(&self, run: u64) {
        let ctx = self.clone();
        self.rt.spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_secs(1)).await;
                if RUN.load(Ordering::SeqCst) != run {
                    return;
                }
                let st = match ACTIVE.lock().unwrap().as_ref() {
                    Some((Active::Chromecast(c), _)) => c.status(),
                    _ => return,
                };
                let c = ctx.clone();
                let _ = slint::invoke_from_event_loop(move || {
                    if RUN.load(Ordering::SeqCst) != run {
                        return;
                    }
                    let Some(ui) = c.ui.upgrade() else { return };
                    // The device couldn't play the original file: try again as a transcode it can handle.
                    if st.failed {
                        let retry = INFO.lock().unwrap().as_ref().is_some_and(|i| i.quality == 0 && !i.must_transcode());
                        if retry {
                            c.show_toast(&ui, "The device can't play the original file; switching to a transcode.".into());
                            let g = ui.global::<CastUi>();
                            g.set_quality_index(FALLBACK_QUALITY as i32);
                            c.cast_tracks(None, None, Some(FALLBACK_QUALITY));
                            return;
                        }
                    }
                    if st.ended || st.failed {
                        // Finished, or stopped on the TV itself.
                        *ACTIVE.lock().unwrap() = None;
                        *INFO.lock().unwrap() = None;
                        let g = ui.global::<CastUi>();
                        g.set_active("".into());
                        g.set_sheet(false);
                        return;
                    }
                    if let Some((_, p)) = ACTIVE.lock().unwrap().as_mut() {
                        *p = st.paused;
                    }
                    let g = ui.global::<CastUi>();
                    g.set_paused(st.paused);
                    if !g.get_scrubbing() {
                        g.set_position(st.pos as f32);
                    }
                });
            }
        });
    }

    /// Jump to `secs` on the Cast device.
    pub fn cast_seek(&self, secs: f32) {
        if let Some((Active::Chromecast(c), _)) = ACTIVE.lock().unwrap().as_ref() {
            c.send(gcast::Cmd::Seek(secs.max(0.0) as f64));
        }
        if let Some(ui) = self.ui.upgrade() {
            ui.global::<CastUi>().set_position(secs.max(0.0));
        }
    }

    /// Back (-1) or forward (1) by the same amounts as the player's skip buttons.
    pub fn cast_skip(&self, dir: i32) {
        let (rew, fwd) = {
            let st = self.st.lock().unwrap();
            (st.cfg.rewind_secs.max(1) as f32, st.cfg.forward_secs.max(1) as f32)
        };
        let Some(ui) = self.ui.upgrade() else { return };
        let g = ui.global::<CastUi>();
        let target = g.get_position() + if dir < 0 { -rew } else { fwd };
        let dur = g.get_duration();
        self.cast_seek(if dur > 0.0 { target.min(dur - 1.0).max(0.0) } else { target.max(0.0) });
    }

    /// Restarts the stream, at the same spot, with another audio track (`audio`), subtitle (`sub`, 0 = off) or
    /// quality (`quality`, 0 = original).
    pub fn cast_tracks(&self, audio: Option<usize>, sub: Option<usize>, quality: Option<usize>) {
        let (url, ct, note) = {
            let mut i = INFO.lock().unwrap();
            let Some(info) = i.as_mut() else { return };
            if let Some(a) = audio {
                info.audio_pick = a;
            }
            if let Some(s) = sub {
                info.sub_pick = s;
            }
            if let Some(q) = quality {
                info.quality = q.min(PRESETS.len());
                *QUALITY.lock().unwrap() = info.quality;
            }
            match info.url() {
                Ok((u, ct)) => {
                    IDS.lock().unwrap().2 = info.session.clone();
                    (u, ct, info.must_transcode())
                }
                Err(_) => return,
            }
        };
        let Some(ui) = self.ui.upgrade() else { return };
        let g = ui.global::<CastUi>();
        g.set_transcode_note(if note { "The server is transcoding because of the audio or subtitles chosen." } else { "" }.into());
        let pos = g.get_position() as f64;
        if let Some((Active::Chromecast(c), _)) = ACTIVE.lock().unwrap().as_ref() {
            c.send(gcast::Cmd::Load(url, ct, pos, None));
        }
    }

    /// The previous (-1) or next (1) episode, on the same Cast device.
    pub fn cast_episode(&self, dir: i32) {
        let target = INFO.lock().unwrap().as_ref().and_then(|i| if dir < 0 { i.prev.clone() } else { i.next.clone() });
        let (Some(id), Some((client, uid))) = (target, self.session()) else { return };
        let ctx = self.clone();
        self.rt.spawn(async move {
            let prep = match prepare(&client, &uid, &id).await {
                Ok(p) => p,
                Err(e) => {
                    let c = ctx.clone();
                    let _ = slint::invoke_from_event_loop(move || {
                        if let Some(ui) = c.ui.upgrade() {
                            c.show_toast(&ui, format!("Couldn't switch episode: {e}"));
                        }
                    });
                    return;
                }
            };
            // The server's view: the old episode stopped where it was, the new one started.
            let t = |s: f64| (s.max(0.0) * 10_000_000.0) as i64;
            let old = IDS.lock().unwrap().clone();
            let pos = match ACTIVE.lock().unwrap().as_ref() {
                Some((Active::Chromecast(c), _)) => c.status().pos,
                _ => return,
            };
            let _ = client.report_stopped(&old.0, &old.1, &old.2, t(pos), "Transcode").await;
            *IDS.lock().unwrap() = (prep.info.item.clone(), prep.info.source.clone(), prep.info.session.clone());
            let _ = client.report_start(&prep.info.item, &prep.info.source, &prep.info.session, 0, "Transcode").await;
            let c = ctx.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some((Active::Chromecast(cast), p)) = ACTIVE.lock().unwrap().as_mut() {
                    cast.send(gcast::Cmd::Load(prep.url.clone(), prep.content_type.clone(), 0.0, Some((prep.title.clone(), prep.image.clone()))));
                    *p = false;
                } else {
                    return;
                }
                let Some(ui) = c.ui.upgrade() else { return };
                let g = ui.global::<CastUi>();
                g.set_paused(false);
                g.set_position(0.0);
                g.set_duration(prep.duration as f32);
                g.set_has_prev(prep.info.prev.is_some());
                g.set_has_next(prep.info.next.is_some());
                g.set_audio_index(prep.info.audio_pick as i32);
                g.set_sub_index(prep.info.sub_pick as i32);
                g.set_audio_names(ModelRc::new(VecModel::from(prep.audio_names.into_iter().map(SharedString::from).collect::<Vec<_>>())));
                g.set_sub_names(ModelRc::new(VecModel::from(prep.sub_names.into_iter().map(SharedString::from).collect::<Vec<_>>())));
                g.set_title(prep.title.clone().into());
                g.set_transcode_note(prep.info.must_transcode().then_some("The server is transcoding because of the audio or subtitles chosen.").unwrap_or("").into());
                *INFO.lock().unwrap() = Some(prep.info);
            });
        });
    }

    pub fn cast_toggle_pause(&self) {
        let paused = {
            let mut a = ACTIVE.lock().unwrap();
            let Some((active, paused)) = a.as_mut() else { return };
            *paused = !*paused;
            match active {
                Active::Chromecast(c) => c.send(if *paused { gcast::Cmd::Pause } else { gcast::Cmd::Play }),
                Active::Session { client, id } => {
                    let (c, id, cmd) = (client.clone(), id.clone(), if *paused { "Pause" } else { "Unpause" });
                    self.rt.spawn(async move {
                        let _ = c.session_playstate(&id, cmd).await;
                    });
                }
            }
            *paused
        };
        if let Some(ui) = self.ui.upgrade() {
            ui.global::<CastUi>().set_paused(paused);
        }
    }

    /// A command from the server's dashboard while a Cast device is playing: pass it on. False if nothing is being cast.
    pub fn cast_remote(&self, ev: &emby_api::remote::RemoteEvent) -> bool {
        use emby_api::remote::RemoteEvent as E;
        let (pos, paused) = {
            let a = ACTIVE.lock().unwrap();
            let Some((Active::Chromecast(c), paused)) = a.as_ref() else { return false };
            (c.status().pos, *paused)
        };
        let ctx = self.clone();
        let (rew, fwd) = {
            let st = self.st.lock().unwrap();
            (st.cfg.rewind_secs.max(1) as f64, st.cfg.forward_secs.max(1) as f64)
        };
        let ev = ev.clone();
        let _ = slint::invoke_from_event_loop(move || {
            let seek = |s: f64| {
                if let Some((Active::Chromecast(c), _)) = ACTIVE.lock().unwrap().as_ref() {
                    c.send(gcast::Cmd::Seek(s.max(0.0)));
                }
            };
            match ev {
                E::Pause if !paused => ctx.cast_toggle_pause(),
                E::Unpause if paused => ctx.cast_toggle_pause(),
                E::PlayPause => ctx.cast_toggle_pause(),
                E::Stop => ctx.cast_stop(),
                E::SeekTicks(t) => seek(t as f64 / TICKS_PER_SEC as f64),
                E::Rewind => seek(pos - rew),
                E::FastForward => seek(pos + fwd),
                _ => {}
            }
        });
        true
    }

    pub fn cast_stop(&self) {
        RUN.fetch_add(1, Ordering::SeqCst);
        *INFO.lock().unwrap() = None;
        match ACTIVE.lock().unwrap().take() {
            Some((Active::Chromecast(c), _)) => c.send(gcast::Cmd::Stop),
            Some((Active::Session { client, id }, _)) => {
                self.rt.spawn(async move {
                    let _ = client.session_playstate(&id, "Stop").await;
                });
            }
            None => {}
        }
        if let Some(ui) = self.ui.upgrade() {
            let g = ui.global::<CastUi>();
            g.set_active("".into());
            g.set_sheet(false);
        }
    }
}
