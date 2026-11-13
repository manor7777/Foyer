//! Notification centre (the log of everything Foyer has announced), "notify me of new episodes"
//! for followed series, and the system tray icon.

use super::*;
use chrono::{Local, TimeZone};

/// One logged notification.
#[derive(Serialize, Deserialize, Clone)]
pub struct NoteRec {
    pub id: u64,
    pub title: String,
    pub body: String,
    /// Library item id, or "issue:N"; empty when there is nowhere to go.
    pub item: String,
    /// Unix seconds.
    pub time: i64,
    pub read: bool,
}

/// A series the user asked to be told about. `known` is every episode id seen so far.
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Follow {
    pub id: String,
    pub name: String,
    pub known: Vec<String>,
}

const MAX_NOTES: usize = 200;

/// Ids of followed series, so cards (built without access to the config) can show a filled bell.
static FOLLOWED: std::sync::Mutex<Vec<String>> = std::sync::Mutex::new(Vec::new());

pub fn sync_followed(list: &[Follow]) {
    *FOLLOWED.lock().unwrap() = list.iter().map(|f| f.id.clone()).collect();
}

pub fn is_followed_id(id: &str) -> bool {
    FOLLOWED.lock().unwrap().iter().any(|x| x == id)
}

fn notes_path() -> PathBuf {
    data_dir().join("notifications.json")
}

pub fn load_notes() -> Vec<NoteRec> {
    std::fs::read_to_string(notes_path()).ok().and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default()
}

fn save_notes(n: &[NoteRec]) {
    let _ = std::fs::create_dir_all(data_dir());
    if let Ok(t) = serde_json::to_string(n) {
        let _ = std::fs::write(notes_path(), t);
    }
}

fn when_label(secs: i64) -> String {
    let Some(t) = Local.timestamp_opt(secs, 0).single() else { return String::new() };
    if t.date_naive() == Local::now().date_naive() {
        format!("Today {}", t.format("%H:%M"))
    } else {
        t.format("%d %b %H:%M").to_string()
    }
}

impl Ctx {
    /// Adds to the log (newest first). Safe from any thread; the UI refreshes itself.
    pub fn record_note(&self, title: &str, body: &str, item: &str) {
        {
            let mut s = self.st.lock().unwrap();
            let now = Local::now();
            let mut id = now.timestamp_millis() as u64;
            while s.notes.iter().any(|n| n.id == id) {
                id += 1;
            }
            s.notes.insert(0, NoteRec { id, title: title.into(), body: body.into(), item: item.into(), time: now.timestamp(), read: false });
            s.notes.truncate(MAX_NOTES);
            save_notes(&s.notes);
        }
        self.refresh_notes();
    }

