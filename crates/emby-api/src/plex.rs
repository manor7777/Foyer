//! Plex Media Server backend. Plex's API looks nothing like Emby's, so this maps what Plex
//! returns onto the Emby-shaped `Item` / `MediaSource` types the rest of the app already uses.
//!
//! Conventions that differ from Emby:
//! - Library sections get the id `sec<key>` so they can't clash with item rating keys.
//! - `MediaSource.id` is the file part's URL path (that is all direct play needs).
//! - `MediaStream.index` is Plex's stream id.
//! - People ids are `<filter>|<name>` (e.g. `actor=123|Tom Hanks`), usable as a library filter.
//! - "Favourite" is a user rating of 10.

use super::*;
use serde_json::Value;

const PRODUCT: &str = "Foyer";
const IDENT: &str = "com.plexapp.plugins.library";
const SEC: &str = "sec";
const MS_TO_TICKS: i64 = 10_000;

fn s(v: &Value, k: &str) -> Option<String> {
    match v.get(k)? {
        Value::String(x) => Some(x.clone()),
        Value::Number(n) => Some(n.to_string()),
        _ => None,
    }
}

fn n(v: &Value, k: &str) -> Option<i64> {
    v.get(k).and_then(|x| x.as_i64().or_else(|| x.as_f64().map(|f| f as i64)).or_else(|| x.as_str()?.parse().ok()))
}

fn f(v: &Value, k: &str) -> Option<f64> {
    v.get(k).and_then(|x| x.as_f64().or_else(|| x.as_str()?.parse().ok()))
}

fn arr<'a>(v: &'a Value, k: &str) -> &'a [Value] {
    v.get(k).and_then(|x| x.as_array()).map_or(&[], |a| a.as_slice())
}

fn truthy(v: &Value, k: &str) -> bool {
    v.get(k).is_some_and(|x| x.as_bool().unwrap_or_else(|| x.as_i64().unwrap_or(0) != 0 || x.as_str() == Some("1")))
}

/// Items (flattening hubs) in a MediaContainer.
fn metas(v: &Value) -> Vec<&Value> {
    let mc = &v["MediaContainer"];
    let mut out: Vec<&Value> = arr(mc, "Metadata").iter().collect();
    for h in arr(mc, "Hub") {
        out.extend(arr(h, "Metadata").iter());
    }
    out
}

fn section_key(parent: &str) -> Option<&str> {
    parent.strip_prefix(SEC)
}

fn sort_key(emby: &str) -> &'static str {
    match emby.split(',').next().unwrap_or("") {
        "DateCreated" => "addedAt",
        "PremiereDate" | "ProductionYear" => "originallyAvailableAt",
        "CommunityRating" => "rating",
        "Random" => "random",
        _ => "titleSort",
    }
}

fn pmedia(m: &Value) -> Vec<MediaSource> {
    arr(m, "Media")
        .iter()
        .filter_map(|media| {
            let part = arr(media, "Part").first()?;
            let streams = arr(part, "Stream")
                .iter()
                .map(|st| {
                    let kind = match n(st, "streamType") {
                        Some(1) => "Video",
                        Some(2) => "Audio",
                        Some(3) => "Subtitle",
                        _ => "Other",
                    };
                    let key = s(st, "key");
                    MediaStream {
                        index: n(st, "id").unwrap_or(0) as i32,
                        kind: kind.into(),
                        is_external: kind == "Subtitle" && key.is_some(),
                        is_text_subtitle_stream: false, // embedded subs play from the file itself
                        delivery_url: key,
                        codec: s(st, "codec"),
                        language: s(st, "languageCode").or_else(|| s(st, "language")),
                        display_title: s(st, "displayTitle").or_else(|| s(st, "extendedDisplayTitle")),
                        title: s(st, "title"),
                        profile: s(st, "profile"),
                        width: n(st, "width").map(|x| x as i32),
                        height: n(st, "height").map(|x| x as i32),
                        average_frame_rate: f(st, "frameRate"),
                        bit_rate: n(st, "bitrate").map(|k| k * 1000),
                        bit_depth: n(st, "bitDepth").map(|x| x as i32),
                        channels: n(st, "channels").map(|x| x as i32),
                        channel_layout: s(st, "audioChannelLayout"),
                        sample_rate: n(st, "samplingRate").map(|x| x as i32),
                        video_range: match s(st, "colorTrc").as_deref() {
                            Some("smpte2084") => Some("HDR10".into()),
                            Some("arib-std-b67") => Some("HLG".into()),
                            _ => None,
                        },
                        is_default: truthy(st, "default") || truthy(st, "selected"),
                        is_forced: truthy(st, "forced"),
                    }
                })
                .collect();
            let key = s(part, "key")?;
            Some(MediaSource {
                id: key.clone(),
                name: s(media, "videoResolution"),
                container: s(media, "container"),
                run_time_ticks: n(media, "duration").map(|d| d * MS_TO_TICKS),
                direct_stream_url: Some(key),
                transcoding_url: None,
                live_stream_id: None,
                size: n(part, "size"),
                path: s(part, "file"),
                media_streams: streams,
                bitrate: n(media, "bitrate").map(|k| k * 1000),
            })
        })
        .collect()
}

