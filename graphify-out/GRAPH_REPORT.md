# Graph Report - Foyer  (2026-11-13)

## Corpus Check
- 26 files · ~44,084 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 6 file(s) not represented in the graph (top: (none) 3, .rc 1, .ico 1)

## Summary
- 949 nodes · 2030 edges · 44 communities (36 shown, 8 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 48 edges (avg confidence: 0.75)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `af22ed2f`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Ctx
- player/src/lib.rs
- Ctx
- update.rs
- Foyer (native Emby client)
- main.rs
- seerr-api/src/lib.rs
- Control
- emby-api/src/lib.rs
- notes.rs
- Seerr
- Result
- steps
- Client
- RemoteEvent
- play_one
- seerr.rs
- Page
- .default
- .get
- Slot
- State
- .get
- Media
- Config
- media_info
- Foyer 0.22.0
- Error
- Detail
- apply_image
- Error
- SharedString
- ImageKind
- RequestAction
- .visible
- foyer
- .browse
- .user_quota
- rgb_of
- Discover
- AGENTS.md
- save_seen

## God Nodes (most connected - your core abstractions)
1. `Client` - 83 edges
2. `Ctx` - 75 edges
3. `main()` - 57 edges
4. `Ctx` - 51 edges
5. `Result` - 51 edges
6. `Seerr` - 48 edges
7. `Item` - 45 edges
8. `Result` - 35 edges
9. `play_one()` - 25 edges
10. `Control` - 25 edges

## Surprising Connections (you probably didn't know these)
- `libmpv .so with MediaCodec (ExoPlayer fallback)` --semantically_similar_to--> `libmpv playback (hw decoding, cache, direct play, transcode presets)`  [INFERRED] [semantically similar]
  docs/ANDROID-PLAN.md → README.md
- `Signed APK self-update` --semantically_similar_to--> `Signed self-update (ECDSA P-256)`  [EXTRACTED] [semantically similar]
  docs/ANDROID-PLAN.md → README.md
- `Android and Android TV plan` --conceptually_related_to--> `Foyer (native Emby client)`  [INFERRED]
  docs/ANDROID-PLAN.md → README.md
- `hidden()` --references--> `Config`  [INFERRED]
  crates/app/src/hide.rs → crates/app/src/main.rs
- `notes_path()` --calls--> `data_dir()`  [INFERRED]
  crates/app/src/notes.rs → crates/app/src/main.rs

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Foyer Cargo workspace crates** — readme_crate_app, readme_crate_emby_api, readme_crate_seerr_api, readme_crate_player [EXTRACTED 0.95]
- **Platform-specific areas needing Android replacements** — docs_android_plan_libmpv_android, docs_android_plan_apk_update, docs_android_plan_material_you, docs_android_plan_background_notifications [INFERRED 0.75]
- **Playback safety mechanisms** — plan_playback_state_machine, plan_fixemby_patches, plan_self_contained_design [INFERRED 0.75]

## Communities (44 total, 8 thin omitted)

### Community 0 - "Ctx"
Cohesion: 0.06
Nodes (25): applog(), apply_material(), Ctx, data_dir(), explicit_choices_and_config_roundtrip(), goto_episode(), load_config(), load_posters() (+17 more)

### Community 1 - "player/src/lib.rs"
Cohesion: 0.07
Nodes (43): yes(), Api, api(), available(), DEFAULT_OPTIONS, dll_candidates(), END_EOF, END_ERROR (+35 more)

### Community 2 - "Ctx"
Cohesion: 0.09
Nodes (6): cards_model(), dlog(), fav_group(), Episode, Ctx, nice_date()

### Community 3 - "update.rs"
Cohesion: 0.06
Nodes (30): Asset, find_dll(), install(), Release, RELEASES, Api, ApiAsset, ASSET (+22 more)

### Community 4 - "Foyer (native Emby client)"
Cohesion: 0.05
Nodes (46): Android and Android TV plan, Android TV (D-pad focus, 10-foot layouts), APK signing key backup, Signed APK self-update, Background notifications (new feature on Android), CI Android job publishing exe and apk, libmpv .so with MediaCodec (ExoPlayer fallback), Material You accent colour (+38 more)

### Community 5 - "main.rs"
Cohesion: 0.06
Nodes (26): apply_accent(), auto_hw_mode(), AUTOPLAY_STREAK_LIMIT, cached_image(), CARD_SCALES, ChanData, clock(), DEBUG (+18 more)

### Community 6 - "seerr-api/src/lib.rs"
Cohesion: 0.05
Nodes (31): ADMIN, AUTO_APPROVE, CREATE_ISSUES, Download, DownloadEp, downloads_label(), ISSUE_OPEN, ISSUE_RESOLVED (+23 more)

### Community 7 - "Control"
Cohesion: 0.06
Nodes (22): Media, sync(), Control, AddSubtitle, AudioDelayBy, CycleAudio, CycleSubtitles, Finish (+14 more)

### Community 8 - "emby-api/src/lib.rs"
Cohesion: 0.06
Nodes (21): audio_streams(), ActivityEntry, AuthResult, CLIENT_NAME, CLIENT_VERSION, discover(), DiscoveredServer, FIELDS (+13 more)

### Community 9 - "notes.rs"
Cohesion: 0.07
Nodes (16): Ctx, Ctx, Follow, FOLLOWED, init_tray_events(), INSTANCE_ADDR, load_notes(), MAX_NOTES (+8 more)

### Community 10 - "Seerr"
Cohesion: 0.15
Nodes (3): log(), Result, Seerr

### Community 11 - "Result"
Cohesion: 0.10
Nodes (8): card_for(), CardData, episode_row(), episode_tag(), next_up_items(), to_card(), Chapter, Item

### Community 12 - "steps"
Cohesion: 0.07
Nodes (28): run, GH_TOKEN, NOTES, RUSTFLAGS, UPDATE_SIGNING_KEY, notes, jobs, release (+20 more)

### Community 14 - "RemoteEvent"
Cohesion: 0.10
Nodes (19): parse(), RemoteEvent, FastForward, Message, Mute, Next, Pause, Play (+11 more)

### Community 15 - "play_one"
Cohesion: 0.13
Nodes (17): auto_steps_down_when_link_is_slow(), decide(), default_audio(), is_english(), keep_display_on(), play_one(), PlayResult, Done (+9 more)

### Community 16 - "seerr.rs"
Cohesion: 0.11
Nodes (13): DetailCtx, EDIT_BITS, ISSUE_FILTERS, labels_request_status(), media_badge(), media_card(), NOTIFY_INTERVALS, REQ_FILTERS (+5 more)

### Community 17 - "Page"
Cohesion: 0.10
Nodes (21): Page, Detail, Genre, Home, Library, LiveTv, Notifications, Person (+13 more)

### Community 18 - ".default"
Cohesion: 0.29
Nodes (6): default_accent_mode(), default_accent_rgb(), default_card_scale(), default_forward_secs(), default_notify_interval(), default_rewind_secs()

### Community 19 - ".get"
Cohesion: 0.16
Nodes (5): Comment, Issue, Person, PublicSettings, User

### Community 20 - "Slot"
Cohesion: 0.11
Nodes (18): Slot, Backdrop, Cast, DetailLogo, Grid, Home, Logo, Poster (+10 more)

### Community 21 - "State"
Cohesion: 0.20
Nodes (5): AUDIO_PICK, NowPlaying, PlayEnv, SKIP, SkipState

### Community 22 - ".get"
Cohesion: 0.14
Nodes (5): log(), RemoteSubtitle, SystemInfo, Task, VirtualFolder

### Community 23 - "Media"
Cohesion: 0.20
Nodes (4): Media, Page, PersonCredits, WatchItem

### Community 24 - "Config"
Cohesion: 0.16
Nodes (9): Hide, HideTarget, Config, debug_banner(), goto_with(), home_order(), home_rows_model(), LibView (+1 more)

### Community 25 - "media_info"
Cohesion: 0.50
Nodes (3): fmt_bps(), fmt_size(), media_info()

### Community 26 - "Foyer 0.22.0"
Cohesion: 0.12
Nodes (16): 🛠 Admin, Changelog, Foyer 0.22.0, Foyer 0.23.0, Foyer 0.23.1, Foyer 0.23.2, ✨ Highlights, 🏠 Home (+8 more)

### Community 27 - "Error"
Cohesion: 0.22
Nodes (8): de_id(), de_opt_id(), Error, Http, Io, NoSession, Other, LiveStream

### Community 28 - "Detail"
Cohesion: 0.25
Nodes (5): Cast, Credits, Detail, Genre, Season

### Community 30 - "Error"
Cohesion: 0.33
Nodes (6): Error, BadLogin, Forbidden, Network, NotSignedIn, Server

### Community 31 - "SharedString"
Cohesion: 0.18
Nodes (6): SIssue, SReq, DetailData, IssueData, IssueDetailData, ReqData

### Community 32 - "ImageKind"
Cohesion: 0.33
Nodes (5): ImageKind, Backdrop, Logo, Primary, Thumb

### Community 33 - "RequestAction"
Cohesion: 0.40
Nodes (5): RequestAction, Approve, Decline, Delete, Retry

### Community 34 - ".visible"
Cohesion: 0.40
Nodes (5): hidden(), Item, describe_eps(), ep(), summarises_new_episodes_in_ranges()

### Community 35 - "foyer"
Cohesion: 0.50
Nodes (4): emby-api, foyer, player, seerr-api

### Community 40 - "Discover"
Cohesion: 0.33
Nodes (6): Discover, Movies, Trending, Tv, UpcomingMovies, UpcomingTv

### Community 44 - "save_seen"
Cohesion: 0.67
Nodes (4): load_seen(), save_seen(), Seen, seen_path()

## Knowledge Gaps
- **110 isolated node(s):** `Auto`, `Direct`, `Home`, `Backdrop`, `Poster` (+105 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 304 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **8 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Client` connect `Client` to `Ctx`, `ImageKind`, `Ctx`, `.browse`, `main.rs`, `emby-api/src/lib.rs`, `Result`, `RemoteEvent`, `play_one`, `.get`, `Config`, `Error`?**
  _High betweenness centrality (0.135) - this node is a cross-community bridge._
- **What connects `Auto`, `Direct`, `Home` to the rest of the system?**
  _110 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Ctx` be split into smaller, more focused modules?**
  _Cohesion score 0.05673274094326726 - nodes in this community are weakly interconnected._
- **Why does `Seerr` connect `Seerr` to `Ctx`, `.user_quota`, `seerr-api/src/lib.rs`, `seerr.rs`, `.get`, `Media`, `Detail`?**
  _High betweenness centrality (0.092) - this node is a cross-community bridge._
- **Should `player/src/lib.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.06949152542372881 - nodes in this community are weakly interconnected._
- **Why does `Ctx` connect `Ctx` to `Ctx`, `update.rs`, `main.rs`, `notes.rs`, `Result`, `play_one`, `State`, `Config`?**
  _High betweenness centrality (0.077) - this node is a cross-community bridge._
- **Should `Ctx` be split into smaller, more focused modules?**
  _Cohesion score 0.09224109224109224 - nodes in this community are weakly interconnected._