    pub fn refresh_notes(&self) {
        let ctx = self.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = ctx.ui.upgrade() {
                ctx.apply_notes(&ui);
            }
        });
    }

    pub fn apply_notes(&self, ui: &AppWindow) {
        let s = self.st.lock().unwrap();
        let rows: Vec<NoteRow> = s
            .notes
            .iter()
            .map(|n| NoteRow {
                id: n.id.to_string().into(),
                title: n.title.clone().into(),
                body: n.body.clone().into(),
                when: when_label(n.time).into(),
                item: n.item.clone().into(),
                read: n.read,
            })
            .collect();
        ui.set_unread_count(s.notes.iter().filter(|n| !n.read).count() as i32);
        ui.set_notes(ModelRc::new(VecModel::from(rows)));
    }

    /// Mutates the log on the UI thread, saves it and redraws.
    fn edit_notes(&self, f: impl FnOnce(&mut Vec<NoteRec>)) {
        {
            let mut s = self.st.lock().unwrap();
            f(&mut s.notes);
            save_notes(&s.notes);
        }
        if let Some(ui) = self.ui.upgrade() {
            self.apply_notes(&ui);
        }
    }

    pub fn show_notifications(&self) {
        self.begin();
        if let Some(ui) = self.ui.upgrade() {
            ui.set_busy(false);
            self.apply_notes(&ui);
            ui.set_page("notifications".into());
        }
    }

    pub fn note_mark_read(&self, id: &str) {
        self.edit_notes(|n| n.iter_mut().filter(|x| x.id.to_string() == id).for_each(|x| x.read = true));
    }

    pub fn notes_mark_all_read(&self) {
        self.edit_notes(|n| n.iter_mut().for_each(|x| x.read = true));
    }

    pub fn notes_clear(&self) {
        self.edit_notes(|n| n.clear());
    }

    /// Click on a logged notification (or the in-app toast): mark it read and go to its target.
    pub fn open_note_item(&self, item: &str) {
        if !item.is_empty() {
            self.edit_notes(|n| n.iter_mut().filter(|x| x.item == item).for_each(|x| x.read = true));
        }
        if let Some(issue) = item.strip_prefix("issue:") {
            if let Ok(n) = issue.parse::<i64>() {
                self.navigate(Page::SeerrIssue { id: n }, true);
            }
        } else if !item.is_empty() {
            self.navigate(Page::Detail { id: item.to_string() }, true);
        }
    }

    // ---------------------------------------------------------------- follow a series

    pub fn is_following(&self, series_id: &str) -> bool {
        self.st.lock().unwrap().cfg.followed.iter().any(|f| f.id == series_id)
    }

    /// Starts or stops following a series; returns whether it is now followed.
    fn flip_follow(&self, id: &str, name: &str) -> bool {
        let (id, name) = (id.to_string(), name.to_string());
        if self.is_following(&id) {
            self.update_cfg(|c| c.followed.retain(|f| f.id != id));
            return false;
        }
        // Remember what is already there so only genuinely new episodes are announced.
        self.update_cfg(|c| c.followed.push(Follow { id: id.clone(), name, known: Vec::new() }));
        if let Some((client, uid)) = self.session() {
            let ctx = self.clone();
            self.rt.spawn(async move {
                let ids: Vec<String> = client.series_episodes(&uid, &id).await.unwrap_or_default().into_iter().map(|e| e.id).collect();
                ctx.update_cfg(|c| {
                    if let Some(f) = c.followed.iter_mut().find(|f| f.id == id) {
                        f.known = ids;
                    }
                });
            });
        }
        true
    }

    /// The bell on a series page: start or stop being told about its new episodes.
    pub fn toggle_follow(&self) {
        let Some(ui) = self.ui.upgrade() else { return };
        let d = ui.get_detail();
        if d.kind.as_str() != "Series" {
            return;
        }
        let on = self.flip_follow(d.id.as_str(), d.title.as_str());
        ui.set_following(on);
    }

    /// The bell on a series card in a library or search grid.
    pub fn follow_card(&self, id: String, name: String) {
        let on = self.flip_follow(&id, &name);
        let Some(ui) = self.ui.upgrade() else { return };
        let m = ui.get_cards();
        for i in 0..m.row_count() {
            if let Some(mut c) = m.row_data(i) {
                if c.id.as_str() == id {
                    c.followed = on;
                    m.set_row_data(i, c);
                }
            }
        }
    }

    pub fn start_follow_checker(&self) {
        sync_followed(&self.st.lock().unwrap().cfg.followed);
        let ctx = self.clone();
        self.rt.spawn(async move {
            tokio::time::sleep(Duration::from_secs(60)).await;
            loop {
                if let Some((client, uid)) = ctx.session() {
                    ctx.check_followed(&client, &uid).await;
                }
                tokio::time::sleep(Duration::from_secs(15 * 60)).await;
            }
        });
    }

    async fn check_followed(&self, client: &Client, uid: &str) {
        let (list, on) = {
            let s = self.st.lock().unwrap();
            (s.cfg.followed.clone(), s.cfg.notify_enabled)
        };
        for f in list {
            let Ok(eps) = client.series_episodes(uid, &f.id).await else { continue };
            // A follow that hasn't finished taking its first snapshot is left alone.
            if f.known.is_empty() {
                if !eps.is_empty() {
                    self.update_cfg(|c| {
                        if let Some(x) = c.followed.iter_mut().find(|x| x.id == f.id) {
                            x.known = eps.iter().map(|e| e.id.clone()).collect();
                        }
                    });
                }
                continue;
            }
            let mut new: Vec<&Item> = eps.iter().filter(|e| e.parent_index_number != Some(0) && !f.known.contains(&e.id)).collect();
            if new.is_empty() {
                continue;
            }
            new.sort_by_key(|e| (e.parent_index_number.unwrap_or(0), e.index_number.unwrap_or(0)));
            let first = new[0].id.clone();
            let title = if new.len() == 1 { format!("New episode of {}", f.name) } else { format!("New episodes of {}", f.name) };
            let body = seerr::describe_eps(&new);
            let ids: Vec<String> = new.iter().map(|e| e.id.clone()).collect();
            self.update_cfg(|c| {
                if let Some(x) = c.followed.iter_mut().find(|x| x.id == f.id) {
                    x.known.extend(ids);
                }
            });
            if on {
                self.notify_user(title, body, Some(first));
            }
        }
    }
}

