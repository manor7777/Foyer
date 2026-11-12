//! Seerr (Overseerr / Jellyseerr) integration: discover, request, manage requests, issues.
//! Everything here is behind the "Enable Seerr" setting and stays inert until it is on.

use super::*;
use seerr_api as sr;
use seerr_api::{perm, status, Discover, RequestAction, Seerr};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Clone, Default)]
pub struct TitleInfo {
    pub title: String,
    pub poster: Option<String>,
    pub year: String,
}

/// What the Seerr detail page (or a library item being reported) refers to.
#[derive(Clone, Default)]
pub struct DetailCtx {
    pub kind: String,
    pub tmdb: i64,
    pub media_id: i64,
    pub title: String,
}


// Slint structs with an `image` field can't be sent between threads, so rows are built as plain
// data on the worker and converted on the UI thread.
struct DetailData {
    kind: SharedString,
    id: SharedString,
    title: SharedString,
    meta: SharedString,
    tagline: SharedString,
    overview: SharedString,
    status_text: SharedString,
    status_kind: i32,
    request_label: SharedString,
    can_request: bool,
    can_report: bool,
    in_library: bool,
}

impl DetailData {
    fn into_ui(self) -> SDetail {
        SDetail {
            kind: self.kind,
            id: self.id,
            title: self.title,
            meta: self.meta,
            tagline: self.tagline,
            overview: self.overview,
            backdrop: Image::default(),
            poster: Image::default(),
            status_text: self.status_text,
            status_kind: self.status_kind,
            request_label: self.request_label,
            can_request: self.can_request,
            can_report: self.can_report,
            in_library: self.in_library,
        }
    }
}

struct ReqData {
    id: SharedString,
    title: SharedString,
    subtitle: SharedString,
    status_text: SharedString,
    status_kind: i32,
    requester: SharedString,
    date: SharedString,
    can_act: bool,
    pending: bool,
    available: bool,
    tmdb: SharedString,
    kind: SharedString,
}

impl ReqData {
    fn into_ui(self) -> SReq {
        SReq {
            id: self.id,
            title: self.title,
            subtitle: self.subtitle,
            poster: Image::default(),
            status_text: self.status_text,
            status_kind: self.status_kind,
            requester: self.requester,
            date: self.date,
            can_act: self.can_act,
            pending: self.pending,
            available: self.available,
            tmdb: self.tmdb,
            kind: self.kind,
        }
    }
}

struct IssueData {
    id: SharedString,
    title: SharedString,
    subtitle: SharedString,
    status_text: SharedString,
    open: bool,
}

impl IssueData {
    fn into_ui(self) -> SIssue {
        SIssue { id: self.id, title: self.title, subtitle: self.subtitle, poster: Image::default(), status_text: self.status_text, open: self.open }
    }
}

struct IssueDetailData {
    id: SharedString,
    title: SharedString,
    subtitle: SharedString,
    status_text: SharedString,
    open: bool,
    can_manage: bool,
}

impl IssueDetailData {
    fn into_ui(self) -> SIssueDetail {
        SIssueDetail {
            id: self.id,
            title: self.title,
            subtitle: self.subtitle,
            status_text: self.status_text,
            open: self.open,
            poster: Image::default(),
            can_manage: self.can_manage,
        }
    }
}

/// The Seerr user being edited on the admin screen, as the server last reported them.
#[derive(Clone)]
pub struct UserEdit {
    pub id: i64,
    pub settings: serde_json::Value,
    pub perms: i64,
}

/// Permission bits the editor can switch (admin is shown but never changed here).
const EDIT_BITS: [i64; 7] = [32, 128, 16, 1024, 16384, 4_194_304, 1_048_576];

#[derive(Default)]
pub struct SeerrState {
    pub req_scope: usize,
    pub user_edit: Option<UserEdit>,
    pub client: Option<Seerr>,
    pub user: Option<sr::User>,
    pub req_filter: usize,
    pub req_loaded: usize,
    pub issue_filter: usize,
    pub issue_loaded: usize,
    pub issue_open: i64,
    pub detail: Option<DetailCtx>,
    pub library_id: Option<String>,
    pub titles: HashMap<(String, i64), TitleInfo>,
}

pub const REQ_FILTERS: [&str; 6] = ["all", "pending", "approved", "processing", "available", "unavailable"];
pub const ISSUE_FILTERS: [&str; 3] = ["open", "resolved", "all"];

fn media_badge(info: &Option<sr::MediaInfo>) -> (String, i32) {
    let Some(i) = info else { return (String::new(), 0) };
    match i.status {
        status::MEDIA_AVAILABLE => ("Available".into(), 4),
        status::MEDIA_PARTIAL => ("Partial".into(), 3),
        // "Processing" in Seerr only means the request was handed to Radarr / Sonarr. It is only
        // shown while a download is actually running; otherwise it is simply requested.
        status::MEDIA_PROCESSING => {
            if i.downloads().is_empty() {
                ("Requested".into(), 1)
            } else {
                ("Processing".into(), 2)
            }
        }
        status::MEDIA_PENDING => ("Requested".into(), 1),
        _ => (String::new(), 0),
    }
}

fn media_card(m: &sr::Media) -> CardData {
    let (badge, badge_kind) = media_badge(&m.media_info);
    let year = m.year();
    let what = if m.media_type == "tv" { "Series" } else { "Movie" };
    CardData {
        id: m.id.to_string(),
        kind: m.media_type.clone(),
        title: m.display_title(),
        subtitle: if year.is_empty() { what.to_string() } else { format!("{year} · {what}") },
        progress: 0.0,
        played: false,
        image: m.poster_path.as_ref().map(|p| sr::image_url(p, "w342")),
        current: false,
        next_season: false,
        badge,
        badge_kind,
    }
}

fn nice_date(s: &Option<String>) -> String {
    s.as_deref()
        .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
        .map(|d| d.with_timezone(&chrono::Local).format("%-d %b %Y").to_string())
        .unwrap_or_default()
}

fn runtime_label(mins: i32) -> String {
    if mins >= 60 {
        format!("{}h {}m", mins / 60, mins % 60)
    } else {
        format!("{mins}m")
    }
}

impl Ctx {
    // ---------------------------------------------------------------- session helpers

    fn sr_session(&self) -> Option<(Seerr, sr::User)> {
        let s = self.st.lock().unwrap();
        Some((s.sr.client.clone()?, s.sr.user.clone()?))
    }

    fn sr_notify(&self, header: &str, text: &str) {
        let (header, text) = (header.to_string(), text.to_string());
        let weak = self.ui.clone();
        let _ = slint::invoke_from_event_loop(move || {
            if let Some(ui) = weak.upgrade() {
                ui.set_message_header(header.into());
                ui.set_message_text(text.into());
                ui.set_message_open(true);
            }
        });
    }

    fn sr_apply_user(&self, ui: &AppWindow) {
        let (user, connected) = {
            let s = self.st.lock().unwrap();
            (s.sr.user.clone(), s.sr.client.is_some())
        };
        ui.set_sr_connected(connected);
        if let Some(u) = user {
            ui.set_sr_user(u.name().into());
            ui.set_sr_manage_requests(u.has(perm::MANAGE_REQUESTS));
            ui.set_sr_manage_issues(u.has(perm::MANAGE_ISSUES));
            ui.set_sr_can_report(u.has(perm::CREATE_ISSUES) || u.has(perm::MANAGE_ISSUES));
            ui.set_sr_manage_users(u.has(perm::MANAGE_USERS));
        } else {
            ui.set_sr_user("".into());
            ui.set_sr_manage_requests(false);
            ui.set_sr_manage_issues(false);
            ui.set_sr_can_report(false);
            ui.set_sr_manage_users(false);
        }
    }

    /// Session ended (or never existed): back to the connect screen.
    fn sr_signed_out(&self, message: &str) {
        {
            let mut s = self.st.lock().unwrap();
            s.sr.client = None;
            s.sr.user = None;
            s.cfg.seerr_cookie.clear();
            save_config(&s.cfg);
        }
        if let Some(ui) = self.ui.upgrade() {
            self.sr_apply_user(&ui);
            ui.set_sr_error(message.into());
        }
        self.show_sr_connect();
    }

    fn sr_error(&self, my: u64, e: sr::Error) {
        let ctx = self.clone();
        self.post(my, move |ui| {
            ui.set_busy(false);
            ui.set_sr_connecting(false);
            match e {
                sr::Error::NotSignedIn => ctx.sr_signed_out("Your session expired. Sign in again."),
                other => ctx.sr_notify("Requests", &other.to_string()),
            }
        });
    }

    // ---------------------------------------------------------------- connect / disconnect

    /// Nav entry: straight to Discover when connected, otherwise the connect screen.
    pub fn sr_enter(&self) {
        if !self.st.lock().unwrap().cfg.seerr_enabled {
            return;
        }
        if self.sr_session().is_some() {
            self.navigate(Page::SeerrDiscover, true);
        } else {
            self.navigate(Page::SeerrConnect, true);
        }
    }

