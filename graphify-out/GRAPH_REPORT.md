# Graph Report - Foyer  (2026-11-13)

## Corpus Check
- Corpus is ~41,330 words - fits in a single context window. You may not need a graph.

## Summary
- 920 nodes · 1988 edges · 40 communities (35 shown, 5 thin omitted)
- Extraction: 98% EXTRACTED · 2% INFERRED · 0% AMBIGUOUS · INFERRED: 48 edges (avg confidence: 0.75)
- Token cost: 0 input · 0 output

## Community Hubs (Navigation)
- Community 0
- Community 1
- Community 2
- Community 3
- Community 4
- Community 5
- Community 6
- Community 7
- Community 8
- Community 9
- Community 10
- Community 11
- Community 12
- Community 13
- Community 14
- Community 15
- Community 16
- Community 17
- Community 18
- Community 19
- Community 20
- Community 21
- Community 22
- Community 23
- Community 24
- Community 25
- Community 26
- Community 27
- Community 28
- Community 29
- Community 30
- Community 31
- Community 32
- Community 33
- Community 34
- Community 35
- Community 36
- Community 37
- Community 38

## God Nodes (most connected - your core abstractions)
1. `Client` - 80 edges
2. `Ctx` - 75 edges
3. `main()` - 56 edges
4. `Ctx` - 50 edges
5. `Result` - 48 edges
6. `Seerr` - 48 edges
7. `Item` - 43 edges
8. `Result` - 35 edges
9. `play_one()` - 25 edges
10. `Page` - 23 edges

## Surprising Connections (you probably didn't know these)
- `libmpv .so with MediaCodec (ExoPlayer fallback)` --semantically_similar_to--> `libmpv playback (hw decoding, cache, direct play, transcode presets)`  [INFERRED] [semantically similar]
  docs/ANDROID-PLAN.md → README.md
- `set_tray()` --calls--> `applog()`  [INFERRED]
  crates/app/src/notes.rs → crates/app/src/main.rs
- `Android and Android TV plan` --conceptually_related_to--> `Foyer (native Emby client)`  [INFERRED]
  docs/ANDROID-PLAN.md → README.md
- `Signed APK self-update` --semantically_similar_to--> `Signed self-update (ECDSA P-256)`  [EXTRACTED] [semantically similar]
  docs/ANDROID-PLAN.md → README.md
- `hidden()` --references--> `Config`  [INFERRED]
  crates/app/src/hide.rs → crates/app/src/main.rs

## Import Cycles
- None detected.

## Hyperedges (group relationships)
- **Foyer Cargo workspace crates** — readme_crate_app, readme_crate_emby_api, readme_crate_seerr_api, readme_crate_player [EXTRACTED 0.95]
- **Platform-specific areas needing Android replacements** — docs_android_plan_libmpv_android, docs_android_plan_apk_update, docs_android_plan_material_you, docs_android_plan_background_notifications [INFERRED 0.75]
- **Playback safety mechanisms** — plan_playback_state_machine, plan_fixemby_patches, plan_self_contained_design [INFERRED 0.75]

## Communities (40 total, 5 thin omitted)

### Community 0 - "Community 0"
Cohesion: 0.06
Nodes (25): applog(), apply_accent(), apply_material(), card_for(), CardData, cards_model(), Ctx, episode_row() (+17 more)

### Community 1 - "Community 1"
Cohesion: 0.07
Nodes (43): yes(), Api, api(), available(), DEFAULT_OPTIONS, dll_candidates(), END_EOF, END_ERROR (+35 more)

### Community 2 - "Community 2"
Cohesion: 0.11
Nodes (3): dlog(), Ctx, nice_date()

### Community 3 - "Community 3"
Cohesion: 0.06
Nodes (30): Asset, find_dll(), install(), Release, RELEASES, Api, ApiAsset, ASSET (+22 more)

### Community 4 - "Community 4"
Cohesion: 0.05
Nodes (46): Android and Android TV plan, Android TV (D-pad focus, 10-foot layouts), APK signing key backup, Signed APK self-update, Background notifications (new feature on Android), CI Android job publishing exe and apk, libmpv .so with MediaCodec (ExoPlayer fallback), Material You accent colour (+38 more)

### Community 5 - "Community 5"
Cohesion: 0.05
Nodes (30): auto_hw_mode(), AUTOPLAY_STREAK_LIMIT, cached_image(), CARD_SCALES, ChanData, clock(), DEBUG, default_audio() (+22 more)

### Community 6 - "Community 6"
Cohesion: 0.05
Nodes (31): ADMIN, AUTO_APPROVE, CREATE_ISSUES, Download, DownloadEp, downloads_label(), ISSUE_OPEN, ISSUE_RESOLVED (+23 more)

### Community 7 - "Community 7"
Cohesion: 0.06
Nodes (20): Media, sync(), Control, AddSubtitle, AudioDelayBy, CycleAudio, CycleSubtitles, Finish (+12 more)

### Community 8 - "Community 8"
Cohesion: 0.07
Nodes (19): audio_streams(), ActivityEntry, AuthResult, CLIENT_NAME, CLIENT_VERSION, discover(), DiscoveredServer, FIELDS (+11 more)

### Community 9 - "Community 9"
Cohesion: 0.09
Nodes (11): Ctx, FOLLOWED, init_tray_events(), INSTANCE_ADDR, MAX_NOTES, serve_instance(), set_tray(), show_window() (+3 more)

### Community 10 - "Community 10"
Cohesion: 0.15
Nodes (3): log(), Result, Seerr