fn pperson(p: &Value, kind: &str) -> Option<Person> {
    let tag = s(p, "tag")?;
    let filter = s(p, "filter").or_else(|| Some(format!("{}={}", kind.to_lowercase(), n(p, "id")?)))?;
    Some(Person {
        id: format!("{filter}|{tag}"),
        name: tag,
        role: s(p, "role"),
        kind: Some(kind.into()),
        primary_image_tag: s(p, "thumb"),
    })
}

impl Client {
    // ---- plumbing

    fn preq(&self, m: reqwest::Method, url: String) -> reqwest::RequestBuilder {
        let device = std::env::var("COMPUTERNAME").unwrap_or_else(|_| "Windows PC".into());
        let mut r = self
            .http
            .request(m, url)
            .header("Accept", "application/json")
            .header("X-Plex-Client-Identifier", &self.device_id)
            .header("X-Plex-Product", PRODUCT)
            .header("X-Plex-Version", CLIENT_VERSION)
            .header("X-Plex-Platform", "Windows")
            .header("X-Plex-Device-Name", device);
        if let Some(t) = &self.token {
            r = r.header("X-Plex-Token", t);
        }
        r
    }

    async fn pget(&self, path: &str, q: &[(&str, &str)]) -> Result<Value> {
        let started = std::time::Instant::now();
        let resp = self.preq(reqwest::Method::GET, format!("{}{}", self.base, path)).query(q).send().await;
        match resp {
            Ok(r) => {
                log(&format!("GET {path} -> {} in {} ms", r.status().as_u16(), started.elapsed().as_millis()));
                Ok(r.error_for_status()?.json().await?)
            }
            Err(e) => {
                log(&format!("GET {path} failed after {} ms: {e}", started.elapsed().as_millis()));
                Err(e.into())
            }
        }
    }

    /// Fire-and-forget style call whose body we don't need.
    async fn pcall(&self, m: reqwest::Method, path: &str, q: &[(&str, &str)]) -> Result<()> {
        self.preq(m, format!("{}{}", self.base, path)).query(q).send().await?.error_for_status()?;
        Ok(())
    }

    /// Movie and TV sections: (key, "movie" | "show", title).
    async fn plex_sections(&self) -> Result<Vec<(String, String, String)>> {
        let v = self.pget("/library/sections", &[]).await?;
        Ok(arr(&v["MediaContainer"], "Directory")
            .iter()
            .filter_map(|d| {
                let ty = s(d, "type")?;
                matches!(ty.as_str(), "movie" | "show").then(|| Some((s(d, "key")?, ty, s(d, "title").unwrap_or_default())))?
            })
            .collect())
    }

    fn pitem(&self, m: &Value) -> Item {
        let ty = s(m, "type").unwrap_or_default();
        let kind = match ty.as_str() {
            "movie" => "Movie",
            "show" => "Series",
            "season" => "Season",
            "episode" => "Episode",
            "collection" => "BoxSet",
            "clip" => "Video",
            o => o,
        };
        let id = s(m, "ratingKey").unwrap_or_default();
        let rt = n(m, "duration").map(|d| d * MS_TO_TICKS);
        if let Some(rt) = rt {
            self.plex_dur.lock().unwrap().insert(id.clone(), rt / MS_TO_TICKS);
        }
        let leaf = n(m, "leafCount").unwrap_or(0);
        let viewed_leaf = n(m, "viewedLeafCount").unwrap_or(0);
        let container = matches!(kind, "Series" | "Season" | "BoxSet");
        let played = if container { leaf > 0 && viewed_leaf >= leaf } else { n(m, "viewCount").unwrap_or(0) > 0 };
        let mut tags = HashMap::new();
        let thumb = s(m, "thumb").or_else(|| s(m, "parentThumb")).or_else(|| s(m, "grandparentThumb"));
        if let Some(t) = &thumb {
            tags.insert("Primary".to_string(), t.clone());
        }
        let art = s(m, "art");
        if let Some(a) = art.clone().or(thumb) {
            tags.insert("Thumb".to_string(), a);
        }
        if let Some(l) = arr(m, "Image").iter().find(|i| s(i, "type").as_deref() == Some("clearLogo")).and_then(|i| s(i, "url")) {
            tags.insert("Logo".to_string(), l);
        }
        let (series_id, season_id, series_name) = match kind {
            "Episode" => (s(m, "grandparentRatingKey"), s(m, "parentRatingKey"), s(m, "grandparentTitle")),
            "Season" => (s(m, "parentRatingKey"), None, s(m, "parentTitle")),
            _ => (None, None, None),
        };
        let mut chapters: Vec<Chapter> = arr(m, "Chapter")
            .iter()
            .map(|c| Chapter { start_position_ticks: n(c, "startTimeOffset").unwrap_or(0) * MS_TO_TICKS, name: s(c, "tag"), marker_type: Some("Chapter".into()) })
            .collect();
        for mk in arr(m, "Marker") {
            let (Some(a), Some(b)) = (n(mk, "startTimeOffset"), n(mk, "endTimeOffset")) else { continue };
            match s(mk, "type").as_deref() {
                Some("intro") => {
                    chapters.push(Chapter { start_position_ticks: a * MS_TO_TICKS, name: None, marker_type: Some("IntroStart".into()) });
                    chapters.push(Chapter { start_position_ticks: b * MS_TO_TICKS, name: None, marker_type: Some("IntroEnd".into()) });
                }
                Some("credits") => chapters.push(Chapter { start_position_ticks: a * MS_TO_TICKS, name: None, marker_type: Some("CreditsStart".into()) }),
                _ => {}
            }
        }
        let mut people = Vec::new();
        for (key, kind) in [("Role", "Actor"), ("Director", "Director"), ("Writer", "Writer")] {
            people.extend(arr(m, key).iter().filter_map(|p| pperson(p, kind)));
        }
        let provider_ids = arr(m, "Guid")
            .iter()
            .filter_map(|g| {
                let (k, v) = s(g, "id")?.split_once("://").map(|(a, b)| (a.to_string(), b.to_string()))?;
                let k = match k.as_str() {
                    "tmdb" => "Tmdb",
                    "imdb" => "Imdb",
                    "tvdb" => "Tvdb",
                    _ => return None,
                };
                Some((k.to_string(), v))
            })
            .collect();
        Item {
            id,
            name: s(m, "title").unwrap_or_default(),
            kind: kind.into(),
            overview: s(m, "summary").filter(|x| !x.is_empty()),
            production_year: n(m, "year").map(|y| y as i32),
            run_time_ticks: rt,
            community_rating: f(m, "rating").or_else(|| f(m, "audienceRating")).map(|r| r as f32),
            official_rating: s(m, "contentRating"),
            series_name,
            series_id,
            season_id,
            parent_index_number: n(m, "parentIndex").map(|x| x as i32),
            index_number: n(m, "index").map(|x| x as i32),
            image_tags: tags,
            backdrop_image_tags: art.into_iter().collect(),
            user_data: Some(UserData {
                playback_position_ticks: n(m, "viewOffset").unwrap_or(0) * MS_TO_TICKS,
                played,
                played_percentage: None,
                unplayed_item_count: container.then_some((leaf - viewed_leaf).max(0) as i32),
                is_favorite: f(m, "userRating").is_some_and(|r| r >= 9.0),
            }),
            media_sources: pmedia(m),
            genres: arr(m, "Genre").iter().filter_map(|g| s(g, "tag")).collect(),
            is_folder: container,
            media_type: matches!(kind, "Movie" | "Episode" | "Video").then(|| "Video".to_string()),
            people,
            provider_ids,
            parent_backdrop_item_id: series_id_for_art(m),
            parent_backdrop_image_tags: s(m, "grandparentArt").or_else(|| s(m, "parentArt")).into_iter().collect(),
            premiere_date: s(m, "originallyAvailableAt"),
            chapters,
            ..Default::default()
        }
    }