    /// Re-establishes a saved session at startup.
    pub fn sr_restore(&self) {
        let (enabled, url, cookie) = {
            let s = self.st.lock().unwrap();
            (s.cfg.seerr_enabled, s.cfg.seerr_url.clone(), s.cfg.seerr_cookie.clone())
        };
        if !enabled || url.is_empty() || cookie.is_empty() {
            return;
        }
        let client = Seerr::new(&url).with_cookie(cookie);
        let ctx = self.clone();
        self.rt.spawn(async move {
            match client.me().await {
                Ok(user) => {
                    let _ = slint::invoke_from_event_loop(move || {
                        {
                            let mut s = ctx.st.lock().unwrap();
                            s.sr.client = Some(client);
                            s.sr.user = Some(user);
                        }
                        if let Some(ui) = ctx.ui.upgrade() {
                            ctx.sr_apply_user(&ui);
                        }
                    });
                }
                Err(sr::Error::NotSignedIn) => {
                    ctx.update_cfg(|c| c.seerr_cookie.clear());
                }
                Err(e) => applog(&format!("seerr restore failed: {e}")),
            }
        });
    }

    pub fn show_sr_connect(&self) {
        let Some(ui) = self.ui.upgrade() else { return };
        let (url, emby) = {
            let s = self.st.lock().unwrap();
            (s.cfg.seerr_url.clone(), s.cfg.server.clone())
        };
        // Suggest the media server's host on Seerr's default port.
        let guess = if url.is_empty() {
            let host = emby.split("://").last().unwrap_or("").split('/').next().unwrap_or("").split(':').next().unwrap_or("").to_string();
            if host.is_empty() { String::new() } else { format!("{host}:5055") }
        } else {
            url
        };
        if ui.get_sr_url().is_empty() {
            ui.set_sr_url(guess.into());
        }
        ui.set_sr_connecting(false);
        ui.set_busy(false);
        ui.set_page("sr-connect".into());
    }

    pub fn sr_connect(&self, url: String, kind: i32, login: String, pass: String) {
        if url.trim().is_empty() || login.trim().is_empty() {
            if let Some(ui) = self.ui.upgrade() {
                ui.set_sr_error("Enter the Seerr address and your sign-in.".into());
            }
            return;
        }
        if let Some(ui) = self.ui.upgrade() {
            ui.set_sr_connecting(true);
            ui.set_sr_error("".into());
        }
        let ctx = self.clone();
        self.rt.spawn(async move {
            let mut client = Seerr::new(&url);
            let res = if kind == 0 { client.login_media_server(login.trim(), &pass).await } else { client.login_local(login.trim(), &pass).await };
            let _ = slint::invoke_from_event_loop(move || {
                let Some(ui) = ctx.ui.upgrade() else { return };
                ui.set_sr_connecting(false);
                match res {
                    Ok(user) => {
                        {
                            let mut s = ctx.st.lock().unwrap();
                            s.cfg.seerr_url = client.base().to_string();
                            s.cfg.seerr_cookie = client.cookie().unwrap_or_default().to_string();
                            save_config(&s.cfg);
                            s.sr.client = Some(client);
                            s.sr.user = Some(user);
                        }
                        ui.set_sr_pass("".into());
                        ctx.sr_apply_user(&ui);
                        // Came here from Settings -> go back there so it shows the signed-in state.
                        let from_settings = {
                            let mut s = ctx.st.lock().unwrap();
                            let n = s.nav.len();
                            let yes = n >= 2 && matches!(s.nav[n - 1], Page::SeerrConnect) && matches!(s.nav[n - 2], Page::Settings);
                            if yes {
                                s.nav.pop();
                            }
                            yes
                        };
                        if from_settings {
                            ctx.navigate(Page::Settings, false);
                        } else {
                            ctx.navigate(Page::SeerrDiscover, true);
                        }
                    }
                    Err(e) => ui.set_sr_error(e.to_string().into()),
                }
            });
        });
    }

    pub fn sr_disconnect(&self) {
        let client = {
            let mut s = self.st.lock().unwrap();
            let c = s.sr.client.take();
            s.sr.user = None;
            s.cfg.seerr_cookie.clear();
            save_config(&s.cfg);
            c
        };
        if let Some(c) = client {
            self.rt.spawn(async move { c.logout().await });
        }
        if let Some(ui) = self.ui.upgrade() {
            self.sr_apply_user(&ui);
        }
    }

    // ---------------------------------------------------------------- discover + search

    pub fn show_sr_discover(&self) {
        let Some((seerr, _)) = self.sr_session() else { return self.show_sr_connect() };
        let Some((client, _)) = self.session() else { return };
        let my = self.begin();
        if let Some(ui) = self.ui.upgrade() {
            ui.set_sr_rows(ModelRc::new(VecModel::<Row>::default()));
            ui.set_sr_cards(ModelRc::new(VecModel::<Card>::default()));
            ui.set_sr_searching(false);
            ui.set_sr_query("".into());
            ui.set_page("sr-discover".into());
        }
        let ctx = self.clone();
        self.rt.spawn(async move {
            let (a, b, c, d, e) = tokio::join!(
                seerr.discover(Discover::Trending, 1),
                seerr.discover(Discover::Movies, 1),
                seerr.discover(Discover::Tv, 1),
                seerr.discover(Discover::UpcomingMovies, 1),
                seerr.discover(Discover::UpcomingTv, 1)
            );
            if let (Err(sr::Error::NotSignedIn), _) = (&a, &b) {
                return ctx.sr_error(my, sr::Error::NotSignedIn);
            }
            let sets: Vec<(&str, Option<sr::Page<sr::Media>>)> =
                vec![("Trending", a.ok()), ("Popular movies", b.ok()), ("Popular series", c.ok()), ("Upcoming movies", d.ok()), ("Upcoming series", e.ok())];
            let mut rows: Vec<(String, Vec<CardData>)> = Vec::new();
            for (title, page) in sets {
                if let Some(p) = page {
                    let cards: Vec<CardData> = p.results.iter().filter(|m| m.media_type == "movie" || m.media_type == "tv").map(media_card).collect();
                    if !cards.is_empty() {
                        rows.push((title.to_string(), cards));
                    }
                }
            }
            if rows.is_empty() {
                return ctx.sr_error(my, sr::Error::Server("Seerr returned nothing to show. Is it set up and connected to a media server?".into()));
            }
            let mut jobs: Vec<(Slot, String)> = Vec::new();
            for (r, (_, cards)) in rows.iter().enumerate() {
                for (i, c) in cards.iter().enumerate() {
                    if let Some(u) = &c.image {
                        jobs.push((Slot::SRow(r, i), u.clone()));
                    }
                }
            }
            ctx.post(my, move |ui| {
                ui.set_busy(false);
                let rows: Vec<Row> = rows.into_iter().map(|(t, cards)| Row { title: t.into(), wide: false, cards: cards_model(&cards) }).collect();
                ui.set_sr_rows(ModelRc::new(VecModel::from(rows)));
            });
            ctx.spawn_images(&client, my, jobs);
        });
    }

    pub fn sr_search(&self, q: String) {
        let q = q.trim().to_string();
        let Some(ui) = self.ui.upgrade() else { return };
        if q.is_empty() {
            ui.set_sr_searching(false);
            return;
        }
        let Some((seerr, _)) = self.sr_session() else { return };
        let Some((client, _)) = self.session() else { return };
        let my = self.begin();
        ui.set_sr_searching(true);
        ui.set_sr_cards(ModelRc::new(VecModel::<Card>::default()));
        let ctx = self.clone();
        self.rt.spawn(async move {
            match seerr.search(&q, 1).await {
                Ok(p) => {
                    let cards: Vec<CardData> = p.results.iter().filter(|m| m.media_type == "movie" || m.media_type == "tv").map(media_card).collect();
                    let jobs = cards.iter().enumerate().filter_map(|(i, c)| c.image.clone().map(|u| (Slot::SCard(i), u))).collect();
                    ctx.post(my, move |ui| {
                        ui.set_busy(false);
                        ui.set_sr_cards(cards_model(&cards));
                    });
                    ctx.spawn_images(&client, my, jobs);
                }
                Err(e) => ctx.sr_error(my, e),
            }
        });
    }

    // ---------------------------------------------------------------- title page

