//! Client for Seerr (Overseerr / Jellyseerr). Signs in with a user's own credentials and keeps
//! the session cookie, so no API key is needed.

use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("couldn't reach the server: {0}")]
    Network(String),
    #[error("{0}")]
    Server(String),
    #[error("not signed in")]
    NotSignedIn,
    #[error("sign-in was refused: check the name and password")]
    BadLogin,
    #[error("you don't have permission to do that")]
    Forbidden,
}

pub type Result<T> = std::result::Result<T, Error>;

static LOG_HOOK: std::sync::OnceLock<fn(&str)> = std::sync::OnceLock::new();

/// Lets the host app receive debug lines (network timings etc.). Cookies/passwords are never logged.
pub fn set_log_hook(f: fn(&str)) {
    let _ = LOG_HOOK.set(f);
}

fn log(msg: &str) {
    if let Some(f) = LOG_HOOK.get() {
        f(msg);
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Network(e.without_url().to_string())
    }
}

// ------------------------------------------------------------------ constants
pub mod status {
    pub const MEDIA_PENDING: i32 = 2;
    pub const MEDIA_PROCESSING: i32 = 3;
    pub const MEDIA_PARTIAL: i32 = 4;
    pub const MEDIA_AVAILABLE: i32 = 5;
    pub const REQ_PENDING: i32 = 1;
    pub const REQ_APPROVED: i32 = 2;
    pub const REQ_DECLINED: i32 = 3;
    pub const ISSUE_OPEN: i32 = 1;
    pub const ISSUE_RESOLVED: i32 = 2;
}

pub mod perm {
    pub const ADMIN: i64 = 2;
    pub const MANAGE_USERS: i64 = 8;
    pub const MANAGE_REQUESTS: i64 = 16;
    pub const REQUEST: i64 = 32;
    pub const AUTO_APPROVE: i64 = 128;
    pub const REQUEST_4K: i64 = 1024;
    pub const REQUEST_VIEW: i64 = 16384;
    pub const MANAGE_ISSUES: i64 = 1_048_576;
    pub const VIEW_ISSUES: i64 = 2_097_152;
    pub const CREATE_ISSUES: i64 = 4_194_304;
}

// ------------------------------------------------------------------ models
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct User {
    pub id: i64,
    pub display_name: Option<String>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub permissions: i64,
    pub request_count: i64,
    pub movie_quota_limit: Option<i64>,
    pub movie_quota_days: Option<i64>,
    pub tv_quota_limit: Option<i64>,
    pub tv_quota_days: Option<i64>,
}

impl User {
    pub fn name(&self) -> String {
        self.display_name.clone().or(self.username.clone()).or(self.email.clone()).unwrap_or_else(|| format!("User {}", self.id))
    }
    pub fn is_admin(&self) -> bool {
        self.permissions & perm::ADMIN != 0
    }
    /// Admins have every permission.
    pub fn has(&self, p: i64) -> bool {
        self.is_admin() || self.permissions & p != 0
    }
}

/// An item Radarr / Sonarr is currently downloading for a request.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Download {
    pub title: Option<String>,
    pub size: f64,
    pub size_left: f64,
    pub status: Option<String>,
    pub episode: Option<DownloadEp>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct DownloadEp {
    pub season_number: i32,
    pub episode_number: i32,
}

