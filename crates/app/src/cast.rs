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
            let res: Result<Active, String> = async {
                match &target {
                    Target::Session { id, .. } => {
                        client.session_play(id, &item_id, ticks(start)).await.map_err(|e| e.to_string())?;
                        Ok(Active::Session { client: client.clone(), id: id.clone() })
                    }
                    Target::Chromecast(dev) => {
                        let item = client.item(&uid, &item_id).await.map_err(|e| e.to_string())?;
                        let source = item.media_sources.first().map(|s| s.id.clone()).unwrap_or_else(|| item.id.clone());
                        // A Chromecast can't play most original files, so ask the server for a 1080p H.264 stream.
                        let play_session = uuid::Uuid::new_v4().simple().to_string();
                        let url = client.stream_url(&item.id, &source, &play_session, Some((12_000_000, 1080)), None).map_err(|e| e.to_string())?;
                        let title = match item.kind.as_str() {
                            "Episode" => format!("{} {} {}", item.series_name.clone().unwrap_or_default(), episode_tag(&item), item.name),
                            _ => item.name.clone(),
                        };
                        let cast = gcast::start(dev, url, "application/x-mpegURL", &title, start).await?;
                        // The device streams straight from the server, so tell the server what is playing: it then shows
                        // the session (and its progress) like any other player, and remembers where you got to.
                        let (c, st) = (client.clone(), cast.status_handle());
                        let ids = (item.id.clone(), source, play_session);
                        tokio::spawn(async move {
                            let (item, source, sess) = ids;
                            let t = |s: f64| (s.max(0.0) * 10_000_000.0) as i64;
                            let _ = c.report_start(&item, &source, &sess, t(start), "Transcode").await;
                            loop {
                                tokio::time::sleep(Duration::from_secs(5)).await;
                                let s = *st.lock().unwrap();
                                if s.ended {
                                    let _ = c.report_stopped(&item, &source, &sess, t(s.pos), "Transcode").await;
                                    break;
                                }
                                let _ = c.report_progress(&item, &source, &sess, t(s.pos), s.paused, "Transcode").await;
                            }
                        });
                        Ok(Active::Chromecast(cast))
                    }
                }
            }
            .await;
            let _ = slint::invoke_from_event_loop(move || {
                let Some(ui) = ctx.ui.upgrade() else { return };
                match res {
                    Ok(active) => {
                        *ACTIVE.lock().unwrap() = Some((active, false));
                        let g = ui.global::<CastUi>();
                        g.set_active(name.clone().into());
                        g.set_paused(false);
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

    pub fn cast_stop(&self) {
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
            ui.global::<CastUi>().set_active("".into());
        }
    }
}