    fn pitems(&self, v: &Value) -> Vec<Item> {
        metas(v).into_iter().map(|m| self.pitem(m)).collect()
    }

    /// Items from a path, with optional extra query pairs; also the total match count.
    async fn plist(&self, path: &str, q: &[(&str, &str)], start: usize, size: usize) -> Result<(Vec<Item>, usize)> {
        let (st, sz) = (start.to_string(), size.to_string());
        let mut qq: Vec<(&str, &str)> = q.to_vec();
        qq.push(("X-Plex-Container-Start", &st));
        qq.push(("X-Plex-Container-Size", &sz));
        let v = self.pget(path, &qq).await?;
        let items = self.pitems(&v);
        let total = n(&v["MediaContainer"], "totalSize").unwrap_or(0) as usize;
        let total = total.max(items.len());
        Ok((items, total))
    }

    // ---- sign in

    pub(crate) async fn plex_login(&mut self, user: &str, password: &str) -> Result<User> {
        let r = self
            .preq(reqwest::Method::POST, "https://plex.tv/api/v2/users/signin".into())
            .form(&[("login", user), ("password", password), ("rememberMe", "true")])
            .send()
            .await?;
        let status = r.status();
        let body: Value = r.json().await.unwrap_or(Value::Null);
        if !status.is_success() {
            let text = body.to_string().to_lowercase();
            if text.contains("verification") || text.contains("two-factor") {
                return Err(Error::Other("Two-step verification is on: add your 6-digit code to the end of your password.".into()));
            }
            if status.as_u16() == 401 || status.as_u16() == 400 {
                return Err(Error::Other("Wrong user name or password.".into()));
            }
            return Err(Error::Other(format!("plex.tv answered {status}")));
        }
        self.token = s(&body, "authToken");
        if self.token.is_none() {
            return Err(Error::Other("plex.tv sent no sign-in token".into()));
        }
        Ok(User { id: s(&body, "id").unwrap_or_default(), name: s(&body, "title").or_else(|| s(&body, "username")).unwrap_or_default(), policy: None })
    }

    /// Checks a saved server token still works. The name is filled in by the caller (kept from sign-in).
    pub(crate) async fn plex_validate(&self, user_id: &str) -> Result<User> {
        self.pget("/library/sections", &[]).await?;
        Ok(User { id: user_id.to_string(), name: String::new(), policy: Some(Policy { is_administrator: self.plex_is_owner().await }) })
    }

    // ---- browsing

    pub(crate) async fn plex_views(&self) -> Result<Vec<Item>> {
        Ok(self
            .plex_sections()
            .await?
            .into_iter()
            .map(|(key, ty, title)| Item {
                id: format!("{SEC}{key}"),
                name: title,
                kind: "CollectionFolder".into(),
                is_folder: true,
                collection_type: Some(if ty == "movie" { "movies" } else { "tvshows" }.into()),
                ..Default::default()
            })
            .collect())
    }