/// "Processing S01E03" / "Processing S01E03 +2" / "Processing · 63%". None when nothing is
/// actually downloading (a request can be approved and "processing" for a long time before
/// Radarr / Sonarr starts a download).
pub fn downloads_label(d: &[Download]) -> Option<String> {
    let first = d.first()?;
    if let Some(ep) = d.iter().find_map(|x| x.episode.as_ref()) {
        let more = d.len().saturating_sub(1);
        return Some(format!("Processing S{:02}E{:02}{}", ep.season_number, ep.episode_number, if more > 0 { format!(" +{more}") } else { String::new() }));
    }
    if first.size > 0.0 {
        let pct = ((first.size - first.size_left) / first.size * 100.0).clamp(0.0, 100.0);
        return Some(format!("Processing · {pct:.0}%"));
    }
    Some("Processing".to_string())
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct SeasonStatus {
    pub season_number: i32,
    pub status: i32,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct MediaInfo {
    /// Seerr's own id for this media (needed to report issues).
    pub id: i64,
    pub tmdb_id: Option<i64>,
    pub status: i32,
    pub status4k: i32,
    pub seasons: Vec<SeasonStatus>,
    pub download_status: Vec<Download>,
    pub download_status4k: Vec<Download>,
}

impl MediaInfo {
    /// Everything being downloaded right now, HD and 4K together.
    pub fn downloads(&self) -> Vec<Download> {
        self.download_status.iter().chain(self.download_status4k.iter()).cloned().collect()
    }
}

/// A search / discover result.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Media {
    pub id: i64,
    /// movie | tv | person
    pub media_type: String,
    pub title: Option<String>,
    pub name: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub release_date: Option<String>,
    pub first_air_date: Option<String>,
    pub vote_average: Option<f64>,
    pub vote_count: Option<i64>,
    pub overview: Option<String>,
    pub media_info: Option<MediaInfo>,
}

impl Media {
    pub fn display_title(&self) -> String {
        self.title.clone().or(self.name.clone()).unwrap_or_default()
    }
    pub fn year(&self) -> String {
        self.release_date.clone().or(self.first_air_date.clone()).unwrap_or_default().chars().take(4).collect()
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct WatchItem {
    pub tmdb_id: i64,
    /// movie | tv
    pub media_type: String,
    pub title: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Page<T> {
    pub page: i64,
    pub total_pages: i64,
    pub total_results: i64,
    pub results: Vec<T>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Genre {
    pub name: String,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Season {
    pub season_number: i32,
    pub name: Option<String>,
    pub episode_count: Option<i32>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Cast {
    pub id: i64,
    pub name: String,
    pub character: Option<String>,
    pub profile_path: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Credits {
    pub cast: Vec<Cast>,
}

/// Full movie / TV details.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Detail {
    pub id: i64,
    pub title: Option<String>,
    pub name: Option<String>,
    pub tagline: Option<String>,
    pub overview: Option<String>,
    pub poster_path: Option<String>,
    pub backdrop_path: Option<String>,
    pub release_date: Option<String>,
    pub first_air_date: Option<String>,
    pub vote_average: Option<f64>,
    pub runtime: Option<i32>,
    pub genres: Vec<Genre>,
    pub seasons: Vec<Season>,
    pub number_of_seasons: Option<i32>,
    pub media_info: Option<MediaInfo>,
    pub credits: Option<Credits>,
    /// Whether the signed-in user has this on their Seerr watchlist.
    pub on_user_watchlist: Option<bool>,
}

/// A cast member's profile.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Person {
    pub id: i64,
    pub name: String,
    pub biography: Option<String>,
    pub birthday: Option<String>,
    pub deathday: Option<String>,
    pub place_of_birth: Option<String>,
    pub profile_path: Option<String>,
    pub known_for_department: Option<String>,
}

/// Everything a person has been in (acting credits only; they come back as ordinary movie / TV results).
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct PersonCredits {
    pub cast: Vec<Media>,
}

impl Detail {
    pub fn display_title(&self) -> String {
        self.title.clone().or(self.name.clone()).unwrap_or_default()
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ReqMedia {
    pub id: i64,
    pub tmdb_id: Option<i64>,
    pub media_type: String,
    pub status: i32,
    pub download_status: Vec<Download>,
    pub download_status4k: Vec<Download>,
}

impl ReqMedia {
    pub fn downloads(&self) -> Vec<Download> {
        self.download_status.iter().chain(self.download_status4k.iter()).cloned().collect()
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct ReqSeason {
    pub season_number: i32,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Request {
    pub id: i64,
    pub status: i32,
    pub media: Option<ReqMedia>,
    pub requested_by: Option<User>,
    pub created_at: Option<String>,
    pub seasons: Vec<ReqSeason>,
    pub is4k: bool,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Comment {
    pub id: i64,
    pub message: String,
    pub user: Option<User>,
    pub created_at: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct Issue {
    pub id: i64,
    pub issue_type: i32,
    pub status: i32,
    pub problem_season: i32,
    pub problem_episode: i32,
    pub media: Option<ReqMedia>,
    pub created_by: Option<User>,
    pub comments: Vec<Comment>,
    pub created_at: Option<String>,
}

impl Issue {
    pub fn type_label(&self) -> &'static str {
        match self.issue_type {
            1 => "Video",
            2 => "Audio",
            3 => "Subtitles",
            _ => "Other",
        }
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct PublicSettings {
    pub initialized: bool,
    pub media_server_type: i32,
    pub local_login: bool,
    pub media_server_login: bool,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "camelCase", default)]
pub struct QuotaPart {
    pub days: Option<i64>,
    pub limit: Option<i64>,
    pub used: i64,
    pub remaining: Option<i64>,
    pub restricted: bool,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(default)]
pub struct Quota {
    pub movie: QuotaPart,
    pub tv: QuotaPart,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Discover {
    Trending,
    Movies,
    Tv,
    UpcomingMovies,
    UpcomingTv,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RequestAction {
    Approve,
    Decline,
    Delete,
    Retry,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct IssueBody<'a> {
    issue_type: i32,
    message: &'a str,
    media_id: i64,
    problem_season: i32,
    problem_episode: i32,
}

// ------------------------------------------------------------------ helpers
/// Normalises what a person types into a base URL: adds a scheme, strips `/api/v1` and
/// trailing slashes. Hosts with a port, IP addresses, `localhost` and `.local` names default to
/// http (typical for a LAN install); anything else defaults to https.
pub fn normalize_url(input: &str) -> String {
    let s = input.trim().trim_end_matches('/');
    let s = s.strip_suffix("/api/v1").unwrap_or(s).trim_end_matches('/');
    if s.contains("://") {
        return s.to_string();
    }
    let host = s.split('/').next().unwrap_or(s);
    let hostname = host.split(':').next().unwrap_or(host);
    let has_port = host.rsplit_once(':').is_some_and(|(_, p)| p.parse::<u16>().is_ok());
    let is_ip = hostname.split('.').count() == 4 && hostname.split('.').all(|p| p.parse::<u8>().is_ok());
    let lan = has_port || is_ip || hostname == "localhost" || hostname.ends_with(".local") || !hostname.contains('.');
    format!("{}://{}", if lan { "http" } else { "https" }, s)
}

/// Percent-encoding Seerr's search expects (spaces as %20, not +).
pub fn encode(s: &str) -> String {
    let mut out = String::new();
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || b"-_.~".contains(&b) {
            out.push(b as char);
        } else {
            out.push_str(&format!("%{b:02X}"));
        }
    }
    out
}

/// TMDB image URL (poster / backdrop / profile) at a given width bucket, e.g. "w342".
pub fn image_url(path: &str, size: &str) -> String {
    format!("https://image.tmdb.org/t/p/{size}{path}")
}

// ------------------------------------------------------------------ client
#[derive(Clone)]
pub struct Seerr {
    base: String,
    http: reqwest::Client,
    cookie: Option<String>,
}

impl Seerr {
    pub fn new(base: &str) -> Self {
        Self {
            base: normalize_url(base),
            http: reqwest::Client::builder()
                .connect_timeout(Duration::from_secs(8))
                .timeout(Duration::from_secs(25))
                .user_agent("Foyer")
                .build()
                .expect("http client"),
            cookie: None,
        }
    }

    pub fn base(&self) -> &str {
        &self.base
    }

    pub fn with_cookie(mut self, cookie: impl Into<String>) -> Self {
        self.cookie = Some(cookie.into());
        self
    }

    pub fn cookie(&self) -> Option<&str> {
        self.cookie.as_deref()
    }

    fn url(&self, path: &str) -> String {
        format!("{}/api/v1{}", self.base, path)
    }

    fn req(&self, m: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        let mut r = self.http.request(m, self.url(path)).header("Accept", "application/json");
        if let Some(c) = &self.cookie {
            r = r.header("Cookie", c);
        }
        r
    }

    async fn send(&self, r: reqwest::RequestBuilder) -> Result<reqwest::Response> {
        let started = std::time::Instant::now();
        let resp = r.send().await?;
        let code = resp.status();
        log(&format!("seerr {} -> {} in {} ms", resp.url().path(), code.as_u16(), started.elapsed().as_millis()));
        if code.is_success() {
            return Ok(resp);
        }
        match code.as_u16() {
            401 => Err(Error::NotSignedIn),
            c => {
                let body = resp.text().await.unwrap_or_default();
                let msg = serde_json::from_str::<serde_json::Value>(&body).ok().and_then(|v| v["message"].as_str().map(String::from));
                match (c, msg) {
                    // Seerr's own reason ("No remaining requests", ...) beats a generic "no permission".
                    (403, None) => Err(Error::Forbidden),
                    (_, Some(m)) => Err(Error::Server(m)),
                    (c, None) => Err(Error::Server(format!("the server answered {c}"))),
                }
            }
        }
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str) -> Result<T> {
        let resp = self.send(self.req(reqwest::Method::GET, path)).await?;
        Ok(resp.json().await?)
    }

    async fn post_json<B: Serialize + ?Sized>(&self, path: &str, body: &B) -> Result<()> {
        self.send(self.req(reqwest::Method::POST, path).json(body)).await?;
        Ok(())
    }

    // ---- sign-in
    pub async fn public_settings(&self) -> Result<PublicSettings> {
        self.get("/settings/public").await
    }

    async fn login(&mut self, path: &str, body: serde_json::Value) -> Result<User> {
        let resp = match self.http.post(self.url(path)).json(&body).send().await {
            Ok(r) => r,
            Err(e) => return Err(e.into()),
        };
        let code = resp.status();
        if code.as_u16() == 401 || code.as_u16() == 403 {
            return Err(Error::BadLogin);
        }
        if !code.is_success() {
            let text = resp.text().await.unwrap_or_default();
            let msg = serde_json::from_str::<serde_json::Value>(&text)
                .ok()
                .and_then(|v| v["message"].as_str().map(String::from))
                .unwrap_or_else(|| format!("the server answered {}", code.as_u16()));
            return Err(Error::Server(msg));
        }
        // The session cookie is the credential from here on.
        let cookie = resp
            .headers()
            .get_all(reqwest::header::SET_COOKIE)
            .iter()
            .filter_map(|v| v.to_str().ok())
            .find(|c| c.starts_with("connect.sid="))
            .and_then(|c| c.split(';').next())
            .map(String::from);
        let user: User = resp.json().await?;
        self.cookie = Some(cookie.ok_or_else(|| Error::Server("the server didn't start a session".into()))?);
        Ok(user)
    }

    /// Sign in with the media server account (Emby / Jellyfin) the Seerr instance is linked to.
    pub async fn login_media_server(&mut self, username: &str, password: &str) -> Result<User> {
        self.login("/auth/jellyfin", serde_json::json!({ "username": username, "password": password })).await
    }

    /// Sign in with a local Seerr account (email + password).
    pub async fn login_local(&mut self, email: &str, password: &str) -> Result<User> {
        self.login("/auth/local", serde_json::json!({ "email": email, "password": password })).await
    }

    pub async fn me(&self) -> Result<User> {
        self.get("/auth/me").await
    }

    pub async fn logout(&self) {
        let _ = self.req(reqwest::Method::POST, "/auth/logout").send().await;
    }

    // ---- browsing
    pub async fn search(&self, query: &str, page: u32) -> Result<Page<Media>> {
        self.get(&format!("/search?query={}&page={page}&language=en", encode(query))).await
    }

    pub async fn discover(&self, kind: Discover, page: u32) -> Result<Page<Media>> {
        let p = match kind {
            Discover::Trending => "/discover/trending",
            Discover::Movies => "/discover/movies",
            Discover::Tv => "/discover/tv",
            Discover::UpcomingMovies => "/discover/movies/upcoming",
            Discover::UpcomingTv => "/discover/tv/upcoming",
        };
        self.get(&format!("{p}?page={page}&language=en")).await
    }

    /// A user's Seerr watchlist (title + TMDB id only; posters need a detail lookup).
    pub async fn watchlist(&self, user_id: i64, page: u32) -> Result<Page<WatchItem>> {
        self.get(&format!("/user/{user_id}/watchlist?page={page}")).await
    }

    /// Every page of a user's watchlist (Seerr serves 20 at a time), up to a sane cap.
    pub async fn watchlist_all(&self, user_id: i64) -> Result<Vec<WatchItem>> {
        let mut out = Vec::new();
        let mut page = 1u32;
        loop {
            let p = self.watchlist(user_id, page).await?;
            let done = p.results.is_empty() || i64::from(page) >= p.total_pages || page >= 25;
            out.extend(p.results);
            if done {
                return Ok(out);
            }
            page += 1;
        }
    }

    pub async fn watchlist_add(&self, media_type: &str, tmdb_id: i64, title: &str) -> Result<()> {
        self.post_json("/watchlist", &serde_json::json!({ "tmdbId": tmdb_id, "mediaType": media_type, "title": title })).await
    }

    pub async fn watchlist_remove(&self, media_type: &str, tmdb_id: i64) -> Result<()> {
        self.send(self.req(reqwest::Method::DELETE, &format!("/watchlist/{tmdb_id}?mediaType={media_type}"))).await?;
        Ok(())
    }

    pub async fn person(&self, id: i64) -> Result<Person> {
        self.get(&format!("/person/{id}?language=en")).await
    }

    pub async fn person_credits(&self, id: i64) -> Result<PersonCredits> {
        self.get(&format!("/person/{id}/combined_credits?language=en")).await
    }

    pub async fn movie(&self, tmdb_id: i64) -> Result<Detail> {
        self.get(&format!("/movie/{tmdb_id}?language=en")).await
    }

    pub async fn tv(&self, tmdb_id: i64) -> Result<Detail> {
        self.get(&format!("/tv/{tmdb_id}?language=en")).await
    }

    // ---- requests
    /// `seasons`: None for movies; Some(list) for specific seasons of a show.
    pub async fn request(&self, media_type: &str, tmdb_id: i64, seasons: Option<Vec<i32>>, is4k: bool) -> Result<()> {
        let mut body = serde_json::json!({ "mediaType": media_type, "mediaId": tmdb_id, "is4k": is4k });
        if let Some(s) = seasons {
            body["seasons"] = serde_json::json!(s);
        }
        self.post_json("/request", &body).await
    }

    /// `filter`: all | pending | approved | available | processing | unavailable | failed | declined
    pub async fn requests(&self, filter: &str, take: u32, skip: u32, requested_by: Option<i64>) -> Result<(Vec<Request>, i64)> {
        #[derive(Deserialize, Default)]
        #[serde(rename_all = "camelCase", default)]
        struct Info {
            results: i64,
        }
        #[derive(Deserialize, Default)]
        #[serde(rename_all = "camelCase", default)]
        struct R {
            page_info: Info,
            results: Vec<Request>,
        }
        let mut p = format!("/request?take={take}&skip={skip}&filter={filter}&sort=added");
        if let Some(u) = requested_by {
            p.push_str(&format!("&requestedBy={u}"));
        }
        let r: R = self.get(&p).await?;
        let total = r.page_info.results.max(r.results.len() as i64);
        Ok((r.results, total))
    }

    pub async fn request_action(&self, id: i64, action: RequestAction) -> Result<()> {
        let (m, suffix) = match action {
            RequestAction::Approve => (reqwest::Method::POST, "/approve"),
            RequestAction::Decline => (reqwest::Method::POST, "/decline"),
            RequestAction::Retry => (reqwest::Method::POST, "/retry"),
            RequestAction::Delete => (reqwest::Method::DELETE, ""),
        };
        self.send(self.req(m, &format!("/request/{id}{suffix}"))).await?;
        Ok(())
    }

    // ---- users (needs the Manage Users permission)
    pub async fn users(&self, take: u32, skip: u32) -> Result<(Vec<User>, i64)> {
        #[derive(Deserialize, Default)]
        #[serde(rename_all = "camelCase", default)]
        struct Info {
            results: i64,
        }
        #[derive(Deserialize, Default)]
        #[serde(rename_all = "camelCase", default)]
        struct R {
            page_info: Info,
            results: Vec<User>,
        }
        let r: R = self.get(&format!("/user?take={take}&skip={skip}&sort=created")).await?;
        let total = r.page_info.results.max(r.results.len() as i64);
        Ok((r.results, total))
    }

    pub async fn user(&self, id: i64) -> Result<User> {
        self.get(&format!("/user/{id}")).await
    }

    pub async fn user_quota(&self, id: i64) -> Result<Quota> {
        self.get(&format!("/user/{id}/quota")).await
    }

    /// The user's "general" settings as the server has them (quotas live here).
    pub async fn user_settings(&self, id: i64) -> Result<serde_json::Value> {
        self.get(&format!("/user/{id}/settings/main")).await
    }

    /// Writes back a full settings object (read it first, change the quota fields, send it all).
    pub async fn save_user_settings(&self, id: i64, settings: &serde_json::Value) -> Result<()> {
        self.post_json(&format!("/user/{id}/settings/main"), settings).await
    }

    pub async fn user_permissions(&self, id: i64) -> Result<i64> {
        let v: serde_json::Value = self.get(&format!("/user/{id}/settings/permissions")).await?;
        Ok(v["permissions"].as_i64().unwrap_or(0))
    }

    pub async fn set_user_permissions(&self, id: i64, permissions: i64) -> Result<()> {
        self.post_json(&format!("/user/{id}/settings/permissions"), &serde_json::json!({ "permissions": permissions })).await
    }

    // ---- issues
    /// `filter`: all | open | resolved
    pub async fn issues(&self, filter: &str, take: u32, skip: u32, created_by: Option<i64>) -> Result<(Vec<Issue>, i64)> {
        #[derive(Deserialize, Default)]
        #[serde(rename_all = "camelCase", default)]
        struct Info {
            results: i64,
        }
        #[derive(Deserialize, Default)]
        #[serde(rename_all = "camelCase", default)]
        struct R {
            page_info: Info,
            results: Vec<Issue>,
        }
        let mut p = format!("/issue?take={take}&skip={skip}&filter={filter}&sort=added");
        if let Some(u) = created_by {
            p.push_str(&format!("&createdBy={u}"));
        }
        let r: R = self.get(&p).await?;
        let total = r.page_info.results.max(r.results.len() as i64);
        Ok((r.results, total))
    }

    pub async fn issue(&self, id: i64) -> Result<Issue> {
        self.get(&format!("/issue/{id}")).await
    }

    /// `media_id` is Seerr's own media id (`MediaInfo::id`), not the TMDB id.
    /// `issue_type`: 1 video, 2 audio, 3 subtitles, 4 other.
    pub async fn create_issue(&self, issue_type: i32, message: &str, media_id: i64, season: i32, episode: i32) -> Result<()> {
        self.post_json(
            "/issue",
            &IssueBody { issue_type, message, media_id, problem_season: season, problem_episode: episode },
        )
        .await
    }

    pub async fn issue_comment(&self, id: i64, message: &str) -> Result<()> {
        self.post_json(&format!("/issue/{id}/comment"), &serde_json::json!({ "message": message })).await
    }

    pub async fn issue_set_resolved(&self, id: i64, resolved: bool) -> Result<()> {
        self.send(self.req(reqwest::Method::POST, &format!("/issue/{id}/{}", if resolved { "resolved" } else { "open" }))).await?;
        Ok(())
    }

    pub async fn issue_delete(&self, id: i64) -> Result<()> {
        self.send(self.req(reqwest::Method::DELETE, &format!("/issue/{id}"))).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_addresses() {
        assert_eq!(normalize_url("192.168.1.5:5055"), "http://192.168.1.5:5055");
        assert_eq!(normalize_url("192.168.1.5"), "http://192.168.1.5");
        assert_eq!(normalize_url("nas:5055/"), "http://nas:5055");
        assert_eq!(normalize_url("requests.example.com"), "https://requests.example.com");
        assert_eq!(normalize_url("https://r.example.com/api/v1/"), "https://r.example.com");
        assert_eq!(normalize_url("http://localhost:5055"), "http://localhost:5055");
        assert_eq!(normalize_url("localhost:5055"), "http://localhost:5055");
    }

    #[test]
    fn encodes_queries_with_percent20() {
        assert_eq!(encode("the matrix"), "the%20matrix");
        assert_eq!(encode("a&b"), "a%26b");
    }

    #[test]
    fn processing_only_when_downloading() {
        assert_eq!(downloads_label(&[]), None);
        let ep = |s, e| Download { episode: Some(DownloadEp { season_number: s, episode_number: e }), ..Default::default() };
        assert_eq!(downloads_label(&[ep(1, 3)]).as_deref(), Some("Processing S01E03"));
        assert_eq!(downloads_label(&[ep(1, 3), ep(1, 4), ep(1, 5)]).as_deref(), Some("Processing S01E03 +2"));
        let movie = Download { size: 1000.0, size_left: 370.0, ..Default::default() };
        assert_eq!(downloads_label(&[movie]).as_deref(), Some("Processing · 63%"));
    }

    #[test]
    fn parses_download_status_in_requests() {
        let r: Request = serde_json::from_str(r#"{"id":4,"status":2,"media":{"id":9,"tmdbId":1,"mediaType":"tv","status":3,"downloadStatus":[{"size":100,"sizeLeft":50,"episode":{"seasonNumber":1,"episodeNumber":2}}]}}"#).unwrap();
        let d = r.media.unwrap().downloads();
        assert_eq!(downloads_label(&d).as_deref(), Some("Processing S01E02"));
    }

    #[test]
    fn permissions_follow_admin() {
        let admin = User { permissions: perm::ADMIN, ..Default::default() };
        assert!(admin.has(perm::MANAGE_ISSUES));
        let user = User { permissions: perm::REQUEST, ..Default::default() };
        assert!(user.has(perm::REQUEST) && !user.has(perm::MANAGE_REQUESTS));
    }

    #[test]
    fn parses_search_and_request_json() {
        let p: Page<Media> = serde_json::from_str(r#"{"page":1,"totalPages":2,"totalResults":30,"results":[{"id":603,"mediaType":"movie","title":"The Matrix","posterPath":"/a.jpg","releaseDate":"1999-03-30","mediaInfo":{"id":9,"tmdbId":603,"status":5}}]}"#).unwrap();
        assert_eq!(p.results[0].display_title(), "The Matrix");
        assert_eq!(p.results[0].year(), "1999");
        assert_eq!(p.results[0].media_info.as_ref().unwrap().status, status::MEDIA_AVAILABLE);
        let r: Request = serde_json::from_str(r#"{"id":4,"status":1,"is4k":false,"media":{"id":9,"tmdbId":603,"mediaType":"movie","status":2},"requestedBy":{"id":2,"displayName":"Sam","permissions":32}}"#).unwrap();
        assert_eq!(r.requested_by.unwrap().name(), "Sam");
    }
}