    pub fn show_sr_detail(&self, kind: String, id: i64) {
        let Some((seerr, user)) = self.sr_session() else { return self.show_sr_connect() };
        let Some((client, uid)) = self.session() else { return };
        let my = self.begin();
        if let Some(ui) = self.ui.upgrade() {
            ui.set_sr_detail(SDetail::default());
            ui.set_sr_cast(ModelRc::new(VecModel::<Card>::default()));
            ui.set_sr_seasons(ModelRc::new(VecModel::<SSeason>::default()));
            ui.set_page("sr-detail".into());
        }
        let ctx = self.clone();
        self.rt.spawn(async move {
            let d = if kind == "tv" { seerr.tv(id).await } else { seerr.movie(id).await };
            let d = match d {
                Ok(d) => d,
                Err(e) => return ctx.sr_error(my, e),
            };
            let info = d.media_info.clone().unwrap_or_default();
            // Already in the library? Then the page can open it directly.
            let library_id = if info.status >= status::MEDIA_PARTIAL { client.find_by_tmdb(&uid, &kind, id).await } else { None };

            let year: String = d.release_date.clone().or(d.first_air_date.clone()).unwrap_or_default().chars().take(4).collect();
            let mut meta: Vec<String> = Vec::new();
            if !year.is_empty() {
                meta.push(year);
            }
            if let Some(rt) = d.runtime.filter(|r| *r > 0) {
                meta.push(runtime_label(rt));
            }
            if kind == "tv" {
                if let Some(n) = d.number_of_seasons.filter(|n| *n > 0) {
                    meta.push(format!("{n} season{}", if n == 1 { "" } else { "s" }));
                }
            }
            if let Some(r) = d.vote_average.filter(|r| *r > 0.0) {
                meta.push(format!("★ {r:.1}"));
            }
            if !d.genres.is_empty() {
                meta.push(d.genres.iter().take(3).map(|g| g.name.clone()).collect::<Vec<_>>().join(", "));
            }

            let downloads = info.downloads();
            let (status_text, status_kind): (String, i32) = match info.status {
                status::MEDIA_AVAILABLE => ("Available".into(), 4),
                status::MEDIA_PARTIAL => ("Partially available".into(), 3),
                status::MEDIA_PROCESSING => match sr::downloads_label(&downloads) {
                    Some(l) => (l, 2),
                    None => ("Requested".into(), 1),
                },
                status::MEDIA_PENDING => ("Requested".into(), 1),
                _ => ("Not requested".into(), 0),
            };
            let can_request = user.has(perm::REQUEST) && info.status != status::MEDIA_AVAILABLE;

            // Seasons the person can still ask for.
            let mut seasons: Vec<SSeason> = Vec::new();
            if kind == "tv" {
                for s in d.seasons.iter().filter(|s| s.season_number > 0) {
                    let st = info.seasons.iter().find(|x| x.season_number == s.season_number).map(|x| x.status).unwrap_or(0);
                    let (text, requestable) = match st {
                        status::MEDIA_AVAILABLE => ("Available", false),
                        status::MEDIA_PARTIAL => ("Partially available", true),
                        status::MEDIA_PROCESSING => (
                            if downloads.iter().any(|dl| dl.episode.as_ref().is_some_and(|e| e.season_number == s.season_number)) { "Processing" } else { "Requested" },
                            false,
                        ),
                        status::MEDIA_PENDING => ("Requested", false),
                        _ => ("Not requested", true),
                    };
                    let eps = s.episode_count.map(|n| format!(" · {n} episodes")).unwrap_or_default();
                    seasons.push(SSeason {
                        number: s.season_number,
                        name: format!("{}{eps}", s.name.clone().unwrap_or_else(|| format!("Season {}", s.season_number))).into(),
                        status: text.into(),
                        requestable,
                        selected: requestable,
                    });
                }
            }

            let cast: Vec<CardData> = d
                .credits
                .as_ref()
                .map(|c| c.cast.iter().take(20).collect::<Vec<_>>())
                .unwrap_or_default()
                .into_iter()
                .map(|c| CardData {
                    id: c.id.to_string(),
                    kind: "person".into(),
                    title: c.name.clone(),
                    subtitle: c.character.clone().unwrap_or_default(),
                    progress: 0.0,
                    played: false,
                    image: c.profile_path.as_ref().map(|p| sr::image_url(p, "w185")),
                    current: false,
                    next_season: false,
                    badge: String::new(),
                    badge_kind: 0,
                })
                .collect();

            let mut jobs: Vec<(Slot, String)> = Vec::new();
            if let Some(p) = &d.backdrop_path {
                jobs.push((Slot::SBackdrop, sr::image_url(p, "w1280")));
            }
            if let Some(p) = &d.poster_path {
                jobs.push((Slot::SPoster, sr::image_url(p, "w500")));
            }
            jobs.extend(cast.iter().enumerate().filter_map(|(i, c)| c.image.clone().map(|u| (Slot::SCast(i), u))));

            let title = d.display_title();
            {
                let mut s = ctx.st.lock().unwrap();
                s.sr.detail = Some(DetailCtx { kind: kind.clone(), tmdb: id, media_id: info.id, title: title.clone() });
                s.sr.library_id = library_id.clone();
            }
            let detail = DetailData {
                kind: kind.clone().into(),
                id: id.to_string().into(),
                title: title.into(),
                meta: meta.join("  ·  ").into(),
                tagline: d.tagline.clone().unwrap_or_default().into(),
                overview: d.overview.clone().unwrap_or_default().into(),
                status_text: status_text.into(),
                status_kind,
                request_label: if kind == "tv" { "Request seasons" } else { "Request" }.into(),
                can_request,
                can_report: info.id > 0,
                in_library: library_id.is_some() || info.status == status::MEDIA_AVAILABLE,
            };
            ctx.post(my, move |ui| {
                ui.set_busy(false);
                ui.set_sr_detail(detail.into_ui());
                ui.set_sr_cast(cards_model(&cast));
                ui.set_sr_seasons(ModelRc::new(VecModel::from(seasons)));
            });
            ctx.spawn_images(&client, my, jobs);
        });
    }

    /// Opens a title that is in the library on the connected media server, to play it.
    pub fn open_in_library(&self, kind: String, tmdb: i64) {
        let Some((client, uid)) = self.session() else { return };
        let known = self.st.lock().unwrap().sr.library_id.clone();
        if let (Some(id), true) = (known, self.st.lock().unwrap().sr.detail.as_ref().is_some_and(|d| d.tmdb == tmdb)) {
            self.navigate(Page::Detail { id }, true);
            return;
        }
        let ctx = self.clone();
        self.rt.spawn(async move {
            let found = client.find_by_tmdb(&uid, &kind, tmdb).await;
            let _ = slint::invoke_from_event_loop(move || match found {
                Some(id) => ctx.navigate(Page::Detail { id }, true),
                None => ctx.sr_notify("Not in your library yet", "It's marked available, but your media server hasn't indexed it yet. Try again after its next library scan."),
            });
        });
    }

    // ---------------------------------------------------------------- requesting

    pub fn sr_request_start(&self) {
        let (Some(ui), Some(d)) = (self.ui.upgrade(), self.st.lock().unwrap().sr.detail.clone()) else { return };
        if d.kind == "tv" {
            ui.set_sr_request_open(true);
        } else {
            self.sr_request_submit();
        }
    }

    pub fn sr_season_toggled(&self, i: i32, v: bool) {
        if let Some(ui) = self.ui.upgrade() {
            let m = ui.get_sr_seasons();
            if let Some(mut s) = m.row_data(i.max(0) as usize) {
                if s.requestable {
                    s.selected = v;
                    m.set_row_data(i as usize, s);
                }
            }
        }
    }

