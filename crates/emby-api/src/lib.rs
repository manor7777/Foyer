//! Emby REST client: discovery, authentication, browsing, search, playback URLs
//! and playback reporting. Talks only to the user's own server.

pub mod remote;

use serde::{Deserialize, Deserializer};
use std::collections::HashMap;
use std::time::Duration;
use tokio::net::UdpSocket;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("network error: {0}")]
    Http(#[from] reqwest::Error),
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error("not signed in")]
    NoSession,
    #[error("{0}")]
    Other(String),
}

pub type Result<T> = std::result::Result<T, Error>;

static LOG_HOOK: std::sync::OnceLock<fn(&str)> = std::sync::OnceLock::new();

/// Lets the host app receive debug lines (network timings etc.). Secrets are never passed.
pub fn set_log_hook(f: fn(&str)) {
    let _ = LOG_HOOK.set(f);
}

pub(crate) fn log(msg: &str) {
    if let Some(f) = LOG_HOOK.get() {
        f(msg);
    }
}

/// Emby ids arrive as numbers on most endpoints and as strings on others.
fn de_id<'de, D: Deserializer<'de>>(d: D) -> std::result::Result<String, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Id {
        S(String),
        N(i64),
    }
    Ok(match Id::deserialize(d)? {
        Id::S(s) => s,
        Id::N(n) => n.to_string(),
    })
}

fn de_opt_id<'de, D: Deserializer<'de>>(d: D) -> std::result::Result<Option<String>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Id {
        S(String),
        N(i64),
    }
    Ok(Option::<Id>::deserialize(d)?.map(|i| match i {
        Id::S(s) => s,
        Id::N(n) => n.to_string(),
    }))
}

