//! Hiding titles: "remove from Continue Watching" (a movie, series or season) and a general
//! hide that drops a title from every list. Both are managed under Settings > Hidden titles.

use super::*;

#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Hide {
    pub id: String,
    pub name: String,
}

/// What the open detail page can hide.
#[derive(Clone, Default)]
pub struct HideTarget {
    pub kind: String,
    pub id: String,
    pub name: String,
    pub series_id: String,
    pub series_name: String,
    pub season_id: String,
    pub season_name: String,
}

/// True when `i` (or its series / season) is hidden; `cw` also applies the Continue Watching list.
pub fn hidden(cfg: &Config, i: &Item, cw: bool) -> bool {
    let hit = |l: &[Hide], ids: &[Option<&String>]| l.iter().any(|h| ids.iter().flatten().any(|x| **x == h.id));
    hit(&cfg.hidden_items, &[Some(&i.id), i.series_id.as_ref()]) || (cw && hit(&cfg.cw_hidden, &[Some(&i.id), i.series_id.as_ref(), i.season_id.as_ref()]))
}

impl Ctx {
    pub fn visible(&self, items: Vec<Item>) -> Vec<Item> {
        let s = self.st.lock().unwrap();
        items.into_iter().filter(|i| !hidden(&s.cfg, i, false)).collect()
    }

    pub fn sync_hidden(&self, ui: &AppWindow) {
        let s = self.st.lock().unwrap();
        let row = |h: &Hide, cw| HideRow { id: h.id.clone().into(), name: h.name.clone().into(), cw };
        let rows: Vec<HideRow> = s.cfg.hidden_items.iter().map(|h| row(h, false)).chain(s.cfg.cw_hidden.iter().map(|h| row(h, true))).collect();
        ui.set_hidden_rows(ModelRc::new(VecModel::from(rows)));
    }

    pub fn hide_item(&self) {
        let Some(t) = self.st.lock().unwrap().hide_target.clone() else { return };
        self.add_hide(false, Hide { id: t.id, name: t.name });
    }

    pub fn cw_hide(&self, season: bool) {
        let Some(t) = self.st.lock().unwrap().hide_target.clone() else { return };
        let h = match (t.kind.as_str(), season) {
            ("Season", _) => Hide { id: t.id, name: format!("{} – {}", t.series_name, t.name) },
            ("Episode", true) => Hide { id: t.season_id, name: format!("{} – {}", t.series_name, t.season_name) },
            ("Episode", false) => Hide { id: t.series_id, name: t.series_name },
            _ => Hide { id: t.id, name: t.name },
        };
        if !h.id.is_empty() {
            self.add_hide(true, h);
        }
    }

    fn add_hide(&self, cw: bool, h: Hide) {
        self.update_cfg(|c| {
            let l = if cw { &mut c.cw_hidden } else { &mut c.hidden_items };
            if !l.iter().any(|x| x.id == h.id) {
                l.push(h);
            }
        });
        self.go_back();
    }

    pub fn unhide(&self, id: &str, cw: bool) {
        self.update_cfg(|c| (if cw { &mut c.cw_hidden } else { &mut c.hidden_items }).retain(|h| h.id != id));
        if let Some(ui) = self.ui.upgrade() {
            self.sync_hidden(&ui);
        }
    }
}