    pub fn sr_request_submit(&self) {
        let Some((seerr, _)) = self.sr_session() else { return };
        let Some(d) = self.st.lock().unwrap().sr.detail.clone() else { return };
        let Some(ui) = self.ui.upgrade() else { return };
        let seasons: Option<Vec<i32>> = if d.kind == "tv" {
            let picked: Vec<i32> = ui.get_sr_seasons().iter().filter(|s| s.selected && s.requestable).map(|s| s.number).collect();
            if picked.is_empty() {
                self.sr_notify("Pick a season", "Select at least one season to request.");
                return;
            }
            Some(picked)
        } else {
            None
        };
        ui.set_sr_request_open(false);
        ui.set_sr_busy(true);
        let ctx = self.clone();
        self.rt.spawn(async move {
            let res = seerr.request(&d.kind, d.tmdb, seasons, false).await;
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = ctx.ui.upgrade() {
                    ui.set_sr_busy(false);
                }
                match res {
                    Ok(()) => {
                        ctx.sr_notify("Requested", &format!("{} has been requested.", d.title));
                        ctx.show_sr_detail(d.kind.clone(), d.tmdb);
                    }
                    Err(e) => ctx.sr_notify("Couldn't request", &e.to_string()),
                }
            });
        });
    }

    // ---------------------------------------------------------------- request list (and admin)

    pub fn show_sr_requests(&self) {
        let Some(_) = self.sr_session() else { return self.show_sr_connect() };
        let filter = self.st.lock().unwrap().sr.req_filter;
        if let Some(ui) = self.ui.upgrade() {
            ui.set_sr_requests(ModelRc::new(VecModel::<SReq>::default()));
            ui.set_sr_req_filter(filter as i32);
            ui.set_sr_req_scope(self.st.lock().unwrap().sr.req_scope as i32);
            ui.set_sr_req_more(false);
            ui.set_page("sr-requests".into());
        }
        self.st.lock().unwrap().sr.req_loaded = 0;
        self.load_sr_requests(false);
    }

    pub fn sr_req_filter(&self, i: i32) {
        self.st.lock().unwrap().sr.req_filter = i.max(0) as usize;
        self.show_sr_requests();
    }

    pub fn load_sr_requests(&self, append: bool) {
        let Some((seerr, user)) = self.sr_session() else { return };
        let Some((client, _)) = self.session() else { return };
        let my = if append { self.gen.load(Ordering::SeqCst) } else { self.begin() };
        let (filter, offset) = {
            let s = self.st.lock().unwrap();
            (REQ_FILTERS[s.sr.req_filter.min(REQ_FILTERS.len() - 1)], if append { s.sr.req_loaded } else { 0 })
        };
        let scope = self.st.lock().unwrap().sr.req_scope;
        let can_see_all = user.has(perm::MANAGE_REQUESTS) || user.has(perm::REQUEST_VIEW);
        let mine = if scope == 1 || !can_see_all { Some(user.id) } else { None };
        dlog(&format!(
            "requests: user {} permissions {} admin={} can_see_all={can_see_all} scope={scope} filter={filter}",
            user.id,
            user.permissions,
            user.is_admin()
        ));
        let scope_note: &'static str = if mine.is_none() {
            "Showing everyone's requests."
        } else if can_see_all {
            "Showing only your requests."
        } else {
            "Showing only your requests: your Seerr account can't see other people's."
        };
        let can_act = user.has(perm::MANAGE_REQUESTS);
        let ctx = self.clone();
        self.rt.spawn(async move {
            let (list, total) = match seerr.requests(filter, 30, offset as u32, mine).await {
                Ok(r) => r,
                Err(e) => return ctx.sr_error(my, e),
            };
            ctx.sr_fill_titles(&seerr, list.iter().filter_map(|r| r.media.as_ref().map(|m| (m.media_type.clone(), m.tmdb_id.unwrap_or(0)))).collect()).await;

            let rows: Vec<(ReqData, Option<String>)> = {
                let s = ctx.st.lock().unwrap();
                list.iter()
                    .map(|r| {
                        let media = r.media.clone().unwrap_or_default();
                        let kind = media.media_type.clone();
                        let tmdb = media.tmdb_id.unwrap_or(0);
                        let t = s.sr.titles.get(&(kind.clone(), tmdb)).cloned().unwrap_or_default();
                        let (stext, skind) = req_status(r);
                        let who = r.requested_by.as_ref().map(|u| u.name()).unwrap_or_default();
                        let seasons = if kind == "tv" && !r.seasons.is_empty() {
                            format!(" · S{}", r.seasons.iter().map(|s| s.season_number.to_string()).collect::<Vec<_>>().join(", S"))
                        } else {
                            String::new()
                        };
                        let when = nice_date(&r.created_at);
                        let row = ReqData {
                            id: r.id.to_string().into(),
                            title: if t.title.is_empty() { format!("{} #{tmdb}", if kind == "tv" { "Series" } else { "Movie" }) } else { t.title.clone() }.into(),
                            subtitle: format!("{who}{}{seasons}{}", if who.is_empty() { "" } else { " · " }, if when.is_empty() { String::new() } else { format!(" · {when}") }).into(),
                            status_text: stext.into(),
                            status_kind: skind,
                            requester: who.into(),
                            date: when.into(),
                            can_act: can_act && r.status == status::REQ_PENDING,
                            pending: r.status == status::REQ_PENDING,
                            available: media.status == status::MEDIA_AVAILABLE || media.status == status::MEDIA_PARTIAL,
                            tmdb: tmdb.to_string().into(),
                            kind: kind.into(),
                        };
                        (row, t.poster.map(|p| sr::image_url(&p, "w185")))
                    })
                    .collect()
            };
            ctx.st.lock().unwrap().sr.req_loaded = offset + rows.len();
            let jobs: Vec<(Slot, String)> = rows.iter().enumerate().filter_map(|(i, (_, p))| p.clone().map(|u| (Slot::SReq(offset + i), u))).collect();
            let loaded = offset + rows.len();
            ctx.post(my, move |ui| {
                ui.set_busy(false);
                ui.set_sr_scope_note(scope_note.into());
                ui.set_sr_req_total(total as i32);
                ui.set_sr_req_more((loaded as i64) < total);
                let new: Vec<SReq> = rows.into_iter().map(|(r, _)| r.into_ui()).collect();
                if append {
                    if let Some(m) = ui.get_sr_requests().as_any().downcast_ref::<VecModel<SReq>>() {
                        for r in new {
                            m.push(r);
                        }
                    }
                } else {
                    ui.set_sr_requests(ModelRc::new(VecModel::from(new)));
                }
            });
            ctx.spawn_images(&client, my, jobs);
        });
    }

    /// Looks up (and caches) titles and posters for a set of (kind, tmdb id).
    async fn sr_fill_titles(&self, seerr: &Seerr, keys: Vec<(String, i64)>) {
        let missing: Vec<(String, i64)> = {
            let s = self.st.lock().unwrap();
            let mut seen = std::collections::HashSet::new();
            keys.into_iter().filter(|k| k.1 > 0 && !s.sr.titles.contains_key(k) && seen.insert(k.clone())).collect()
        };
        let mut tasks = Vec::new();
        for (kind, id) in missing {
            let s = seerr.clone();
            tasks.push(tokio::spawn(async move {
                let d = if kind == "tv" { s.tv(id).await } else { s.movie(id).await };
                (kind, id, d.ok())
            }));
        }
        for t in tasks {
            if let Ok((kind, id, Some(d))) = t.await {
                let year: String = d.release_date.clone().or(d.first_air_date.clone()).unwrap_or_default().chars().take(4).collect();
                let info = TitleInfo {
                    title: d.display_title(),
                    poster: d.poster_path.clone(),
                    year,
                };
                self.st.lock().unwrap().sr.titles.insert((kind, id), info);
            }
        }
    }

    pub fn sr_req_action(&self, id: String, action: String) {
        let Some((seerr, _)) = self.sr_session() else { return };
        let Ok(id) = id.parse::<i64>() else { return };
        let act = match action.as_str() {
            "approve" => RequestAction::Approve,
            "decline" => RequestAction::Decline,
            "retry" => RequestAction::Retry,
            _ => RequestAction::Delete,
        };
        let ctx = self.clone();
        self.rt.spawn(async move {
            let res = seerr.request_action(id, act).await;
            let _ = slint::invoke_from_event_loop(move || match res {
                Ok(()) => ctx.show_sr_requests(),
                Err(e) => ctx.sr_notify("Couldn't update the request", &e.to_string()),
            });
        });
    }

    /// Click on a request / title: take you to it in your library if it's there, else its page.
    pub fn sr_open_target(&self, kind: String, tmdb: String, available: bool) {
        let Ok(id) = tmdb.parse::<i64>() else { return };
        if available {
            self.open_in_library(kind, id);
        } else {
            self.navigate(Page::SeerrDetail { kind, id }, true);
        }
    }

    // ---------------------------------------------------------------- issues

    pub fn show_sr_issues(&self) {
        let Some(_) = self.sr_session() else { return self.show_sr_connect() };
        let filter = self.st.lock().unwrap().sr.issue_filter;
        if let Some(ui) = self.ui.upgrade() {
            ui.set_sr_issues(ModelRc::new(VecModel::<SIssue>::default()));
            ui.set_sr_issue_filter(filter as i32);
            ui.set_sr_issue_more(false);
            ui.set_page("sr-issues".into());
        }
        self.st.lock().unwrap().sr.issue_loaded = 0;
        self.load_sr_issues(false);
    }

    pub fn sr_issue_filter(&self, i: i32) {
        self.st.lock().unwrap().sr.issue_filter = i.max(0) as usize;
        self.show_sr_issues();
    }

    pub fn load_sr_issues(&self, append: bool) {
        let Some((seerr, user)) = self.sr_session() else { return };
        let Some((client, _)) = self.session() else { return };
        let my = if append { self.gen.load(Ordering::SeqCst) } else { self.begin() };
        let (filter, offset) = {
            let s = self.st.lock().unwrap();
            (ISSUE_FILTERS[s.sr.issue_filter.min(ISSUE_FILTERS.len() - 1)], if append { s.sr.issue_loaded } else { 0 })
        };
        let mine = if user.has(perm::MANAGE_ISSUES) || user.has(perm::VIEW_ISSUES) { None } else { Some(user.id) };
        let ctx = self.clone();
        self.rt.spawn(async move {
            let (list, total) = match seerr.issues(filter, 20, offset as u32, mine).await {
                Ok(r) => r,
                Err(e) => return ctx.sr_error(my, e),
            };
            ctx.sr_fill_titles(&seerr, list.iter().filter_map(|i| i.media.as_ref().map(|m| (m.media_type.clone(), m.tmdb_id.unwrap_or(0)))).collect()).await;
            let rows: Vec<(IssueData, Option<String>)> = {
                let s = ctx.st.lock().unwrap();
                list.iter()
                    .map(|i| {
                        let media = i.media.clone().unwrap_or_default();
                        let t = s.sr.titles.get(&(media.media_type.clone(), media.tmdb_id.unwrap_or(0))).cloned().unwrap_or_default();
                        let who = i.created_by.as_ref().map(|u| u.name()).unwrap_or_default();
                        let place = if i.problem_season > 0 {
                            format!(" · S{}{}", i.problem_season, if i.problem_episode > 0 { format!("E{}", i.problem_episode) } else { String::new() })
                        } else {
                            String::new()
                        };
                        let row = IssueData {
                            id: i.id.to_string().into(),
                            title: if t.title.is_empty() { format!("Issue #{}", i.id) } else { t.title.clone() }.into(),
                            subtitle: format!("{}{place} · by {who} · {}", i.type_label(), nice_date(&i.created_at)).into(),
                            status_text: if i.status == status::ISSUE_RESOLVED { "Resolved" } else { "Open" }.into(),
                            open: i.status != status::ISSUE_RESOLVED,
                        };
                        (row, t.poster.map(|p| sr::image_url(&p, "w185")))
                    })
                    .collect()
            };
            ctx.st.lock().unwrap().sr.issue_loaded = offset + rows.len();
            let loaded = offset + rows.len();
            let jobs: Vec<(Slot, String)> = rows.iter().enumerate().filter_map(|(i, (_, p))| p.clone().map(|u| (Slot::SIssue(offset + i), u))).collect();
            ctx.post(my, move |ui| {
                ui.set_busy(false);
                ui.set_sr_issue_more((loaded as i64) < total);
                let new: Vec<SIssue> = rows.into_iter().map(|(r, _)| r.into_ui()).collect();
                if append {
                    if let Some(m) = ui.get_sr_issues().as_any().downcast_ref::<VecModel<SIssue>>() {
                        for r in new {
                            m.push(r);
                        }
                    }
                } else {
                    ui.set_sr_issues(ModelRc::new(VecModel::from(new)));
                }
            });
            ctx.spawn_images(&client, my, jobs);
        });
    }

    pub fn show_sr_issue(&self, id: i64) {
        let Some((seerr, user)) = self.sr_session() else { return self.show_sr_connect() };
        let Some((client, _)) = self.session() else { return };
        let my = self.begin();
        self.st.lock().unwrap().sr.issue_open = id;
        if let Some(ui) = self.ui.upgrade() {
            ui.set_sr_comments(ModelRc::new(VecModel::<SComment>::default()));
            ui.set_page("sr-issue".into());
        }
        let ctx = self.clone();
        self.rt.spawn(async move {
            let issue = match seerr.issue(id).await {
                Ok(i) => i,
                Err(e) => return ctx.sr_error(my, e),
            };
            let media = issue.media.clone().unwrap_or_default();
            ctx.sr_fill_titles(&seerr, vec![(media.media_type.clone(), media.tmdb_id.unwrap_or(0))]).await;
            let t = ctx.st.lock().unwrap().sr.titles.get(&(media.media_type.clone(), media.tmdb_id.unwrap_or(0))).cloned().unwrap_or_default();
            let who = issue.created_by.as_ref().map(|u| u.name()).unwrap_or_default();
            let place = if issue.problem_season > 0 {
                format!(" · S{}{}", issue.problem_season, if issue.problem_episode > 0 { format!("E{}", issue.problem_episode) } else { String::new() })
            } else {
                String::new()
            };
            let comments: Vec<SComment> = issue
                .comments
                .iter()
                .map(|c| SComment { who: c.user.as_ref().map(|u| u.name()).unwrap_or_default().into(), text: c.message.clone().into(), when: nice_date(&c.created_at).into() })
                .collect();
            let open = issue.status != status::ISSUE_RESOLVED;
            let detail = IssueDetailData {
                id: issue.id.to_string().into(),
                title: if t.title.is_empty() { format!("Issue #{}", issue.id) } else { t.title.clone() }.into(),
                subtitle: format!("{}{place} · reported by {who} · {}", issue.type_label(), nice_date(&issue.created_at)).into(),
                status_text: if open { "Open" } else { "Resolved" }.into(),
                open,
                can_manage: user.has(perm::MANAGE_ISSUES),
            };
            let jobs: Vec<(Slot, String)> = t.poster.map(|p| vec![(Slot::SIssuePoster, sr::image_url(&p, "w342"))]).unwrap_or_default();
            ctx.post(my, move |ui| {
                ui.set_busy(false);
                ui.set_sr_issue(detail.into_ui());
                ui.set_sr_comments(ModelRc::new(VecModel::from(comments)));
            });
            ctx.spawn_images(&client, my, jobs);
        });
    }

    pub fn sr_issue_comment(&self, text: String) {
        let text = text.trim().to_string();
        if text.is_empty() {
            return;
        }
        let Some((seerr, _)) = self.sr_session() else { return };
        let id = self.st.lock().unwrap().sr.issue_open;
        let ctx = self.clone();
        self.rt.spawn(async move {
            let res = seerr.issue_comment(id, &text).await;
            let _ = slint::invoke_from_event_loop(move || match res {
                Ok(()) => ctx.show_sr_issue(id),
                Err(e) => ctx.sr_notify("Couldn't comment", &e.to_string()),
            });
        });
    }

    pub fn sr_issue_resolve(&self, resolved: bool) {
        let Some((seerr, _)) = self.sr_session() else { return };
        let id = self.st.lock().unwrap().sr.issue_open;
        let ctx = self.clone();
        self.rt.spawn(async move {
            let res = seerr.issue_set_resolved(id, resolved).await;
            let _ = slint::invoke_from_event_loop(move || match res {
                Ok(()) => ctx.show_sr_issue(id),
                Err(e) => ctx.sr_notify("Couldn't update the issue", &e.to_string()),
            });
        });
    }

    pub fn sr_issue_delete(&self) {
        let Some((seerr, _)) = self.sr_session() else { return };
        let id = self.st.lock().unwrap().sr.issue_open;
        let ctx = self.clone();
        self.rt.spawn(async move {
            let res = seerr.issue_delete(id).await;
            let _ = slint::invoke_from_event_loop(move || match res {
                Ok(()) => {
                    ctx.go_back();
                    ctx.show_sr_issues();
                }
                Err(e) => ctx.sr_notify("Couldn't delete the issue", &e.to_string()),
            });
        });
    }

    // ---------------------------------------------------------------- reporting a problem

    /// From a Seerr title page.
    pub fn sr_report_start(&self) {
        let (Some(ui), Some(d)) = (self.ui.upgrade(), self.st.lock().unwrap().sr.detail.clone()) else { return };
        if d.media_id <= 0 {
            self.sr_notify("Can't report yet", "Seerr doesn't track this title yet. Request it first, or report it once it's available.");
            return;
        }
        ui.set_sr_report_title(d.title.into());
        ui.set_sr_report_tv(d.kind == "tv");
        ui.set_sr_report_type(0);
        ui.set_sr_report_text("".into());
        ui.set_sr_report_season("".into());
        ui.set_sr_report_episode("".into());
        ui.set_sr_report_open(true);
    }

    /// From an item page in the media library: finds the title in Seerr through its TMDB id.
    pub fn report_from_library(&self) {
        let Some((seerr, _)) = self.sr_session() else {
            self.sr_notify("Requests isn't connected", "Open Requests and sign in to Seerr first.");
            return;
        };
        let Some((client, uid)) = self.session() else { return };
        let Some(ui) = self.ui.upgrade() else { return };
        let id = ui.get_detail().id.to_string();
        let ctx = self.clone();
        self.rt.spawn(async move {
            let fail = |ctx: &Ctx, msg: &str| {
                let (ctx, msg) = (ctx.clone(), msg.to_string());
                let _ = slint::invoke_from_event_loop(move || ctx.sr_notify("Can't report this", &msg));
            };
            let Ok(item) = client.item(&uid, &id).await else { return fail(&ctx, "Couldn't read the item from your media server.") };
            let tmdb_of = |i: &Item| i.provider_ids.iter().find(|(k, _)| k.eq_ignore_ascii_case("tmdb")).and_then(|(_, v)| v.parse::<i64>().ok());
            let (kind, tmdb, season, episode) = match item.kind.as_str() {
                "Movie" => ("movie", tmdb_of(&item), 0, 0),
                "Series" => ("tv", tmdb_of(&item), 0, 0),
                "Episode" => {
                    let series = match &item.series_id {
                        Some(s) => client.item(&uid, s).await.ok(),
                        None => None,
                    };
                    ("tv", series.as_ref().and_then(tmdb_of), item.parent_index_number.unwrap_or(0), item.index_number.unwrap_or(0))
                }
                _ => return fail(&ctx, "Only movies, series and episodes can be reported."),
            };
            let Some(tmdb) = tmdb else { return fail(&ctx, "This item has no TMDB id, so Seerr can't match it.") };
            let d = if kind == "tv" { seerr.tv(tmdb).await } else { seerr.movie(tmdb).await };
            let d = match d {
                Ok(d) => d,
                Err(e) => return fail(&ctx, &e.to_string()),
            };
            let media_id = d.media_info.as_ref().map(|m| m.id).unwrap_or(0);
            if media_id <= 0 {
                return fail(&ctx, "Seerr doesn't know this title yet.");
            }
            let title = d.display_title();
            ctx.st.lock().unwrap().sr.detail = Some(DetailCtx { kind: kind.to_string(), tmdb, media_id, title: title.clone() });
            let ctx2 = ctx.clone();
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = ctx2.ui.upgrade() {
                    ui.set_sr_report_title(title.into());
                    ui.set_sr_report_tv(kind == "tv");
                    ui.set_sr_report_type(0);
                    ui.set_sr_report_text("".into());
                    ui.set_sr_report_season(if season > 0 { season.to_string() } else { String::new() }.into());
                    ui.set_sr_report_episode(if episode > 0 { episode.to_string() } else { String::new() }.into());
                    ui.set_sr_report_open(true);
                }
            });
        });
    }

    pub fn sr_report_submit(&self) {
        let Some((seerr, _)) = self.sr_session() else { return };
        let Some(d) = self.st.lock().unwrap().sr.detail.clone() else { return };
        let Some(ui) = self.ui.upgrade() else { return };
        let text = ui.get_sr_report_text().trim().to_string();
        if text.is_empty() {
            self.sr_notify("Add a description", "Tell the admins what's wrong.");
            return;
        }
        let kind = ui.get_sr_report_type() + 1;
        let season = ui.get_sr_report_season().trim().parse::<i32>().unwrap_or(0).max(0);
        let episode = ui.get_sr_report_episode().trim().parse::<i32>().unwrap_or(0).max(0);
        ui.set_sr_report_open(false);
        ui.set_sr_busy(true);
        let ctx = self.clone();
        self.rt.spawn(async move {
            let res = seerr.create_issue(kind, &text, d.media_id, season, episode).await;
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = ctx.ui.upgrade() {
                    ui.set_sr_busy(false);
                }
                match res {
                    Ok(()) => ctx.sr_notify("Issue reported", &format!("Thanks. The admins can now see your report about {}.", d.title)),
                    Err(e) => ctx.sr_notify("Couldn't report the issue", &e.to_string()),
                }
            });
        });
    }
}

