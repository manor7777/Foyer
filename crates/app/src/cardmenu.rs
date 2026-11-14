//! The press-and-hold (touch) / right-click (desktop) menu on a poster. Only offers what Foyer
//! can do: play, mark watched, favourite, follow, remove from Continue Watching, request.

use super::*;
use crate::hide::Hide;

impl Ctx {
    /// The menu opened on a library card: ask the server for the live watched / favourite state
    /// so the labels are right even if the card on screen is stale.
    pub fn card_menu_open(&self, id: String) {
        let Some((client, uid)) = self.session() else { return };
        let weak = self.ui.clone();
        self.rt.spawn(async move {
            let Ok(item) = client.item(&uid, &id).await else { return };
            let u = item.user_data.unwrap_or_default();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = weak.upgrade() {
                    let m = ui.global::<CardMenu>();
                    if m.get_card().id.as_str() == id {
                        m.set_played(u.played);
                        m.set_favorite(u.is_favorite);
                    }
                }
            });
        });
    }

    pub fn card_menu_flag(&self, id: String, favorite: bool, value: bool) {
        let Some((client, uid)) = self.session() else { return };
        self.rt.spawn(async move {
            let res = if favorite { client.set_favorite(&uid, &id, value).await } else { client.set_played(&uid, &id, value).await };
            if let Err(e) = res {
                applog(&format!("card menu {} failed: {e}", if favorite { "favourite" } else { "watched" }));
            }
        });
        // The cards on screen are stale now; reload quietly so ticks and progress catch up.
        let ctx = self.clone();
        self.rt.spawn(async move {
            tokio::time::sleep(Duration::from_millis(600)).await;
            let _ = slint::invoke_from_event_loop(move || ctx.reload_current());
        });
    }

    /// "Remove from Continue Watching": an episode hides its whole series, like the detail page.
    pub fn card_menu_cw_hide(&self, id: String, name: String) {
        let Some((client, uid)) = self.session() else { return };
        let ctx = self.clone();
        self.rt.spawn(async move {
            let h = match client.item(&uid, &id).await {
                Ok(i) if i.kind == "Episode" => match i.series_id.clone() {
                    Some(sid) => Hide { id: sid, name: i.series_name.clone().unwrap_or(name) },
                    None => Hide { id, name },
                },
                _ => Hide { id, name },
            };
            let _ = slint::invoke_from_event_loop(move || {
                ctx.update_cfg(|c| {
                    if !c.cw_hidden.iter().any(|x| x.id == h.id) {
                        c.cw_hidden.push(h);
                    }
                });
                ctx.reload_current();
            });
        });
    }
}
