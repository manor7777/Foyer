# Graph Report - Foyer  (2026-11-17)

## Corpus Check
- 152 files · ~309,105 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 10 file(s) not represented in the graph (top: (none) 4, .xml 2, .rc 1)

## Summary
- 6856 nodes · 17689 edges · 260 communities (222 shown, 38 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 952 edges (avg confidence: 0.56)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `bb39eecc`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- CachedRenderingData
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
- ItemRc
- seerr.rs
- gcast.rs
- TextInput
- User
- sharedvector.rs
- Page
- ImageKind
- Media
- window.rs
- Ctx
- Foyer 0.22.0
- Coord
- Detail
- system_tray/windows.rs
- Error
- BorderRadius<T, U>
- Timer
- .send
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
- Window
- properties/ffi.rs
- format!
- graphics/path.rs
- item_tree.rs
- flickable.rs
- items.rs
- PhysicalLength
- SelectionSpan
- Slot
- i-slint-core/layout.rs
- DataTransfer
- .row_data
- items/image.rs
- steps
- Community 62
- Item
- drag_n_drop.rs
- input.rs
- VisitChildrenResult
- ModelNotify
- component_container.rs
- Model
- android_media.rs
- items/path.rs
- TestItemTree
- ksni.rs
- graphics/image.rs
- .get
- translations.rs
- BoxShadowOptions
- TextCursorAffinity
- .show_detail
- appkit.rs
- brush.rs
- android.rs
- with_env
- BackendMouseEvent
- two_way_binding.rs
- steps
- IntSize
- ParsedSVG
- i-slint-core/string.rs
- properties_animations.rs
- Brush
- pt
- .init
- constant_deceleration.rs
- ItemTreeVTable
- LogicalLength
- downloads.rs
- linebreaker.rs
- RenderBorderRectangle
- vec
- Rgba8Pixel
- GraphicsAPI
- Repeater<C>
- callbacks.rs
- DirtyRegion
- RepeaterInstanceOpsVTable
- .compute_dirty_regions
- T
- ChangeTracker
- LengthType
- textlayout.rs
- SlintContext
- ConicGradientBrush
- .fmt
- MenuVTable
- Slice
- textlayout/shaping.rs
- spring.rs
- .update_check_inner
- Platform
- gdb_pretty_printers.py
- timers.rs
- wgpu_29.rs
- TextCursorDirection
- i-slint-core/accessibility.rs
- Layout
- .evaluate
- ItemCache<T>
- wgpu_30.rs
- graphics.rs
- fragments.rs
- rtti.rs
- GridLayoutOrganizedData
- RendererSealed
- .window_item
- ModelTracker
- data_transfer.rs
- RenderImage
- Inner
- OnceCell
- AccessibleStringProperty
- MouseEvent
- TouchState
- desktop.rs
- .insert
- StyledText
- Image
- Instant
- MouseInputState
- LogicalRect
- DependencyNode
- item_focus.rs
- erased_bindings.rs
- KeyEventType
- solve_flexbox_layout_with_measure
- super
- FittedPathBox
- TestView
- platform/windows.rs
- &rustybuzz::Face<'_>
- constant_deceleration_spring_damper.rs
- debug_log.rs
- repeater.rs
- NoVerify
- platform.rs
- api.rs
- phantomdata
- data_ringbuffer.rs
- PartialRenderer<'_, T>
- LyonPathEventIterator
- Changelog
- android_notify.rs
- play_inner
- date_time.rs
- animations.rs
- TextCursorBlinker
- Error
- TexturePixelFormat
- Length
- CheckedAdd
- Surface
- .evaluate
- rendering_metrics_collector.rs
- lengths.rs
- PartialRendererCache
- Slint Software License
- media.rs
- StyledTextItem
- InterpolatedPropertyValue
- SystemTrayIconDataBox
- ItemRef
- MediaCb
- .get
- KeyboardModifiers
- mod.rs
- steps
- PhysicalPx
- RequestedGraphicsAPI
- refcell
- SingleLinkedListPinHead<T>
- AnimatedBindingState
- Any
- winit_ops.rs
- InternalToken
- WGPUSettings
- WGPUSettings
- StandardShortcut
- LayoutData
- Export.java
- preview.rs
- src/color.rs
- c_void
- ScaleFactor
- component_factory.rs
- AnimatedBindingKind
- Client
- frame
- T
- menus.rs
- Renderer
- DrawOutcome
- RepaintBufferType
- LineBreakIterator<'a>
- Conditional<C>
- Immersive
- benches/string.rs
- android
- TextLineBreaker
- pathbuf
- windows
- bitmapfont.rs
- PlatformError
- Cmd
- Slint Royalty-free Desktop, Mobile, and Web Applications License
- SurfaceTarget
- SurfaceTarget
- Saturating
- BreakOpportunity
- TextureImportError
- TextureImportError
- Foyer 0.24.0
- Foyer's patch to i-slint-core 1.18.1
- Foyer 0.26.3
- Foyer 0.26.2
- Foyer 0.25.1
- Foyer 0.25.0
- Foyer 0.26.0
- main
- Texture
- Texture
- Foyer 0.26.4
- Foyer 0.24.1
- build-mediacb.sh
- RepeaterTracker<C>
- time::Instant
- README.md
- WithUserData<T>
- i-slint-core

## God Nodes (most connected - your core abstractions)
1. `ItemRc` - 508 edges
2. `SharedString` - 161 edges
3. `Coord` - 139 edges
4. `Item` - 120 edges
5. `MouseEvent` - 113 edges
6. `TextInput` - 112 edges
7. `Property` - 107 edges
8. `Client` - 98 edges
9. `WindowInner` - 98 edges
10. `Ctx` - 94 edges

## Surprising Connections (you probably didn't know these)
- `libmpv .so with MediaCodec (ExoPlayer fallback)` --semantically_similar_to--> `libmpv playback (hw decoding, cache, direct play, transcode presets)`  [INFERRED] [semantically similar]
  docs/ANDROID-PLAN.md → README.md
- `with_global_context()` --calls--> `f()`  [INFERRED]
  vendor/i-slint-core/context.rs → crates/emby-api/src/plex.rs
- `with_event_loop_proxy()` --calls--> `f()`  [INFERRED]
  vendor/i-slint-core/platform.rs → crates/emby-api/src/plex.rs
- `set()` --calls--> `f()`  [INFERRED]
  vendor/i-slint-core/properties.rs → crates/emby-api/src/plex.rs
- `with()` --calls--> `f()`  [INFERRED]
  vendor/i-slint-core/properties.rs → crates/emby-api/src/plex.rs

## Import Cycles
- 1-file cycle: `vendor/i-slint-core/platform.rs -> vendor/i-slint-core/platform.rs`
- 1-file cycle: `vendor/i-slint-core/graphics/image.rs -> vendor/i-slint-core/graphics/image.rs`
- 1-file cycle: `vendor/i-slint-core/model.rs -> vendor/i-slint-core/model.rs`
- 1-file cycle: `vendor/i-slint-core/layout.rs -> vendor/i-slint-core/layout.rs`
- 1-file cycle: `vendor/i-slint-core/animations/simulations.rs -> vendor/i-slint-core/animations/simulations.rs`
- 1-file cycle: `vendor/i-slint-core/input.rs -> vendor/i-slint-core/input.rs`
- 1-file cycle: `vendor/i-slint-core/window.rs -> vendor/i-slint-core/window.rs`
- 1-file cycle: `vendor/i-slint-core/properties.rs -> vendor/i-slint-core/properties.rs`
- 1-file cycle: `vendor/i-slint-core/translations.rs -> vendor/i-slint-core/translations.rs`
- 2-file cycle: `crates/app/src/platform/android.rs -> crates/app/src/platform/android_media.rs -> crates/app/src/platform/android.rs`
- 2-file cycle: `crates/app/src/platform/android.rs -> crates/app/src/platform/android_notify.rs -> crates/app/src/platform/android.rs`
- 2-file cycle: `vendor/i-slint-core/model.rs -> vendor/i-slint-core/model/repeater.rs -> vendor/i-slint-core/model.rs`
- 3-file cycle: `vendor/i-slint-core/api.rs -> vendor/i-slint-core/future.rs -> vendor/i-slint-core/context.rs -> vendor/i-slint-core/api.rs`
- 3-file cycle: `vendor/i-slint-core/context.rs -> vendor/i-slint-core/input.rs -> vendor/i-slint-core/lib.rs -> vendor/i-slint-core/context.rs`
- 3-file cycle: `vendor/i-slint-core/context.rs -> vendor/i-slint-core/lengths.rs -> vendor/i-slint-core/lib.rs -> vendor/i-slint-core/context.rs`
- 3-file cycle: `vendor/i-slint-core/graphics/color.rs -> vendor/i-slint-core/properties/properties_animations.rs -> vendor/i-slint-core/items.rs -> vendor/i-slint-core/graphics/color.rs`
- 3-file cycle: `vendor/i-slint-core/api.rs -> vendor/i-slint-core/platform.rs -> vendor/i-slint-core/renderer.rs -> vendor/i-slint-core/api.rs`
- 3-file cycle: `vendor/i-slint-core/api.rs -> vendor/i-slint-core/input.rs -> vendor/i-slint-core/item_tree.rs -> vendor/i-slint-core/api.rs`
- 3-file cycle: `vendor/i-slint-core/input.rs -> vendor/i-slint-core/item_tree.rs -> vendor/i-slint-core/items.rs -> vendor/i-slint-core/input.rs`
- 3-file cycle: `vendor/i-slint-core/input.rs -> vendor/i-slint-core/item_tree.rs -> vendor/i-slint-core/window.rs -> vendor/i-slint-core/input.rs`

## Hyperedges (group relationships)
- **Foyer Cargo workspace crates** — readme_crate_app, readme_crate_emby_api, readme_crate_seerr_api, readme_crate_player [EXTRACTED 0.95]
- **Platform-specific areas needing Android replacements** — docs_android_plan_libmpv_android, docs_android_plan_apk_update, docs_android_plan_material_you, docs_android_plan_background_notifications [INFERRED 0.75]
- **Playback safety mechanisms** — plan_playback_state_machine, plan_fixemby_patches, plan_self_contained_design [INFERRED 0.75]

## Communities (260 total, 38 thin omitted)

### Community 0 - "CachedRenderingData"
Cohesion: 0.07
Nodes (35): BasicBorderRectangle, .cached_rendering_data_offset, BorderRectangle, .cached_rendering_data_offset, .cached_rendering_data_offset, .cached_rendering_data_offset, .cached_rendering_data_offset, .cached_rendering_data_offset (+27 more)

### Community 1 - "player/src/lib.rs"
Cohesion: 0.10
Nodes (30): Api, api(), available(), DEFAULT_OPTIONS, dll_candidates(), END_EOF, END_ERROR, engine_dir() (+22 more)

### Community 2 - "Ctx"
Cohesion: 0.09
Nodes (5): cards_model(), to_card(), Ctx, episode_summary(), nice_date()

### Community 3 - "update.rs"
Cohesion: 0.09
Nodes (23): Api, ApiAsset, ASSET, bad_signature(), check(), check_signature(), Checked, Latest (+15 more)

### Community 4 - "Foyer (native Emby client)"
Cohesion: 0.05
Nodes (46): Android and Android TV plan, Android TV (D-pad focus, 10-foot layouts), APK signing key backup, Signed APK self-update, Background notifications (new feature on Android), CI Android job publishing exe and apk, libmpv .so with MediaCodec (ExoPlayer fallback), Material You accent colour (+38 more)

### Community 5 - "app/src/lib.rs"
Cohesion: 0.03
Nodes (50): HideTarget, android_main(), apply_image(), AUDIO_PICK, auto_hw_mode(), cached_image(), ChanData, DEBUG (+42 more)

### Community 6 - "seerr-api/src/lib.rs"
Cohesion: 0.05
Nodes (31): ADMIN, AUTO_APPROVE, CREATE_ISSUES, Download, DownloadEp, downloads_label(), ISSUE_OPEN, ISSUE_RESOLVED (+23 more)

### Community 7 - "Control"
Cohesion: 0.09
Nodes (22): Control, AddSubtitle, AudioDelayBy, CycleAudio, CycleSubtitles, Finish, Quit, ResetAudioDelay (+14 more)

### Community 8 - "emby-api/src/lib.rs"
Cohesion: 0.05
Nodes (28): AuthResult, CLIENT_NAME, CLIENT_VERSION, de_id(), de_opt_id(), detect(), discover(), DiscoveredServer (+20 more)

### Community 9 - "notes.rs"
Cohesion: 0.06
Nodes (18): Ctx, Ctx, FOLLOWED, init_tray_events(), INSTANCE_ADDR, load_notes(), MAX_NOTES, NoteRec (+10 more)

### Community 10 - "Seerr"
Cohesion: 0.14
Nodes (7): Person, PublicSettings, Quota, QuotaPart, Result, Seerr, WatchItem

### Community 11 - "Result"
Cohesion: 0.09
Nodes (4): Browse, Chapter, Item, section_key()

### Community 12 - "sharedparley.rs"
Cohesion: 0.09
Nodes (17): ContentWidths, InnerTextLayoutCache, TextLayoutCache, draw_text(), draw_text_input(), link_in_layout(), link_under_cursor(), PhysicalPoint (+9 more)

### Community 13 - "Client"
Cohesion: 0.08
Nodes (4): Client, Other, LiveStream, Result

### Community 14 - "RemoteEvent"
Cohesion: 0.10
Nodes (19): parse(), RemoteEvent, FastForward, Message, Mute, Next, Pause, Play (+11 more)

### Community 15 - "ItemRc"
Cohesion: 0.04
Nodes (17): FocusEvent, FocusIn, FocusOut, FocusEventResult, FocusAccepted, FocusIgnored, InternalKeyEvent, KeyEventResult (+9 more)

### Community 16 - "seerr.rs"
Cohesion: 0.05
Nodes (35): offline_item(), title_of(), hidden(), SIssue, SReq, Item, describe_eps(), DetailCtx (+27 more)

### Community 17 - "gcast.rs"
Cohesion: 0.11
Nodes (14): ask(), DEFAULT_RECEIVER, Device, discover(), name(), NS_BEAT, NS_CONN, NS_MEDIA (+6 more)

### Community 18 - "TextInput"
Cohesion: 0.06
Nodes (41): KeyEventArg, PointArg, cursor_uses_selection_color(), mask_password(), next_paragraph_boundary(), next_word_boundary(), PASSWORD_CHARACTER, PreEditSelection (+33 more)

### Community 19 - "User"
Cohesion: 0.22
Nodes (3): Comment, Issue, User

### Community 20 - "sharedvector.rs"
Cohesion: 0.07
Nodes (33): alloc_with_capacity(), capacity_for_grow(), compute_inner_layout(), data_ptr(), drop_inner(), insert_out_of_bounds_test(), insert_test(), IntoIter (+25 more)

### Community 21 - "Page"
Cohesion: 0.10
Nodes (21): Page, Detail, Downloads, Genre, Library, LiveTv, Notifications, Person (+13 more)

### Community 22 - "ImageKind"
Cohesion: 0.22
Nodes (6): ImageKind, Backdrop, Logo, Primary, Thumb, Person

### Community 23 - "Media"
Cohesion: 0.25
Nodes (3): Media, Page, PersonCredits

### Community 24 - "window.rs"
Cohesion: 0.07
Nodes (42): Size, PropertyDirtyHandler, a_host_overrides_the_default_title(), accent_color(), context_for_root(), next_focus_item(), PopupWindow, PopupWindowLocation (+34 more)

### Community 25 - "Ctx"
Cohesion: 0.05
Nodes (23): applog(), apply_accent(), apply_material(), close(), Ctx, debug_banner(), dlog(), fav_group() (+15 more)

### Community 26 - "Foyer 0.22.0"
Cohesion: 0.17
Nodes (12): 🛠 Admin, Foyer 0.22.0, ✨ Highlights, 🏠 Home, 🔎 Libraries & search, 🖼 Look & feel, 🔔 Notifications, 🎬 Playback (+4 more)

### Community 27 - "Coord"
Cohesion: 0.13
Nodes (6): Orientation, text_layout_info(), grid_layout_info(), LayoutInfo, slint_grid_layout_info(), Coord

### Community 28 - "Detail"
Cohesion: 0.25
Nodes (5): Cast, Credits, Detail, Genre, Season

### Community 29 - "system_tray/windows.rs"
Cohesion: 0.03
Nodes (5): CLASS_NAME, MAX_DEPTH, MENU_CMD_BASE, TRAY_UID, WM_TRAYICON

### Community 30 - "Error"
Cohesion: 0.33
Nodes (6): Error, BadLogin, Forbidden, Network, NotSignedIn, Server

### Community 31 - "BorderRadius<T, U>"
Cohesion: 0.07
Nodes (10): ApproxEq, BorderRadius, BorderRadius<T, U>, BorderRadius<T, U1>, BorderRadius<T, U2>, IntBorderRadius, approx_eq!, PhysicalBorderRadius (+2 more)

### Community 32 - "Timer"
Cohesion: 0.12
Nodes (12): ActiveTimer, SingleShot, current_timers(), list_from_handle(), list_handle(), set_owning_context(), single_shot_on(), take_pending_timers() (+4 more)

### Community 33 - ".send"
Cohesion: 0.15
Nodes (6): log(), RequestAction, Approve, Decline, Delete, Retry

### Community 34 - "WindowInner"
Cohesion: 0.06
Nodes (14): WindowEventDispatchResult, Accepted, Rejected, InputMethodProperties, LayoutConstraints, MouseDispatchResult, NativePendingDrag, previous_focus_item() (+6 more)

### Community 35 - "foyer"
Cohesion: 0.50
Nodes (5): emby-api, foyer, foyer-core, player, seerr-api

### Community 36 - "Quality"
Cohesion: 0.13
Nodes (14): quality_rows(), log(), LOG, LogFn, set_log(), auto_steps_down_when_link_is_slow(), decide(), explicit_choices_and_config_roundtrip() (+6 more)

### Community 37 - "Client"
Cohesion: 0.12
Nodes (18): ActivityEntry, log(), arr(), f(), IDENT, lang2(), maps_an_episode(), metas() (+10 more)

### Community 38 - "Color"
Cohesion: 0.06
Nodes (26): Channel, Color, HsvaColor, linear_to_srgb(), OklabColor, OklchColor, quantize(), RgbaColor (+18 more)

### Community 40 - "adapters.rs"
Cohesion: 0.10
Nodes (30): AscendingSortHelper, check_content(), F, FilterModel, FilterModelInner, MapModel, ReverseModel, ReverseModel<M> (+22 more)

### Community 41 - "T"
Cohesion: 0.09
Nodes (16): (), report_model_error(), SharedVectorModel<T>, test_any_change_subsumes_row_tracking(), test_any_change_tracking(), test_data_tracking(), test_data_tracking_outside_a_binding(), test_serialize_deserialize_modelrc() (+8 more)

### Community 42 - "Android"
Cohesion: 0.40
Nodes (4): Android, Every feature works on both, and lands on `main`, graphify, Rules for keeping it compatible

### Community 44 - "super"
Cohesion: 0.10
Nodes (17): home_rows_model(), AUTOPLAY_STREAK_LIMIT, BASE, CARD_SCALES, Config, default_accent_mode(), DEFAULT_ACCENT_MODE, default_accent_rgb() (+9 more)

### Community 45 - "SharedString"
Cohesion: 0.06
Nodes (13): StandardListViewItem, SharedString, slint_shared_string_append(), slint_shared_string_bytes(), slint_shared_string_clone(), test_extend_from_chars(), test_slint_shared_string_append(), threading() (+5 more)

### Community 46 - "tests.rs"
Cohesion: 0.17
Nodes (19): paragraph_ranges(), bidi_selection_spans_are_ascending_in_x(), layout_text(), layout_text_with_builder(), layout_text_with_options(), layout_with_max_lines(), paragraphs(), test_cursor_affinity_at_soft_line_break() (+11 more)

### Community 47 - "Window"
Cohesion: 0.08
Nodes (13): logical_physical_pos(), logical_physical_size(), LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize, WindowPosition, Logical (+5 more)

### Community 48 - "properties/ffi.rs"
Cohesion: 0.10
Nodes (41): c_set_animated_binding(), c_set_animated_value(), ChangeTrackerOpaque, drop_fn(), eval_fn(), EvalState, ffi_change_tracker_does_not_leak_dep_nodes(), make_c_function_binding() (+33 more)

### Community 49 - "format!"
Cohesion: 0.40
Nodes (3): format!, T, ToSharedString

### Community 50 - "graphics/path.rs"
Cohesion: 0.08
Nodes (35): elements_path(), events_path(), fitted_path_sample_at_adds_offset_to_position(), fitted_path_sample_at_returns_none_for_zero_length_path(), fitted_path_sample_at_wraps_percent_for_open_path(), line_path(), LyonPathIteratorVariant, FromEvents (+27 more)

### Community 51 - "item_tree.rs"
Cohesion: 0.08
Nodes (41): assert_point_approx_eq(), create_children_nodes(), create_empty_subtree(), create_item_subtree_item(), create_nested_subtrees(), create_one_node_component(), create_subsubtree_items(), create_subsubtree_items_dynamic_elements() (+33 more)

### Community 52 - "flickable.rs"
Cohesion: 0.08
Nodes (22): abs(), CaptureEvents, MouseOrTouchScreen, MouseWheel, DECELERATION, DISTANCE_THRESHOLD, DURATION_THRESHOLD, ensure_in_bound() (+14 more)

### Community 53 - "items.rs"
Cohesion: 0.08
Nodes (15): ContextMenu, builtin_struct_field_default!, builtin_struct_field_default_doc!, declare_builtin_structs!, declare_enums!, declare_item_vtable!, MenuEntryArg, MenuEntryModel (+7 more)

### Community 54 - "PhysicalLength"
Cohesion: 0.16
Nodes (9): Glyph, GlyphRenderer, Layout, RectangleBorder, TextParagraph, visible_band(), visible_x_range(), ElisionInfo (+1 more)

### Community 55 - "SelectionSpan"
Cohesion: 0.14
Nodes (11): Hide, PhysicalRect, Layout, run_coverage(), RunCoverage, Full, Partial, Unselected (+3 more)

### Community 56 - "Slot"
Cohesion: 0.11
Nodes (13): Active, ACTIVE, Chromecast, Session, CAST_GEN, Ctx, PENDING, Target (+5 more)

### Community 57 - "i-slint-core/layout.rs"
Cohesion: 0.06
Nodes (49): collect_from_organized_data(), Constraint, CrossAxisSizing, FromMeasure, Minimum, Preferred, Errors, FlexboxLayoutCacheGenerator (+41 more)

### Community 58 - "DataTransfer"
Cohesion: 0.08
Nodes (25): DataTransfer, DataTransferError, TypeNotFound, CppUserData, DataTransferOpaque, path_from_units(), path_to_units(), PathValueType (+17 more)

### Community 59 - ".row_data"
Cohesion: 0.11
Nodes (9): FilterModelInner<M, F>, ModelChecker<Data>, ReverseModelInner<M>, SortModel<M, S>, SortModelInner<M, S>, test_filter_model_broken_model(), test_reversed_broken_model(), test_sorted_broken_model() (+1 more)

### Community 61 - "steps"
Cohesion: 0.05
Nodes (44): required, required, runs-on, steps, GH_TOKEN, KEYSTORE_B64, KEYSTORE_PASSWORD, OVERRIDE_URL (+36 more)

### Community 62 - "Community 62"
Cohesion: 0.08
Nodes (27): (), alloc_binding_holder(), BINDING_BORROWED, BINDING_POINTER_MASK, BINDING_POINTER_TO_BINDING, BindingCallable, .IS_TWO_WAY_BINDING, BindingHolder (+19 more)

### Community 63 - "Item"
Cohesion: 0.09
Nodes (15): Value, Item, CallbackInfo, common_property(), (F1, F2), FieldOffset<Item, crate::Callback<(), Ret>>, FieldOffset<Item, crate::Callback<(T,), Ret>>, FieldOffset<Item, Property<T>> (+7 more)

### Community 64 - "drag_n_drop.rs"
Cohesion: 0.10
Nodes (16): ALL, AllowedDragActions, clamp_action_to_allowed(), compute_proposed_action(), compute_proposed_action_modifier_table(), COPY_AND_MOVE, COPY_ONLY, cursor_for_action() (+8 more)

### Community 65 - "input.rs"
Cohesion: 0.09
Nodes (27): Keys, keys_from_parts(), keys_from_parts_inner(), KeysInner, KeysParseError, KeysParseErrorInner, IncompatibleModifiers, MultipleGraphemeClusters (+19 more)

### Community 66 - "VisitChildrenResult"
Cohesion: 0.11
Nodes (28): State, Deceleration, Done, SpringDamper, ItemTreeNode, DynamicTree, ItemTreeRc, ItemVisitorRefMut (+20 more)

### Community 67 - "ModelNotify"
Cohesion: 0.08
Nodes (16): BrokenModel, DependencyListHead, ModelChangeListenerBox<T>, ModelChangeListenerContainer, ModelChangeListenerContainer<T>, ModelNotify, ModelNotifyInner, ModelPeer (+8 more)

### Community 69 - "Model"
Cohesion: 0.08
Nodes (14): AbsentRowModel, Model, model_all(), model_any(), model_find_index(), ModelError, ModelIterator, ModelIterator<'a, T> (+6 more)

### Community 70 - "android_media.rs"
Cohesion: 0.13
Nodes (22): as_class(), ASKED_PERMISSION, CB, CB_CLASS, CHANNEL, create_session(), DEX, Handler (+14 more)

### Community 71 - "items/path.rs"
Cohesion: 0.14
Nodes (3): Path, slint_path_angle_at(), slint_path_point_at()

### Community 72 - "TestItemTree"
Cohesion: 0.08
Nodes (5): core::ops::Range<usize>, IndexRange, ItemTreeWeak, TestItemTree, TransformTestItemTree

### Community 73 - "ksni.rs"
Cohesion: 0.11
Nodes (16): build_ksni_menu(), dispatch_loop(), entry_to_node(), Event, Activate, Menu, image_to_argb_icon(), KsniTray (+8 more)

### Community 74 - "graphics/image.rs"
Cohesion: 0.13
Nodes (9): load_as_html_image(), load_image_from_data_uri(), LoadImageError, SharedPixelBuffer<Pixel>, slint_image_load_from_data(), test_image_invalid_svg(), test_image_load_from_data_svg(), test_image_load_from_data_svgz() (+1 more)

### Community 75 - ".get"
Cohesion: 0.12
Nodes (3): const_sentinel(), Property<T>, PropertyHandle

### Community 76 - "translations.rs"
Cohesion: 0.08
Nodes (31): c_char, slint_shared_string_from_bytes(), DisplayOrInt, Display, Int, format(), FormatArgs, FormatResult (+23 more)

### Community 78 - "TextCursorAffinity"
Cohesion: 0.15
Nodes (7): CachedTextInputAccessibilityState, compose_text_selection(), position_for_byte_offset(), selection_offsets(), to_actual_offset(), TextInputLayout<'a>, TextInputParagraph

### Community 79 - ".show_detail"
Cohesion: 0.06
Nodes (36): audio_streams(), card_for(), CardData, default_audio(), default_sub(), episode_row(), episode_tag(), is_english() (+28 more)

### Community 80 - "appkit.rs"
Cohesion: 0.06
Nodes (8): AppearanceObserver, build_menu(), entry_to_nsmenuitem(), image_to_nsimage(), MAX_DEPTH, MenuAction, PlatformTray, scheme_for_appearance()

### Community 81 - "brush.rs"
Cohesion: 0.11
Nodes (22): GradientStop, line_for_angle(), LinearGradientBrush, ResolvedConicGradient, ResolvedLinearGradient, ResolvedRadialGradient, sanitize_color_stops(), test_conic_gradient_all_stops_above_one() (+14 more)

### Community 82 - "android.rs"
Cohesion: 0.10
Nodes (17): AndroidCtx, bring_to_front(), CHANNEL, CTX, CUSTOM_RESIZE, drag_resize(), drag_window(), export_logs() (+9 more)

### Community 83 - "with_env"
Cohesion: 0.15
Nodes (14): font_scale(), form_factor(), install_package(), enter_pip(), in_system_pip(), pip_params(), sdk(), set_pip_auto() (+6 more)

### Community 84 - "BackendMouseEvent"
Cohesion: 0.10
Nodes (18): BackendDragEvent, Drop, Leave, Move, BackendMouseEvent, Exit, Moved, PinchGesture (+10 more)

### Community 85 - "two_way_binding.rs"
Cohesion: 0.11
Nodes (14): Property<T>, property_two_ways_binding_of_two_two_way_bindings(), property_two_ways_binding_of_two_way_binding_first(), property_two_ways_binding_of_two_way_binding_second(), property_two_ways_recurse_from_binding(), property_two_ways_test(), property_two_ways_test_binding(), test_two_way_with_map() (+6 more)

### Community 86 - "steps"
Cohesion: 0.05
Nodes (39): runs-on, steps, strategy, timeout-minutes, env, GH_TOKEN, IN_CI, MPV_ANDROID_REF (+31 more)

### Community 87 - "IntSize"
Cohesion: 0.09
Nodes (9): BorrowedOpenGLTexture, BorrowedOpenGLTextureBuilder, BorrowedOpenGLTextureOrigin, BottomLeft, TopLeft, HTMLImage, svg_intrinsic_size(), slint_image_size() (+1 more)

### Community 88 - "ParsedSVG"
Cohesion: 0.09
Nodes (11): CachedPath, ImageCacheKey, EmbeddedData, Invalid, Path, URL, OpaqueImageVTable, load_from_data() (+3 more)

### Community 89 - "i-slint-core/string.rs"
Cohesion: 0.10
Nodes (26): format(), shared_string_from_number(), shared_string_from_number_fixed(), shared_string_from_number_precision(), shared_string_from_number_unlocalized(), shared_string_replace_all(), simple_test(), slint_shared_string_drop() (+18 more)

### Community 90 - "properties_animations.rs"
Cohesion: 0.16
Nodes (27): AnimationDetail, Component, DELAY, DURATION, get_prop_value(), properties_test_animation_direction_triggered_by_set(), properties_test_animation_negative_delay_triggered_by_set(), properties_test_animation_triggered_by_binding() (+19 more)

### Community 91 - "Brush"
Cohesion: 0.20
Nodes (16): Brush, ConicGradient, LinearGradient, RadialGradient, SolidColor, resolve_brush(), ResolvedBrush, ConicGradient (+8 more)

### Community 92 - "pt"
Cohesion: 0.14
Nodes (29): classify(), Ev, Exit, Moved, PinchCancelled, PinchEnded, PinchMoved, PinchStarted (+21 more)

### Community 93 - ".init"
Cohesion: 0.18
Nodes (4): MenuDirtyHandler, MenuState, SystemTrayIconData, SystemTrayIconHandle

### Community 94 - "constant_deceleration.rs"
Cohesion: 0.16
Nodes (14): constant_deceleration_decreasing_limit_not_reached(), constant_deceleration_decreasing_limit_reached(), constant_deceleration_increasing_limit_not_reached(), constant_deceleration_increasing_limit_reached(), constant_deceleration_start_eq_limit(), ConstantDeceleration, ConstantDecelerationParameters, Direction (+6 more)

### Community 95 - "ItemTreeVTable"
Cohesion: 0.18
Nodes (4): ensure_item_tree_instantiated(), find_sibling_outside_repeater(), ItemTreeVTable, step_into_node()

### Community 96 - "LogicalLength"
Cohesion: 0.15
Nodes (4): LogicalLength, ListViewProperties, TypedListViewProps, update_visible_instances()

### Community 97 - "downloads.rs"
Cohesion: 0.11
Nodes (13): Ctx, CANCEL, Ctx, dir(), Entry, expand(), fetch_one(), key() (+5 more)

### Community 98 - "linebreaker.rs"
Cohesion: 0.12
Nodes (28): char_wrap_sentences(), test_basic_line_break(), test_basic_line_break_anywhere_fallback(), test_basic_line_break_anywhere_fallback_max_lines(), test_basic_line_break_anywhere_fallback_multi_line(), test_basic_line_break_anywhere_fallback_multi_line_char_wrap(), test_basic_line_break_anywhere_fallback_multi_line_v2(), test_basic_line_break_char_wrap() (+20 more)

### Community 99 - "RenderBorderRectangle"
Cohesion: 0.13
Nodes (4): adjust_rect_and_border_for_inner_drawing(), Rect, BorderRectLayout, RenderBorderRectangle

### Community 100 - "vec"
Cohesion: 0.08
Nodes (13): EventLoopError, EventLoopTerminated, NoEventLoopProvider, quit_event_loop(), FutureRunner, FutureRunner<T>, FutureRunnerInner, FutureState (+5 more)

### Community 101 - "Rgba8Pixel"
Cohesion: 0.09
Nodes (23): ImageWeightInBytes, replace_cached_image(), dynamic_image_to_shared_image_buffer(), image_to_rgba8_with_target_size(), ImageInner, BackendStorage, BorrowedOpenGLTexture, EmbeddedImage (+15 more)

### Community 102 - "GraphicsAPI"
Cohesion: 0.07
Nodes (19): F, GraphicsAPI, NativeOpenGL, WebGL, WGPU29, WGPU30, RenderingNotifier, RenderingState (+11 more)

### Community 104 - "callbacks.rs"
Cohesion: 0.23
Nodes (7): Callback<Arg, Ret>, callback_simple_test(), CallbackOpaque, slint_callback_call(), slint_callback_drop(), slint_callback_init(), slint_callback_set_handler()

### Community 105 - "DirtyRegion"
Cohesion: 0.11
Nodes (12): Box2D, ItemRendererFeatures, .SUPPORTS_TRANSFORMATIONS, LogicalPx, dirty_region_ignores_empty_rects(), dirty_region_no_intersection(), DirtyRegion, .MAX_COUNT (+4 more)

### Community 106 - "RepeaterInstanceOpsVTable"
Cohesion: 0.09
Nodes (6): RepeaterInstanceOps, RepeaterInstanceOpsVTable, RepeaterTracker<T>, RustRepeaterOps<'_, C>, slint_repeater_ensure_updated(), update_all_instances()

### Community 107 - ".compute_dirty_regions"
Cohesion: 0.17
Nodes (7): ItemTransform, CachedItemBoundingBoxAndTransform, ClipItem, ItemWithTransform, RegularItem, clipped_screen_rect(), PartialRenderer<'a, T>

### Community 108 - "T"
Cohesion: 0.14
Nodes (10): PropertyAnimation, Binding, AnimatedBindingCallable, AnimatedBindingCallable<T, A>, AnimatedValueBinding, AnimatedValueBinding<T>, LogicalLength, Property<T> (+2 more)

### Community 109 - "ChangeTracker"
Cohesion: 0.13
Nodes (5): change_multiple_dependencies(), change_tracker(), ChangeTracker, ChangeTrackerInner, delete_from_eval_fn()

### Community 110 - "LengthType"
Cohesion: 0.13
Nodes (7): euclid::Point2D<T, U>, euclid::Rect<T, U>, euclid::Size2D<T, U>, euclid::Vector2D<T, U>, PointLengths, RectLengths, SizeLengths

### Community 111 - "textlayout.rs"
Cohesion: 0.07
Nodes (8): DEFAULT_FONT_SIZE, FixedTestFont, render_lines(), test_content_widths(), test_content_widths_max_lines(), test_content_widths_min_never_exceeds_max(), TextLayout, TextParagraphLayout

### Community 112 - "SlintContext"
Cohesion: 0.06
Nodes (9): set_window_event_hook(), set_window_shown_hook(), SlintContext, SlintContextInner, SlintContextWeak, WindowEventHook, with_global_context(), detect_operating_system() (+1 more)

### Community 113 - "ConicGradientBrush"
Cohesion: 0.16
Nodes (8): center_or_bbox(), ConicGradientBrush, .HEADER, nan_eq(), RadialGradientBrush, .HEADER, slint_conic_gradient_apply_rotation(), slint_conic_gradient_normalize_stops()

### Community 114 - ".fmt"
Cohesion: 0.23
Nodes (3): DisplayOrInt<T>, FormatResult<'_, T>, WithPlural<'_, T>

### Community 115 - "MenuVTable"
Cohesion: 0.08
Nodes (13): MenuVTable, DragRequest, has_display_handle(), InputMethodRequest, Disable, Enable, Update, slint_windowrc_setup_menu_bar_shortcuts() (+5 more)

### Community 116 - "Slice"
Cohesion: 0.28
Nodes (20): box_layout_info(), box_layout_info_ortho(), BoxLayoutData, BoxLayoutOrthoData, flexbox_layout_info_cross_axis(), flexbox_layout_info_cross_axis_with_measure(), flexbox_layout_info_main_axis(), flexbox_layout_unwrapped_main() (+12 more)

### Community 117 - "textlayout/shaping.rs"
Cohesion: 0.13
Nodes (11): ShapeBoundaries, ShapeBoundaries<'a>, ShapeBuffer<Length>, test_byte_offsets_are_absolute(), test_letter_spacing(), test_shape_boundaries_empty(), test_shape_boundaries_script_change(), test_shape_boundaries_simple() (+3 more)

### Community 118 - "spring.rs"
Cohesion: 0.10
Nodes (16): regime_decays_to_rest_over_time(), regime_matches_initial_conditions(), SpringDurationBounceParameters, SpringParameters, SpringPhysicalParameters, SpringRegime, Critical, .CRITICAL_ZETA_EPSILON (+8 more)

### Community 119 - ".update_check_inner"
Cohesion: 0.31
Nodes (3): cleanup(), Ctx, exe_path()

### Community 120 - "Platform"
Cohesion: 0.11
Nodes (4): Clipboard, DefaultClipboard, SelectionClipboard, Platform

### Community 121 - "gdb_pretty_printers.py"
Cohesion: 0.10
Nodes (5): SharedVectorProvider, SharedVectorSubPrinter, SliceProvider, SliceSubPrinter, SlintCorePrettyPrinter

### Community 122 - "timers.rs"
Cohesion: 0.07
Nodes (33): _BUG3019, _BUG6141_SET_INTERVAL_FROM_CALLBACK, CallbackVariant, Empty, MultiFire, _DOUBLY_REGISTER_ACTIVE_TIMER, _DOUBLY_REGISTER_ACTIVE_TIMER_2, LIST_SHIFT (+25 more)

### Community 123 - "wgpu_29.rs"
Cohesion: 0.18
Nodes (11): any_wgpu29_adapters_with_gpu(), async_init_instance_adapter_device_queue_surface(), default_backends_to_avoid(), default_device_descriptor(), device_descriptor_from_settings(), init_instance_adapter_device_queue_surface(), init_instance_adapter_device_queue_surface_then(), mask_backends() (+3 more)

### Community 124 - "TextCursorDirection"
Cohesion: 0.08
Nodes (23): TextShortcut, DeleteBackward, DeleteForward, DeleteToStartOfLine, DeleteWordBackward, DeleteWordForward, Move, TextCursorDirection (+15 more)

### Community 125 - "i-slint-core/accessibility.rs"
Cohesion: 0.12
Nodes (16): AccessibilityAction, Decrement, Default, Expand, Increment, ReplaceSelectedText, SetSelectionOffsets, SetValue (+8 more)

### Community 126 - "Layout"
Cohesion: 0.20
Nodes (4): ElisionCut, Layout, line_fits_height(), TextCursorAffinity

### Community 127 - ".evaluate"
Cohesion: 0.17
Nodes (9): PropertyTracker<NEEDS_SET_DIRTY, ()>, PropertyTracker<NEEDS_SET_DIRTY, DirtyHandler>, PropertyTracker<true, DirtyHandler>, test_nested_property_tracker_dirty(), test_nested_property_tracker_evaluate_if_dirty(), test_nested_property_trackers(), test_property_dirty_handler(), test_property_listener_scope() (+1 more)

### Community 128 - "ItemCache<T>"
Cohesion: 0.12
Nodes (3): ItemCache<T>, LayerRenderer, render_layer()

### Community 129 - "wgpu_30.rs"
Cohesion: 0.20
Nodes (11): any_wgpu30_adapters_with_gpu(), async_init_instance_adapter_device_queue_surface(), default_backends_to_avoid(), default_device_descriptor(), device_descriptor_from_settings(), init_instance_adapter_device_queue_surface(), init_instance_adapter_device_queue_surface_then(), mask_backends() (+3 more)

### Community 130 - "graphics.rs"
Cohesion: 0.13
Nodes (7): create_graphics_api_wgpu_29(), create_graphics_api_wgpu_30(), physical_position_from_api(), physical_position_to_api(), physical_size_from_api(), Point, Transform

### Community 131 - "fragments.rs"
Cohesion: 0.15
Nodes (14): fragment_iterator_break_anywhere(), fragment_iterator_forced_break(), fragment_iterator_forced_break_multi(), fragment_iterator_leading_nbsp(), fragment_iterator_nbsp(), fragment_iterator_simple(), fragment_iterator_simple_v2(), TextFragment (+6 more)

### Community 132 - "rtti.rs"
Cohesion: 0.13
Nodes (6): BuiltinGlobal, BuiltinItem, FieldInfo, declare_ValueType!, declare_ValueType_2!, MaybeAnimatedPropertyInfoWrapper

### Community 133 - "GridLayoutOrganizedData"
Cohesion: 0.25
Nodes (8): GridLayoutInputData, GridLayoutOrganizedData, organize_dialog_button_layout(), organize_grid_layout(), OrganizedDataGenerator, slint_organize_dialog_button_layout(), slint_organize_grid_layout(), to_layout_data()

### Community 134 - "RendererSealed"
Cohesion: 0.22
Nodes (3): Renderer, RendererSealed, T

### Community 135 - ".window_item"
Cohesion: 0.11
Nodes (7): slint_windowrc_is_fullscreen(), slint_windowrc_is_maximized(), slint_windowrc_is_minimized(), slint_windowrc_set_fullscreen(), slint_windowrc_set_maximized(), slint_windowrc_set_minimized(), WindowProperties

### Community 136 - "ModelTracker"
Cohesion: 0.08
Nodes (5): BrokenModel<T>, FilterModel<M, F>, MapModel<M, F>, ModelChecker, ModelTracker

### Community 137 - "data_transfer.rs"
Cohesion: 0.24
Nodes (8): cleared_transfer_compares_equal_to_default(), DataTransferInner, every_payload_counts_as_native_data(), set_file_paths_with_empty_list_clears(), set_image_with_default_image_clears(), set_image_with_default_image_on_default_stays_empty(), set_plain_text_with_empty_string_clears(), set_plain_text_with_empty_string_on_default_stays_empty()

### Community 139 - "Inner"
Cohesion: 0.13
Nodes (9): append_menu_entry(), create_hicon(), ensure_class_registered(), Inner, notify_icon_data(), PlatformTray, show_popup_menu(), taskbar_created_message() (+1 more)

### Community 140 - "OnceCell"
Cohesion: 0.15
Nodes (6): FakeThreadStorage, FakeThreadStorage<T>, FakeThreadStorage<T, F>, SLINT__thread_local_inner!, thread_local_!, OnceCell<T>

### Community 141 - "AccessibleStringProperty"
Cohesion: 0.10
Nodes (20): AccessibleStringProperty, Checkable, Checked, DelegateFocus, Description, Enabled, Expandable, Expanded (+12 more)

### Community 142 - "MouseEvent"
Cohesion: 0.03
Nodes (47): Callback, MouseCursorInner, BuiltIn, CustomMouseCursor, InputEventFilterResult, ForwardAndIgnore, ForwardAndInterceptGrab, ForwardAndObserve (+39 more)

### Community 143 - "TouchState"
Cohesion: 0.19
Nodes (8): GestureRecognitionState, Idle, Pinching, TwoFingersDown, TouchEventBuffer, TouchState, .PINCH_THRESHOLD, .ROTATION_THRESHOLD

### Community 144 - "desktop.rs"
Cohesion: 0.11
Nodes (10): form_factor(), set_material(), SUPPORTS_MATERIAL, system_notify(), system_notify_rich(), FormFactor, Desktop, Touch (+2 more)

### Community 145 - ".insert"
Cohesion: 0.23
Nodes (10): touch_map_capacity(), touch_map_first(), touch_map_first_two_ids(), touch_map_get_mut(), touch_map_insert_and_get(), touch_map_remove(), touch_map_remove_nonexistent(), touch_map_update_existing() (+2 more)

### Community 146 - "StyledText"
Cohesion: 0.17
Nodes (12): color_to_styled_text(), get_raw_text(), parse_markdown(), slint_styled_text_clone(), slint_styled_text_drop(), slint_styled_text_eq(), slint_styled_text_from_markdown(), slint_styled_text_from_plain_text() (+4 more)

### Community 147 - "Image"
Cohesion: 0.10
Nodes (11): &'a ImageInner, ImageCache, Image, NineSlice, NineSliceImage, slint_image_compare_equal(), slint_image_load_from_path(), slint_image_set_nine_slice_edges() (+3 more)

### Community 148 - "Instant"
Cohesion: 0.14
Nodes (5): animation_tick(), AnimationDriver, current_tick(), Instant, update_animations()

### Community 149 - "MouseInputState"
Cohesion: 0.17
Nodes (10): handle_mouse_grab(), DelayForwarding, MouseGrabResult, MouseInputResult, MouseInputState, offer_native_drag(), process_delayed_event(), process_mouse_input() (+2 more)

### Community 150 - "LogicalRect"
Cohesion: 0.05
Nodes (23): clip_content_box(), item_children_bounding_rect(), item_children_bounding_rect_transformed(), item_with_children_bounding_rect_transformed(), ItemRenderer, render_component_items(), render_item_children(), RenderRectangle (+15 more)

### Community 151 - "DependencyNode"
Cohesion: 0.21
Nodes (5): DependencyListHead, DependencyListHead<T>, DependencyNode, DependencyNode<T>, mark_dependencies_dirty()

### Community 152 - "item_focus.rs"
Cohesion: 0.44
Nodes (10): default_next_in_local_focus_chain(), default_previous_in_local_focus_chain(), step_into_node(), step_out_of_node(), test_focus_chain_complex_tree(), test_focus_chain_one_child(), test_focus_chain_root_only(), test_focus_chain_three_children() (+2 more)

### Community 153 - "erased_bindings.rs"
Cohesion: 0.29
Nodes (11): change_tracker_init_erased(), change_tracker_init_impl(), set_animated_property_binding_erased(), set_animated_property_binding_impl(), set_callback_handler_erased(), set_callback_handler_impl(), set_property_binding_erased(), set_property_binding_impl() (+3 more)

### Community 154 - "KeyEventType"
Cohesion: 0.33
Nodes (6): KeyEventType, CommitComposition, KeyPressed, KeyReleased, UpdateComposition, slint_windowrc_dispatch_key_event()

### Community 155 - "solve_flexbox_layout_with_measure"
Cohesion: 0.10
Nodes (19): agrees(), agrees_for_a_column(), assert_agrees(), assert_agrees_column(), cell(), constraints(), FlexboxLayoutItemInfo, FlexboxMeasureFn (+11 more)

### Community 156 - "super"
Cohesion: 0.16
Nodes (11): cached_paragraphs(), CachedParagraphs, CachedParagraphsGuard, ENTRY_LIMIT, layout(), LayoutOptions, line_limit_cut(), LineBreakingInputs (+3 more)

### Community 157 - "FittedPathBox"
Cohesion: 0.24
Nodes (5): FittedPath, FittedPathBox, FittedPathInner, slint_path_fitted_cache_free(), slint_path_fitted_cache_init()

### Community 159 - "platform/windows.rs"
Cohesion: 0.13
Nodes (7): RichNote, APP_ID, set_material(), SUPPORTS_MATERIAL, system_notify(), system_notify_rich(), toast_xml()

### Community 160 - "&rustybuzz::Face<'_>"
Cohesion: 0.16
Nodes (5): AbstractFont, FontMetrics, &rustybuzz::Face<'_>, T, TextShaper

### Community 161 - "constant_deceleration_spring_damper.rs"
Cohesion: 0.23
Nodes (8): calculate_parameters(), constant_deceleration_decreasing_limit_not_reached(), constant_deceleration_increasing_limit_not_reached(), constant_deceleration_spring_damper_decreasing_limit_reached(), constant_deceleration_spring_damper_increasing_limit_reached(), constant_deceleration_start_eq_limit(), ConstantDecelerationSpringDamper, ConstantDecelerationSpringDamperParameters

### Community 162 - "debug_log.rs"
Cohesion: 0.19
Nodes (9): default_log_message(), log_message(), LogMessage, LogMessage<'a>, LogMessageHandler, LogMessageLocation, LogMessageSource, Runtime (+1 more)

### Community 163 - "repeater.rs"
Cohesion: 0.17
Nodes (12): Conditional, ItemTreeRc, MAX_EAGER_INSTANCE_COUNT, RepeatedInstanceState, Clean, Dirty, Repeater, RepeaterInner (+4 more)

### Community 165 - "platform.rs"
Cohesion: 0.06
Nodes (27): logical_position_to_api(), _ANIM_TICK_UPDATED_ON_PLATFORM_SET, EventLoopProxy, InternalEvent, Key, Mouse, Touch, InternalEventBox (+19 more)

### Community 166 - "api.rs"
Cohesion: 0.12
Nodes (6): Global, Weak, WindowHandle, WindowHandleInner, HandleByAdapter, HandleByRcRWH

### Community 168 - "data_ringbuffer.rs"
Cohesion: 0.22
Nodes (8): test_empty_buffer(), test_push_beyond_capacity(), test_push_beyond_capacity_wrap_back(), test_push_single_element(), test_push_two_elements(), test_push_until_full(), VelocityRingBuffer, VelocityRingBuffer<N>

### Community 170 - "LyonPathEventIterator"
Cohesion: 0.20
Nodes (4): LyonPathEventIterator, FromEvents, FromPath, TransformedLyonPathIterator<EventIt>

### Community 171 - "Changelog"
Cohesion: 0.10
Nodes (19): Changelog, Foyer 0.23.0, Foyer 0.23.1, Foyer 0.23.2, Foyer 0.26.10, Foyer 0.26.11, Foyer 0.26.12, Foyer 0.26.5 (+11 more)

### Community 172 - "android_notify.rs"
Cohesion: 0.18
Nodes (9): active_ids(), EXTRA, launch_note_target(), LAUNCH_TAKEN, poll_note_tap(), POSTED, sdk(), system_notify_rich() (+1 more)

### Community 173 - "play_inner"
Cohesion: 0.14
Nodes (16): yes(), ExtSub, is_text_codec(), Options, Outcome, Failed, Finished, Stopped (+8 more)

### Community 174 - "date_time.rs"
Cohesion: 0.17
Nodes (12): date_now(), format_date(), month_day_count(), month_offset(), parse_date(), slint_date_time_date_now(), slint_date_time_format_date(), slint_date_time_month_day_count() (+4 more)

### Community 175 - "animations.rs"
Cohesion: 0.08
Nodes (24): CubicBezierSegment, ease_out_bounce_curve(), easing_curve(), EasingCurve, CubicBezier, EaseInBounce, EaseInElastic, EaseInOutBounce (+16 more)

### Community 177 - "Error"
Cohesion: 0.10
Nodes (11): ItemWeak, activate_entry(), AppearanceObserverIvars, MenuActionIvars, PlatformTray, Error, EventLoopError, PlatformError (+3 more)

### Community 178 - "TexturePixelFormat"
Cohesion: 0.25
Nodes (7): test_image_load_from_data_png(), TexturePixelFormat, AlphaMap, Rgb, Rgba, RgbaPremultiplied, SignedDistanceField

### Community 179 - "Length"
Cohesion: 0.26
Nodes (4): PositionedGlyph, test_text_size_with_max_lines(), TextLayout<'_, Font>, TextParagraphLayout<'_, Font>

### Community 180 - "CheckedAdd"
Cohesion: 0.12
Nodes (6): CheckedAdd, DivCount, euclid::Length<i16, U>, euclid::Length<T, U>, f32, i16

### Community 181 - "Surface"
Cohesion: 0.25
Nodes (6): GlFns, GlLoader, GlSaved, MiniTex, SurfaceInner, SurfState

### Community 182 - ".evaluate"
Cohesion: 0.16
Nodes (8): AnimationState, Animating, Delaying, Done, Property<Length<crate::Coord, Unit>>, PropertyPhysicsAnimationData, PropertyPhysicsAnimationData<S>, RefCell<PropertyPhysicsAnimationData<S>>

### Community 183 - "rendering_metrics_collector.rs"
Cohesion: 0.21
Nodes (7): debug_log!, FrameData, RefreshMode, FullSpeed, Lazy, RenderingMetrics, RenderingMetricsCollector

### Community 184 - "lengths.rs"
Cohesion: 0.15
Nodes (7): logical_point_from_api(), logical_size_from_api(), logical_size_to_api(), LogicalEdges, LogicalSize, PhysicalBorderRadius, PhysicalEdges

### Community 186 - "Slint Software License"
Cohesion: 0.13
Nodes (14): 1. Grant of Rights, 2. License Conditions, 3. Limitations, 4. Audit Rights, 5. Termination, 6. Assignment, 7. Severability, 8. Governing Law (+6 more)

### Community 188 - "StyledTextItem"
Cohesion: 0.03
Nodes (11): HasFont, PlainOrStyledText, Plain, Styled, RenderString, RenderText, (SharedString, Brush), StringArg (+3 more)

### Community 189 - "InterpolatedPropertyValue"
Cohesion: 0.13
Nodes (5): f32, i32, i64, InterpolatedPropertyValue, u8

### Community 190 - "SystemTrayIconDataBox"
Cohesion: 0.29
Nodes (3): slint_system_tray_icon_data_free(), slint_system_tray_icon_data_init(), SystemTrayIconDataBox

### Community 191 - "ItemRef"
Cohesion: 0.38
Nodes (3): ItemTreeRefPin, slint_unregister_item_tree(), unregister_item_tree()

### Community 193 - ".get"
Cohesion: 0.16
Nodes (4): RemoteSubtitle, SystemInfo, Task, VirtualFolder

### Community 194 - "KeyboardModifiers"
Cohesion: 0.26
Nodes (6): InternalKeyboardModifierState, KeyboardModifiers, modifiers(), AnchorMode, KeepAnchor, MoveAnchor

### Community 195 - "mod.rs"
Cohesion: 0.10
Nodes (13): MediaAction, Forward, Next, Pause, Play, Prev, Rewind, Seek (+5 more)

### Community 196 - "steps"
Cohesion: 0.16
Nodes (16): GH_TOKEN, NOTES, RUSTFLAGS, UPDATE_SIGNING_KEY, steps, env, id, name (+8 more)

### Community 197 - "PhysicalPx"
Cohesion: 0.09
Nodes (23): clone_from_slice_rejects_a_long_slice(), clone_from_slice_rejects_a_short_slice(), f32, fit(), fit9slice(), FitResult, StaticTextures, load_image_from_embedded_data() (+15 more)

### Community 198 - "RequestedGraphicsAPI"
Cohesion: 0.19
Nodes (10): RequestedGraphicsAPI, Direct3D, Metal, OpenGL, Vulkan, WGPU29, WGPU30, RequestedOpenGLVersion (+2 more)

### Community 199 - "refcell"
Cohesion: 0.38
Nodes (3): BoxShadowCache, CacheEntry, MAX_CACHED_SHADOWS

### Community 200 - "SingleLinkedListPinHead<T>"
Cohesion: 0.32
Nodes (3): big_list(), SingleLinkedListPinHead<T>, test_list()

### Community 201 - "AnimatedBindingState"
Cohesion: 0.50
Nodes (4): AnimatedBindingState, Animating, NotAnimating, ShouldStart

### Community 203 - "winit_ops.rs"
Cohesion: 0.16
Nodes (9): bring_to_front(), CUSTOM_RESIZE, drag_resize(), drag_window(), monitor_area(), set_cursor_visible(), set_decorations(), set_window_rect() (+1 more)

### Community 204 - "InternalToken"
Cohesion: 0.29
Nodes (4): InternalToken, ErrorImpl, OutOfBounds, Unsupported

### Community 207 - "StandardShortcut"
Cohesion: 0.17
Nodes (11): StandardShortcut, Copy, Cut, Find, Paste, Print, Redo, Refresh (+3 more)

### Community 208 - "LayoutData"
Cohesion: 0.18
Nodes (5): Adjust, adjust_items(), Grow, LayoutData, Shrink

### Community 211 - "src/color.rs"
Cohesion: 0.50
Nodes (3): parse_hex(), rgb_hsv_roundtrip_matches_known_colours(), rgb_to_hsv()

### Community 212 - "c_void"
Cohesion: 0.11
Nodes (11): CloseRequestResponse, HideWindow, KeepWindowShown, has_window_handle(), slint_windowrc_hinstance_win32(), slint_windowrc_hwnd_win32(), slint_windowrc_nsview_appkit(), slint_windowrc_on_close_requested() (+3 more)

### Community 213 - "ScaleFactor"
Cohesion: 0.12
Nodes (15): FontRequest, ScaleFactor, char_size(), font_metrics(), FontContext, Brush, content_widths_builder(), create_text_paragraphs() (+7 more)

### Community 214 - "component_factory.rs"
Cohesion: 0.31
Nodes (3): ComponentFactory, ComponentFactoryInner, FactoryContext

### Community 215 - "AnimatedBindingKind"
Cohesion: 0.33
Nodes (4): AnimatedBindingKind, Animation, NotAnimated, Transition

### Community 217 - "frame"
Cohesion: 0.40
Nodes (5): decode(), field(), frame(), frames_round_trip(), varint()

### Community 219 - "menus.rs"
Cohesion: 0.15
Nodes (7): CachedGraphicsData, CachedGraphicsData<T>, ItemCache, MenuFromItemTree, ShadowTreeNode, slint_menus_create_wrapper(), PropertyTracker

### Community 220 - "Renderer"
Cohesion: 0.09
Nodes (3): ItemTreeRef, Renderer, ItemRef

### Community 221 - "DrawOutcome"
Cohesion: 0.40
Nodes (5): DrawOutcome, Occluded, Skipped, Success, Timeout

### Community 222 - "RepaintBufferType"
Cohesion: 0.50
Nodes (4): RepaintBufferType, NewBuffer, ReusedBuffer, SwappedBuffers

### Community 226 - "benches/string.rs"
Cohesion: 0.31
Nodes (5): REFERENCE_NUMBER, RESULT_NUMBER, string_replace(), string_replace_own_character(), string_replacen()

### Community 227 - "android"
Cohesion: 0.09
Nodes (22): secrets, uses, with, notes, jobs, android, publish, name (+14 more)

### Community 231 - "pathbuf"
Cohesion: 0.32
Nodes (5): Asset, find_dll(), install(), Release, RELEASES

### Community 235 - "windows"
Cohesion: 0.25
Nodes (8): run, windows, tag, version, shell, defaults, outputs, runs-on

### Community 236 - "bitmapfont.rs"
Cohesion: 0.43
Nodes (4): BitmapFont, BitmapGlyph, BitmapGlyphs, CharacterMapEntry

### Community 239 - "PlatformError"
Cohesion: 0.05
Nodes (15): ComponentHandle, invoke_from_event_loop(), PlatformError, NoEventLoopProvider, NoPlatform, Other, OtherError, SetPlatformError (+7 more)

### Community 241 - "Cmd"
Cohesion: 0.28
Nodes (7): Cast, Cmd, Pause, Play, Seek, Stop, start()

### Community 242 - "Slint Royalty-free Desktop, Mobile, and Web Applications License"
Cohesion: 0.29
Nodes (6): 1. Grant of Rights, 2. License Conditions - Attribution, 3. Limitations, 4. Warranty and Liability, Preamble, Slint Royalty-free Desktop, Mobile, and Web Applications License

### Community 245 - "SurfaceTarget"
Cohesion: 0.47
Nodes (3): SurfaceTarget, Drm, WindowHandle

### Community 246 - "SurfaceTarget"
Cohesion: 0.47
Nodes (3): SurfaceTarget, Drm, WindowHandle

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

## Knowledge Gaps
- **459 isolated node(s):** `Session`, `Chromecast`, `Session`, `Home`, `Backdrop` (+454 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1708 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **38 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SharedString` connect `SharedString` to `graphics.rs`, `app/src/lib.rs`, `.window_item`, `data_transfer.rs`, `AccessibleStringProperty`, `MouseEvent`, `ItemRc`, `TextInput`, `Image`, `LogicalRect`, `window.rs`, `KeyEventType`, `WindowInner`, `platform.rs`, `api.rs`, `T`, `date_time.rs`, `format!`, `graphics/path.rs`, `item_tree.rs`, `items.rs`, `i-slint-core/layout.rs`, `DataTransfer`, `StyledTextItem`, `input.rs`, `KeyboardModifiers`, `PhysicalPx`, `TestItemTree`, `translations.rs`, `ScaleFactor`, `ParsedSVG`, `i-slint-core/string.rs`, `menus.rs`, `ItemTreeVTable`, `benches/string.rs`, `Rgba8Pixel`, `PlatformError`, `SlintContext`, `i-slint-core/accessibility.rs`?**
  _High betweenness centrality (0.101) - this node is a cross-community bridge._
- **Are the 46 inferred relationships involving `ItemRc` (e.g. with `.font_request()` and `item_children_bounding_rect()`) actually correct?**
  _`ItemRc` has 46 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Session`, `Chromecast`, `Session` to the rest of the system?**
  _459 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `CachedRenderingData` be split into smaller, more focused modules?**
  _Cohesion score 0.07020408163265306 - nodes in this community are weakly interconnected._
- **Why does `ItemRc` connect `ItemRc` to `ItemCache<T>`, `CachedRenderingData`, `sharedparley.rs`, `AccessibleStringProperty`, `MouseEvent`, `TextInput`, `MouseInputState`, `LogicalRect`, `window.rs`, `Coord`, `super`, `WindowInner`, `PartialRenderer<'_, T>`, `SharedString`, `Error`, `item_tree.rs`, `flickable.rs`, `items.rs`, `PartialRendererCache`, `StyledTextItem`, `items/image.rs`, `drag_n_drop.rs`, `input.rs`, `component_container.rs`, `refcell`, `TestItemTree`, `items/path.rs`, `TextCursorAffinity`, `ScaleFactor`, `menus.rs`, `Renderer`, `.init`, `ItemTreeVTable`, `DirtyRegion`, `.compute_dirty_regions`, `SlintContext`, `MenuVTable`, `i-slint-core/accessibility.rs`?**
  _High betweenness centrality (0.060) - this node is a cross-community bridge._
- **Are the 4 inferred relationships involving `SharedString` (e.g. with `.set_xdg_app_id()` and `.xdg_app_id()`) actually correct?**
  _`SharedString` has 4 INFERRED edges - model-reasoned connections that need verification._
- **Should `player/src/lib.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.0951219512195122 - nodes in this community are weakly interconnected._