// ====================================================================== home row + notifications

/// (label, tag colour kind) for a request, combining its own status with the media's.
pub fn req_status(r: &sr::Request) -> (String, i32) {
    let media = r.media.clone().unwrap_or_default();
    if r.status == status::REQ_DECLINED {
        return ("Declined".into(), 5);
    }
    match media.status {
        status::MEDIA_AVAILABLE => ("Available".into(), 4),
        status::MEDIA_PARTIAL => ("Partial".into(), 3),
        _ => match sr::downloads_label(&media.downloads()) {
            // e.g. "Processing S01E03": only when something is really downloading
            Some(label) if r.status != status::REQ_PENDING => (label, 2),
            _ => ("Requested".into(), 1),
        },
    }
}

#[derive(Default, Serialize, Deserialize, Clone)]
struct Seen {
    #[serde(default)]
    issues: Vec<i64>,
    #[serde(default)]
    issues_init: bool,
    /// my issue id -> (status, comment count) as last seen
    #[serde(default)]
    my_issues: HashMap<String, (i32, usize)>,
    movies: Vec<i64>,
    /// series tmdb id -> ids of episodes (and "season:N" markers) already announced
    series: HashMap<String, Vec<String>>,
}

fn seen_path() -> PathBuf {
    data_dir().join("seen.json")
}