    pub(crate) async fn plex_on_deck(&self, resume: bool) -> Result<Vec<Item>> {
        let v = self.pget("/library/onDeck", &[("X-Plex-Container-Size", "40")]).await?;
        Ok(self.pitems(&v).into_iter().filter(|i| (i.resume_ticks() > 0) == resume).take(16).collect())
    }

    pub(crate) async fn plex_latest(&self, parent: &str) -> Result<Vec<Item>> {
        let Some(key) = section_key(parent) else { return Ok(vec![]) };
        Ok(self.plist(&format!("/library/sections/{key}/all"), &[("sort", "addedAt:desc")], 0, 20).await?.0)
    }

    pub(crate) async fn plex_children(&self, parent: &str, limit: u32) -> Result<Vec<Item>> {
        let size = limit as usize;
        match section_key(parent) {
            Some(key) => Ok(self.plist(&format!("/library/sections/{key}/all"), &[("sort", "titleSort")], 0, size).await?.0),
            None => Ok(self.plist(&format!("/library/metadata/{parent}/children"), &[], 0, size).await?.0),
        }
    }

    pub(crate) async fn plex_browse(&self, parent: &str, b: &Browse) -> Result<(Vec<Item>, usize)> {
        let Some(key) = section_key(parent) else {
            let items = self.plex_children(parent, b.limit as u32).await?;
            let n = items.len();
            return Ok((items, n));
        };
        let sort = format!("{}{}", sort_key(&b.sort_by), if b.descending { ":desc" } else { "" });
        let mut q: Vec<(&str, String)> = vec![("sort", sort)];
        if let Some(g) = &b.genre {
            if let Some(id) = self.plex_genre_id(key, g).await {
                q.push(("genre", id));
            }
        }
        if let Some(t) = b.search.as_deref().filter(|t| !t.is_empty()) {
            q.push(("title", t.to_string()));
        }
        let path = match b.letter.as_deref() {
            Some(l) => format!("/library/sections/{key}/firstCharacter/{}", encode_segment(l)),
            None => format!("/library/sections/{key}/all"),
        };
        let q: Vec<(&str, &str)> = q.iter().map(|(k, v)| (*k, v.as_str())).collect();
        self.plist(&path, &q, b.start, b.limit).await
    }

    async fn plex_genre_id(&self, section: &str, name: &str) -> Option<String> {
        let v = self.pget(&format!("/library/sections/{section}/genre"), &[]).await.ok()?;
        arr(&v["MediaContainer"], "Directory").iter().find(|d| s(d, "title").is_some_and(|t| t.eq_ignore_ascii_case(name))).and_then(|d| s(d, "key"))
    }

    pub(crate) async fn plex_genres(&self, parent: &str) -> Result<Vec<String>> {
        let Some(key) = section_key(parent) else { return Ok(vec![]) };
        let v = self.pget(&format!("/library/sections/{key}/genre"), &[]).await?;
        Ok(arr(&v["MediaContainer"], "Directory").iter().filter_map(|d| s(d, "title")).collect())
    }

    pub(crate) async fn plex_item(&self, id: &str) -> Result<Item> {
        if let Some((_, name)) = id.split_once('|') {
            return Ok(Item { id: id.into(), name: name.into(), kind: "Person".into(), ..Default::default() });
        }
        if id.starts_with(SEC) {
            return self.plex_views().await?.into_iter().find(|v| v.id == id).ok_or_else(|| Error::Other("library not found".into()));
        }
        let v = self
            .pget(&format!("/library/metadata/{id}"), &[("includeMarkers", "1"), ("includeChapters", "1"), ("includeGuids", "1"), ("includeExternalMedia", "1")])
            .await?;
        let item = self.pitems(&v).into_iter().next().ok_or_else(|| Error::Other("item not found".into()))?;
        *self.plex_part.lock().unwrap() = item.media_sources.first().map(|m| m.id.clone());
        Ok(item)
    }

    /// Runs `f` over every section of the wanted kind ("movie" / "show" / any), concatenating results.
    async fn plex_each(&self, ty: Option<&str>, path: &str, q: &[(&str, &str)], size: usize) -> Result<Vec<Item>> {
        let mut out = Vec::new();
        for (key, t, _) in self.plex_sections().await? {
            if ty.is_some_and(|w| w != t) {
                continue;
            }
            if let Ok((items, _)) = self.plist(&path.replace("{}", &key), q, 0, size).await {
                out.extend(items);
            }
        }
        Ok(out)
    }

    pub(crate) async fn plex_favorites(&self, limit: u32) -> Result<Vec<Item>> {
        self.plex_each(None, "/library/sections/{}/all", &[("userRating>>", "9")], limit as usize).await
    }

    pub(crate) async fn plex_random(&self, limit: u32) -> Result<Vec<Item>> {
        let mut v = self.plex_each(None, "/library/sections/{}/all", &[("sort", "random")], limit as usize).await?;
        v.retain(|i| i.image_tags.contains_key("Primary"));
        v.truncate(limit as usize);
        Ok(v)
    }

    pub(crate) async fn plex_person_items(&self, filter: &str) -> Result<Vec<Item>> {
        let filter = filter.split('|').next().unwrap_or(filter);
        let Some((k, val)) = filter.split_once('=') else { return Ok(vec![]) };
        let mut v = self.plex_each(None, "/library/sections/{}/all", &[(k, val), ("sort", "year:desc")], 300).await?;
        v.sort_by(|a, b| b.production_year.cmp(&a.production_year));
        Ok(v)
    }

