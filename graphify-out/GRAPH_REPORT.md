# Graph Report - Foyer  (2026-11-20)

## Corpus Check
- 153 files · ~316,382 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 10 file(s) not represented in the graph (top: (none) 4, .xml 2, .rc 1)

## Summary
- 6996 nodes · 17879 edges · 324 communities (227 shown, 97 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 968 edges (avg confidence: 0.61)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `bb36868f`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- input_items.rs
- player/src/lib.rs
- Ctx
- update.rs
- Foyer (native Emby client)
- app/src/lib.rs
- seerr-api/src/lib.rs
- Control
- emby-api/src/lib.rs
- notes.rs
- Seerr
- Result
- sharedparley.rs
- Client
- RemoteEvent
- LogicalRect
- seerr.rs
- gcast.rs
- window.rs
- ModelTracker
- sharedvector.rs
- Page
- ImageKind
- .get
- Coord
- Ctx
- Foyer 0.22.0
- component_factory.rs
- WindowEvent
- system_tray/windows.rs
- Error
- BorderRadius<T, U>
- LogicalSize
- WindowInner
- foyer
- Quality
- Client
- Color
- adapters.rs
- T
- Android
- super
- SharedString
- tests.rs
- items.rs
- properties/ffi.rs
- format!
- graphics/path.rs
- item_tree.rs
- flickable.rs
- PlatformError
- PhysicalLength
- RenderText
- Slot
- i-slint-core/layout.rs
- DataTransfer
- .show_detail
- items/image.rs
- android.yml
- T
- Item
- drag_n_drop.rs
- input.rs
- VisitChildrenResult
- ModelNotify
- items/path.rs
- CachedRenderingData
- android_media.rs
- platform.rs
- TestItemTree
- ksni.rs
- .load_from_data_with_cache_key
- .get
- translations.rs
- BoxShadowOptions
- TextCursorAffinity
- .compute_dirty_regions
- appkit.rs
- brush.rs
- android.rs
- render_layer
- MouseEvent
- two_way_binding.rs
- steps
- IntSize
- ImageCacheKey
- i-slint-core/string.rs
- properties_animations.rs
- Brush
- pt
- RenderBorderRectangle
- constant_deceleration.rs
- popup.rs
- LogicalLength
- downloads.rs
- linebreaker.rs
- api.rs
- vec
- Rgba8Pixel
- GraphicsAPI
- Repeater<C>
- callbacks.rs
- TextInput
- LogicalPosition
- RendererSealed
- T
- ChangeTracker
- LengthType
- textlayout.rs
- SlintContext
- ConicGradientBrush
- T
- TouchPhase
- Slice
- textlayout/shaping.rs
- spring.rs
- Cast
- tvnav.rs
- gdb_pretty_printers.py
- timers.rs
- wgpu_29.rs
- TextCursorDirection
- AccessibilityAction
- step
- .evaluate
- ItemRc
- wgpu_30.rs
- graphics.rs
- fragments.rs
- rtti.rs
- RepeaterInstanceOpsVTable
- Download
- PartialRenderer<'_, T>
- RepeatedItemTree
- data_transfer.rs
- .find_sibling
- LayoutData
- OnceCell
- AccessibleStringProperty
- InternalKeyEvent
- TouchState
- desktop.rs
- .insert
- StyledText
- Image
- Instant
- MouseInputState
- .fmt
- DependencyNode
- MenuFromItemTree
- erased_bindings.rs
- lengths.rs
- solve_flexbox_layout_with_measure
- Layout
- PlatformTray
- Property
- platform/windows.rs
- FontMetrics
- LogicalPoint
- debug_log.rs
- DirtyRegion
- NoVerify
- PropertyTracker
- .window_item
- .from_pub
- data_ringbuffer.rs
- Any
- partial_renderer.rs
- Changelog
- android_notify.rs
- play_inner
- date_time.rs
- animations.rs
- c_void
- Error
- ScaleFactor
- Length
- CheckedAdd
- Surface
- .evaluate
- rendering_metrics_collector.rs
- Scalar
- Kind
- Slint Software License
- media.rs
- clipped_screen_rect
- InterpolatedPropertyValue
- Immersive
- item_tree.rs
- MediaCb
- TimerList
- KeyboardModifiers
- mod.rs
- steps
- graphics/image.rs
- RequestedGraphicsAPI
- FittedPathBox
- SingleLinkedListPinHead<T>
- .init
- TwoWayBindingMapping
- winit_ops.rs
- SurfaceTarget
- WGPUSettings
- WGPUSettings
- StandardShortcut
- phantomdata
- Export.java
- preview.rs
- DragRequest
- WindowPosition
- SystemTrayIconDataBox
- Surface
- User
- T
- CloseRequestResponse
- Renderer
- LineBreakIterator<'a>
- KeyEventType
- .plex_folders
- Conditional<C>
- .next
- benches/string.rs
- android
- TimerListRc
- TextLineBreaker
- LyonPathEventIterator
- pathbuf
- .next
- steps
- bitmapfont.rs
- Inner
- env
- TextCursorBlinker
- Community 240
- FixedTestFont
- Slint Royalty-free Desktop, Mobile, and Web Applications License
- CrossAxisSizing
- SurfaceTarget
- armv7_only
- Saturating
- BreakOpportunity
- TextureImportError
- TextureImportError
- Foyer 0.24.0
- Foyer's patch to i-slint-core 1.18.1
- WindowAdapterInternal
- Foyer 0.26.3
- Foyer 0.26.2
- Foyer 0.25.1
- Foyer 0.25.0
- Foyer 0.26.0
- main
- Texture
- Texture
- secrets
- BackendMouseEvent
- Foyer 0.26.4
- Foyer 0.24.1
- build-mediacb.sh
- RepeaterTracker<C>
- time::Instant
- README.md
- WithUserData<T>
- item_focus.rs
- Detail

## God Nodes (most connected - your core abstractions)
1. `ItemRc` - 516 edges
2. `SharedString` - 160 edges
3. `Coord` - 139 edges
4. `Item` - 120 edges
5. `TextInput` - 113 edges
6. `MouseEvent` - 113 edges
7. `Property` - 107 edges
8. `WindowInner` - 101 edges
9. `Client` - 98 edges
10. `Ctx` - 94 edges

## Surprising Connections (you probably didn't know these)
- `focus_start()` --references--> `key`  [INFERRED]
  crates/app/src/tvnav.rs → .github/workflows/android.yml
- `libmpv .so with MediaCodec (ExoPlayer fallback)` --semantically_similar_to--> `libmpv playback (hw decoding, cache, direct play, transcode presets)`  [INFERRED] [semantically similar]
  docs/ANDROID-PLAN.md → README.md
- `with_global_context()` --calls--> `f()`  [INFERRED]
  vendor/i-slint-core/context.rs → crates/emby-api/src/plex.rs
- `with_text_input_layout_impl()` --calls--> `f()`  [INFERRED]
  vendor/i-slint-core/textlayout/sharedparley.rs → crates/emby-api/src/plex.rs
- `with_text_layout()` --calls--> `f()`  [INFERRED]
  vendor/i-slint-core/textlayout/sharedparley.rs → crates/emby-api/src/plex.rs

## Import Cycles
- 1-file cycle: `vendor/i-slint-core/window.rs -> vendor/i-slint-core/window.rs`
- 1-file cycle: `vendor/i-slint-core/graphics/image.rs -> vendor/i-slint-core/graphics/image.rs`
- 1-file cycle: `vendor/i-slint-core/model.rs -> vendor/i-slint-core/model.rs`
- 1-file cycle: `vendor/i-slint-core/properties.rs -> vendor/i-slint-core/properties.rs`
- 1-file cycle: `vendor/i-slint-core/platform.rs -> vendor/i-slint-core/platform.rs`
- 1-file cycle: `vendor/i-slint-core/animations/simulations.rs -> vendor/i-slint-core/animations/simulations.rs`
- 1-file cycle: `vendor/i-slint-core/layout.rs -> vendor/i-slint-core/layout.rs`
- 1-file cycle: `vendor/i-slint-core/translations.rs -> vendor/i-slint-core/translations.rs`
- 1-file cycle: `vendor/i-slint-core/input.rs -> vendor/i-slint-core/input.rs`
- 2-file cycle: `crates/app/src/platform/android.rs -> crates/app/src/platform/android_media.rs -> crates/app/src/platform/android.rs`
- 2-file cycle: `crates/app/src/platform/android.rs -> crates/app/src/platform/android_notify.rs -> crates/app/src/platform/android.rs`
- 2-file cycle: `vendor/i-slint-core/items.rs -> vendor/i-slint-core/menus.rs -> vendor/i-slint-core/items.rs`
- 2-file cycle: `vendor/i-slint-core/items.rs -> vendor/i-slint-core/items/system_tray.rs -> vendor/i-slint-core/items.rs`
- 2-file cycle: `vendor/i-slint-core/items.rs -> vendor/i-slint-core/window.rs -> vendor/i-slint-core/items.rs`
- 2-file cycle: `vendor/i-slint-core/items.rs -> vendor/i-slint-core/items/flickable.rs -> vendor/i-slint-core/items.rs`
- 2-file cycle: `vendor/i-slint-core/items.rs -> vendor/i-slint-core/items/path.rs -> vendor/i-slint-core/items.rs`
- 2-file cycle: `vendor/i-slint-core/items.rs -> vendor/i-slint-core/partial_renderer.rs -> vendor/i-slint-core/items.rs`
- 2-file cycle: `vendor/i-slint-core/items.rs -> vendor/i-slint-core/items/input_items.rs -> vendor/i-slint-core/items.rs`
- 2-file cycle: `vendor/i-slint-core/items.rs -> vendor/i-slint-core/rtti.rs -> vendor/i-slint-core/items.rs`
- 2-file cycle: `vendor/i-slint-core/items.rs -> vendor/i-slint-core/items/text.rs -> vendor/i-slint-core/items.rs`

## Hyperedges (group relationships)
- **Foyer Cargo workspace crates** — readme_crate_app, readme_crate_emby_api, readme_crate_seerr_api, readme_crate_player [EXTRACTED 0.95]
- **Platform-specific areas needing Android replacements** — docs_android_plan_libmpv_android, docs_android_plan_apk_update, docs_android_plan_material_you, docs_android_plan_background_notifications [INFERRED 0.75]
- **Playback safety mechanisms** — plan_playback_state_machine, plan_fixemby_patches, plan_self_contained_design [INFERRED 0.75]

## Communities (324 total, 97 thin omitted)

### Community 0 - "input_items.rs"
Cohesion: 0.04
Nodes (19): FocusEvent, FocusIn, FocusOut, FocusEventResult, FocusAccepted, FocusIgnored, FocusReasonArg, FocusScope (+11 more)

### Community 1 - "player/src/lib.rs"
Cohesion: 0.11
Nodes (23): api(), available(), DEFAULT_OPTIONS, dll_candidates(), END_EOF, END_ERROR, engine_dir(), EV_END_FILE (+15 more)

### Community 2 - "Ctx"
Cohesion: 0.09
Nodes (3): Ctx, episode_summary(), nice_date()

### Community 3 - "update.rs"
Cohesion: 0.09
Nodes (27): accepts_only_a_valid_signature(), Api, ApiAsset, ASSET, bad_signature(), check(), check_signature(), Checked (+19 more)

### Community 4 - "Foyer (native Emby client)"
Cohesion: 0.05
Nodes (46): Android and Android TV plan, Android TV (D-pad focus, 10-foot layouts), APK signing key backup, Signed APK self-update, Background notifications (new feature on Android), CI Android job publishing exe and apk, libmpv .so with MediaCodec (ExoPlayer fallback), Material You accent colour (+38 more)

### Community 5 - "app/src/lib.rs"
Cohesion: 0.03
Nodes (55): Ctx, HideTarget, applog(), AUDIO_PICK, auto_hw_mode(), cached_image(), ChanData, DEBUG (+47 more)

### Community 6 - "seerr-api/src/lib.rs"
Cohesion: 0.05
Nodes (32): ADMIN, AUTO_APPROVE, Cast, CREATE_ISSUES, Credits, ISSUE_OPEN, ISSUE_RESOLVED, IssueBody (+24 more)

### Community 7 - "Control"
Cohesion: 0.09
Nodes (22): Control, AddSubtitle, AudioDelayBy, CycleAudio, CycleSubtitles, Finish, Quit, ResetAudioDelay (+14 more)

### Community 8 - "emby-api/src/lib.rs"
Cohesion: 0.06
Nodes (27): sub_streams(), AuthResult, CLIENT_NAME, CLIENT_VERSION, de_id(), de_opt_id(), DEVICE_NAME, discover() (+19 more)

### Community 9 - "notes.rs"
Cohesion: 0.06
Nodes (16): Ctx, FOLLOWED, init_tray_events(), INSTANCE_ADDR, load_notes(), MAX_NOTES, NoteRec, notes_path() (+8 more)

### Community 11 - "Result"
Cohesion: 0.10
Nodes (5): Browse, Chapter, Item, Client, section_key()

### Community 12 - "sharedparley.rs"
Cohesion: 0.08
Nodes (34): RenderString, RenderText, ScaleFactor, ContentWidths, InnerTextLayoutCache, TextLayoutCache, char_size(), draw_text() (+26 more)

### Community 13 - "Client"
Cohesion: 0.08
Nodes (3): Client, Other, Result

### Community 14 - "RemoteEvent"
Cohesion: 0.10
Nodes (19): parse(), RemoteEvent, FastForward, Message, Mute, Next, Pause, Play (+11 more)

### Community 15 - "LogicalRect"
Cohesion: 0.06
Nodes (12): clip_content_box(), item_children_bounding_rect(), item_children_bounding_rect_transformed(), item_with_children_bounding_rect_transformed(), ItemRenderer, render_component_items(), render_item_children(), RenderImage (+4 more)

### Community 16 - "seerr.rs"
Cohesion: 0.06
Nodes (33): SIssue, SReq, DetailCtx, DetailData, EDIT_BITS, ep(), ISSUE_FILTERS, IssueData (+25 more)

### Community 17 - "gcast.rs"
Cohesion: 0.12
Nodes (14): browse(), decode(), DEFAULT_RECEIVER, Device, discover(), field(), frame(), frames_round_trip() (+6 more)

### Community 18 - "window.rs"
Cohesion: 0.08
Nodes (46): Size, a_host_overrides_the_default_title(), GraphicsAPI, Inaccessible, NativeOpenGL, has_window_handle(), next_focus_item(), set_default_window_title() (+38 more)

### Community 19 - "ModelTracker"
Cohesion: 0.05
Nodes (13): BrokenModel<T>, FilterModel<M, F>, FilterModelInner<M, F>, MapModel<M, F>, ModelChecker, ModelChecker<Data>, ReverseModelInner<M>, SortModel<M, S> (+5 more)

### Community 20 - "sharedvector.rs"
Cohesion: 0.07
Nodes (33): alloc_with_capacity(), capacity_for_grow(), compute_inner_layout(), data_ptr(), drop_inner(), insert_out_of_bounds_test(), insert_test(), IntoIter (+25 more)

### Community 21 - "Page"
Cohesion: 0.09
Nodes (22): Page, Detail, Downloads, Genre, Home, Library, LiveTv, Notifications (+14 more)

### Community 22 - "ImageKind"
Cohesion: 0.22
Nodes (6): ImageKind, Backdrop, Logo, Primary, Thumb, Person

### Community 23 - ".get"
Cohesion: 0.11
Nodes (9): encode(), encodes_queries_with_percent20(), log(), Media, Page, Person, PersonCredits, PublicSettings (+1 more)

### Community 24 - "Coord"
Cohesion: 0.11
Nodes (8): Orientation, text_layout_info(), Constraint, grid_layout_info(), LayoutCacheGenerator<'a>, LayoutInfo, slint_grid_layout_info(), Coord

### Community 25 - "Ctx"
Cohesion: 0.05
Nodes (15): Ctx, apply_accent(), apply_material(), close(), Ctx, fuzzy_match(), load_posters(), login_error() (+7 more)

### Community 26 - "Foyer 0.22.0"
Cohesion: 0.17
Nodes (12): 🛠 Admin, Foyer 0.22.0, ✨ Highlights, 🏠 Home, 🔎 Libraries & search, 🖼 Look & feel, 🔔 Notifications, 🎬 Playback (+4 more)

### Community 27 - "component_factory.rs"
Cohesion: 0.31
Nodes (3): ComponentFactory, ComponentFactoryInner, FactoryContext

### Community 28 - "WindowEvent"
Cohesion: 0.09
Nodes (20): logical_position_to_api(), InternalEvent, Key, Mouse, Touch, InternalEventBox, WindowEvent, CloseRequested (+12 more)

### Community 29 - "system_tray/windows.rs"
Cohesion: 0.03
Nodes (5): CLASS_NAME, MAX_DEPTH, MENU_CMD_BASE, TRAY_UID, WM_TRAYICON

### Community 30 - "Error"
Cohesion: 0.33
Nodes (6): Error, BadLogin, Forbidden, Network, NotSignedIn, Server

### Community 31 - "BorderRadius<T, U>"
Cohesion: 0.07
Nodes (10): ApproxEq, BorderRadius, BorderRadius<T, U>, BorderRadius<T, U1>, BorderRadius<T, U2>, IntBorderRadius, approx_eq!, PhysicalBorderRadius (+2 more)

### Community 32 - "LogicalSize"
Cohesion: 0.14
Nodes (6): logical_physical_size(), LogicalSize, PhysicalSize, WindowSize, Logical, Physical

### Community 34 - "WindowInner"
Cohesion: 0.09
Nodes (5): previous_focus_item(), slint_windowrc_set_focus_item(), slint_windowrc_supports_native_menu_bar(), WindowAdapter, WindowInner

### Community 35 - "foyer"
Cohesion: 0.40
Nodes (6): emby-api, foyer, foyer-core, i-slint-core, player, seerr-api

### Community 36 - "Quality"
Cohesion: 0.11
Nodes (17): quality_rows(), Source, Live, log(), LOG, LogFn, set_log(), auto_steps_down_when_link_is_slow() (+9 more)

### Community 37 - "Client"
Cohesion: 0.12
Nodes (20): ActivityEntry, log(), RemoteSubtitle, SystemInfo, arr(), f(), IDENT, lang2() (+12 more)

### Community 38 - "Color"
Cohesion: 0.07
Nodes (26): Channel, Color, HsvaColor, linear_to_srgb(), OklabColor, OklchColor, quantize(), RgbaColor (+18 more)

### Community 40 - "adapters.rs"
Cohesion: 0.10
Nodes (30): AscendingSortHelper, check_content(), F, FilterModel, FilterModelInner, MapModel, ReverseModel, ReverseModel<M> (+22 more)

### Community 41 - "T"
Cohesion: 0.08
Nodes (10): ErrorImpl, OutOfBounds, Unsupported, Model, ModelError, ModelIterator, ModelIterator<'a, T>, ModelRc<T> (+2 more)

### Community 42 - "Android"
Cohesion: 0.40
Nodes (4): Android, Every feature works on both, and lands on `main`, graphify, Rules for keeping it compatible

### Community 44 - "super"
Cohesion: 0.09
Nodes (22): dir(), home_rows_model(), sync_followed(), AUTOPLAY_STREAK_LIMIT, BASE, CARD_SCALES, Config, data_dir() (+14 more)

### Community 45 - "SharedString"
Cohesion: 0.06
Nodes (16): StandardListViewItem, c_char, SharedString, slint_shared_string_append(), slint_shared_string_bytes(), slint_shared_string_clone(), slint_shared_string_from_bytes(), test_extend_from_chars() (+8 more)

### Community 46 - "tests.rs"
Cohesion: 0.18
Nodes (18): paragraph_ranges(), bidi_selection_spans_are_ascending_in_x(), layout_text(), layout_text_with_builder(), layout_text_with_options(), layout_with_max_lines(), paragraphs(), test_cursor_affinity_at_soft_line_break() (+10 more)

### Community 47 - "items.rs"
Cohesion: 0.07
Nodes (24): accessible_descendents(), find_exposed_text_input(), find_text_input(), find_text_input_with_rc(), nearest_accessible_item(), Callback, ItemTreeVTable, ContextMenu (+16 more)

### Community 48 - "properties/ffi.rs"
Cohesion: 0.10
Nodes (42): PropertyAnimation, c_set_animated_binding(), c_set_animated_value(), ChangeTrackerOpaque, drop_fn(), eval_fn(), EvalState, ffi_change_tracker_does_not_leak_dep_nodes() (+34 more)

### Community 49 - "format!"
Cohesion: 0.40
Nodes (3): format!, T, ToSharedString

### Community 50 - "graphics/path.rs"
Cohesion: 0.08
Nodes (36): elements_path(), events_path(), fitted_path_sample_at_adds_offset_to_position(), fitted_path_sample_at_returns_none_for_zero_length_path(), fitted_path_sample_at_wraps_percent_for_open_path(), FittedPath, line_path(), LyonPathIteratorVariant (+28 more)

### Community 51 - "item_tree.rs"
Cohesion: 0.15
Nodes (24): create_children_nodes(), create_item_subtree_item(), create_nested_subtrees(), create_one_node_component(), create_subsubtree_items(), create_subtrees_item(), ItemTreeNodeArray<'a>, test_map_to_ancestor() (+16 more)

### Community 52 - "flickable.rs"
Cohesion: 0.08
Nodes (21): abs(), CaptureEvents, MouseOrTouchScreen, MouseWheel, DECELERATION, DISTANCE_THRESHOLD, DURATION_THRESHOLD, ensure_in_bound() (+13 more)

### Community 53 - "PlatformError"
Cohesion: 0.05
Nodes (14): ComponentHandle, PlatformError, NoEventLoopProvider, NoPlatform, Other, OtherError, SetPlatformError, Unsupported (+6 more)

### Community 54 - "PhysicalLength"
Cohesion: 0.10
Nodes (17): Glyph, GlyphRenderer, Layout, RectangleBorder, TextParagraph, visible_band(), visible_x_range(), PhysicalRect (+9 more)

### Community 55 - "RenderText"
Cohesion: 0.10
Nodes (4): PlainOrStyledText, Plain, Styled, (SharedString, Brush)

### Community 56 - "Slot"
Cohesion: 0.07
Nodes (24): Active, ACTIVE, Chromecast, Session, CAST_GEN, Ctx, FALLBACK_QUALITY, IDS (+16 more)

### Community 57 - "i-slint-core/layout.rs"
Cohesion: 0.07
Nodes (46): BoxLayoutData, collect_from_organized_data(), Errors, FlexboxLayoutCacheGenerator, FlexboxLayoutCacheGenerator<'a>, FlexboxMeasureFnC, GridLayoutCacheGenerator, GridLayoutCacheGenerator<'a> (+38 more)

### Community 58 - "DataTransfer"
Cohesion: 0.08
Nodes (25): DataTransfer, DataTransferError, TypeNotFound, CppUserData, DataTransferOpaque, path_from_units(), path_to_units(), PathValueType (+17 more)

### Community 59 - ".show_detail"
Cohesion: 0.08
Nodes (32): apply_image(), card_for(), CardData, cards_model(), episode_row(), episode_tag(), fav_group(), media_info() (+24 more)

### Community 61 - "android.yml"
Cohesion: 0.25
Nodes (7): runs-on, jobs, apk, name, permissions, actions, contents

### Community 62 - "T"
Cohesion: 0.08
Nodes (27): (), alloc_binding_holder(), BINDING_BORROWED, BINDING_POINTER_MASK, BINDING_POINTER_TO_BINDING, BindingCallable, .IS_TWO_WAY_BINDING, BindingHolder (+19 more)

### Community 63 - "Item"
Cohesion: 0.11
Nodes (10): Value, Item, CallbackInfo, common_property(), FieldOffset<Item, crate::Callback<(), Ret>>, FieldOffset<Item, crate::Callback<(T,), Ret>>, FieldOffset<Item, Property<T>>, FieldOffset<Item, T> (+2 more)

### Community 64 - "drag_n_drop.rs"
Cohesion: 0.09
Nodes (15): ALL, AllowedDragActions, clamp_action_to_allowed(), compute_proposed_action(), compute_proposed_action_modifier_table(), COPY_AND_MOVE, COPY_ONLY, cursor_for_action() (+7 more)

### Community 65 - "input.rs"
Cohesion: 0.09
Nodes (27): Keys, keys_from_parts(), keys_from_parts_inner(), KeysInner, KeysParseError, KeysParseErrorInner, IncompatibleModifiers, MultipleGraphemeClusters (+19 more)

### Community 66 - "VisitChildrenResult"
Cohesion: 0.12
Nodes (26): State, Deceleration, Done, SpringDamper, ItemTreeNode, DynamicTree, ItemTreeRc, ItemVisitorRefMut (+18 more)

### Community 67 - "ModelNotify"
Cohesion: 0.07
Nodes (16): BrokenModel, TestView, DependencyListHead, ModelChangeListener, ModelChangeListenerBox<T>, ModelChangeListenerContainer, ModelChangeListenerContainer<T>, ModelNotify (+8 more)

### Community 68 - "items/path.rs"
Cohesion: 0.15
Nodes (3): Path, slint_path_angle_at(), slint_path_point_at()

### Community 69 - "CachedRenderingData"
Cohesion: 0.06
Nodes (35): BasicBorderRectangle, .cached_rendering_data_offset, BorderRectangle, .cached_rendering_data_offset, .cached_rendering_data_offset, .cached_rendering_data_offset, .cached_rendering_data_offset, .cached_rendering_data_offset (+27 more)

### Community 70 - "android_media.rs"
Cohesion: 0.13
Nodes (23): as_class(), ASKED_PERMISSION, CB, CB_CLASS, CHANNEL, create_session(), enter_pip(), Handler (+15 more)

### Community 71 - "platform.rs"
Cohesion: 0.06
Nodes (12): _ANIM_TICK_UPDATED_ON_PLATFORM_SET, Clipboard, DefaultClipboard, SelectionClipboard, EVENTLOOP_PROXY, EventLoopProxy, Platform, set_platform() (+4 more)

### Community 72 - "TestItemTree"
Cohesion: 0.08
Nodes (5): core::ops::Range<usize>, IndexRange, ItemTreeWeak, TestItemTree, TransformTestItemTree

### Community 73 - "ksni.rs"
Cohesion: 0.11
Nodes (16): build_ksni_menu(), dispatch_loop(), entry_to_node(), Event, Activate, Menu, image_to_argb_icon(), KsniTray (+8 more)

### Community 74 - ".load_from_data_with_cache_key"
Cohesion: 0.11
Nodes (12): clone_from_slice_rejects_a_long_slice(), clone_from_slice_rejects_a_short_slice(), Svg, load_as_html_image(), load_image_from_data_uri(), LoadImageError, SharedPixelBuffer<Pixel>, slint_image_load_from_data() (+4 more)

### Community 75 - ".get"
Cohesion: 0.12
Nodes (3): const_sentinel(), Property<T>, PropertyHandle

### Community 76 - "translations.rs"
Cohesion: 0.09
Nodes (28): DisplayOrInt, Display, Int, format(), FormatArgs, FormatResult, gettext_bindtextdomain(), global_translation_property() (+20 more)

### Community 77 - "BoxShadowOptions"
Cohesion: 0.15
Nodes (4): BoxShadowCache, BoxShadowOptions, CacheEntry, MAX_CACHED_SHADOWS

### Community 78 - "TextCursorAffinity"
Cohesion: 0.12
Nodes (8): Hide, CachedTextInputAccessibilityState, compose_text_selection(), position_for_byte_offset(), selection_offsets(), to_actual_offset(), TextInputLayout<'a>, TextInputParagraph

### Community 79 - ".compute_dirty_regions"
Cohesion: 0.18
Nodes (6): CachedItemBoundingBoxAndTransform, ClipItem, ItemWithTransform, RegularItem, PartialRendererCache, PartialRenderingCachedData

### Community 80 - "appkit.rs"
Cohesion: 0.07
Nodes (6): ItemWeak, activate_entry(), AppearanceObserverIvars, MAX_DEPTH, MenuActionIvars, scheme_for_appearance()

### Community 81 - "brush.rs"
Cohesion: 0.11
Nodes (20): GradientStop, line_for_angle(), LinearGradientBrush, ResolvedConicGradient, ResolvedLinearGradient, ResolvedRadialGradient, sanitize_color_stops(), test_conic_gradient_basic() (+12 more)

### Community 82 - "android.rs"
Cohesion: 0.08
Nodes (23): android_main(), AndroidCtx, APP, CHANNEL, CTX, CUSTOM_RESIZE, export_logs(), font_scale() (+15 more)

### Community 83 - "render_layer"
Cohesion: 0.10
Nodes (4): ItemCache, ItemCache<T>, LayerRenderer, render_layer()

### Community 84 - "MouseEvent"
Cohesion: 0.04
Nodes (29): MouseCursorInner, BuiltIn, CustomMouseCursor, InputEventFilterResult, ForwardAndIgnore, ForwardAndInterceptGrab, ForwardAndObserve, ForwardEvent (+21 more)

### Community 85 - "two_way_binding.rs"
Cohesion: 0.11
Nodes (14): Property<T>, property_two_ways_binding_of_two_two_way_bindings(), property_two_ways_binding_of_two_way_binding_first(), property_two_ways_binding_of_two_way_binding_second(), property_two_ways_recurse_from_binding(), property_two_ways_test(), property_two_ways_test_binding(), test_two_way_with_map() (+6 more)

### Community 86 - "steps"
Cohesion: 0.05
Nodes (39): runs-on, steps, strategy, timeout-minutes, env, GH_TOKEN, IN_CI, MPV_ANDROID_REF (+31 more)

### Community 87 - "IntSize"
Cohesion: 0.09
Nodes (9): BorrowedOpenGLTexture, BorrowedOpenGLTextureBuilder, BorrowedOpenGLTextureOrigin, BottomLeft, TopLeft, HTMLImage, svg_intrinsic_size(), slint_image_size() (+1 more)

### Community 88 - "ImageCacheKey"
Cohesion: 0.06
Nodes (21): ImageWeightInBytes, replace_cached_image(), CachedPath, ImageCacheKey, EmbeddedData, Invalid, Path, URL (+13 more)

### Community 89 - "i-slint-core/string.rs"
Cohesion: 0.10
Nodes (26): format(), shared_string_from_number(), shared_string_from_number_fixed(), shared_string_from_number_precision(), shared_string_from_number_unlocalized(), shared_string_replace_all(), simple_test(), slint_shared_string_drop() (+18 more)

### Community 90 - "properties_animations.rs"
Cohesion: 0.17
Nodes (27): AnimationDetail, Component, DELAY, DURATION, get_prop_value(), properties_test_animation_direction_triggered_by_set(), properties_test_animation_negative_delay_triggered_by_set(), properties_test_animation_triggered_by_binding() (+19 more)

### Community 91 - "Brush"
Cohesion: 0.18
Nodes (18): Brush, ConicGradient, LinearGradient, RadialGradient, SolidColor, resolve_brush(), ResolvedBrush, ConicGradient (+10 more)

### Community 92 - "pt"
Cohesion: 0.14
Nodes (29): classify(), Ev, Exit, Moved, PinchCancelled, PinchEnded, PinchMoved, PinchStarted (+21 more)

### Community 93 - "RenderBorderRectangle"
Cohesion: 0.13
Nodes (4): adjust_rect_and_border_for_inner_drawing(), Rect, BorderRectLayout, RenderBorderRectangle

### Community 94 - "constant_deceleration.rs"
Cohesion: 0.10
Nodes (22): constant_deceleration_decreasing_limit_not_reached(), constant_deceleration_decreasing_limit_reached(), constant_deceleration_increasing_limit_not_reached(), constant_deceleration_increasing_limit_reached(), constant_deceleration_start_eq_limit(), ConstantDeceleration, ConstantDecelerationParameters, calculate_parameters() (+14 more)

### Community 95 - "popup.rs"
Cohesion: 0.31
Nodes (7): fixed_placement(), place_popup(), Placement, Fixed, r(), test_place_popup_fixed_clipped(), test_place_popup_fixed_unclipped()

### Community 96 - "LogicalLength"
Cohesion: 0.14
Nodes (6): LogicalLength, ListViewProperties, RepeaterLayoutState, slint_repeater_ensure_updated_listview(), TypedListViewProps, update_visible_instances()

### Community 97 - "downloads.rs"
Cohesion: 0.09
Nodes (17): CANCEL, Ctx, Entry, expand(), fetch_one(), key(), LIST, offline_item() (+9 more)

### Community 98 - "linebreaker.rs"
Cohesion: 0.13
Nodes (26): char_wrap_sentences(), test_basic_line_break(), test_basic_line_break_anywhere_fallback(), test_basic_line_break_anywhere_fallback_max_lines(), test_basic_line_break_anywhere_fallback_multi_line(), test_basic_line_break_anywhere_fallback_multi_line_char_wrap(), test_basic_line_break_anywhere_fallback_multi_line_v2(), test_basic_line_break_char_wrap() (+18 more)

### Community 99 - "api.rs"
Cohesion: 0.07
Nodes (18): EventLoopError, EventLoopTerminated, NoEventLoopProvider, Global, invoke_from_event_loop(), quit_event_loop(), set_xdg_app_id(), StrongHandle (+10 more)

### Community 100 - "vec"
Cohesion: 0.09
Nodes (9): FutureRunner, FutureRunner<T>, FutureRunnerInner, FutureState, Finished, Running, JoinHandle, JoinHandle<T> (+1 more)

### Community 101 - "Rgba8Pixel"
Cohesion: 0.16
Nodes (13): dynamic_image_to_shared_image_buffer(), image_to_rgba8_with_target_size(), Rgb8Pixel, Rgba8Pixel, SharedImageBuffer, RGB8, RGBA8, RGBA8Premultiplied (+5 more)

### Community 102 - "GraphicsAPI"
Cohesion: 0.08
Nodes (15): F, GraphicsAPI, NativeOpenGL, WebGL, WGPU29, WGPU30, RenderingNotifier, RenderingState (+7 more)

### Community 103 - "Repeater<C>"
Cohesion: 0.14
Nodes (6): Conditional, ItemTreeRc, Repeater<C>, RepeaterInner, RepeaterInner<C>, RustRepeaterOps

### Community 104 - "callbacks.rs"
Cohesion: 0.23
Nodes (7): Callback<Arg, Ret>, callback_simple_test(), CallbackOpaque, slint_callback_call(), slint_callback_drop(), slint_callback_init(), slint_callback_set_handler()

### Community 105 - "TextInput"
Cohesion: 0.03
Nodes (41): HasFont, KeyEventArg, StringArg, ComplexText, cursor_uses_selection_color(), mask_password(), next_paragraph_boundary(), next_word_boundary() (+33 more)

### Community 106 - "LogicalPosition"
Cohesion: 0.12
Nodes (10): logical_physical_pos(), LogicalPosition, PhysicalPosition, physical_position_from_api(), physical_position_to_api(), MenuVTable, slint_windowrc_setup_menu_bar_shortcuts(), slint_windowrc_setup_native_menu_bar() (+2 more)

### Community 107 - "RendererSealed"
Cohesion: 0.11
Nodes (8): DrawOutcome, Occluded, Skipped, Success, Timeout, Renderer, RendererSealed, T

### Community 108 - "T"
Cohesion: 0.10
Nodes (14): Binding, AnimatedBindingCallable, AnimatedBindingCallable<T, A>, AnimatedBindingState, Animating, NotAnimating, ShouldStart, AnimatedValueBinding (+6 more)

### Community 109 - "ChangeTracker"
Cohesion: 0.14
Nodes (5): change_multiple_dependencies(), change_tracker(), ChangeTracker, ChangeTrackerInner, delete_from_eval_fn()

### Community 110 - "LengthType"
Cohesion: 0.13
Nodes (7): euclid::Point2D<T, U>, euclid::Rect<T, U>, euclid::Size2D<T, U>, euclid::Vector2D<T, U>, PointLengths, RectLengths, SizeLengths

### Community 111 - "textlayout.rs"
Cohesion: 0.10
Nodes (7): DEFAULT_FONT_SIZE, render_lines(), test_content_widths(), test_content_widths_max_lines(), test_content_widths_min_never_exceeds_max(), TextLayout, TextParagraphLayout

### Community 112 - "SlintContext"
Cohesion: 0.06
Nodes (7): set_window_event_hook(), set_window_shown_hook(), SlintContext, SlintContextInner, SlintContextWeak, WindowEventHook, with_global_context()

### Community 113 - "ConicGradientBrush"
Cohesion: 0.16
Nodes (8): center_or_bbox(), ConicGradientBrush, .HEADER, nan_eq(), RadialGradientBrush, .HEADER, slint_conic_gradient_apply_rotation(), slint_conic_gradient_normalize_stops()

### Community 114 - "T"
Cohesion: 0.14
Nodes (4): ModelRc, SharedVectorModel<T>, T, VecModel<T>

### Community 115 - "TouchPhase"
Cohesion: 0.15
Nodes (9): TouchPhase, Cancelled, Ended, Moved, Started, WindowEventDispatchResult, Accepted, Rejected (+1 more)

### Community 116 - "Slice"
Cohesion: 0.17
Nodes (25): box_layout_info(), box_layout_info_ortho(), BoxLayoutOrthoData, flexbox_layout_info_cross_axis(), flexbox_layout_info_main_axis(), flexbox_layout_unwrapped_main(), FlexboxLayoutData, FlexboxLayoutParams (+17 more)

### Community 117 - "textlayout/shaping.rs"
Cohesion: 0.12
Nodes (11): ShapeBoundaries, ShapeBoundaries<'a>, ShapeBuffer<Length>, test_byte_offsets_are_absolute(), test_letter_spacing(), test_shape_boundaries_empty(), test_shape_boundaries_script_change(), test_shape_boundaries_simple() (+3 more)

### Community 118 - "spring.rs"
Cohesion: 0.10
Nodes (16): regime_decays_to_rest_over_time(), regime_matches_initial_conditions(), SpringDurationBounceParameters, SpringParameters, SpringPhysicalParameters, SpringRegime, Critical, .CRITICAL_ZETA_EPSILON (+8 more)

### Community 119 - "Cast"
Cohesion: 0.21
Nodes (8): Cast, Cmd, Load, Pause, Play, Seek, Stop, Status

### Community 120 - "tvnav.rs"
Cohesion: 0.16
Nodes (12): candidates(), DOWN, focusable(), gap(), keep_clear(), LEFT, NAV_EDGE, rect() (+4 more)

### Community 121 - "gdb_pretty_printers.py"
Cohesion: 0.10
Nodes (5): SharedVectorProvider, SharedVectorSubPrinter, SliceProvider, SliceSubPrinter, SlintCorePrettyPrinter

### Community 122 - "timers.rs"
Cohesion: 0.09
Nodes (24): _BUG3019, _BUG6141_SET_INTERVAL_FROM_CALLBACK, _DOUBLY_REGISTER_ACTIVE_TIMER, _DOUBLY_REGISTER_ACTIVE_TIMER_2, LIST_SHIFT, _RESTART_TIMER_BEING_ACTIVATED, _SINGLESHOT_START, slint_timer_destroy() (+16 more)

### Community 123 - "wgpu_29.rs"
Cohesion: 0.18
Nodes (11): any_wgpu29_adapters_with_gpu(), async_init_instance_adapter_device_queue_surface(), default_backends_to_avoid(), default_device_descriptor(), device_descriptor_from_settings(), init_instance_adapter_device_queue_surface(), init_instance_adapter_device_queue_surface_then(), mask_backends() (+3 more)

### Community 124 - "TextCursorDirection"
Cohesion: 0.08
Nodes (23): TextShortcut, DeleteBackward, DeleteForward, DeleteToStartOfLine, DeleteWordBackward, DeleteWordForward, Move, TextCursorDirection (+15 more)

### Community 125 - "AccessibilityAction"
Cohesion: 0.20
Nodes (8): AccessibilityAction, Decrement, Default, Expand, Increment, ReplaceSelectedText, SetSelectionOffsets, SetValue

### Community 126 - "step"
Cohesion: 0.37
Nodes (9): current(), focus(), focus_in_nav(), focus_start(), has_focus(), R, settle(), step() (+1 more)

### Community 127 - ".evaluate"
Cohesion: 0.17
Nodes (9): PropertyTracker<NEEDS_SET_DIRTY, ()>, PropertyTracker<NEEDS_SET_DIRTY, DirtyHandler>, PropertyTracker<true, DirtyHandler>, test_nested_property_tracker_dirty(), test_nested_property_tracker_evaluate_if_dirty(), test_nested_property_trackers(), test_property_dirty_handler(), test_property_listener_scope() (+1 more)

### Community 128 - "ItemRc"
Cohesion: 0.07
Nodes (8): ItemRc, BoxShadow, ItemRendererRef, ItemVTable, Layer, RenderingResult, ContinueRenderingChildren, ContinueRenderingWithoutChildren

### Community 129 - "wgpu_30.rs"
Cohesion: 0.20
Nodes (11): any_wgpu30_adapters_with_gpu(), async_init_instance_adapter_device_queue_surface(), default_backends_to_avoid(), default_device_descriptor(), device_descriptor_from_settings(), init_instance_adapter_device_queue_surface(), init_instance_adapter_device_queue_surface_then(), mask_backends() (+3 more)

### Community 130 - "graphics.rs"
Cohesion: 0.12
Nodes (7): CachedGraphicsData, CachedGraphicsData<T>, create_graphics_api_wgpu_29(), create_graphics_api_wgpu_30(), physical_size_from_api(), Point, Transform

### Community 131 - "fragments.rs"
Cohesion: 0.15
Nodes (14): fragment_iterator_break_anywhere(), fragment_iterator_forced_break(), fragment_iterator_forced_break_multi(), fragment_iterator_leading_nbsp(), fragment_iterator_nbsp(), fragment_iterator_simple(), fragment_iterator_simple_v2(), TextFragment (+6 more)

### Community 132 - "rtti.rs"
Cohesion: 0.10
Nodes (10): AnimatedBindingKind, Animation, NotAnimated, Transition, BuiltinGlobal, BuiltinItem, FieldInfo, declare_ValueType! (+2 more)

### Community 133 - "RepeaterInstanceOpsVTable"
Cohesion: 0.10
Nodes (9): MAX_EAGER_INSTANCE_COUNT, RepeatedInstanceState, Clean, Dirty, RepeaterInstanceOps, RepeaterInstanceOpsVTable, RustRepeaterOps<'_, C>, slint_repeater_ensure_updated() (+1 more)

### Community 134 - "Download"
Cohesion: 0.21
Nodes (9): Download, DownloadEp, downloads_label(), MediaInfo, parses_download_status_in_requests(), ReqMedia, ReqSeason, Request (+1 more)

### Community 137 - "data_transfer.rs"
Cohesion: 0.24
Nodes (8): cleared_transfer_compares_equal_to_default(), DataTransferInner, every_payload_counts_as_native_data(), set_file_paths_with_empty_list_clears(), set_image_with_default_image_clears(), set_image_with_default_image_on_default_stays_empty(), set_plain_text_with_empty_string_clears(), set_plain_text_with_empty_string_on_default_stays_empty()

### Community 138 - ".find_sibling"
Cohesion: 0.16
Nodes (3): find_sibling_outside_repeater(), register_item_tree(), step_into_node()

### Community 139 - "LayoutData"
Cohesion: 0.18
Nodes (5): Adjust, adjust_items(), Grow, LayoutData, Shrink

### Community 140 - "OnceCell"
Cohesion: 0.14
Nodes (8): INITIAL_INSTANT, FakeThreadStorage, FakeThreadStorage<T>, FakeThreadStorage<T, F>, SLINT__thread_local_inner!, thread_local_!, OnceCell, OnceCell<T>

### Community 141 - "AccessibleStringProperty"
Cohesion: 0.10
Nodes (20): AccessibleStringProperty, Checkable, Checked, DelegateFocus, Description, Enabled, Expandable, Expanded (+12 more)

### Community 142 - "InternalKeyEvent"
Cohesion: 0.06
Nodes (9): InternalKeyEvent, KeyEventResult, EventAccepted, EventIgnored, Clip, Empty, Opacity, Rectangle (+1 more)

### Community 143 - "TouchState"
Cohesion: 0.18
Nodes (9): GestureRecognitionState, Idle, Pinching, TwoFingersDown, send_exit_events(), TouchEventBuffer, TouchState, .PINCH_THRESHOLD (+1 more)

### Community 144 - "desktop.rs"
Cohesion: 0.10
Nodes (11): form_factor(), form_factor(), set_material(), SUPPORTS_MATERIAL, system_notify(), system_notify_rich(), FormFactor, Desktop (+3 more)

### Community 145 - ".insert"
Cohesion: 0.23
Nodes (10): touch_map_capacity(), touch_map_first(), touch_map_first_two_ids(), touch_map_get_mut(), touch_map_insert_and_get(), touch_map_remove(), touch_map_remove_nonexistent(), touch_map_update_existing() (+2 more)

### Community 146 - "StyledText"
Cohesion: 0.17
Nodes (12): color_to_styled_text(), get_raw_text(), parse_markdown(), slint_styled_text_clone(), slint_styled_text_drop(), slint_styled_text_eq(), slint_styled_text_from_markdown(), slint_styled_text_from_plain_text() (+4 more)

### Community 147 - "Image"
Cohesion: 0.11
Nodes (13): &'a ImageInner, ImageCache, Image, NineSlice, load_image_from_embedded_data(), NineSliceImage, slint_image_load_from_embedded_data(), slint_image_load_from_path() (+5 more)

### Community 148 - "Instant"
Cohesion: 0.14
Nodes (5): animation_tick(), AnimationDriver, current_tick(), Instant, update_animations()

### Community 149 - "MouseInputState"
Cohesion: 0.19
Nodes (9): handle_mouse_grab(), DelayForwarding, MouseGrabResult, MouseInputResult, MouseInputState, offer_native_drag(), process_delayed_event(), process_mouse_input() (+1 more)

### Community 150 - ".fmt"
Cohesion: 0.23
Nodes (3): DisplayOrInt<T>, FormatResult<'_, T>, WithPlural<'_, T>

### Community 151 - "DependencyNode"
Cohesion: 0.18
Nodes (6): ModelPeer, DependencyListHead, DependencyListHead<T>, DependencyNode, DependencyNode<T>, mark_dependencies_dirty()

### Community 152 - "MenuFromItemTree"
Cohesion: 0.19
Nodes (3): MenuFromItemTree, ShadowTreeNode, slint_menus_create_wrapper()

### Community 153 - "erased_bindings.rs"
Cohesion: 0.29
Nodes (11): change_tracker_init_erased(), change_tracker_init_impl(), set_animated_property_binding_erased(), set_animated_property_binding_impl(), set_callback_handler_erased(), set_callback_handler_impl(), set_property_binding_erased(), set_property_binding_impl() (+3 more)

### Community 154 - "lengths.rs"
Cohesion: 0.15
Nodes (7): logical_point_from_api(), logical_size_from_api(), logical_size_to_api(), LogicalEdges, LogicalSize, PhysicalBorderRadius, PhysicalEdges

### Community 155 - "solve_flexbox_layout_with_measure"
Cohesion: 0.14
Nodes (18): agrees(), agrees_for_a_column(), assert_agrees(), assert_agrees_column(), cell(), constraints(), flexbox_layout_info_cross_axis_with_measure(), FlexboxLayoutItemInfo (+10 more)

### Community 156 - "Layout"
Cohesion: 0.10
Nodes (17): cached_paragraphs(), CachedParagraphs, CachedParagraphsGuard, ENTRY_LIMIT, ElisionCut, ElisionInfo, Layout, layout() (+9 more)

### Community 157 - "PlatformTray"
Cohesion: 0.16
Nodes (6): AppearanceObserver, build_menu(), entry_to_nsmenuitem(), image_to_nsimage(), MenuAction, PlatformTray

### Community 158 - "Property"
Cohesion: 0.09
Nodes (4): ComponentContainer, slint_system_tray_icon_set_menu(), SystemTrayIcon, Property

### Community 159 - "platform/windows.rs"
Cohesion: 0.13
Nodes (7): RichNote, APP_ID, set_material(), SUPPORTS_MATERIAL, system_notify(), system_notify_rich(), toast_xml()

### Community 160 - "FontMetrics"
Cohesion: 0.16
Nodes (5): AbstractFont, FontMetrics, &rustybuzz::Face<'_>, T, TextShaper

### Community 161 - "LogicalPoint"
Cohesion: 0.14
Nodes (9): assert_point_approx_eq(), create_subsubtree_items_dynamic_elements(), create_transform_test_items(), ensure_item_tree_instantiated(), test_absolute_clip_rect_and_geometry_under_transform(), test_map_to_native_window_popup_dynamic_element(), test_map_to_window_through_transform_roundtrip(), test_visibility_with_clip_under_transform() (+1 more)

### Community 162 - "debug_log.rs"
Cohesion: 0.18
Nodes (10): default_log_message(), log_message(), LogMessage, LogMessage<'a>, LogMessageHandler, LogMessageLocation, LogMessageSource, Runtime (+2 more)

### Community 163 - "DirtyRegion"
Cohesion: 0.17
Nodes (7): Box2D, LogicalPx, dirty_region_ignores_empty_rects(), dirty_region_no_intersection(), DirtyRegion, .MAX_COUNT, PartialRenderingState

### Community 165 - "PropertyTracker"
Cohesion: 0.12
Nodes (12): MenuDirtyHandler, MenuState, PropertyDirtyHandler, PropertyTracker, PopupWindow, PopupWindowPropertiesTracker, WindowKind, Menu (+4 more)

### Community 167 - ".from_pub"
Cohesion: 0.22
Nodes (7): accent_color(), context_for_root(), PopupWindowLocation, ChildWindow, TopLevel, slint_windowrc_resolved_default_font_size(), slint_windowrc_set_component()

### Community 168 - "data_ringbuffer.rs"
Cohesion: 0.22
Nodes (8): test_empty_buffer(), test_push_beyond_capacity(), test_push_beyond_capacity_wrap_back(), test_push_single_element(), test_push_two_elements(), test_push_until_full(), VelocityRingBuffer, VelocityRingBuffer<N>

### Community 170 - "partial_renderer.rs"
Cohesion: 0.15
Nodes (8): ItemRendererFeatures, .SUPPORTS_TRANSFORMATIONS, forward_rendering_call!, forward_rendering_call2!, RepaintBufferType, NewBuffer, ReusedBuffer, SwappedBuffers

### Community 171 - "Changelog"
Cohesion: 0.05
Nodes (36): Changelog, Foyer 0.23.0, Foyer 0.23.1, Foyer 0.23.2, Foyer 0.26.10, Foyer 0.26.11, Foyer 0.26.12, Foyer 0.26.5 (+28 more)

### Community 172 - "android_notify.rs"
Cohesion: 0.18
Nodes (9): active_ids(), EXTRA, launch_note_target(), LAUNCH_TAKEN, poll_note_tap(), POSTED, sdk(), system_notify_rich() (+1 more)

### Community 173 - "play_inner"
Cohesion: 0.19
Nodes (15): yes(), ExtSub, is_text_codec(), Options, Outcome, Failed, Finished, Stopped (+7 more)

### Community 174 - "date_time.rs"
Cohesion: 0.17
Nodes (12): date_now(), format_date(), month_day_count(), month_offset(), parse_date(), slint_date_time_date_now(), slint_date_time_format_date(), slint_date_time_month_day_count() (+4 more)

### Community 175 - "animations.rs"
Cohesion: 0.08
Nodes (23): CubicBezierSegment, ease_out_bounce_curve(), easing_curve(), EasingCurve, CubicBezier, EaseInBounce, EaseInElastic, EaseInOutBounce (+15 more)

### Community 176 - "c_void"
Cohesion: 0.24
Nodes (8): Api, GetProc, gl_trampoline(), GlInitParams, MpvEvent, on_update(), RenderParam, set_java_vm()

### Community 178 - "ScaleFactor"
Cohesion: 0.14
Nodes (5): FontRequest, font_metrics(), FontContext, line_height_ratio(), text_line_height()

### Community 179 - "Length"
Cohesion: 0.26
Nodes (4): PositionedGlyph, test_text_size_with_max_lines(), TextLayout<'_, Font>, TextParagraphLayout<'_, Font>

### Community 180 - "CheckedAdd"
Cohesion: 0.12
Nodes (6): CheckedAdd, DivCount, euclid::Length<i16, U>, euclid::Length<T, U>, f32, i16

### Community 181 - "Surface"
Cohesion: 0.31
Nodes (5): GlFns, GlLoader, GlSaved, MiniTex, SurfState

### Community 183 - "rendering_metrics_collector.rs"
Cohesion: 0.23
Nodes (6): FrameData, RefreshMode, FullSpeed, Lazy, RenderingMetrics, RenderingMetricsCollector

### Community 184 - "Scalar"
Cohesion: 0.25
Nodes (7): f32, Scalar, .HALF, .NINE, .ONE, .SIX, .THREE

### Community 185 - "Kind"
Cohesion: 0.17
Nodes (7): detect(), Kind, .ALL, Emby, Jellyfin, Plex, Task

### Community 186 - "Slint Software License"
Cohesion: 0.13
Nodes (14): 1. Grant of Rights, 2. License Conditions, 3. Limitations, 4. Audit Rights, 5. Termination, 6. Assignment, 7. Severability, 8. Governing Law (+6 more)

### Community 188 - "clipped_screen_rect"
Cohesion: 0.24
Nodes (3): clipped_screen_rect(), PartialRenderer, PartialRenderer<'a, T>

### Community 189 - "InterpolatedPropertyValue"
Cohesion: 0.13
Nodes (5): f32, i32, i64, InterpolatedPropertyValue, u8

### Community 191 - "item_tree.rs"
Cohesion: 0.08
Nodes (17): create_empty_subtree(), GEOMETRY_HEIGHT, GEOMETRY_POSITION_X, GEOMETRY_POSITION_Y, GEOMETRY_WIDTH, ItemTreeRef, ParentItemTraversalMode, FindAllParents (+9 more)

### Community 193 - "TimerList"
Cohesion: 0.13
Nodes (14): ActiveTimer, CallbackVariant, Empty, MultiFire, SingleShot, current_timers(), single_shot_on(), SingleShotTimerCallback (+6 more)

### Community 194 - "KeyboardModifiers"
Cohesion: 0.26
Nodes (6): InternalKeyboardModifierState, KeyboardModifiers, modifiers(), AnchorMode, KeepAnchor, MoveAnchor

### Community 195 - "mod.rs"
Cohesion: 0.10
Nodes (13): MediaAction, Forward, Next, Pause, Play, Prev, Rewind, Seek (+5 more)

### Community 196 - "steps"
Cohesion: 0.10
Nodes (24): run, GH_TOKEN, NOTES, RUSTFLAGS, UPDATE_SIGNING_KEY, windows, tag, version (+16 more)

### Community 197 - "graphics/image.rs"
Cohesion: 0.09
Nodes (25): f32, fit(), fit9slice(), FitResult, StaticTextures, RemEuclid, scalable_render_size(), slint_image_from_embedded_textures() (+17 more)

### Community 198 - "RequestedGraphicsAPI"
Cohesion: 0.19
Nodes (10): RequestedGraphicsAPI, Direct3D, Metal, OpenGL, Vulkan, WGPU29, WGPU30, RequestedOpenGLVersion (+2 more)

### Community 199 - "FittedPathBox"
Cohesion: 0.31
Nodes (4): FittedPathBox, FittedPathInner, slint_path_fitted_cache_free(), slint_path_fitted_cache_init()

### Community 200 - "SingleLinkedListPinHead<T>"
Cohesion: 0.32
Nodes (3): big_list(), SingleLinkedListPinHead<T>, test_list()

### Community 202 - "TwoWayBindingMapping"
Cohesion: 0.38
Nodes (3): (F1, F2), link_common_properties(), TwoWayBindingMapping

### Community 203 - "winit_ops.rs"
Cohesion: 0.09
Nodes (18): bring_to_front(), drag_resize(), drag_window(), monitor_area(), set_cursor_visible(), set_decorations(), set_material(), set_window_rect() (+10 more)

### Community 204 - "SurfaceTarget"
Cohesion: 0.47
Nodes (3): SurfaceTarget, Drm, WindowHandle

### Community 207 - "StandardShortcut"
Cohesion: 0.17
Nodes (11): StandardShortcut, Copy, Cut, Find, Paste, Print, Redo, Refresh (+3 more)

### Community 212 - "WindowPosition"
Cohesion: 0.29
Nodes (5): WindowPosition, Logical, Physical, slint_windowrc_set_logical_position(), slint_windowrc_set_physical_position()

### Community 213 - "SystemTrayIconDataBox"
Cohesion: 0.29
Nodes (3): slint_system_tray_icon_data_free(), slint_system_tray_icon_data_init(), SystemTrayIconDataBox

### Community 217 - "User"
Cohesion: 0.20
Nodes (3): Comment, Issue, User

### Community 219 - "CloseRequestResponse"
Cohesion: 0.29
Nodes (4): CloseRequestResponse, HideWindow, KeepWindowShown, WithUserData<extern "C" fn(user_data: *mut c_void) -> CloseRequestResponse>

### Community 220 - "Renderer"
Cohesion: 0.09
Nodes (3): Renderer, WindowAdapter, ItemTransform

### Community 222 - "KeyEventType"
Cohesion: 0.40
Nodes (5): KeyEventType, CommitComposition, KeyPressed, KeyReleased, UpdateComposition

### Community 226 - "benches/string.rs"
Cohesion: 0.31
Nodes (5): REFERENCE_NUMBER, RESULT_NUMBER, string_replace(), string_replace_own_character(), string_replacen()

### Community 227 - "android"
Cohesion: 0.09
Nodes (22): secrets, uses, with, notes, jobs, android, publish, name (+14 more)

### Community 228 - "TimerListRc"
Cohesion: 0.20
Nodes (7): list_from_handle(), list_handle(), set_owning_context(), take_pending_timers(), ThreadTimers, Timer, TimerListRc

### Community 229 - "TextLineBreaker"
Cohesion: 0.36
Nodes (4): TextLine, TextLine<Length>, TextLineBreaker, TextLineBreaker<'_, Font>

### Community 230 - "LyonPathEventIterator"
Cohesion: 0.20
Nodes (4): LyonPathEventIterator, FromEvents, FromPath, TransformedLyonPathIterator<EventIt>

### Community 231 - "pathbuf"
Cohesion: 0.32
Nodes (5): Asset, find_dll(), install(), Release, RELEASES

### Community 235 - "steps"
Cohesion: 0.15
Nodes (13): steps, id, if, name, run, uses, with, working-directory (+5 more)

### Community 236 - "bitmapfont.rs"
Cohesion: 0.43
Nodes (4): BitmapFont, BitmapGlyph, BitmapGlyphs, CharacterMapEntry

### Community 237 - "Inner"
Cohesion: 0.13
Nodes (9): append_menu_entry(), create_hicon(), ensure_class_registered(), Inner, notify_icon_data(), PlatformTray, show_popup_menu(), taskbar_created_message() (+1 more)

### Community 238 - "env"
Cohesion: 0.25
Nodes (8): ARMV7_ONLY, FOYER_CLASSES_DEX, GH_TOKEN, KEYSTORE_B64, KEYSTORE_PASSWORD, OVERRIDE_URL, RELEASE, env

### Community 240 - "Community 240"
Cohesion: 0.11
Nodes (20): InternalToken, (), AbsentRowModel, model_all(), model_any(), model_find_index(), ModelIterator<'_, T>, test_any_change_subsumes_row_tracking() (+12 more)

### Community 242 - "Slint Royalty-free Desktop, Mobile, and Web Applications License"
Cohesion: 0.29
Nodes (6): 1. Grant of Rights, 2. License Conditions - Attribution, 3. Limitations, 4. Warranty and Liability, Preamble, Slint Royalty-free Desktop, Mobile, and Web Applications License

### Community 244 - "CrossAxisSizing"
Cohesion: 0.50
Nodes (4): CrossAxisSizing, FromMeasure, Minimum, Preferred

### Community 245 - "SurfaceTarget"
Cohesion: 0.47
Nodes (3): SurfaceTarget, Drm, WindowHandle

### Community 246 - "armv7_only"
Cohesion: 0.12
Nodes (17): default, description, type, armv7_only, release, on, pull_request, push (+9 more)

### Community 247 - "Saturating"
Cohesion: 0.33
Nodes (3): f32, i32, Saturating

### Community 248 - "BreakOpportunity"
Cohesion: 0.33
Nodes (4): BreakOpportunity, Allowed, Mandatory, LineBreakIterator

### Community 250 - "TextureImportError"
Cohesion: 0.40
Nodes (3): TextureImportError, InvalidFormat, InvalidUsage

### Community 251 - "TextureImportError"
Cohesion: 0.40
Nodes (3): TextureImportError, InvalidFormat, InvalidUsage

### Community 253 - "Foyer 0.24.0"
Cohesion: 0.50
Nodes (4): Foyer 0.24.0, ✨ Highlights, 🔧 Other, 🎬 Player

### Community 256 - "Foyer's patch to i-slint-core 1.18.1"
Cohesion: 0.50
Nodes (3): Foyer's patch to i-slint-core 1.18.1, Updating Slint, What changed

### Community 257 - "WindowAdapterInternal"
Cohesion: 0.09
Nodes (8): ItemTreeRefPin, has_display_handle(), InputMethodProperties, InputMethodRequest, Disable, Enable, Update, WindowAdapterInternal

### Community 258 - "Foyer 0.26.3"
Cohesion: 0.67
Nodes (3): 🐞 Fixes, Foyer 0.26.3, ✨ Highlights

### Community 259 - "Foyer 0.26.2"
Cohesion: 0.67
Nodes (3): 🐞 Fixes, Foyer 0.26.2, ✨ Highlights

### Community 260 - "Foyer 0.25.1"
Cohesion: 0.67
Nodes (3): 🐞 Fixes, Foyer 0.25.1, ✨ Highlights

### Community 261 - "Foyer 0.25.0"
Cohesion: 0.67
Nodes (3): 🐞 Fixes, Foyer 0.25.0, ✨ Highlights

### Community 262 - "Foyer 0.26.0"
Cohesion: 0.67
Nodes (3): Foyer 0.26.0, ✨ Highlights, 🔧 Under the hood

### Community 267 - "secrets"
Cohesion: 0.40
Nodes (5): required, required, ANDROID_KEYSTORE_BASE64, ANDROID_KEYSTORE_PASSWORD, secrets

### Community 268 - "BackendMouseEvent"
Cohesion: 0.13
Nodes (13): BackendDragEvent, Drop, Leave, Move, BackendMouseEvent, Exit, Moved, PinchGesture (+5 more)

### Community 278 - "item_focus.rs"
Cohesion: 0.44
Nodes (10): default_next_in_local_focus_chain(), default_previous_in_local_focus_chain(), step_into_node(), step_out_of_node(), test_focus_chain_complex_tree(), test_focus_chain_one_child(), test_focus_chain_root_only(), test_focus_chain_three_children() (+2 more)

### Community 284 - "Detail"
Cohesion: 0.33
Nodes (3): Detail, Genre, Season

## Knowledge Gaps
- **460 isolated node(s):** `Latest`, `CachedGraphicsData<T>`, `FakeThreadStorage<T, F>`, `ShapeBuffer<Length>`, `NativeOpenGL` (+455 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1789 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **97 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `ItemRc` connect `ItemRc` to `input_items.rs`, `PartialRenderer<'_, T>`, `.find_sibling`, `sharedparley.rs`, `AccessibleStringProperty`, `InternalKeyEvent`, `LogicalRect`, `window.rs`, `MouseInputState`, `Coord`, `MenuFromItemTree`, `Layout`, `Property`, `LogicalPoint`, `WindowInner`, `.from_pub`, `partial_renderer.rs`, `SharedString`, `items.rs`, `graphics/path.rs`, `item_tree.rs`, `flickable.rs`, `RenderText`, `items/image.rs`, `clipped_screen_rect`, `item_tree.rs`, `drag_n_drop.rs`, `input.rs`, `items/path.rs`, `CachedRenderingData`, `TestItemTree`, `.init`, `BoxShadowOptions`, `TextCursorAffinity`, `render_layer`, `MouseEvent`, `Renderer`, `popup.rs`, `TextInput`, `LogicalPosition`, `RendererSealed`, `SlintContext`, `tvnav.rs`, `AccessibilityAction`, `step`?**
  _High betweenness centrality (0.080) - this node is a cross-community bridge._
- **Are the 46 inferred relationships involving `ItemRc` (e.g. with `.font_request()` and `item_children_bounding_rect()`) actually correct?**
  _`ItemRc` has 46 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Latest`, `CachedGraphicsData<T>`, `FakeThreadStorage<T, F>` to the rest of the system?**
  _460 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `input_items.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.042105263157894736 - nodes in this community are weakly interconnected._
- **Why does `SharedString` connect `SharedString` to `input_items.rs`, `WindowAdapterInternal`, `graphics.rs`, `data_transfer.rs`, `AccessibleStringProperty`, `InternalKeyEvent`, `LogicalRect`, `window.rs`, `Image`, `MenuFromItemTree`, `WindowEvent`, `Property`, `WindowInner`, `.window_item`, `date_time.rs`, `items.rs`, `format!`, `ScaleFactor`, `graphics/path.rs`, `PlatformError`, `RenderText`, `i-slint-core/layout.rs`, `DataTransfer`, `item_tree.rs`, `input.rs`, `KeyboardModifiers`, `graphics/image.rs`, `platform.rs`, `TestItemTree`, `translations.rs`, `ImageCacheKey`, `i-slint-core/string.rs`, `benches/string.rs`, `api.rs`, `TextInput`, `SlintContext`, `Community 240`, `AccessibilityAction`?**
  _High betweenness centrality (0.065) - this node is a cross-community bridge._
- **Are the 4 inferred relationships involving `SharedString` (e.g. with `.set_xdg_app_id()` and `.xdg_app_id()`) actually correct?**
  _`SharedString` has 4 INFERRED edges - model-reasoned connections that need verification._
- **Should `player/src/lib.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.1053763440860215 - nodes in this community are weakly interconnected._