fn load_seen() -> Seen {
    std::fs::read_to_string(seen_path()).ok().and_then(|s| serde_json::from_str(&s).ok()).unwrap_or_default()
}

fn save_seen(s: &Seen) {
    let _ = std::fs::create_dir_all(data_dir());
    if let Ok(t) = serde_json::to_string(s) {
        let _ = std::fs::write(seen_path(), t);
    }
}

pub const NOTIFY_INTERVALS: [u64; 4] = [5, 10, 30, 60];

/// "S1 E1-E3, E5" style summary of newly arrived episodes.
fn describe_eps(eps: &[&Item]) -> String {
    let mut by_season: std::collections::BTreeMap<i32, Vec<i32>> = std::collections::BTreeMap::new();
    for e in eps {
        by_season.entry(e.parent_index_number.unwrap_or(0)).or_default().push(e.index_number.unwrap_or(0));
    }
    let mut parts = Vec::new();
    for (season, mut list) in by_season {
        list.sort_unstable();
        list.dedup();
        let mut ranges: Vec<String> = Vec::new();
        let mut i = 0;
        while i < list.len() {
            let mut j = i;
            while j + 1 < list.len() && list[j + 1] == list[j] + 1 {
                j += 1;
            }
            ranges.push(if i == j { format!("E{}", list[i]) } else { format!("E{}–E{}", list[i], list[j]) });
            i = j + 1;
        }
        parts.push(format!("S{season} {}", ranges.join(", ")));
    }
    parts.join(" · ")
}

impl Ctx {
    /// Recent requests as cards for the home screen, tagged with their status.
    pub async fn sr_home_cards(&self) -> Vec<CardData> {
        let Some((seerr, user)) = self.sr_session() else { return Vec::new() };
        let Ok((list, _)) = seerr.requests("all", 14, 0, Some(user.id)).await else { return Vec::new() };
        self.sr_fill_titles(&seerr, list.iter().filter_map(|r| r.media.as_ref().map(|m| (m.media_type.clone(), m.tmdb_id.unwrap_or(0)))).collect()).await;
        let s = self.st.lock().unwrap();
        list.iter()
            .filter_map(|r| {
                let media = r.media.as_ref()?;
                let tmdb = media.tmdb_id?;
                let t = s.sr.titles.get(&(media.media_type.clone(), tmdb))?;
                let (badge, badge_kind) = req_status(r);
                let available = media.status == status::MEDIA_AVAILABLE || media.status == status::MEDIA_PARTIAL;
                let what = if media.media_type == "tv" { "Series" } else { "Movie" };
                Some(CardData {
                    id: tmdb.to_string(),
                    kind: format!("seerr:{}:{}", media.media_type, if available { 1 } else { 0 }),
                    title: t.title.clone(),
                    subtitle: if t.year.is_empty() { what.to_string() } else { format!("{} · {what}", t.year) },
                    progress: 0.0,
                    played: false,
                    image: t.poster.as_ref().map(|p| sr::image_url(p, "w342")),
                    current: false,
                    next_season: false,
                    badge: badge.to_string(),
                    badge_kind,
                })
            })
            .collect()
    }

    /// Starts the background check that announces when requested titles show up in the library.
    pub fn sr_start_notifier(&self) {
        let ctx = self.clone();
        self.rt.spawn(async move {
            tokio::time::sleep(Duration::from_secs(25)).await;
            loop {
                let (on, interval) = {
                    let s = ctx.st.lock().unwrap();
                    (s.cfg.seerr_enabled && s.cfg.notify_enabled, NOTIFY_INTERVALS[s.cfg.notify_interval.min(NOTIFY_INTERVALS.len() - 1)])
                };
                if on && ctx.sr_session().is_some() && ctx.session().is_some() {
                    ctx.sr_check_available().await;
                    tokio::time::sleep(Duration::from_secs(interval * 60)).await;
                } else {
                    tokio::time::sleep(Duration::from_secs(30)).await;
                }
            }
        });
    }

