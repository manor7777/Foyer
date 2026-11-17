//! Casting: send what you are watching to another signed-in app on the server (Emby / Jellyfin
//! "remote control" sessions) or to a Chromecast, from a title's page or from the player.

use super::*;
use foyer_core::gcast;

#[derive(Clone)]
enum Target {
    /// Another app signed in to the same server.
    Session { id: String, name: String },
    Chromecast(gcast::Device),
}

impl Target {
    fn name(&self) -> &str {
        match self {
            Target::Session { name, .. } => name,
            Target::Chromecast(d) => &d.name,
        }
    }
}

/// What the picker is showing, and what it will cast.
static PENDING: Mutex<Option<(String, f64)>> = Mutex::new(None);
static TARGETS: Mutex<Vec<Target>> = Mutex::new(Vec::new());

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
        let ctx = self.clone();
        let session = self.session();
        self.rt.spawn(async move {
            let mut found: Vec<Target> = Vec::new();
            let own = session.as_ref().map(|(c, _)| c.device_id().to_string()).unwrap_or_default();
            let (sessions, devices) = tokio::join!(
                async {
                    match &session {
                        Some((c, _)) if c.kind() != emby_api::Kind::Plex => c.sessions().await.unwrap_or_default(),
                        _ => Vec::new(),
                    }
                },
                gcast::discover(Duration::from_millis(1800))
            );
            for s in sessions.into_iter().filter(|s| s.supports_remote_control && s.device_id.as_deref() != Some(own.as_str())) {
                let name = format!("{} ({})", s.device_name.clone().unwrap_or_else(|| "Device".into()), s.client.clone().unwrap_or_default());
                found.push(Target::Session { id: s.id, name });
            }
            found.extend(devices.into_iter().map(Target::Chromecast));
            let rows: Vec<CastTarget> = found
                .iter()
                .map(|t| CastTarget { name: t.name().into(), kind: if matches!(t, Target::Chromecast(_)) { "Chromecast" } else { "App on your server" }.into() })
                .collect();
            *TARGETS.lock().unwrap() = found;
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = ctx.ui.upgrade() {
                    let g = ui.global::<CastUi>();
                    g.set_targets(ModelRc::new(VecModel::from(rows)));
                    g.set_searching(false);
                }
            });
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
                        let url = client.stream_url(&item.id, &source, &uuid::Uuid::new_v4().simple().to_string(), Some((12_000_000, 1080)), None).map_err(|e| e.to_string())?;
                        let title = match item.kind.as_str() {
                            "Episode" => format!("{} {} {}", item.series_name.clone().unwrap_or_default(), episode_tag(&item), item.name),
                            _ => item.name.clone(),
                        };
                        Ok(Active::Chromecast(gcast::start(dev, url, "application/x-mpegURL", &title, start).await?))
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
