# Graph Report - Foyer  (2026-11-14)

## Corpus Check
- 27 files · ~51,639 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 6 file(s) not represented in the graph (top: (none) 3, .rc 1, .ico 1)

## Summary
- 1070 nodes · 2419 edges · 43 communities (38 shown, 5 thin omitted)
- Extraction: 96% EXTRACTED · 4% INFERRED · 0% AMBIGUOUS · INFERRED: 97 edges (avg confidence: 0.67)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `c3a38316`
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
- .plex_folders
- User
- .plex_tasks
- State
- User
- Media
- super
- Ctx
- Foyer 0.22.0
- Slot
- Detail
- Discover
- Error
- ImageKind
- .send
- MediaStream
- foyer
- save_seen
- Client
- AGENTS.md
- applog
- AppWindow
- Quality

## God Nodes (most connected - your core abstractions)
1. `Client` - 91 edges
2. `Ctx` - 85 edges
3. `main()` - 65 edges
4. `Item` - 65 edges
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

## Communities (43 total, 5 thin omitted)

### Community 0 - "SharedString"
Cohesion: 0.18
Nodes (6): SIssue, SReq, DetailData, IssueData, IssueDetailData, ReqData

### Community 1 - "player/src/lib.rs"
Cohesion: 0.06
Nodes (48): yes(), Api, api(), available(), DEFAULT_OPTIONS, dll_candidates(), END_EOF, END_ERROR (+40 more)

### Community 2 - "Ctx"
Cohesion: 0.09
Nodes (10): card_for(), CardData, cards_model(), episode_row(), episode_tag(), Episode, Grid, to_card() (+2 more)

### Community 3 - "update.rs"
Cohesion: 0.06
Nodes (30): Asset, find_dll(), install(), Release, RELEASES, Api, ApiAsset, ASSET (+22 more)

### Community 4 - "Foyer (native Emby client)"
Cohesion: 0.05
Nodes (46): Android and Android TV plan, Android TV (D-pad focus, 10-foot layouts), APK signing key backup, Signed APK self-update, Background notifications (new feature on Android), CI Android job publishing exe and apk, libmpv .so with MediaCodec (ExoPlayer fallback), Material You accent colour (+38 more)

### Community 5 - "main.rs"
Cohesion: 0.04
Nodes (41): apply_accent(), apply_image(), AUDIO_PICK, auto_hw_mode(), AUTOPLAY_STREAK_LIMIT, cached_image(), CARD_SCALES, ChanData (+33 more)

### Community 6 - "seerr-api/src/lib.rs"
Cohesion: 0.05
Nodes (31): ADMIN, AUTO_APPROVE, CREATE_ISSUES, Download, DownloadEp, downloads_label(), ISSUE_OPEN, ISSUE_RESOLVED (+23 more)

### Community 7 - "Control"
Cohesion: 0.06
Nodes (22): Media, sync(), Control, AddSubtitle, AudioDelayBy, CycleAudio, CycleSubtitles, Finish (+14 more)

### Community 8 - "emby-api/src/lib.rs"
Cohesion: 0.05
Nodes (28): Chapter, CLIENT_NAME, CLIENT_VERSION, de_id(), de_opt_id(), detect(), discover(), DiscoveredServer (+20 more)

### Community 9 - "notes.rs"
Cohesion: 0.07
Nodes (17): Ctx, Ctx, Follow, FOLLOWED, init_tray_events(), INSTANCE_ADDR, load_notes(), MAX_NOTES (+9 more)

### Community 10 - "Seerr"
Cohesion: 0.14
Nodes (7): Person, PublicSettings, Quota, QuotaPart, Result, Seerr, WatchItem

### Community 11 - "Result"
Cohesion: 0.11
Nodes (4): Browse, Item, Client, section_key()

### Community 12 - "steps"
Cohesion: 0.07
Nodes (28): run, GH_TOKEN, NOTES, RUSTFLAGS, UPDATE_SIGNING_KEY, notes, jobs, release (+20 more)

### Community 13 - "Client"
Cohesion: 0.08
Nodes (4): next_up_items(), Client, RemoteSubtitle, Result

### Community 14 - "RemoteEvent"
Cohesion: 0.10
Nodes (19): parse(), RemoteEvent, FastForward, Message, Mute, Next, Pause, Play (+11 more)

### Community 15 - "play_one"
Cohesion: 0.16
Nodes (13): keep_display_on(), play_one(), PlayResult, Done, Failed, Next, Restart, Rep (+5 more)