### Community 11 - "Community 11"
Cohesion: 0.12
Nodes (5): next_up_items(), Browse, Chapter, Item, Result

### Community 12 - "Community 12"
Cohesion: 0.07
Nodes (28): run, GH_TOKEN, NOTES, RUSTFLAGS, UPDATE_SIGNING_KEY, notes, jobs, release (+20 more)

### Community 14 - "Community 14"
Cohesion: 0.10
Nodes (19): parse(), RemoteEvent, FastForward, Message, Mute, Next, Pause, Play (+11 more)

### Community 15 - "Community 15"
Cohesion: 0.13
Nodes (17): auto_steps_down_when_link_is_slow(), decide(), keep_display_on(), play_one(), PlayResult, Done, Failed, Next (+9 more)

### Community 16 - "Community 16"
Cohesion: 0.12
Nodes (13): EDIT_BITS, ISSUE_FILTERS, labels_request_status(), load_seen(), media_badge(), media_card(), NOTIFY_INTERVALS, REQ_FILTERS (+5 more)

### Community 17 - "Community 17"
Cohesion: 0.10
Nodes (20): Page, Detail, Home, Library, LiveTv, Notifications, Person, SeerrBrowse (+12 more)

### Community 18 - "Community 18"
Cohesion: 0.14
Nodes (17): Config, data_dir(), debug_banner(), default_accent_mode(), default_accent_rgb(), default_card_scale(), default_forward_secs(), default_notify_interval() (+9 more)

### Community 19 - "Community 19"
Cohesion: 0.16
Nodes (5): Comment, Issue, Person, PublicSettings, User

### Community 20 - "Community 20"
Cohesion: 0.12
Nodes (16): Slot, Backdrop, Cast, DetailLogo, Home, Logo, Poster, Related (+8 more)

### Community 21 - "Community 21"
Cohesion: 0.15
Nodes (8): AUDIO_PICK, goto_with(), LibView, NowPlaying, PlayEnv, SKIP, SkipState, State

### Community 22 - "Community 22"
Cohesion: 0.14
Nodes (5): log(), RemoteSubtitle, SystemInfo, Task, VirtualFolder

### Community 23 - "Community 23"
Cohesion: 0.20
Nodes (4): Media, Page, PersonCredits, WatchItem

### Community 24 - "Community 24"
Cohesion: 0.24
Nodes (3): Ctx, Hide, HideTarget

### Community 25 - "Community 25"
Cohesion: 0.18
Nodes (6): SIssue, SReq, DetailData, IssueData, IssueDetailData, ReqData

### Community 26 - "Community 26"
Cohesion: 0.18
Nodes (10): DetailCtx, SeerrState, TitleInfo, UserEdit, Discover, Movies, Trending, Tv (+2 more)

### Community 27 - "Community 27"
Cohesion: 0.22
Nodes (8): de_id(), de_opt_id(), Error, Http, Io, NoSession, Other, LiveStream

### Community 28 - "Community 28"
Cohesion: 0.25
Nodes (5): Cast, Credits, Detail, Genre, Season

### Community 29 - "Community 29"
Cohesion: 0.33
Nodes (4): apply_image(), home_order(), home_rows_model(), set_card_image()

### Community 30 - "Community 30"
Cohesion: 0.33
Nodes (6): Error, BadLogin, Forbidden, Network, NotSignedIn, Server

### Community 31 - "Community 31"
Cohesion: 0.40
Nodes (5): hidden(), Item, describe_eps(), ep(), summarises_new_episodes_in_ranges()

### Community 32 - "Community 32"
Cohesion: 0.33
Nodes (5): ImageKind, Backdrop, Logo, Primary, Thumb

### Community 33 - "Community 33"
Cohesion: 0.40
Nodes (5): RequestAction, Approve, Decline, Delete, Retry

### Community 34 - "Community 34"
Cohesion: 0.50
Nodes (3): fmt_bps(), fmt_size(), media_info()

### Community 35 - "Community 35"
Cohesion: 0.50
Nodes (4): emby-api, foyer, player, seerr-api

## Knowledge Gaps
- **107 isolated node(s):** `Auto`, `Direct`, `Home`, `Backdrop`, `Poster` (+102 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 284 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **5 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `Client` connect `Community 13` to `Community 0`, `Community 32`, `Community 36`, `Community 5`, `Community 8`, `Community 11`, `Community 14`, `Community 15`, `Community 21`, `Community 22`, `Community 27`?**
  _High betweenness centrality (0.138) - this node is a cross-community bridge._
- **What connects `Auto`, `Direct`, `Home` to the rest of the system?**
  _107 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Community 0` be split into smaller, more focused modules?**
  _Cohesion score 0.06013745704467354 - nodes in this community are weakly interconnected._
- **Why does `Seerr` connect `Community 10` to `Community 2`, `Community 37`, `Community 6`, `Community 16`, `Community 19`, `Community 23`, `Community 26`, `Community 28`?**
  _High betweenness centrality (0.096) - this node is a cross-community bridge._
- **Should `Community 1` be split into smaller, more focused modules?**
  _Cohesion score 0.06610259122157588 - nodes in this community are weakly interconnected._
- **Why does `Ctx` connect `Community 0` to `Community 2`, `Community 3`, `Community 5`, `Community 15`, `Community 18`, `Community 21`?**
  _High betweenness centrality (0.081) - this node is a cross-community bridge._
- **Should `Community 2` be split into smaller, more focused modules?**
  _Cohesion score 0.10902255639097744 - nodes in this community are weakly interconnected._