    /// Compares what is in the library with what the user has already been told about.
    /// Seerr only announces whole movies / seasons; a new series can have one or two episodes
    /// ready long before that, so this watches the library directly, episode by episode.
    async fn sr_check_available(&self) {
        let Some((seerr, user)) = self.sr_session() else { return };
        let Some((client, uid)) = self.session() else { return };
        let (movies_on, eps_on, mode) = {
            let s = self.st.lock().unwrap();
            (s.cfg.notify_movies, s.cfg.notify_episodes, s.cfg.notify_mode)
        };
        let Ok((reqs, _)) = seerr.requests("all", 50, 0, Some(user.id)).await else { return };
        self.sr_fill_titles(&seerr, reqs.iter().filter_map(|r| r.media.as_ref().map(|m| (m.media_type.clone(), m.tmdb_id.unwrap_or(0)))).collect()).await;
        let mut seen = load_seen();
        let mut notes: Vec<(String, String, Option<String>)> = Vec::new();
        dlog(&format!("notifier: checking {} requests", reqs.len()));

        for r in reqs.iter().filter(|r| r.status == status::REQ_APPROVED) {
            let Some(m) = &r.media else { continue };
            let Some(tmdb) = m.tmdb_id else { continue };
            let title = self.st.lock().unwrap().sr.titles.get(&(m.media_type.clone(), tmdb)).map(|t| t.title.clone()).unwrap_or_else(|| format!("#{tmdb}"));
            // Old requests are recorded silently the first time, so you are not flooded.
            let recent = r
                .created_at
                .as_deref()
                .and_then(|s| chrono::DateTime::parse_from_rfc3339(s).ok())
                .is_some_and(|d| chrono::Utc::now().signed_duration_since(d).num_days() <= 14);

            if m.media_type == "movie" {
                if !movies_on || seen.movies.contains(&tmdb) {
                    continue;
                }
                if let Some(item_id) = client.find_by_tmdb(&uid, "movie", tmdb).await {
                    seen.movies.push(tmdb);
                    if recent {
                        notes.push((format!("{title} is ready"), "Your requested movie is now available to watch.".into(), Some(item_id)));
                    }
                }
            } else if m.media_type == "tv" && eps_on {
                let Some(series_id) = client.find_by_tmdb(&uid, "tv", tmdb).await else { continue };
                let eps = client.series_episodes(&uid, &series_id).await.unwrap_or_default();
                let wanted: Vec<i32> = r.seasons.iter().map(|s| s.season_number).collect();
                let present: Vec<&Item> = eps
                    .iter()
                    .filter(|e| {
                        let sn = e.parent_index_number.unwrap_or(0);
                        sn != 0 && (wanted.is_empty() || wanted.contains(&sn))
                    })
                    .collect();
                let key = tmdb.to_string();
                let known = seen.series.get(&key).cloned();
                let new: Vec<&Item> = present.iter().copied().filter(|e| known.as_ref().is_none_or(|k| !k.contains(&e.id))).collect();
                let baseline = known.is_none() && (!recent || m.status == status::MEDIA_AVAILABLE);
                let mut ids = known.unwrap_or_default();
                ids.extend(new.iter().map(|e| e.id.clone()));
                if !new.is_empty() && !baseline {
                    if mode == 0 {
                        notes.push((format!("New episodes of {title}"), describe_eps(&new), Some(new[0].id.clone())));
                    } else if let Ok(d) = seerr.tv(tmdb).await {
                        // Only when a whole requested season is in.
                        for s in d.seasons.iter().filter(|s| s.season_number > 0 && (wanted.is_empty() || wanted.contains(&s.season_number))) {
                            let have = present.iter().filter(|e| e.parent_index_number == Some(s.season_number)).count() as i32;
                            let marker = format!("season:{}", s.season_number);
                            if s.episode_count.is_some_and(|t| t > 0 && have >= t) && !ids.contains(&marker) {
                                ids.push(marker);
                                let first = present.iter().find(|e| e.parent_index_number == Some(s.season_number)).map(|e| e.id.clone());
                                notes.push((format!("{title}: season {} is ready", s.season_number), format!("All {have} episodes are available."), first));
                            }
                        }
                    }
                }
                seen.series.insert(key, ids);
            }
        }
        self.sr_check_issues(&seerr, &user, &mut seen, &mut notes).await;
        save_seen(&seen);

        // At most four separate notifications per check, then one summary.
        let total = notes.len();
        for (title, body, item) in notes.iter().take(4) {
            self.notify_user(title.clone(), body.clone(), item.clone());
        }
        if total > 4 {
            self.notify_user(format!("{} more titles are ready", total - 4), "Open Requests to see them.".into(), None);
        }
    }


    /// Notices about issues: new reports (for people who manage issues) and replies / resolution
    /// on the issues you reported yourself.
    async fn sr_check_issues(&self, seerr: &Seerr, user: &sr::User, seen: &mut Seen, notes: &mut Vec<(String, String, Option<String>)>) {
        let (mgr_on, mine_on) = {
            let s = self.st.lock().unwrap();
            (s.cfg.notify_issues, s.cfg.notify_my_issues)
        };
        let title_of = |ctx: &Ctx, m: &Option<sr::ReqMedia>| -> String {
            let m = m.clone().unwrap_or_default();
            ctx.st.lock().unwrap().sr.titles.get(&(m.media_type.clone(), m.tmdb_id.unwrap_or(0))).map(|t| t.title.clone()).unwrap_or_else(|| "a title".into())
        };
        let place = |i: &sr::Issue| {
            if i.problem_season > 0 {
                format!(" S{}{}", i.problem_season, if i.problem_episode > 0 { format!("E{}", i.problem_episode) } else { String::new() })
            } else {
                String::new()
            }
        };

        if mgr_on && (user.has(perm::MANAGE_ISSUES) || user.has(perm::VIEW_ISSUES)) {
            if let Ok((list, _)) = seerr.issues("open", 30, 0, None).await {
                self.sr_fill_titles(seerr, list.iter().filter_map(|i| i.media.as_ref().map(|m| (m.media_type.clone(), m.tmdb_id.unwrap_or(0)))).collect()).await;
                let baseline = !seen.issues_init; // the first look is recorded silently
                for i in &list {
                    if seen.issues.contains(&i.id) {
                        continue;
                    }
                    seen.issues.push(i.id);
                    let by_me = i.created_by.as_ref().is_some_and(|u| u.id == user.id);
                    if baseline || by_me {
                        continue;
                    }
                    let who = i.created_by.as_ref().map(|u| u.name()).unwrap_or_else(|| "someone".into());
                    // the first comment is the reporter's description
                    let text = seerr.issue(i.id).await.ok().and_then(|d| d.comments.first().map(|c| c.message.clone())).unwrap_or_default();
                    let snippet: String = text.chars().take(140).collect();
                    notes.push((
                        format!("New issue: {}{}", title_of(self, &i.media), place(i)),
                        format!("{} reported by {who}{}", i.type_label(), if snippet.is_empty() { String::new() } else { format!(": {snippet}") }),
                        Some(format!("issue:{}", i.id)),
                    ));
                }
                seen.issues_init = true;
            }
        }

        if mine_on {
            if let Ok((list, _)) = seerr.issues("all", 30, 0, Some(user.id)).await {
                self.sr_fill_titles(seerr, list.iter().filter_map(|i| i.media.as_ref().map(|m| (m.media_type.clone(), m.tmdb_id.unwrap_or(0)))).collect()).await;
                for i in &list {
                    let key = i.id.to_string();
                    let prev = seen.my_issues.get(&key).copied();
                    // The list may leave comments out, so open issues are read in full.
                    let (count, last_by_other, last_text) = if i.status != status::ISSUE_RESOLVED {
                        match seerr.issue(i.id).await {
                            Ok(d) => (
                                d.comments.len(),
                                d.comments.last().and_then(|c| c.user.as_ref().map(|u| u.id != user.id)).unwrap_or(false),
                                d.comments.last().map(|c| c.message.clone()).unwrap_or_default(),
                            ),
                            Err(_) => (i.comments.len(), false, String::new()),
                        }
                    } else {
                        (i.comments.len(), false, String::new())
                    };
                    if let Some((pstatus, pcount)) = prev {
                        let title = title_of(self, &i.media);
                        if i.status == status::ISSUE_RESOLVED && pstatus != status::ISSUE_RESOLVED {
                            notes.push((format!("Issue resolved: {title}"), "Your reported issue was marked as resolved.".into(), Some(format!("issue:{}", i.id))));
                        } else if count > pcount && last_by_other {
                            let snippet: String = last_text.chars().take(140).collect();
                            notes.push((format!("New reply on your issue: {title}"), snippet, Some(format!("issue:{}", i.id))));
                        }
                    }
                    seen.my_issues.insert(key, (i.status, count));
                }
            }
        }
    }

    // ---------------------------------------------------------------- admin: users

    pub fn sr_req_scope(&self, i: i32) {
        self.st.lock().unwrap().sr.req_scope = i.max(0) as usize;
        self.show_sr_requests();
    }

    pub fn show_sr_users(&self) {
        let Some((seerr, user)) = self.sr_session() else { return self.show_sr_connect() };
        if !user.has(perm::MANAGE_USERS) {
            self.sr_notify("Not allowed", "Your Seerr account doesn't have the Manage Users permission.");
            return;
        }
        let my = self.begin();
        if let Some(ui) = self.ui.upgrade() {
            ui.set_sr_users(ModelRc::new(VecModel::<SUser>::default()));
            ui.set_page("sr-users".into());
        }
        let ctx = self.clone();
        self.rt.spawn(async move {
            let (list, _) = match seerr.users(60, 0).await {
                Ok(r) => r,
                Err(e) => return ctx.sr_error(my, e),
            };
            let quota = |limit: Option<i64>, days: Option<i64>| match limit {
                Some(l) if l > 0 => format!("{l} per {} day{}", days.unwrap_or(7), if days.unwrap_or(7) == 1 { "" } else { "s" }),
                _ => "server default".to_string(),
            };
            let rows: Vec<SUser> = list
                .iter()
                .map(|u| {
                    let name = u.name();
                    SUser {
                        id: u.id.to_string().into(),
                        initial: name.chars().next().map(|c| c.to_uppercase().to_string()).unwrap_or_default().into(),
                        name: name.into(),
                        subtitle: format!(
                            "{} · {} requests · movies {} · series {}",
                            u.email.clone().unwrap_or_default(),
                            u.request_count,
                            quota(u.movie_quota_limit, u.movie_quota_days),
                            quota(u.tv_quota_limit, u.tv_quota_days)
                        )
                        .into(),
                        tag: if u.is_admin() { "Admin" } else { "" }.into(),
                    }
                })
                .collect();
            ctx.post(my, move |ui| {
                ui.set_busy(false);
                ui.set_sr_users(ModelRc::new(VecModel::from(rows)));
            });
        });
    }