    pub(crate) async fn plex_similar(&self, id: &str, limit: u32) -> Result<Vec<Item>> {
        let c = limit.to_string();
        Ok(self.pitems(&self.pget(&format!("/library/metadata/{id}/similar"), &[("count", &c)]).await?))
    }

    pub(crate) async fn plex_series_episodes(&self, series: &str) -> Result<Vec<Item>> {
        Ok(self.plist(&format!("/library/metadata/{series}/allLeaves"), &[], 0, 5000).await?.0)
    }

    pub(crate) async fn plex_children_of(&self, id: &str) -> Result<Vec<Item>> {
        Ok(self.plist(&format!("/library/metadata/{id}/children"), &[], 0, 5000).await?.0)
    }

    pub(crate) async fn plex_find_by_tmdb(&self, kind: &str, tmdb: i64) -> Option<String> {
        let guid = format!("tmdb://{tmdb}");
        let ty = if kind == "tv" { "show" } else { "movie" };
        self.plex_each(Some(ty), "/library/sections/{}/all", &[("guid", &guid)], 1).await.ok()?.into_iter().next().map(|i| i.id)
    }

    pub(crate) async fn plex_by_genre(&self, genre: &str, kind: &str) -> Result<Vec<Item>> {
        let ty = if kind.contains("Series") { "show" } else { "movie" };
        let mut out = Vec::new();
        for (key, t, _) in self.plex_sections().await? {
            if t != ty {
                continue;
            }
            if let Some(g) = self.plex_genre_id(&key, genre).await {
                out.extend(self.plist(&format!("/library/sections/{key}/all"), &[("genre", &g), ("sort", "titleSort")], 0, 500).await?.0);
            }
        }
        Ok(out)
    }

    pub(crate) async fn plex_recently_released(&self, today: &str) -> Result<Vec<Item>> {
        let mut v = self.plex_each(None, "/library/sections/{}/all", &[("sort", "originallyAvailableAt:desc")], 24).await?;
        v.retain(|i| i.premiere_date.as_deref().is_some_and(|d| d <= today));
        v.sort_by(|a, b| b.premiere_date.cmp(&a.premiere_date));
        v.truncate(24);
        Ok(v)
    }

    pub(crate) async fn plex_search(&self, term: &str) -> Result<Vec<Item>> {
        let v = self.pget("/hubs/search", &[("query", term), ("limit", "30")]).await?;
        Ok(self.pitems(&v).into_iter().filter(|i| matches!(i.kind.as_str(), "Movie" | "Series" | "Episode" | "BoxSet")).collect())
    }

    // ---- state changes

    pub(crate) async fn plex_set_played(&self, item: &str, played: bool) -> Result<()> {
        self.pcall(reqwest::Method::GET, if played { "/:/scrobble" } else { "/:/unscrobble" }, &[("key", item), ("identifier", IDENT)]).await
    }

    pub(crate) async fn plex_set_favorite(&self, item: &str, fav: bool) -> Result<()> {
        self.pcall(reqwest::Method::PUT, "/:/rate", &[("key", item), ("identifier", IDENT), ("rating", if fav { "10" } else { "-1" })]).await
    }

    pub(crate) async fn plex_timeline(&self, item: &str, sess: &str, ticks: i64, state: &str) -> Result<()> {
        let (time, dur) = (ticks / MS_TO_TICKS, self.plex_dur.lock().unwrap().get(item).copied().unwrap_or(0));
        let (time, dur) = (time.to_string(), dur.to_string());
        let key = format!("/library/metadata/{item}");
        self.pcall(
            reqwest::Method::GET,
            "/:/timeline",
            &[("ratingKey", item), ("key", &key), ("state", state), ("time", &time), ("duration", &dur), ("identifier", IDENT), ("X-Plex-Session-Identifier", sess)],
        )
        .await
    }

    // ---- images and streams

    /// `path` is a server path (or a full URL, for people).
    fn plex_photo(&self, path: &str, width: u32) -> Option<String> {
        if path.starts_with("http") {
            return Some(path.to_string());
        }
        let token = self.token.as_deref()?;
        Some(format!("{}/photo/:/transcode?width={width}&height={}&minSize=0&url={}&X-Plex-Token={token}", self.base, width * 2, encode_segment(path)))
    }

    pub(crate) fn plex_image_url(&self, item: &Item, kind: ImageKind, width: u32) -> Option<String> {
        let path = match kind {
            ImageKind::Primary => item.image_tags.get("Primary")?,
            ImageKind::Thumb => item.image_tags.get("Thumb")?,
            ImageKind::Logo => item.image_tags.get("Logo")?,
            ImageKind::Backdrop => item.backdrop_image_tags.first().or_else(|| item.parent_backdrop_image_tags.first())?,
        };
        self.plex_photo(path, width)
    }

    pub(crate) fn plex_person_image(&self, p: &Person, width: u32) -> Option<String> {
        self.plex_photo(p.primary_image_tag.as_deref()?, width)
    }

    pub(crate) fn plex_subtitle_url(&self, st: &MediaStream) -> Option<String> {
        Some(format!("{}{}?X-Plex-Token={}", self.base, st.delivery_url.as_deref()?, self.token.as_deref()?))
    }