/// "host" / "host:port" / "http://host" -> "http://host:8096" style base URL.
pub fn normalize_server(input: &str) -> String {
    let s = input.trim().trim_end_matches('/');
    let (scheme, rest) = match s.split_once("://") {
        Some((sc, r)) => (sc.to_ascii_lowercase(), r),
        None => ("http".to_string(), s),
    };
    let rest = rest.trim_end_matches("/emby").trim_end_matches('/');
    let host_part = rest.split('/').next().unwrap_or(rest);
    let has_port = host_part.rsplit_once(':').is_some_and(|(_, p)| p.parse::<u16>().is_ok());
    // A bare host without scheme is assumed to be Emby's default http port.
    if !s.contains("://") && !has_port {
        return format!("{scheme}://{rest}:8096");
    }
    format!("{scheme}://{rest}")
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct DiscoveredServer {
    #[serde(deserialize_with = "de_id")]
    pub id: String,
    pub name: String,
    pub address: String,
}

/// LAN discovery: broadcast "who is EmbyServer?" on UDP 7359.
pub async fn discover(timeout: Duration) -> Result<Vec<DiscoveredServer>> {
    let sock = UdpSocket::bind("0.0.0.0:0").await?;
    sock.set_broadcast(true)?;
    sock.send_to(b"who is EmbyServer?", "255.255.255.255:7359").await?;
    let mut found: Vec<DiscoveredServer> = Vec::new();
    let mut buf = [0u8; 4096];
    let _ = tokio::time::timeout(timeout, async {
        while let Ok((n, _)) = sock.recv_from(&mut buf).await {
            if let Ok(s) = serde_json::from_slice::<DiscoveredServer>(&buf[..n]) {
                if !found.iter().any(|f| f.id == s.id) {
                    found.push(s);
                }
            }
        }
    })
    .await;
    Ok(found)
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct User {
    #[serde(deserialize_with = "de_id")]
    pub id: String,
    pub name: String,
    #[serde(default)]
    pub policy: Option<Policy>,
}

impl User {
    pub fn is_admin(&self) -> bool {
        self.policy.as_ref().is_some_and(|p| p.is_administrator)
    }
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct Policy {
    pub is_administrator: bool,
}

// ---- server administration types

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct SystemInfo {
    pub server_name: String,
    pub version: String,
    pub operating_system: Option<String>,
    pub has_pending_restart: bool,
    pub has_update_available: bool,
    pub local_address: Option<String>,
    pub wan_address: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct PlayState {
    pub is_paused: bool,
    pub position_ticks: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct Session {
    pub id: String,
    pub user_name: Option<String>,
    pub client: Option<String>,
    pub device_name: Option<String>,
    pub now_playing_item: Option<Item>,
    pub play_state: Option<PlayState>,
    pub supports_remote_control: bool,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct Task {
    pub id: String,
    pub name: String,
    pub state: String,
    pub category: Option<String>,
    pub current_progress_percentage: Option<f64>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct VirtualFolder {
    pub name: String,
    pub locations: Vec<String>,
    pub collection_type: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct ActivityEntry {
    pub name: String,
    pub short_overview: Option<String>,
    pub date: Option<String>,
    pub severity: Option<String>,
}

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct AuthResult {
    access_token: String,
    user: User,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct UserData {
    pub playback_position_ticks: i64,
    pub played: bool,
    pub played_percentage: Option<f64>,
    pub unplayed_item_count: Option<i32>,
    pub is_favorite: bool,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct MediaSource {
    #[serde(deserialize_with = "de_id")]
    pub id: String,
    pub name: Option<String>,
    pub container: Option<String>,
    pub run_time_ticks: Option<i64>,
    pub direct_stream_url: Option<String>,
    pub transcoding_url: Option<String>,
    pub live_stream_id: Option<String>,
    pub size: Option<i64>,
    pub path: Option<String>,
    pub media_streams: Vec<MediaStream>,
    /// Overall bitrate of the file in bits per second.
    pub bitrate: Option<i64>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct MediaStream {
    pub index: i32,
    #[serde(rename = "Type")]
    pub kind: String,
    pub is_external: bool,
    pub delivery_url: Option<String>,
    pub codec: Option<String>,
    pub language: Option<String>,
    pub display_title: Option<String>,
    pub title: Option<String>,
    pub profile: Option<String>,
    pub width: Option<i32>,
    pub height: Option<i32>,
    pub average_frame_rate: Option<f64>,
    pub bit_rate: Option<i64>,
    pub bit_depth: Option<i32>,
    pub channels: Option<i32>,
    pub channel_layout: Option<String>,
    pub sample_rate: Option<i32>,
    pub video_range: Option<String>,
    pub is_default: bool,
    pub is_forced: bool,
}

/// A subtitle offered by the server's providers (OpenSubtitles etc.).
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct RemoteSubtitle {
    pub id: String,
    pub provider_name: Option<String>,
    pub name: String,
    pub format: Option<String>,
    pub author: Option<String>,
    pub download_count: Option<i64>,
    pub is_hash_match: Option<bool>,
    pub community_rating: Option<f32>,
}

fn encode_segment(s: &str) -> String {
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

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct Person {
    #[serde(deserialize_with = "de_id")]
    pub id: String,
    pub name: String,
    pub role: Option<String>,
    /// Actor, Director, Writer, GuestStar...
    #[serde(rename = "Type")]
    pub kind: Option<String>,
    pub primary_image_tag: Option<String>,
}

#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct Item {
    #[serde(deserialize_with = "de_id")]
    pub id: String,
    pub name: String,
    #[serde(rename = "Type")]
    pub kind: String,
    pub overview: Option<String>,
    pub production_year: Option<i32>,
    pub run_time_ticks: Option<i64>,
    pub community_rating: Option<f32>,
    pub official_rating: Option<String>,
    pub series_name: Option<String>,
    #[serde(deserialize_with = "de_opt_id")]
    pub series_id: Option<String>,
    #[serde(deserialize_with = "de_opt_id")]
    pub season_id: Option<String>,
    pub parent_index_number: Option<i32>,
    pub index_number: Option<i32>,
    pub image_tags: HashMap<String, String>,
    pub backdrop_image_tags: Vec<String>,
    pub user_data: Option<UserData>,
    pub media_sources: Vec<MediaSource>,
    pub genres: Vec<String>,
    pub is_folder: bool,
    pub collection_type: Option<String>,
    pub media_type: Option<String>,
    /// Live TV: channel number, and program start/end (RFC 3339, UTC).
    pub number: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    #[serde(deserialize_with = "de_opt_id")]
    pub channel_id: Option<String>,
    pub channel_name: Option<String>,
    pub episode_title: Option<String>,
    pub people: Vec<Person>,
    pub provider_ids: HashMap<String, String>,
    #[serde(deserialize_with = "de_opt_id")]
    pub parent_backdrop_item_id: Option<String>,
    pub parent_backdrop_image_tags: Vec<String>,
    pub premiere_date: Option<String>,
    #[serde(deserialize_with = "de_opt_id")]
    pub parent_logo_item_id: Option<String>,
    pub parent_logo_image_tag: Option<String>,
    pub chapters: Vec<Chapter>,
}

/// A chapter; the server also stores intro / credits markers as chapters with a `MarkerType`.
#[derive(Debug, Clone, Default, Deserialize)]
#[serde(rename_all = "PascalCase", default)]
pub struct Chapter {
    pub start_position_ticks: i64,
    pub name: Option<String>,
    pub marker_type: Option<String>,
}

impl Chapter {
    fn marker(&self) -> String {
        self.marker_type.clone().or_else(|| self.name.clone()).unwrap_or_default().to_lowercase().replace([' ', '_', '-'], "")
    }
}

impl Item {
    /// (start, end) of the intro in seconds, when the server has marked one.
    pub fn intro(&self) -> Option<(f64, f64)> {
        let at = |m: &str| self.chapters.iter().find(|c| c.marker() == m).map(|c| c.start_position_ticks as f64 / 10_000_000.0);
        let (s, e) = (at("introstart")?, at("introend")?);
        (e > s).then_some((s, e))
    }

    /// Where the end credits begin, in seconds, when the server has marked it.
    pub fn credits_start(&self) -> Option<f64> {
        self.chapters.iter().find(|c| c.marker() == "creditsstart").map(|c| c.start_position_ticks as f64 / 10_000_000.0)
    }

    pub fn is_playable(&self) -> bool {
        !self.is_folder && matches!(self.media_type.as_deref(), Some("Video") | Some("Audio"))
    }

    pub fn resume_ticks(&self) -> i64 {
        self.user_data.as_ref().map_or(0, |u| u.playback_position_ticks)
    }

    /// 0.0..=1.0, only when partially watched.
    pub fn progress(&self) -> f32 {
        let (Some(rt), Some(ud)) = (self.run_time_ticks, &self.user_data) else {
            return 0.0;
        };
        if rt <= 0 || ud.playback_position_ticks <= 0 {
            return 0.0;
        }
        (ud.playback_position_ticks as f64 / rt as f64).clamp(0.0, 1.0) as f32
    }
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ItemList {
    #[serde(default)]
    items: Vec<Item>,
    #[serde(default)]
    total_record_count: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageKind {
    Primary,
    Backdrop,
    Thumb,
    Logo,
}

/// What list views need: enough for cards and the next-episode lookups, without the heavy
/// per-item data (media sources, people) that the item page fetches.
const LIST_FIELDS: &str = "ProductionYear,UserData,RunTimeTicks,SeriesName,SeriesId,SeasonId,ParentIndexNumber,IndexNumber,CommunityRating,OfficialRating,ParentBackdropItemId,ParentBackdropImageTags";

const FIELDS: &str = "Overview,UserData,ProductionYear,RunTimeTicks,SeriesName,SeriesId,SeasonId,ParentIndexNumber,IndexNumber,CommunityRating,OfficialRating,Genres,MediaSources,People,ParentBackdropItemId,ParentBackdropImageTags,PremiereDate,ParentLogoItemId,ParentLogoImageTag,ProviderIds,Chapters";

#[derive(Debug, Clone)]
pub struct LiveStream {
    pub url: String,
    pub source_id: String,
    pub live_stream_id: Option<String>,
}

pub const TICKS_PER_SEC: i64 = 10_000_000;

/// Reported to the server in the auth header (shown on its dashboard).
pub const CLIENT_NAME: &str = "Emby Theater"; // the server only offers remote control to clients it recognises; "Foyer" broke it (0.21.0)
pub const CLIENT_VERSION: &str = env!("CARGO_PKG_VERSION");

/// How to sort/filter a library listing.
#[derive(Debug, Clone, Default)]
pub struct Browse {
    pub sort_by: String,
    pub descending: bool,
    pub genre: Option<String>,
    /// "A".."Z", or "#" for titles starting with a non-letter.
    pub letter: Option<String>,
    /// Free-text search within the library.
    pub search: Option<String>,
    pub start: usize,
    pub limit: usize,
}

#[derive(Clone)]
pub struct Client {
    base: String,
    http: reqwest::Client,
    device_id: String,
    token: Option<String>,
}

impl Client {
    pub fn new(server: &str, device_id: impl Into<String>) -> Self {
        Self {
            base: normalize_server(server),
            http: reqwest::Client::builder()
                .connect_timeout(Duration::from_secs(8))
                .timeout(Duration::from_secs(30))
                .build()
                .expect("http client"),
            device_id: device_id.into(),
            token: None,
        }
    }

    pub fn base(&self) -> &str {
        &self.base
    }

    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    pub fn token(&self) -> Option<&str> {
        self.token.as_deref()
    }

    fn auth_header(&self) -> String {
        let device = std::env::var("COMPUTERNAME").unwrap_or_else(|_| "Windows PC".into());
        let mut h = format!(
            "MediaBrowser Client=\"{CLIENT_NAME}\", Device=\"{device}\", DeviceId=\"{}\", Version=\"{CLIENT_VERSION}\"",
            self.device_id
        );
        if let Some(t) = &self.token {
            h.push_str(&format!(", Token=\"{t}\""));
        }
        h
    }

    fn req(&self, m: reqwest::Method, path: &str) -> reqwest::RequestBuilder {
        let mut r = self
            .http
            .request(m, format!("{}/emby{}", self.base, path))
            .header("X-Emby-Authorization", self.auth_header())
            .header("Accept", "application/json");
        if let Some(t) = &self.token {
            r = r.header("X-Emby-Token", t);
        }
        r
    }

    async fn get<T: serde::de::DeserializeOwned>(&self, path: &str, q: &[(&str, &str)]) -> Result<T> {
        let started = std::time::Instant::now();
        let resp = self.req(reqwest::Method::GET, path).query(q).send().await;
        let keys: Vec<&str> = q.iter().map(|(k, _)| *k).collect();
        match resp {
            Ok(r) => {
                let status = r.status();
                log(&format!("GET {path} [{}] -> {} in {} ms", keys.join(","), status.as_u16(), started.elapsed().as_millis()));
                Ok(r.error_for_status()?.json().await?)
            }
            Err(e) => {
                let msg = e.to_string();
                log(&format!("GET {path} failed after {} ms: {msg}", started.elapsed().as_millis()));
                Err(e.into())
            }
        }
    }

    pub async fn login(&mut self, user: &str, password: &str) -> Result<User> {
        let r: AuthResult = self
            .req(reqwest::Method::POST, "/Users/AuthenticateByName")
            .json(&serde_json::json!({ "Username": user, "Pw": password }))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        self.token = Some(r.access_token);
        Ok(r.user)
    }

    /// Checks a saved token is still accepted.
    pub async fn validate(&self, user_id: &str) -> Result<User> {
        self.get(&format!("/Users/{user_id}"), &[]).await
    }

    /// Tells the server this client can be remote-controlled (enables Pause/Stop/Message
    /// buttons on the dashboard).
    pub async fn post_capabilities(&self) -> Result<()> {
        self.req(reqwest::Method::POST, "/Sessions/Capabilities/Full")
            .json(&serde_json::json!({
                "PlayableMediaTypes": ["Audio", "Video"],
                "SupportedCommands": ["DisplayMessage", "SetVolume", "Mute", "Unmute", "ToggleMute"],
                "SupportsMediaControl": true,
                "SupportsPersistentIdentifier": true,
            }))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub fn websocket_url(&self) -> Option<String> {
        let token = self.token.as_deref()?;
        let base = self.base.replacen("https://", "wss://", 1).replacen("http://", "ws://", 1);
        Some(format!("{base}/embywebsocket?api_key={token}&deviceId={}", self.device_id))
    }

    pub async fn set_played(&self, uid: &str, item: &str, played: bool) -> Result<()> {
        let m = if played { reqwest::Method::POST } else { reqwest::Method::DELETE };
        self.req(m, &format!("/Users/{uid}/PlayedItems/{item}")).send().await?.error_for_status()?;
        Ok(())
    }

    pub async fn set_favorite(&self, uid: &str, item: &str, fav: bool) -> Result<()> {
        let m = if fav { reqwest::Method::POST } else { reqwest::Method::DELETE };
        self.req(m, &format!("/Users/{uid}/FavoriteItems/{item}")).send().await?.error_for_status()?;
        Ok(())
    }

    pub async fn logout(&self) {
        let _ = self.req(reqwest::Method::POST, "/Sessions/Logout").send().await;
    }

    pub async fn views(&self, uid: &str) -> Result<Vec<Item>> {
        Ok(self.get::<ItemList>(&format!("/Users/{uid}/Views"), &[]).await?.items)
    }

    pub async fn resume(&self, uid: &str) -> Result<Vec<Item>> {
        Ok(self
            .get::<ItemList>(
                &format!("/Users/{uid}/Items/Resume"),
                &[("Limit", "16"), ("Recursive", "true"), ("MediaTypes", "Video"), ("Fields", LIST_FIELDS)],
            )
            .await?
            .items)
    }

    pub async fn next_up(&self, uid: &str) -> Result<Vec<Item>> {
        Ok(self
            .get::<ItemList>("/Shows/NextUp", &[("UserId", uid), ("Limit", "16"), ("Fields", LIST_FIELDS)])
            .await?
            .items)
    }

    pub async fn latest(&self, uid: &str, parent: &str) -> Result<Vec<Item>> {
        self.get(
            &format!("/Users/{uid}/Items/Latest"),
            &[("ParentId", parent), ("Limit", "20"), ("Fields", LIST_FIELDS)],
        )
        .await
    }

    pub async fn children(&self, uid: &str, parent: &str, limit: u32) -> Result<Vec<Item>> {
        let limit = limit.to_string();
        Ok(self
            .get::<ItemList>(
                &format!("/Users/{uid}/Items"),
                &[
                    ("ParentId", parent),
                    ("SortBy", "SortName"),
                    ("SortOrder", "Ascending"),
                    ("Limit", &limit),
                    ("Fields", LIST_FIELDS),
                ],
            )
            .await?
            .items)
    }

    /// Everything the user has favourited.
    pub async fn favorites(&self, uid: &str, limit: u32) -> Result<Vec<Item>> {
        let limit = limit.to_string();
        Ok(self
            .get::<ItemList>(
                &format!("/Users/{uid}/Items"),
                &[("Filters", "IsFavorite"), ("Recursive", "true"), ("SortBy", "SortName"), ("SortOrder", "Ascending"), ("Limit", &limit), ("Fields", LIST_FIELDS)],
            )
            .await?
            .items)
    }

    pub async fn item(&self, uid: &str, id: &str) -> Result<Item> {
        self.get(&format!("/Users/{uid}/Items/{id}"), &[("Fields", FIELDS)]).await
    }

    // ---- server administration (admin users)

    pub async fn system_info(&self) -> Result<SystemInfo> {
        self.get("/System/Info", &[]).await
    }

    pub async fn sessions(&self) -> Result<Vec<Session>> {
        self.get("/Sessions", &[("ActiveWithinSeconds", "960")]).await
    }

    /// command: Pause | Unpause | Stop
    pub async fn session_playstate(&self, session: &str, command: &str) -> Result<()> {
        self.req(reqwest::Method::POST, &format!("/Sessions/{session}/Playing/{command}"))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    /// Seek a remote session to a position.
    pub async fn session_seek(&self, session: &str, ticks: i64) -> Result<()> {
        self.req(reqwest::Method::POST, &format!("/Sessions/{session}/Playing/Seek?SeekPositionTicks={ticks}"))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn session_message(&self, session: &str, header: &str, text: &str) -> Result<()> {
        self.req(reqwest::Method::POST, &format!("/Sessions/{session}/Message"))
            .json(&serde_json::json!({ "Header": header, "Text": text, "TimeoutMs": 15000 }))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn scheduled_tasks(&self) -> Result<Vec<Task>> {
        self.get("/ScheduledTasks", &[("IsHidden", "false")]).await
    }

    pub async fn start_task(&self, id: &str) -> Result<()> {
        self.req(reqwest::Method::POST, &format!("/ScheduledTasks/Running/{id}")).send().await?.error_for_status()?;
        Ok(())
    }

    pub async fn stop_task(&self, id: &str) -> Result<()> {
        self.req(reqwest::Method::DELETE, &format!("/ScheduledTasks/Running/{id}")).send().await?.error_for_status()?;
        Ok(())
    }

    pub async fn library_refresh(&self) -> Result<()> {
        self.req(reqwest::Method::POST, "/Library/Refresh").send().await?.error_for_status()?;
        Ok(())
    }

    pub async fn virtual_folders(&self) -> Result<Vec<VirtualFolder>> {
        self.get("/Library/VirtualFolders", &[]).await
    }

    pub async fn activity(&self, limit: u32) -> Result<Vec<ActivityEntry>> {
        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct L {
            #[serde(default)]
            items: Vec<ActivityEntry>,
        }
        let limit = limit.to_string();
        Ok(self.get::<L>("/System/ActivityLog/Entries", &[("StartIndex", "0"), ("Limit", &limit)]).await?.items)
    }

    /// kind: Restart | Shutdown
    pub async fn server_power(&self, kind: &str) -> Result<()> {
        self.req(reqwest::Method::POST, &format!("/System/{kind}")).send().await?.error_for_status()?;
        Ok(())
    }

    // ---- subtitles

    /// Searches the server's subtitle providers. `lang` is an ISO 639-2 code such as "eng".
    pub async fn search_subtitles(&self, item_id: &str, lang: &str) -> Result<Vec<RemoteSubtitle>> {
        self.get(&format!("/Items/{item_id}/RemoteSearch/Subtitles/{lang}"), &[("IsPerfectMatch", "false")]).await
    }

    /// Downloads a provider subtitle onto the item (stored by the server).
    pub async fn download_subtitle(&self, item_id: &str, subtitle_id: &str) -> Result<()> {
        self.req(reqwest::Method::POST, &format!("/Items/{item_id}/RemoteSearch/Subtitles/{}", encode_segment(subtitle_id)))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    /// URL mpv can load for an external subtitle stream of an item.
    pub fn subtitle_url(&self, item_id: &str, source_id: &str, st: &MediaStream) -> Option<String> {
        let token = self.token.as_deref()?;
        let ext = match st.codec.as_deref().unwrap_or("srt") {
            "subrip" | "srt" => "srt",
            "ass" | "ssa" => "ass",
            "webvtt" | "vtt" => "vtt",
            other => other,
        };
        let rel = st
            .delivery_url
            .clone()
            .unwrap_or_else(|| format!("/Videos/{item_id}/{source_id}/Subtitles/{}/Stream.{ext}", st.index));
        let mut url = if rel.starts_with("http") {
            rel
        } else if rel.starts_with("/emby") {
            format!("{}{}", self.base, rel)
        } else {
            format!("{}/emby{}", self.base, rel)
        };
        if !url.contains("api_key=") {
            url.push_str(if url.contains('?') { "&" } else { "?" });
            url.push_str(&format!("api_key={token}"));
        }
        Some(url)
    }

    // ---- Live TV

    pub async fn live_channels(&self, uid: &str, limit: u32) -> Result<Vec<Item>> {
        let limit = limit.to_string();
        Ok(self
            .get::<ItemList>("/LiveTv/Channels", &[("UserId", uid), ("Limit", &limit), ("AddCurrentProgram", "false")])
            .await?
            .items)
    }

    /// Programs overlapping [from, to] (RFC 3339 UTC) for the given channels.
    pub async fn live_programs(&self, uid: &str, channels: &[String], from: &str, to: &str) -> Result<Vec<Item>> {
        let ids = channels.join(",");
        Ok(self
            .get::<ItemList>(
                "/LiveTv/Programs",
                &[
                    ("UserId", uid),
                    ("ChannelIds", &ids),
                    ("MinEndDate", from),
                    ("MaxStartDate", to),
                    ("SortBy", "StartDate"),
                    ("EnableTotalRecordCount", "false"),
                    ("Limit", "4000"),
                ],
            )
            .await?
            .items)
    }

    /// Opens a live channel and returns a playable URL (direct stream, else the server's HLS).
    pub async fn open_live(&self, uid: &str, channel_id: &str) -> Result<LiveStream> {
        #[derive(Deserialize)]
        #[serde(rename_all = "PascalCase")]
        struct Info {
            #[serde(default)]
            media_sources: Vec<MediaSource>,
        }
        let info: Info = self
            .req(reqwest::Method::POST, &format!("/Items/{channel_id}/PlaybackInfo"))
            .query(&[
                ("UserId", uid),
                ("StartTimeTicks", "0"),
                ("IsPlayback", "true"),
                ("AutoOpenLiveStream", "true"),
                ("MaxStreamingBitrate", "200000000"),
            ])
            .json(&serde_json::json!({
                "DeviceProfile": {
                    "Name": CLIENT_NAME,
                    "MaxStreamingBitrate": 200000000,
                    "DirectPlayProfiles": [{ "Type": "Video" }, { "Type": "Audio" }],
                    "TranscodingProfiles": [{
                        "Container": "ts", "Type": "Video", "Protocol": "hls", "Context": "Streaming",
                        "VideoCodec": "h264,hevc", "AudioCodec": "aac,ac3,eac3,mp3"
                    }],
                    "SubtitleProfiles": [],
                }
            }))
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        let src = info.media_sources.into_iter().next().ok_or_else(|| Error::Other("channel returned no stream".into()))?;
        let token = self.token.as_deref().ok_or(Error::NoSession)?;
        let rel = src
            .direct_stream_url
            .clone()
            .or_else(|| src.transcoding_url.clone())
            .ok_or_else(|| Error::Other("server offered no stream URL".into()))?;
        let mut url = if rel.starts_with("http") {
            rel
        } else if rel.starts_with("/emby") {
            format!("{}{}", self.base, rel)
        } else {
            format!("{}/emby{}", self.base, rel)
        };
        if !url.contains("api_key=") {
            url.push_str(if url.contains('?') { "&" } else { "?" });
            url.push_str(&format!("api_key={token}"));
        }
        Ok(LiveStream { url, source_id: src.id, live_stream_id: src.live_stream_id })
    }

    pub async fn close_live(&self, live_stream_id: &str) {
        let _ = self
            .req(reqwest::Method::POST, "/LiveStreams/Close")
            .json(&serde_json::json!({ "LiveStreamId": live_stream_id }))
            .send()
            .await;
    }

    /// A sorted/filtered page of a library. Returns the items and the total match count.
    pub async fn browse(&self, uid: &str, parent: &str, b: &Browse) -> Result<(Vec<Item>, usize)> {
        let (start, limit) = (b.start.to_string(), b.limit.to_string());
        let mut q: Vec<(&str, &str)> = vec![
            ("ParentId", parent),
            ("SortBy", &b.sort_by),
            ("SortOrder", if b.descending { "Descending" } else { "Ascending" }),
            ("StartIndex", &start),
            ("Limit", &limit),
            ("Fields", LIST_FIELDS),
        ];
        if let Some(g) = &b.genre {
            q.push(("Genres", g));
        }
        if let Some(t) = b.search.as_deref().filter(|t| !t.is_empty()) {
            q.push(("SearchTerm", t));
        }
        match b.letter.as_deref() {
            Some("#") => q.push(("NameLessThan", "A")),
            Some(l) => q.push(("NameStartsWith", l)),
            None => {}
        }
        let r = self.get::<ItemList>(&format!("/Users/{uid}/Items"), &q).await?;
        let total = r.total_record_count.max(r.items.len());
        Ok((r.items, total))
    }

    /// Genre names present in a library.
    pub async fn genres(&self, uid: &str, parent: &str) -> Result<Vec<String>> {
        Ok(self
            .get::<ItemList>("/Genres", &[("UserId", uid), ("ParentId", parent), ("SortBy", "SortName"), ("SortOrder", "Ascending")])
            .await?
            .items
            .into_iter()
            .map(|g| g.name)
            .collect())
    }

    /// Saved position in ticks. Cross-checks the Continue Watching list in case the item
    /// response doesn't carry the position.
    pub async fn saved_position(&self, uid: &str, item: &Item) -> i64 {
        let p = item.resume_ticks();
        if p > 0 {
            return p;
        }
        match self.resume(uid).await {
            Ok(list) => list.into_iter().find(|r| r.id == item.id).map_or(0, |r| r.resume_ticks()),
            Err(_) => 0,
        }
    }

    /// Random Movies and Series that have a poster (for the login backdrop).
    pub async fn random_items(&self, uid: &str, limit: u32) -> Result<Vec<Item>> {
        let limit = limit.to_string();
        Ok(self
            .get::<ItemList>(
                &format!("/Users/{uid}/Items"),
                &[
                    ("Recursive", "true"),
                    ("IncludeItemTypes", "Movie,Series"),
                    ("SortBy", "Random"),
                    ("Limit", &limit),
                    ("ImageTypes", "Primary"),
                    ("Fields", "ProductionYear"),
                ],
            )
            .await?
            .items)
    }

    /// Movies and series a person appears in.
    pub async fn person_items(&self, uid: &str, person: &str) -> Result<Vec<Item>> {
        Ok(self
            .get::<ItemList>(
                &format!("/Users/{uid}/Items"),
                &[("PersonIds", person), ("Recursive", "true"), ("IncludeItemTypes", "Movie,Series"), ("SortBy", "ProductionYear,SortName"), ("SortOrder", "Descending"), ("Limit", "300"), ("Fields", LIST_FIELDS)],
            )
            .await?
            .items)
    }

    /// Items similar to this one (for "More like this").
    pub async fn similar(&self, uid: &str, id: &str, limit: u32) -> Result<Vec<Item>> {
        let limit = limit.to_string();
        Ok(self
            .get::<ItemList>(&format!("/Items/{id}/Similar"), &[("UserId", uid), ("Limit", &limit), ("Fields", LIST_FIELDS)])
            .await?
            .items)
    }

    /// The episode that follows `episode_id` in its series (crossing into the next season).
    pub async fn episode_after(&self, uid: &str, series: &str, season: Option<&str>, episode_id: &str) -> Option<Item> {
        let pick = |list: Vec<Item>| -> Option<Item> {
            let pos = list.iter().position(|e| e.id == episode_id)?;
            list.into_iter().nth(pos + 1)
        };
        if let Ok(r) = self
            .get::<ItemList>(
                &format!("/Shows/{series}/Episodes"),
                &[("UserId", uid), ("StartItemId", episode_id), ("Limit", "2"), ("Fields", LIST_FIELDS)],
            )
            .await
        {
            if let Some(next) = pick(r.items) {
                return Some(next);
            }
        }
        // Fallback: the rest of the same season.
        let season = season?;
        pick(self.episodes(uid, series, season).await.ok()?)
    }

    /// Finds a movie / series in the library by its TMDB id (for "open in library").
    pub async fn find_by_tmdb(&self, uid: &str, kind: &str, tmdb: i64) -> Option<String> {
        let types = if kind == "tv" { "Series" } else { "Movie" };
        let pid = format!("tmdb.{tmdb}");
        let r = self
            .get::<ItemList>(
                &format!("/Users/{uid}/Items"),
                &[("AnyProviderIdEquals", pid.as_str()), ("Recursive", "true"), ("IncludeItemTypes", types), ("Limit", "1")],
            )
            .await
            .ok()?;
        r.items.into_iter().next().map(|i| i.id)
    }

    /// Every episode of a series, across all seasons.
    pub async fn series_episodes(&self, uid: &str, series: &str) -> Result<Vec<Item>> {
        Ok(self
            .get::<ItemList>(&format!("/Shows/{series}/Episodes"), &[("UserId", uid), ("Fields", LIST_FIELDS)])
            .await?
            .items)
    }

    pub async fn seasons(&self, uid: &str, series: &str) -> Result<Vec<Item>> {
        Ok(self
            .get::<ItemList>(&format!("/Shows/{series}/Seasons"), &[("UserId", uid), ("Fields", LIST_FIELDS)])
            .await?
            .items)
    }

    pub async fn episodes(&self, uid: &str, series: &str, season: &str) -> Result<Vec<Item>> {
        Ok(self
            .get::<ItemList>(
                &format!("/Shows/{series}/Episodes"),
                &[("UserId", uid), ("SeasonId", season), ("Fields", &format!("{LIST_FIELDS},Overview"))],
            )
            .await?
            .items)
    }

    /// Movies and series of one genre across every library.
    pub async fn by_genre(&self, uid: &str, genre: &str, kind: &str) -> Result<Vec<Item>> {
        Ok(self
            .get::<ItemList>(
                &format!("/Users/{uid}/Items"),
                &[("Recursive", "true"), ("IncludeItemTypes", kind), ("Genres", genre), ("SortBy", "SortName"), ("Fields", LIST_FIELDS)],
            )
            .await?
            .items)
    }

    /// Newest by air / release date (not by when it reached the server). `today` is YYYY-MM-DD.
    pub async fn recently_released(&self, uid: &str, today: &str) -> Result<Vec<Item>> {
        Ok(self
            .get::<ItemList>(
                &format!("/Users/{uid}/Items"),
                &[
                    ("Recursive", "true"),
                    ("IncludeItemTypes", "Movie,Series"),
                    ("SortBy", "PremiereDate"),
                    ("SortOrder", "Descending"),
                    ("MaxPremiereDate", today),
                    ("Limit", "24"),
                    ("Fields", LIST_FIELDS),
                ],
            )
            .await?
            .items)
    }

    pub async fn search(&self, uid: &str, term: &str) -> Result<Vec<Item>> {
        Ok(self
            .get::<ItemList>(
                &format!("/Users/{uid}/Items"),
                &[
                    ("SearchTerm", term),
                    ("Recursive", "true"),
                    ("IncludeItemTypes", "Movie,Series,Episode,BoxSet,Audio,MusicAlbum,MusicArtist"),
                    ("Limit", "60"),
                    ("Fields", LIST_FIELDS),
                ],
            )
            .await?
            .items)
    }

    /// Image URL sized server-side. Returns None if the item has no such image. A missing
    /// backdrop falls back to the parent's (an episode uses its series' backdrop).
    pub fn image_url(&self, item: &Item, kind: ImageKind, max_width: u32) -> Option<String> {
        let (owner, path, tag) = match kind {
            ImageKind::Primary => (item.id.clone(), "Primary", item.image_tags.get("Primary")?.clone()),
            ImageKind::Thumb => (item.id.clone(), "Thumb", item.image_tags.get("Thumb")?.clone()),
            ImageKind::Logo => match item.image_tags.get("Logo") {
                Some(t) => (item.id.clone(), "Logo", t.clone()),
                None => (item.parent_logo_item_id.clone()?, "Logo", item.parent_logo_image_tag.clone()?),
            },
            ImageKind::Backdrop => match item.backdrop_image_tags.first() {
                Some(t) => (item.id.clone(), "Backdrop/0", t.clone()),
                None => (item.parent_backdrop_item_id.clone()?, "Backdrop/0", item.parent_backdrop_image_tags.first()?.clone()),
            },
        };
        Some(format!("{}/emby/Items/{}/Images/{}?maxWidth={}&quality=90&tag={}", self.base, owner, path, max_width, tag))
    }

    pub fn person_image_url(&self, p: &Person, max_width: u32) -> Option<String> {
        let tag = p.primary_image_tag.as_ref()?;
        Some(format!("{}/emby/Items/{}/Images/Primary?maxWidth={}&quality=90&tag={}", self.base, p.id, max_width, tag))
    }

    pub async fn fetch_bytes(&self, url: &str) -> Result<Vec<u8>> {
        Ok(self.http.get(url).send().await?.error_for_status()?.bytes().await?.to_vec())
    }

    /// Direct-play URL (the server sends the original file; mpv plays nearly anything) when
    /// `transcode` is None, otherwise an HLS transcode capped at (bits per second, height).
    pub fn stream_url(&self, item_id: &str, source_id: &str, play_session: &str, transcode: Option<(u32, u32)>, audio_index: Option<i32>) -> Result<String> {
        let token = self.token.as_deref().ok_or(Error::NoSession)?;
        Ok(match transcode {
            Some((bps, height)) => format!(
                "{}/emby/Videos/{}/master.m3u8?MediaSourceId={}&PlaySessionId={}&VideoCodec=h264&AudioCodec=aac&MaxStreamingBitrate={}&MaxHeight={}&TranscodingMaxAudioChannels=6&SegmentContainer=ts&api_key={}&DeviceId={}{}",
                self.base, item_id, source_id, play_session, bps, height, token, self.device_id,
                audio_index.map(|i| format!("&AudioStreamIndex={i}")).unwrap_or_default()
            ),
            None => format!(
                "{}/emby/Videos/{}/stream?Static=true&MediaSourceId={}&PlaySessionId={}&api_key={}&DeviceId={}",
                self.base, item_id, source_id, play_session, token, self.device_id
            ),
        })
    }

    /// Rough download throughput in bits per second, from the server's bitrate-test endpoint.
    /// Skips the first fifth of the data (TCP slow start). None if the test fails.
    pub async fn measure_bandwidth(&self, bytes: usize, cap: Duration) -> Option<f64> {
        let size = bytes.to_string();
        let mut resp = self
            .req(reqwest::Method::GET, "/Playback/BitrateTest")
            .query(&[("Size", size.as_str())])
            .send()
            .await
            .ok()?
            .error_for_status()
            .ok()?;
        let start = std::time::Instant::now();
        let mut total = 0usize;
        let mut mark: Option<(std::time::Instant, usize)> = None;
        let _ = tokio::time::timeout(cap, async {
            while let Ok(Some(chunk)) = resp.chunk().await {
                total += chunk.len();
                if mark.is_none() && total >= bytes / 5 {
                    mark = Some((std::time::Instant::now(), total));
                }
            }
        })
        .await;
        let (t0, b0) = mark.unwrap_or((start, 0));
        let secs = t0.elapsed().as_secs_f64().max(0.001);
        let got = total.saturating_sub(b0);
        if got < 200_000 {
            return None;
        }
        Some(got as f64 * 8.0 / secs)
    }

    #[allow(clippy::too_many_arguments)]
    async fn report(&self, path: &str, item_id: &str, source_id: &str, session: &str, ticks: i64, paused: bool, method: &str) -> Result<()> {
        self.req(reqwest::Method::POST, path)
            .json(&serde_json::json!({
                "ItemId": item_id,
                "MediaSourceId": source_id,
                "PlaySessionId": session,
                "PositionTicks": ticks,
                "IsPaused": paused,
                "CanSeek": true,
                "PlayMethod": method,
            }))
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    /// `method`: DirectPlay | DirectStream | Transcode
    pub async fn report_start(&self, item: &str, src: &str, sess: &str, ticks: i64, method: &str) -> Result<()> {
        self.report("/Sessions/Playing", item, src, sess, ticks, false, method).await
    }
    pub async fn report_progress(&self, item: &str, src: &str, sess: &str, ticks: i64, paused: bool, method: &str) -> Result<()> {
        self.report("/Sessions/Playing/Progress", item, src, sess, ticks, paused, method).await
    }
    pub async fn report_stopped(&self, item: &str, src: &str, sess: &str, ticks: i64, method: &str) -> Result<()> {
        self.report("/Sessions/Playing/Stopped", item, src, sess, ticks, false, method).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reads_intro_and_credits_markers() {
        let item: Item = serde_json::from_str(r#"{"Id":"1","Name":"x","Chapters":[
            {"StartPositionTicks":0,"Name":"Chapter 1"},
            {"StartPositionTicks":300000000,"Name":"Intro Start","MarkerType":"IntroStart"},
            {"StartPositionTicks":900000000,"Name":"Intro End","MarkerType":"IntroEnd"},
            {"StartPositionTicks":12000000000,"MarkerType":"CreditsStart"}]}"#).unwrap();
        assert_eq!(item.intro(), Some((30.0, 90.0)));
        assert_eq!(item.credits_start(), Some(1200.0));
        assert_eq!(Item::default().intro(), None);
    }

    #[test]
    fn normalizes_servers() {
        assert_eq!(normalize_server("nas"), "http://nas:8096");
        assert_eq!(normalize_server("nas:9000"), "http://nas:9000");
        assert_eq!(normalize_server("http://nas/"), "http://nas");
        assert_eq!(normalize_server("https://emby.example.com/emby"), "https://emby.example.com");
        assert_eq!(normalize_server(" 192.168.1.5:8096/ "), "http://192.168.1.5:8096");
    }

    #[test]
    fn parses_numeric_and_string_ids() {
        let i: Item = serde_json::from_str(r#"{"Id":123,"Name":"x","Type":"Movie","SeriesId":"9"}"#).unwrap();
        assert_eq!(i.id, "123");
        assert_eq!(i.series_id.as_deref(), Some("9"));
    }
}