    pub fn show_sr_user(&self, id: i64) {
        let Some((seerr, _)) = self.sr_session() else { return self.show_sr_connect() };
        let my = self.begin();
        if let Some(ui) = self.ui.upgrade() {
            ui.set_page("sr-user".into());
        }
        let ctx = self.clone();
        self.rt.spawn(async move {
            let (user, quota, settings, perms) = tokio::join!(seerr.user(id), seerr.user_quota(id), seerr.user_settings(id), seerr.user_permissions(id));
            let user = match user {
                Ok(u) => u,
                Err(e) => return ctx.sr_error(my, e),
            };
            let settings = match settings {
                Ok(s) => s,
                Err(e) => return ctx.sr_error(my, e),
            };
            let perms = perms.unwrap_or(user.permissions);
            let usage = match quota {
                Ok(q) => {
                    let part = |name: &str, p: &sr::QuotaPart| match p.limit {
                        Some(l) if l > 0 => format!("{name}: {} of {l} used in the last {} days", p.used, p.days.unwrap_or(7)),
                        _ => format!("{name}: {} used (no limit)", p.used),
                    };
                    format!("{}\n{}", part("Movies", &q.movie), part("Series", &q.tv))
                }
                Err(_) => "Usage isn't available.".to_string(),
            };
            let num = |k: &str| settings.get(k).and_then(|v| v.as_i64()).map(|n| n.to_string()).unwrap_or_default();
            let (ml, md, tl, td) = (num("movieQuotaLimit"), num("movieQuotaDays"), num("tvQuotaLimit"), num("tvQuotaDays"));
            let name = user.name();
            let email = user.email.clone().unwrap_or_default();
            ctx.st.lock().unwrap().sr.user_edit = Some(UserEdit { id, settings, perms });
            ctx.post(my, move |ui| {
                ui.set_busy(false);
                ui.set_sr_ud_name(name.into());
                ui.set_sr_ud_email(email.into());
                ui.set_sr_ud_usage(usage.into());
                ui.set_sr_q_movie_limit(ml.into());
                ui.set_sr_q_movie_days(md.into());
                ui.set_sr_q_tv_limit(tl.into());
                ui.set_sr_q_tv_days(td.into());
                ui.set_sr_p_admin(perms & perm::ADMIN != 0);
                ui.set_sr_p_request(perms & 32 != 0);
                ui.set_sr_p_auto(perms & 128 != 0);
                ui.set_sr_p_manage_requests(perms & 16 != 0);
                ui.set_sr_p_4k(perms & 1024 != 0);
                ui.set_sr_p_view(perms & 16384 != 0);
                ui.set_sr_p_create_issues(perms & 4_194_304 != 0);
                ui.set_sr_p_manage_issues(perms & 1_048_576 != 0);
            });
        });
    }

    pub fn sr_user_save(&self) {
        let Some((seerr, _)) = self.sr_session() else { return };
        let Some(edit) = self.st.lock().unwrap().sr.user_edit.clone() else { return };
        let Some(ui) = self.ui.upgrade() else { return };
        // blank = server default
        let q = |s: SharedString| -> serde_json::Value {
            match s.trim().parse::<i64>() {
                Ok(n) if n > 0 => serde_json::json!(n),
                _ => serde_json::Value::Null,
            }
        };
        let mut settings = edit.settings.clone();
        if let Some(o) = settings.as_object_mut() {
            o.insert("movieQuotaLimit".into(), q(ui.get_sr_q_movie_limit()));
            o.insert("movieQuotaDays".into(), q(ui.get_sr_q_movie_days()));
            o.insert("tvQuotaLimit".into(), q(ui.get_sr_q_tv_limit()));
            o.insert("tvQuotaDays".into(), q(ui.get_sr_q_tv_days()));
        }
        // Keep every permission bit the editor doesn't show; replace the ones it does.
        let wanted = [
            (32, ui.get_sr_p_request()),
            (128, ui.get_sr_p_auto()),
            (16, ui.get_sr_p_manage_requests()),
            (1024, ui.get_sr_p_4k()),
            (16384, ui.get_sr_p_view()),
            (4_194_304, ui.get_sr_p_create_issues()),
            (1_048_576, ui.get_sr_p_manage_issues()),
        ];
        let mut perms = edit.perms & !EDIT_BITS.iter().fold(0, |a, b| a | b);
        for (bit, on) in wanted {
            if on {
                perms |= bit;
            }
        }
        ui.set_sr_busy(true);
        let ctx = self.clone();
        self.rt.spawn(async move {
            let r1 = seerr.save_user_settings(edit.id, &settings).await;
            let r2 = if edit.perms & perm::ADMIN != 0 { Ok(()) } else { seerr.set_user_permissions(edit.id, perms).await };
            let _ = slint::invoke_from_event_loop(move || {
                if let Some(ui) = ctx.ui.upgrade() {
                    ui.set_sr_busy(false);
                }
                match (r1, r2) {
                    (Ok(()), Ok(())) => {
                        ctx.sr_notify("Saved", "The user's limits and permissions were updated.");
                        ctx.show_sr_user(edit.id);
                    }
                    (Err(e), _) | (_, Err(e)) => ctx.sr_notify("Couldn't save", &e.to_string()),
                }
            });
        });
    }

    /// In-app banner plus (optionally) a Windows notification.
    pub fn notify_user(&self, title: String, body: String, item: Option<String>) {
        dlog(&format!("notification: {title} / {body}"));
        if self.st.lock().unwrap().cfg.notify_windows {
            let (t, b) = (title.clone(), body.clone());
            std::thread::spawn(move || {
                crate::platform::system_notify(&t, &b);
            });
        }
        let ctx = self.clone();
        let _ = slint::invoke_from_event_loop(move || {
            let Some(ui) = ctx.ui.upgrade() else { return };
            ui.set_note_title(title.clone().into());
            ui.set_note_body(body.into());
            ui.set_note_item(item.unwrap_or_default().into());
            ui.set_note_open(true);
            let weak = ui.as_weak();
            slint::Timer::single_shot(Duration::from_secs(12), move || {
                if let Some(ui) = weak.upgrade() {
                    if ui.get_note_title().as_str() == title {
                        ui.set_note_open(false);
                    }
                }
            });
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ep(season: i32, episode: i32) -> Item {
        Item { parent_index_number: Some(season), index_number: Some(episode), ..Default::default() }
    }

    #[test]
    fn summarises_new_episodes_in_ranges() {
        let eps = [ep(1, 1), ep(1, 2), ep(1, 3), ep(1, 5), ep(2, 1)];
        let refs: Vec<&Item> = eps.iter().collect();
        assert_eq!(describe_eps(&refs), "S1 E1–E3, E5 · S2 E1");
        let one = [ep(3, 7)];
        let r: Vec<&Item> = one.iter().collect();
        assert_eq!(describe_eps(&r), "S3 E7");
    }

    #[test]
    fn labels_request_status() {
        let mk = |status, media, downloads: Vec<sr::Download>| sr::Request {
            status,
            media: Some(sr::ReqMedia { status: media, download_status: downloads, ..Default::default() }),
            ..Default::default()
        };
        let dl = vec![sr::Download { episode: Some(sr::DownloadEp { season_number: 1, episode_number: 1 }), ..Default::default() }];
        let lab = |r: &sr::Request| {
            let (t, k) = req_status(r);
            (t, k)
        };
        assert_eq!(lab(&mk(status::REQ_PENDING, 0, vec![])), ("Requested".to_string(), 1));
        assert_eq!(lab(&mk(status::REQ_APPROVED, status::MEDIA_AVAILABLE, vec![])), ("Available".to_string(), 4));
        assert_eq!(lab(&mk(status::REQ_APPROVED, status::MEDIA_PARTIAL, vec![])), ("Partial".to_string(), 3));
        // approved and "processing" with nothing downloading is just requested
        assert_eq!(lab(&mk(status::REQ_APPROVED, status::MEDIA_PROCESSING, vec![])), ("Requested".to_string(), 1));
        // processing only while a download is running, with the episode
        assert_eq!(lab(&mk(status::REQ_APPROVED, status::MEDIA_PROCESSING, dl)), ("Processing S01E01".to_string(), 2));
        assert_eq!(lab(&mk(status::REQ_DECLINED, 0, vec![])), ("Declined".to_string(), 5));
    }
}
