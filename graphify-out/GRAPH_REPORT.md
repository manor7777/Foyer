# Graph Report - Foyer  (2026-11-13)

## Corpus Check
- 27 files · ~50,224 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 6 file(s) not represented in the graph (top: (none) 3, .rc 1, .ico 1)

## Summary
- 1055 nodes · 2392 edges · 54 communities (43 shown, 11 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 96 edges (avg confidence: 0.67)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `e44f37c5`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- SharedString
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
- User
- .get
- State
- User
- Media
- .visible
- main
- Foyer 0.22.0
- Slot
- Detail
- Discover
- Error
- Mutex
- ImageKind
- .send
- MediaStream
- foyer
- save_seen
- Client
- rgb_of
- req_status
- .show_detail
- AGENTS.md
- Config
- media_info
- Ctx
- AppWindow
- save_config
- Quality
- .update_cfg
- super
- apply_image
- Source

## God Nodes (most connected - your core abstractions)
1. `Client` - 91 edges
2. `Ctx` - 84 edges
3. `Item` - 65 edges
4. `main()` - 64 edges
5. `Ctx` - 51 edges
6. `Result` - 51 edges
7. `Client` - 50 edges
8. `Seerr` - 49 edges
9. `Result` - 36 edges
10. `play_one()` - 25 edges

## Surprising Connections (you probably didn't know these)
- `libmpv .so with MediaCodec (ExoPlayer fallback)` --semantically_similar_to--> `libmpv playback (hw decoding, cache, direct play, transcode presets)`  [INFERRED] [semantically similar]
  docs/ANDROID-PLAN.md → README.md
- `set_tray()` --calls--> `applog()`  [INFERRED]
  crates/app/src/notes.rs → crates/app/src/main.rs
- `Signed APK self-update` --semantically_similar_to--> `Signed self-update (ECDSA P-256)`  [EXTRACTED] [semantically similar]
  docs/ANDROID-PLAN.md → README.md
- `Android and Android TV plan` --conceptually_related_to--> `Foyer (native Emby client)`  [INFERRED]
  docs/ANDROID-PLAN.md → README.md
- `hidden()` --references--> `Config`  [INFERRED]
  crates/app/src/hide.rs → crates/app/src/main.rs

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Foyer Cargo workspace crates** — readme_crate_app, readme_crate_emby_api, readme_crate_seerr_api, readme_crate_player [EXTRACTED 0.95]
- **Platform-specific areas needing Android replacements** — docs_android_plan_libmpv_android, docs_android_plan_apk_update, docs_android_plan_material_you, docs_android_plan_background_notifications [INFERRED 0.75]
- **Playback safety mechanisms** — plan_playback_state_machine, plan_fixemby_patches, plan_self_contained_design [INFERRED 0.75]

## Communities (54 total, 11 thin omitted)

### Community 0 - "SharedString"
Cohesion: 0.18
Nodes (6): SIssue, SReq, DetailData, IssueData, IssueDetailData, ReqData

### Community 1 - "player/src/lib.rs"
Cohesion: 0.07
Nodes (44): yes(), Api, api(), available(), DEFAULT_OPTIONS, dll_candidates(), END_EOF, END_ERROR (+36 more)

### Community 2 - "Ctx"
Cohesion: 0.10
Nodes (5): CardData, cards_model(), to_card(), Ctx, nice_date()

### Community 3 - "update.rs"
Cohesion: 0.06
Nodes (30): Asset, find_dll(), install(), Release, RELEASES, Api, ApiAsset, ASSET (+22 more)

### Community 4 - "Foyer (native Emby client)"
Cohesion: 0.05
Nodes (46): Android and Android TV plan, Android TV (D-pad focus, 10-foot layouts), APK signing key backup, Signed APK self-update, Background notifications (new feature on Android), CI Android job publishing exe and apk, libmpv .so with MediaCodec (ExoPlayer fallback), Material You accent colour (+38 more)

### Community 5 - "main.rs"
Cohesion: 0.06
Nodes (27): auto_hw_mode(), AUTOPLAY_STREAK_LIMIT, cached_image(), CARD_SCALES, ChanData, clock(), DEBUG, explicit_choices_and_config_roundtrip() (+19 more)

### Community 6 - "seerr-api/src/lib.rs"
Cohesion: 0.05
Nodes (31): ADMIN, AUTO_APPROVE, CREATE_ISSUES, Download, DownloadEp, downloads_label(), ISSUE_OPEN, ISSUE_RESOLVED (+23 more)

### Community 7 - "Control"
Cohesion: 0.06
Nodes (22): Media, sync(), Control, AddSubtitle, AudioDelayBy, CycleAudio, CycleSubtitles, Finish (+14 more)

### Community 8 - "emby-api/src/lib.rs"
Cohesion: 0.05
Nodes (28): ActivityEntry, CLIENT_NAME, CLIENT_VERSION, de_id(), de_opt_id(), detect(), discover(), DiscoveredServer (+20 more)

### Community 9 - "notes.rs"
Cohesion: 0.09
Nodes (16): Ctx, Follow, FOLLOWED, init_tray_events(), INSTANCE_ADDR, load_notes(), MAX_NOTES, NoteRec (+8 more)

### Community 10 - "Seerr"
Cohesion: 0.14
Nodes (7): Person, PublicSettings, Quota, QuotaPart, Result, Seerr, WatchItem

### Community 11 - "Result"
Cohesion: 0.11
Nodes (4): Browse, Chapter, Item, section_key()

### Community 12 - "steps"
Cohesion: 0.07
Nodes (28): run, GH_TOKEN, NOTES, RUSTFLAGS, UPDATE_SIGNING_KEY, notes, jobs, release (+20 more)

### Community 14 - "RemoteEvent"
Cohesion: 0.10
Nodes (19): parse(), RemoteEvent, FastForward, Message, Mute, Next, Pause, Play (+11 more)

### Community 15 - "play_one"
Cohesion: 0.15
Nodes (15): auto_steps_down_when_link_is_slow(), decide(), keep_display_on(), play_one(), PlayResult, Done, Failed, Next (+7 more)

### Community 16 - "seerr.rs"
Cohesion: 0.13
Nodes (11): DetailCtx, EDIT_BITS, ISSUE_FILTERS, media_badge(), media_card(), NOTIFY_INTERVALS, REQ_FILTERS, runtime_label() (+3 more)

### Community 17 - "Page"
Cohesion: 0.10
Nodes (20): Page, Detail, Genre, Library, LiveTv, Notifications, Person, SeerrBrowse (+12 more)

### Community 18 - ".default"
Cohesion: 0.29
Nodes (6): default_accent_mode(), default_accent_rgb(), default_card_scale(), default_forward_secs(), default_notify_interval(), default_rewind_secs()

### Community 19 - "User"
Cohesion: 0.22
Nodes (3): Comment, Issue, User

### Community 20 - ".get"
Cohesion: 0.17
Nodes (4): RemoteSubtitle, SystemInfo, Task, VirtualFolder

### Community 21 - "State"
Cohesion: 0.22
Nodes (5): goto_with(), LibView, NowPlaying, PlayEnv, State

### Community 22 - "User"
Cohesion: 0.21
Nodes (3): AuthResult, Policy, User

### Community 23 - "Media"
Cohesion: 0.25
Nodes (3): Media, Page, PersonCredits

### Community 24 - ".visible"
Cohesion: 0.40
Nodes (5): hidden(), Item, describe_eps(), ep(), summarises_new_episodes_in_ranges()

### Community 25 - "main"
Cohesion: 0.13
Nodes (10): applog(), apply_accent(), data_dir(), debug_banner(), dlog(), load_config(), logs_dir(), main() (+2 more)

### Community 26 - "Foyer 0.22.0"
Cohesion: 0.10
Nodes (20): 🛠 Admin, Changelog, Foyer 0.22.0, Foyer 0.23.0, Foyer 0.23.1, Foyer 0.23.2, Foyer 0.24.0, ✨ Highlights (+12 more)

### Community 27 - "Slot"
Cohesion: 0.12
Nodes (17): Slot, Backdrop, Cast, DetailLogo, Home, Logo, Poster, Related (+9 more)

### Community 28 - "Detail"
Cohesion: 0.25
Nodes (5): Cast, Credits, Detail, Genre, Season

### Community 29 - "Discover"
Cohesion: 0.33
Nodes (6): Discover, Movies, Trending, Tv, UpcomingMovies, UpcomingTv

### Community 30 - "Error"
Cohesion: 0.33
Nodes (6): Error, BadLogin, Forbidden, Network, NotSignedIn, Server

### Community 31 - "Mutex"
Cohesion: 0.50
Nodes (3): AUDIO_PICK, SKIP, SkipState

### Community 32 - "ImageKind"
Cohesion: 0.22
Nodes (6): ImageKind, Backdrop, Logo, Primary, Thumb, Person

### Community 33 - ".send"
Cohesion: 0.15
Nodes (6): log(), RequestAction, Approve, Decline, Delete, Retry

### Community 34 - "MediaStream"
Cohesion: 0.38
Nodes (5): audio_streams(), default_audio(), is_english(), MediaSource, MediaStream

### Community 35 - "foyer"
Cohesion: 0.50
Nodes (4): emby-api, foyer, player, seerr-api

### Community 36 - "save_seen"
Cohesion: 0.67
Nodes (4): load_seen(), save_seen(), Seen, seen_path()

### Community 37 - "Client"
Cohesion: 0.12
Nodes (18): log(), arr(), Client, f(), IDENT, lang2(), maps_an_episode(), metas() (+10 more)

### Community 41 - ".show_detail"
Cohesion: 0.17
Nodes (8): card_for(), duration_label(), episode_row(), episode_tag(), next_up_items(), Home, Episode, Grid

### Community 44 - "Config"
Cohesion: 0.47
Nodes (4): Config, home_order(), home_rows_model(), SavedServer

### Community 45 - "media_info"
Cohesion: 0.50
Nodes (3): fmt_bps(), fmt_size(), media_info()

### Community 46 - "Ctx"
Cohesion: 0.21
Nodes (3): Ctx, fav_group(), goto_episode()

### Community 47 - "AppWindow"
Cohesion: 0.21
Nodes (4): apply_material(), load_posters(), login_error(), posters_dir()

### Community 49 - "Quality"
Cohesion: 0.29
Nodes (6): Quality, Auto, Direct, quality_label(), Max, quality_rows()

## Knowledge Gaps
- **113 isolated node(s):** `Auto`, `Direct`, `Home`, `Backdrop`, `Poster` (+108 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 320 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **11 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Client` connect `Client` to `ImageKind`, `Ctx`, `MediaStream`, `main.rs`, `emby-api/src/lib.rs`, `.show_detail`, `Result`, `Ctx`, `AppWindow`, `play_one`, `RemoteEvent`, `.get`, `State`, `User`?**
  _High betweenness centrality (0.140) - this node is a cross-community bridge._
- **Are the 20 inferred relationships involving `Item` (e.g. with `.pitem()` and `.pitems()`) actually correct?**
  _`Item` has 20 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Auto`, `Direct`, `Home` to the rest of the system?**
  _113 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `player/src/lib.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.06885245901639345 - nodes in this community are weakly interconnected._
- **Why does `Ctx` connect `Ctx` to `Ctx`, `update.rs`, `main.rs`, `.show_detail`, `AppWindow`, `save_config`, `Quality`, `.update_cfg`, `play_one`, `State`, `main`, `Mutex`?**
  _High betweenness centrality (0.095) - this node is a cross-community bridge._
- **Should `Ctx` be split into smaller, more focused modules?**
  _Cohesion score 0.09833187006145742 - nodes in this community are weakly interconnected._
- **Why does `Item` connect `Result` to `ImageKind`, `MediaStream`, `update.rs`, `main.rs`, `Client`, `emby-api/src/lib.rs`, `.show_detail`, `media_info`, `Ctx`, `Client`, `.get`, `State`?**
  _High betweenness centrality (0.067) - this node is a cross-community bridge._