    pub(crate) fn plex_stream_url(&self, item_id: &str, part: &str, session: &str, transcode: Option<(u32, u32)>, audio_index: Option<i32>) -> Result<String> {
        let token = self.token.as_deref().ok_or(Error::NoSession)?;
        Ok(match transcode {
            Some((bps, height)) => format!(
                "{}/video/:/transcode/universal/start.m3u8?path={}&mediaIndex=0&partIndex=0&protocol=hls&fastSeek=1&directPlay=0&directStream=1&subtitleSize=100&audioBoost=100&maxVideoBitrate={}&videoResolution={}x{}&session={session}&X-Plex-Client-Identifier={}&X-Plex-Product={PRODUCT}&X-Plex-Platform=Chrome&X-Plex-Token={token}{}",
                self.base,
                encode_segment(&format!("/library/metadata/{item_id}")),
                bps / 1000,
                height * 16 / 9,
                height,
                self.device_id,
                audio_index.map(|i| format!("&audioStreamID={i}")).unwrap_or_default()
            ),
            None => format!("{}{}?X-Plex-Token={token}", self.base, part),
        })
    }
}

/// An episode / season borrows its series' backdrop; the series id is what the app keys that on.
fn series_id_for_art(m: &Value) -> Option<String> {
    s(m, "grandparentRatingKey").or_else(|| s(m, "parentRatingKey"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_an_episode() {
        let c = Client::new("http://nas:32400", "dev").with_kind(Kind::Plex).with_token("T");
        let m: Value = serde_json::from_str(
            r#"{"ratingKey":"55","type":"episode","title":"Pilot","grandparentTitle":"Show","grandparentRatingKey":"10",
            "parentRatingKey":"11","parentIndex":1,"index":2,"duration":60000,"viewOffset":15000,"thumb":"/library/metadata/55/thumb/1",
            "grandparentArt":"/library/metadata/10/art/1","Marker":[{"type":"intro","startTimeOffset":1000,"endTimeOffset":9000}],
            "Media":[{"container":"mkv","bitrate":8000,"Part":[{"key":"/library/parts/9/1/file.mkv","Stream":[{"id":3,"streamType":2,"languageCode":"eng"}]}]}]}"#,
        )
        .unwrap();
        let i = c.pitem(&m);
        assert_eq!((i.id.as_str(), i.kind.as_str(), i.series_id.as_deref(), i.index_number), ("55", "Episode", Some("10"), Some(2)));
        assert_eq!(i.progress(), 0.25);
        assert_eq!(i.intro(), Some((1.0, 9.0)));
        assert_eq!(i.media_sources[0].id, "/library/parts/9/1/file.mkv");
        assert_eq!(i.media_sources[0].media_streams[0].kind, "Audio");
        assert!(c.plex_image_url(&i, ImageKind::Backdrop, 400).unwrap().contains("X-Plex-Token=T"));
        assert_eq!(c.base(), "http://nas:32400");
    }
}

/// A Plex server the signed-in account can use (from plex.tv).
#[derive(Debug, Clone)]
pub struct PlexServer {
    pub name: String,
    pub owned: bool,
    /// The server-specific access token (narrower than the account token).
    token: String,
    /// (uri, local, relay) in plex.tv's order.
    conns: Vec<(String, bool, bool)>,
    /// Plain addresses ("192.168.1.5") for matching a typed server address.
    pub addresses: Vec<String>,
}

impl PlexServer {
    /// Whether a typed address ("nas", "192.168.1.5:32400", "http://…") points at this server.
    pub fn matches(&self, typed: &str) -> bool {
        let t = typed.trim().split("://").last().unwrap_or("");
        let host = t.split(['/', ':']).next().unwrap_or("").to_ascii_lowercase();
        !host.is_empty() && (self.addresses.iter().any(|a| a.eq_ignore_ascii_case(&host)) || self.conns.iter().any(|(u, _, _)| u.to_ascii_lowercase().contains(&host)))
    }
}

/// Unix seconds -> RFC 3339 (UTC), which is how the app expects dates.
fn rfc3339(secs: i64) -> String {
    let (days, sod) = (secs.div_euclid(86400), secs.rem_euclid(86400));
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!("{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z", sod / 3600, sod % 3600 / 60, sod % 60)
}

/// ISO 639-2 (what the app uses) -> ISO 639-1 (what Plex's subtitle search wants).
fn lang2(code: &str) -> &str {
    match code {
        "eng" => "en",
        "spa" => "es",
        "fre" | "fra" => "fr",
        "ger" | "deu" => "de",
        "ita" => "it",
        "por" => "pt",
        "dut" | "nld" => "nl",
        "rus" => "ru",
        "jpn" => "ja",
        "chi" | "zho" => "zh",
        "kor" => "ko",
        "ara" => "ar",
        "pol" => "pl",
        "swe" => "sv",
        "tur" => "tr",
        other => other,
    }
}

impl Client {
    // ---- account and server choice

    /// Servers this plex.tv account can use. Call on a client that has signed in to plex.tv.
    pub async fn plex_servers(&self) -> Result<Vec<PlexServer>> {
        let v: Value = self
            .preq(reqwest::Method::GET, "https://plex.tv/api/v2/resources".into())
            .query(&[("includeHttps", "1"), ("includeRelay", "1")])
            .send()
            .await?
            .error_for_status()?
            .json()
            .await?;
        Ok(v.as_array()
            .map_or(&[][..], |a| a.as_slice())
            .iter()
            .filter(|r| s(r, "provides").is_some_and(|p| p.split(',').any(|x| x == "server")))
            .filter_map(|r| {
                let conns: Vec<_> = arr(r, "connections").iter().filter_map(|c| Some((s(c, "uri")?, truthy(c, "local"), truthy(c, "relay")))).collect();
                let mut addresses: Vec<String> = arr(r, "connections").iter().filter_map(|c| s(c, "address")).collect();
                addresses.dedup();
                Some(PlexServer { name: s(r, "name").unwrap_or_default(), owned: truthy(r, "owned"), token: s(r, "accessToken")?, conns, addresses })
            })
            .collect())
    }

    /// Finds a working connection to `srv` (LAN first, then remote, then Plex's relay) and
    /// returns a client for it.
    pub async fn plex_connect(&self, srv: &PlexServer) -> Result<Client> {
        let mut cands: Vec<String> = Vec::new();
        for (want_local, want_relay) in [(true, false), (false, false), (false, true)] {
            for (uri, local, relay) in &srv.conns {
                if *local == want_local && *relay == want_relay {
                    cands.push(uri.clone());
                }
            }
        }
        // Some routers block plex.direct names (DNS rebinding protection): also try the bare LAN address.
        for (uri, local, _) in &srv.conns {
            if *local {
                if let Some(rest) = uri.strip_prefix("https://") {
                    let port = rest.rsplit_once(':').map_or("32400", |(_, p)| p.trim_end_matches('/'));
                    for a in &srv.addresses {
                        cands.push(format!("http://{a}:{port}"));
                    }
                }
            }
        }
        cands.dedup();
        let http = reqwest::Client::builder().timeout(Duration::from_secs(4)).build()?;
        let probes = cands.iter().map(|u| {
            let req = http.get(format!("{u}/identity")).header("Accept", "application/json").header("X-Plex-Token", &srv.token);
            async move { req.send().await.ok().is_some_and(|r| r.status().is_success()) }
        });
        let ok = futures_util::future::join_all(probes).await;
        let base = cands.iter().zip(ok).find(|(_, ok)| *ok).map(|(u, _)| u.clone()).ok_or_else(|| Error::Other(format!("Couldn't reach {}. Is it online?", srv.name)))?;
        log(&format!("plex: connected via {}", if base.contains(".plex.direct") { "plex.direct" } else { "address" }));
        let mut c = self.clone();
        c.base = base;
        c.token = Some(srv.token.clone());
        c.plex_dur = Default::default();
        c.plex_part = Default::default();
        Ok(c)
    }

    /// Whether this token administers the server (only the owner can read its preferences).
    pub async fn plex_is_owner(&self) -> bool {
        self.pget("/:/prefs", &[]).await.is_ok()
    }

    // ---- server page

    pub(crate) async fn plex_system_info(&self) -> Result<SystemInfo> {
        let (root, upd) = tokio::join!(self.pget("/", &[]), self.pget("/updater/status", &[]));
        let root = root?;
        let mc = &root["MediaContainer"];
        Ok(SystemInfo {
            server_name: s(mc, "friendlyName").unwrap_or_default(),
            version: s(mc, "version").unwrap_or_default(),
            operating_system: [s(mc, "platform"), s(mc, "platformVersion")].into_iter().flatten().reduce(|a, b| format!("{a} {b}")),
            has_pending_restart: false,
            has_update_available: upd.ok().is_some_and(|u| !arr(&u["MediaContainer"], "Release").is_empty()),
            local_address: Some(self.base.clone()),
            wan_address: None,
        })
    }

    pub(crate) async fn plex_sessions(&self) -> Result<Vec<Session>> {
        let v = self.pget("/status/sessions", &[]).await?;
        Ok(metas(&v)
            .into_iter()
            .map(|m| {
                let player = &m["Player"];
                Session {
                    id: s(&m["Session"], "id").unwrap_or_default(),
                    user_name: s(&m["User"], "title"),
                    client: s(player, "product"),
                    device_name: s(player, "title"),
                    now_playing_item: Some(self.pitem(m)),
                    play_state: Some(PlayState { is_paused: s(player, "state").as_deref() == Some("paused"), position_ticks: n(m, "viewOffset").map(|o| o * MS_TO_TICKS) }),
                    supports_remote_control: false,
                }
            })
            .collect())
    }

    /// Plex can only stop someone else's stream (and only with Plex Pass).
    pub(crate) async fn plex_terminate(&self, session: &str) -> Result<()> {
        self.pcall(reqwest::Method::GET, "/status/sessions/terminate", &[("sessionId", session), ("reason", "Stopped by the server admin")]).await
    }

    pub(crate) async fn plex_tasks(&self) -> Result<Vec<Task>> {
        let v = self.pget("/butler", &[]).await?;
        let list = v.pointer("/ButlerTasks/ButlerTask").or_else(|| v.pointer("/MediaContainer/ButlerTask"));
        Ok(list
            .and_then(|l| l.as_array())
            .map_or(&[][..], |a| a.as_slice())
            .iter()
            .filter(|t| truthy(t, "enabled"))
            .filter_map(|t| Some(Task { id: s(t, "name")?, name: s(t, "title").or_else(|| s(t, "name")).unwrap_or_default(), state: "Scheduled".into(), category: Some("Maintenance".into()), current_progress_percentage: None }))
            .collect())
    }

    pub(crate) async fn plex_task(&self, name: &str, start: bool) -> Result<()> {
        let m = if start { reqwest::Method::POST } else { reqwest::Method::DELETE };
        self.pcall(m, &format!("/butler/{}", encode_segment(name)), &[]).await
    }

    pub(crate) async fn plex_refresh_all(&self) -> Result<()> {
        self.pcall(reqwest::Method::GET, "/library/sections/all/refresh", &[]).await
    }

    pub(crate) async fn plex_folders(&self) -> Result<Vec<VirtualFolder>> {
        let v = self.pget("/library/sections", &[]).await?;
        Ok(arr(&v["MediaContainer"], "Directory")
            .iter()
            .map(|d| VirtualFolder { name: s(d, "title").unwrap_or_default(), locations: arr(d, "Location").iter().filter_map(|l| s(l, "path")).collect(), collection_type: s(d, "type") })
            .collect())
    }

    /// Recent plays across all accounts ("Sam played Dune").
    pub(crate) async fn plex_activity(&self, limit: u32) -> Result<Vec<ActivityEntry>> {
        let size = limit.to_string();
        let q = [("sort", "viewedAt:desc"), ("X-Plex-Container-Size", size.as_str())];
        let (hist, accts) = tokio::join!(self.pget("/status/sessions/history/all", &q), self.pget("/accounts", &[]));
        let accts = accts.unwrap_or(Value::Null);
        let who = |id: Option<String>| arr(&accts["MediaContainer"], "Account").iter().find(|a| s(a, "id") == id).and_then(|a| s(a, "name")).filter(|n| !n.is_empty());
        let hist = hist?;
        Ok(metas(&hist)
            .into_iter()
            .map(|m| {
                let title = match s(m, "grandparentTitle") {
                    Some(show) => format!("{show}: {}", s(m, "title").unwrap_or_default()),
                    None => s(m, "title").unwrap_or_default(),
                };
                ActivityEntry {
                    name: format!("{} played {title}", who(s(m, "accountID")).unwrap_or_else(|| "Someone".into())),
                    short_overview: None,
                    date: n(m, "viewedAt").map(rfc3339),
                    severity: None,
                }
            })
            .collect())
    }

    // ---- subtitles

    pub(crate) async fn plex_search_subs(&self, item: &str, lang: &str) -> Result<Vec<RemoteSubtitle>> {
        let v = self.pget(&format!("/library/metadata/{item}/subtitles"), &[("language", lang2(lang)), ("hearingImpaired", "0"), ("forced", "0")]).await?;
        Ok(arr(&v["MediaContainer"], "Stream")
            .iter()
            .filter_map(|st| {
                Some(RemoteSubtitle {
                    id: format!("{}\n{}\n{}", s(st, "key")?, s(st, "codec").unwrap_or_default(), s(st, "providerTitle").unwrap_or_default()),
                    provider_name: s(st, "providerTitle"),
                    name: s(st, "displayTitle").or_else(|| s(st, "title")).unwrap_or_default(),
                    format: s(st, "codec"),
                    author: None,
                    download_count: None,
                    is_hash_match: st.get("perfectMatch").map(|_| truthy(st, "perfectMatch")),
                    community_rating: f(st, "score").map(|x| x as f32),
                })
            })
            .collect())
    }

    pub(crate) async fn plex_download_sub(&self, item: &str, id: &str) -> Result<()> {
        let mut parts = id.split('\n');
        let (key, codec, provider) = (parts.next().unwrap_or(""), parts.next().unwrap_or(""), parts.next().unwrap_or(""));
        self.pcall(
            reqwest::Method::PUT,
            &format!("/library/metadata/{item}/subtitles"),
            &[("key", key), ("codec", codec), ("providerTitle", provider), ("forced", "0"), ("hearingImpaired", "0")],
        )
        .await
    }

    /// For Auto quality: a ranged download of the file about to play (the item fetched last).
    pub(crate) fn plex_bandwidth_request(&self, bytes: usize) -> Option<reqwest::RequestBuilder> {
        let part = self.plex_part.lock().unwrap().clone()?;
        Some(self.preq(reqwest::Method::GET, format!("{}{}", self.base, part)).header("Range", format!("bytes=0-{}", bytes.saturating_sub(1))))
    }
}

#[cfg(test)]
mod more_tests {
    use super::*;

    #[test]
    fn converts_unix_time() {
        assert_eq!(rfc3339(0), "1970-01-01T00:00:00Z");
        assert_eq!(rfc3339(1_700_000_000), "2023-11-14T22:13:20Z");
        assert_eq!(rfc3339(951_782_400), "2000-02-29T00:00:00Z");
    }

    #[test]
    fn matches_typed_addresses() {
        let srv = PlexServer { name: "nas".into(), owned: true, token: "t".into(), conns: vec![("https://192-168-1-5.abc.plex.direct:32400".into(), true, false)], addresses: vec!["192.168.1.5".into()] };
        assert!(srv.matches("192.168.1.5:32400"));
        assert!(srv.matches("http://192.168.1.5"));
        assert!(!srv.matches("192.168.1.6"));
        assert!(!srv.matches(""));
    }
}