### Community 16 - "seerr.rs"
Cohesion: 0.12
Nodes (11): EDIT_BITS, ep(), ISSUE_FILTERS, labels_request_status(), media_badge(), media_card(), NOTIFY_INTERVALS, REQ_FILTERS (+3 more)

### Community 17 - "Page"
Cohesion: 0.10
Nodes (21): Page, Detail, Genre, Home, Library, LiveTv, Notifications, Person (+13 more)

### Community 19 - "User"
Cohesion: 0.22
Nodes (3): Comment, Issue, User

### Community 21 - "State"
Cohesion: 0.16
Nodes (5): goto_with(), LibView, NowPlaying, PlayEnv, State

### Community 22 - "User"
Cohesion: 0.21
Nodes (3): AuthResult, Policy, User

### Community 23 - "Media"
Cohesion: 0.25
Nodes (3): Media, Page, PersonCredits

### Community 24 - "super"
Cohesion: 0.29
Nodes (5): hidden(), Hide, HideTarget, Item, describe_eps()

### Community 25 - "Ctx"
Cohesion: 0.08
Nodes (8): Ctx, fav_group(), goto_episode(), main(), Search, parse_hex(), save_config(), windows_accent()

### Community 26 - "Foyer 0.22.0"
Cohesion: 0.08
Nodes (25): 🛠 Admin, Changelog, 🐞 Fixes, 🐞 Fixes, Foyer 0.22.0, Foyer 0.23.0, Foyer 0.23.1, Foyer 0.23.2 (+17 more)

### Community 27 - "Slot"
Cohesion: 0.12
Nodes (17): Slot, Backdrop, Cast, DetailLogo, Home, Logo, Poster, Related (+9 more)

### Community 28 - "Detail"
Cohesion: 0.25
Nodes (5): Cast, Credits, Detail, Genre, Season

### Community 29 - "Discover"
Cohesion: 0.18
Nodes (10): DetailCtx, SeerrState, TitleInfo, UserEdit, Discover, Movies, Trending, Tv (+2 more)

### Community 30 - "Error"
Cohesion: 0.33
Nodes (6): Error, BadLogin, Forbidden, Network, NotSignedIn, Server

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
Cohesion: 0.13
Nodes (19): ActivityEntry, log(), SystemInfo, arr(), f(), IDENT, lang2(), maps_an_episode() (+11 more)

### Community 44 - "applog"
Cohesion: 0.18
Nodes (12): applog(), Config, data_dir(), debug_banner(), dlog(), home_order(), home_rows_model(), load_config() (+4 more)

### Community 47 - "AppWindow"
Cohesion: 0.21
Nodes (3): apply_material(), login_error(), skip_step_index()

### Community 49 - "Quality"
Cohesion: 0.16
Nodes (11): auto_steps_down_when_link_is_slow(), decide(), explicit_choices_and_config_roundtrip(), Quality, Auto, Direct, quality_label(), Max (+3 more)

## Knowledge Gaps
- **113 isolated node(s):** `Auto`, `Direct`, `Home`, `Backdrop`, `Poster` (+108 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 327 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **5 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Client` connect `Client` to `ImageKind`, `Ctx`, `MediaStream`, `main.rs`, `Client`, `emby-api/src/lib.rs`, `Result`, `RemoteEvent`, `play_one`, `AppWindow`, `.plex_folders`, `.plex_tasks`, `State`, `User`, `Ctx`?**
  _High betweenness centrality (0.133) - this node is a cross-community bridge._
- **Are the 20 inferred relationships involving `Item` (e.g. with `.pitem()` and `.pitems()`) actually correct?**
  _`Item` has 20 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Auto`, `Direct`, `Home` to the rest of the system?**
  _113 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `player/src/lib.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.06233538191395961 - nodes in this community are weakly interconnected._
- **Why does `Ctx` connect `Ctx` to `Ctx`, `update.rs`, `main.rs`, `notes.rs`, `applog`, `Client`, `AppWindow`, `play_one`, `Quality`, `State`?**
  _High betweenness centrality (0.095) - this node is a cross-community bridge._
- **Should `Ctx` be split into smaller, more focused modules?**
  _Cohesion score 0.08703703703703704 - nodes in this community are weakly interconnected._
- **Why does `Seerr` connect `Seerr` to `.send`, `Ctx`, `seerr-api/src/lib.rs`, `seerr.rs`, `User`, `Media`, `Detail`, `Discover`?**
  _High betweenness centrality (0.067) - this node is a cross-community bridge._