// ---------------------------------------------------------------- tray icon

thread_local! {
    static TRAY: std::cell::RefCell<Option<tray_icon::TrayIcon>> = const { std::cell::RefCell::new(None) };
}

/// Brings the window back from the tray (or from minimised).
pub fn show_window(ui: &AppWindow) {
    use slint::winit_030::WinitWindowAccessor;
    let _ = ui.show();
    ui.window().with_winit_window(|w| {
        w.set_minimized(false);
        w.focus_window();
    });
}

/// Wires the tray's click / menu events to the window. Called once at startup.
pub fn init_tray_events(weak: Weak<AppWindow>) {
    let open = {
        let weak = weak.clone();
        move || {
            let weak = weak.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = weak.upgrade() {
                    show_window(&ui);
                }
            });
        }
    };
    let open2 = open.clone();
    tray_icon::TrayIconEvent::set_event_handler(Some(move |e: tray_icon::TrayIconEvent| {
        if let tray_icon::TrayIconEvent::Click { button: tray_icon::MouseButton::Left, button_state: tray_icon::MouseButtonState::Up, .. } = e {
            open2();
        }
    }));
    tray_icon::menu::MenuEvent::set_event_handler(Some(move |e: tray_icon::menu::MenuEvent| match e.id.0.as_str() {
        "open" => open(),
        "quit" => {
            let _ = slint::invoke_from_event_loop(|| {
                let _ = slint::quit_event_loop();
            });
        }
        _ => {}
    }));
}

/// Shows or removes the tray icon. Must run on the UI thread.
pub fn set_tray(on: bool) {
    TRAY.with(|t| {
        let mut t = t.borrow_mut();
        if !on {
            *t = None;
            return;
        }
        if t.is_some() {
            return;
        }
        use tray_icon::menu::{Menu, MenuItem, PredefinedMenuItem};
        let menu = Menu::new();
        let _ = menu.append_items(&[&MenuItem::with_id("open", "Open Foyer", true, None), &PredefinedMenuItem::separator(), &MenuItem::with_id("quit", "Quit", true, None)]);
        let icon = image::load_from_memory(include_bytes!("../assets/app.png")).ok().and_then(|img| {
            let rgba = img.resize(64, 64, image::imageops::FilterType::Lanczos3).to_rgba8();
            let (w, h) = rgba.dimensions();
            tray_icon::Icon::from_rgba(rgba.into_raw(), w, h).ok()
        });
        let mut b = tray_icon::TrayIconBuilder::new().with_menu(Box::new(menu)).with_menu_on_left_click(false).with_tooltip("Foyer");
        if let Some(i) = icon {
            b = b.with_icon(i);
        }
        match b.build() {
            Ok(icon) => *t = Some(icon),
            Err(e) => applog(&format!("tray icon failed: {e}")),
        }
    });
}
