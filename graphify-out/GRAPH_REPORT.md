# Graph Report - Foyer  (2026-11-17)

## Corpus Check
- 152 files · ~308,945 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 10 file(s) not represented in the graph (top: (none) 4, .xml 2, .rc 1)

## Summary
- 6855 nodes · 17686 edges · 257 communities (215 shown, 42 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 952 edges (avg confidence: 0.56)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `90599563`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- Property
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
- MouseEvent
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
- .process_mouse_input
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
- Image
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
- .first_child
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
- JNIEnv
- BackendMouseEvent
- two_way_binding.rs
- steps
- IntSize
- ParsedSVG
- i-slint-core/string.rs
- properties_animations.rs
- Brush
- pt
- system_tray.rs
- constant_deceleration.rs
- ItemTreeVTable
- LogicalLength
- downloads.rs
- linebreaker.rs
- Api
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
- MenuItem
- gdb_pretty_printers.py
- timers.rs
- wgpu_29.rs
- TextCursorDirection
- AccessibilityAction
- LogicalPosition
- .evaluate
- ItemCache<T>
- wgpu_30.rs
- graphics.rs
- fragments.rs
- register_item_tree
- SharedPixelBuffer<Pixel>
- RendererSealed
- .window_item
- ModelTracker
- i-slint-core/accessibility.rs
- ItemCache
- Inner
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
- LogicalRect
- DependencyNode
- item_focus.rs
- erased_bindings.rs
- KeyEventType
- solve_flexbox_layout_with_measure
- super
- append_menu_entry
- TestView
- platform/windows.rs
- &rustybuzz::Face<'_>
- constant_deceleration_spring_damper.rs
- debug_log.rs
- RepeatedItemTree
- NoVerify
- platform.rs
- api.rs
- data_ringbuffer.rs
- PartialRenderer<'_, T>
- InputMethodProperties
- Changelog
- android_notify.rs
- play_inner
- date_time.rs
- animations.rs
- Error
- LogicalRect
- Length
- CheckedAdd
- Surface
- .evaluate
- rendering_metrics_collector.rs
- lengths.rs
- PartialRendererCache
- Slint Software License
- media.rs
- RenderText
- InterpolatedPropertyValue
- ItemRef
- MediaCb
- .get
- KeyboardModifiers
- mod.rs
- steps
- PhysicalPx
- RequestedGraphicsAPI
- SingleLinkedListPinHead<T>
- AnimatedBindingState
- Any
- winit_ops.rs
- WGPUSettings
- WGPUSettings
- StandardShortcut
- LayoutData
- Export.java
- preview.rs
- src/color.rs
- CloseRequestResponse
- FontContext
- component_factory.rs
- EventLoopError
- User
- T
- SharedVector
- Renderer
- CrossAxisSizing
- Conditional<C>
- Immersive
- benches/string.rs
- android
- TextLineBreaker
- pathbuf
- DragRequest
- WindowAdapter
- windows
- bitmapfont.rs
- FixedTestFont
- i-slint-core/lib.rs
- Ctx
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
- VelocityRingBuffer<N>
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
10. `Ctx` - 93 edges

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

## Communities (257 total, 42 thin omitted)

### Community 0 - "Property"
Cohesion: 0.03
Nodes (61): BasicBorderRectangle, .cached_rendering_data_offset, BorderRectangle, .cached_rendering_data_offset, BoxShadow, .cached_rendering_data_offset, Clip, .cached_rendering_data_offset (+53 more)

### Community 1 - "player/src/lib.rs"
Cohesion: 0.12
Nodes (22): api(), available(), DEFAULT_OPTIONS, dll_candidates(), END_EOF, END_ERROR, engine_dir(), EV_END_FILE (+14 more)

### Community 2 - "Ctx"
Cohesion: 0.10
Nodes (3): Ctx, episode_summary(), nice_date()

### Community 3 - "update.rs"
Cohesion: 0.09
Nodes (23): Api, ApiAsset, ASSET, bad_signature(), check(), check_signature(), Checked, Latest (+15 more)

### Community 4 - "Foyer (native Emby client)"
Cohesion: 0.05
Nodes (46): Android and Android TV plan, Android TV (D-pad focus, 10-foot layouts), APK signing key backup, Signed APK self-update, Background notifications (new feature on Android), CI Android job publishing exe and apk, libmpv .so with MediaCodec (ExoPlayer fallback), Material You accent colour (+38 more)

### Community 5 - "app/src/lib.rs"
Cohesion: 0.03
Nodes (59): HideTarget, android_main(), apply_image(), AUDIO_PICK, audio_streams(), auto_hw_mode(), cached_image(), ChanData (+51 more)

### Community 6 - "seerr-api/src/lib.rs"
Cohesion: 0.05
Nodes (31): ADMIN, AUTO_APPROVE, CREATE_ISSUES, Download, DownloadEp, downloads_label(), ISSUE_OPEN, ISSUE_RESOLVED (+23 more)

### Community 7 - "Control"
Cohesion: 0.09
Nodes (22): Control, AddSubtitle, AudioDelayBy, CycleAudio, CycleSubtitles, Finish, Quit, ResetAudioDelay (+14 more)

### Community 8 - "emby-api/src/lib.rs"
Cohesion: 0.05
Nodes (28): Chapter, CLIENT_NAME, CLIENT_VERSION, de_id(), de_opt_id(), detect(), discover(), DiscoveredServer (+20 more)

### Community 9 - "notes.rs"
Cohesion: 0.07
Nodes (16): Ctx, FOLLOWED, init_tray_events(), INSTANCE_ADDR, load_notes(), MAX_NOTES, NoteRec, notes_path() (+8 more)

### Community 10 - "Seerr"
Cohesion: 0.14
Nodes (7): Person, PublicSettings, Quota, QuotaPart, Result, Seerr, WatchItem

### Community 11 - "Result"
Cohesion: 0.12
Nodes (4): Browse, Item, Client, section_key()

### Community 12 - "sharedparley.rs"
Cohesion: 0.08
Nodes (31): RenderString, ScaleFactor, ContentWidths, InnerTextLayoutCache, TextLayoutCache, draw_text(), draw_text_input(), link_in_layout() (+23 more)

### Community 13 - "Client"
Cohesion: 0.07
Nodes (3): next_up_items(), Client, Result

### Community 14 - "RemoteEvent"
Cohesion: 0.10
Nodes (19): parse(), RemoteEvent, FastForward, Message, Mute, Next, Pause, Play (+11 more)

### Community 15 - "MouseEvent"
Cohesion: 0.04
Nodes (36): MouseCursorInner, BuiltIn, CustomMouseCursor, FocusEvent, FocusIn, FocusOut, FocusEventResult, FocusAccepted (+28 more)

### Community 16 - "seerr.rs"
Cohesion: 0.06
Nodes (33): SIssue, SReq, DetailCtx, DetailData, EDIT_BITS, ep(), ISSUE_FILTERS, IssueData (+25 more)

### Community 17 - "gcast.rs"
Cohesion: 0.10
Nodes (16): ask(), DEFAULT_RECEIVER, Device, discover(), field(), name(), NS_BEAT, NS_CONN (+8 more)

### Community 18 - "TextInput"
Cohesion: 0.03
Nodes (46): FontRequest, HasFont, KeyEventArg, PointArg, StringArg, ComplexText, cursor_uses_selection_color(), mask_password() (+38 more)

### Community 19 - "User"
Cohesion: 0.22
Nodes (3): Comment, Issue, User

### Community 20 - "sharedvector.rs"
Cohesion: 0.07
Nodes (33): alloc_with_capacity(), capacity_for_grow(), compute_inner_layout(), data_ptr(), drop_inner(), insert_out_of_bounds_test(), insert_test(), IntoIter (+25 more)

### Community 21 - "Page"
Cohesion: 0.09
Nodes (22): Page, Detail, Downloads, Genre, Home, Library, LiveTv, Notifications (+14 more)

### Community 22 - "ImageKind"
Cohesion: 0.22
Nodes (6): ImageKind, Backdrop, Logo, Primary, Thumb, Person

### Community 23 - "Media"
Cohesion: 0.25
Nodes (3): Media, Page, PersonCredits

### Community 24 - "window.rs"
Cohesion: 0.07
Nodes (44): Size, a_host_overrides_the_default_title(), accent_color(), context_for_root(), GraphicsAPI, Inaccessible, NativeOpenGL, has_display_handle() (+36 more)

### Community 25 - "Ctx"
Cohesion: 0.06
Nodes (15): applog(), apply_accent(), apply_material(), close(), Ctx, goto_episode(), load_posters(), login_error() (+7 more)

### Community 26 - "Foyer 0.22.0"
Cohesion: 0.17
Nodes (12): 🛠 Admin, Foyer 0.22.0, ✨ Highlights, 🏠 Home, 🔎 Libraries & search, 🖼 Look & feel, 🔔 Notifications, 🎬 Playback (+4 more)

### Community 27 - "Coord"
Cohesion: 0.10
Nodes (10): Orientation, text_layout_info(), Constraint, grid_layout_info(), GridLayoutCacheGenerator<'a>, LayoutCacheGenerator<'a>, LayoutInfo, min_max_size_for_layout_constraints() (+2 more)

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

### Community 32 - ".process_mouse_input"
Cohesion: 0.11
Nodes (7): WindowEventDispatchResult, Accepted, Rejected, MouseDispatchResult, previous_focus_item(), slint_windowrc_set_focus_item(), WindowAdapterInternal

### Community 33 - ".send"
Cohesion: 0.15
Nodes (6): log(), RequestAction, Approve, Decline, Delete, Retry

### Community 35 - "foyer"
Cohesion: 0.50
Nodes (5): emby-api, foyer, foyer-core, player, seerr-api

### Community 36 - "Quality"
Cohesion: 0.11
Nodes (16): quality_rows(), Source, Live, log(), LOG, LogFn, set_log(), auto_steps_down_when_link_is_slow() (+8 more)

### Community 37 - "Client"
Cohesion: 0.13
Nodes (19): ActivityEntry, log(), RemoteSubtitle, arr(), f(), IDENT, lang2(), maps_an_episode() (+11 more)

### Community 38 - "Color"
Cohesion: 0.07
Nodes (26): Channel, Color, HsvaColor, linear_to_srgb(), OklabColor, OklchColor, quantize(), RgbaColor (+18 more)

### Community 40 - "adapters.rs"
Cohesion: 0.10
Nodes (30): AscendingSortHelper, check_content(), F, FilterModel, FilterModelInner, MapModel, ReverseModel, ReverseModel<M> (+22 more)

### Community 41 - "T"
Cohesion: 0.11
Nodes (20): InternalToken, (), AbsentRowModel, model_all(), model_any(), model_find_index(), ModelIterator<'_, T>, test_any_change_subsumes_row_tracking() (+12 more)

### Community 42 - "Android"
Cohesion: 0.40
Nodes (4): Android, Every feature works on both, and lands on `main`, graphify, Rules for keeping it compatible

### Community 44 - "super"
Cohesion: 0.11
Nodes (18): AUTOPLAY_STREAK_LIMIT, BASE, CARD_SCALES, Config, default_accent_mode(), DEFAULT_ACCENT_MODE, default_accent_rgb(), default_card_scale() (+10 more)

### Community 45 - "SharedString"
Cohesion: 0.06
Nodes (14): StandardListViewItem, SharedString, simple_test(), slint_shared_string_append(), slint_shared_string_bytes(), String, test_extend_from_chars(), test_slint_shared_string_append() (+6 more)

### Community 46 - "tests.rs"
Cohesion: 0.22
Nodes (14): paragraph_ranges(), bidi_selection_spans_are_ascending_in_x(), layout_text(), layout_text_with_options(), layout_with_max_lines(), paragraphs(), test_cursor_between_cr_and_lf(), test_max_lines_below_line_limit() (+6 more)

### Community 47 - "Window"
Cohesion: 0.08
Nodes (8): logical_physical_pos(), logical_physical_size(), LogicalSize, PhysicalSize, Window, WindowSize, Logical, Physical

### Community 48 - "properties/ffi.rs"
Cohesion: 0.10
Nodes (42): PropertyAnimation, c_set_animated_binding(), c_set_animated_value(), ChangeTrackerOpaque, drop_fn(), eval_fn(), EvalState, ffi_change_tracker_does_not_leak_dep_nodes() (+34 more)

### Community 49 - "format!"
Cohesion: 0.40
Nodes (3): format!, T, ToSharedString

### Community 50 - "graphics/path.rs"
Cohesion: 0.06
Nodes (40): elements_path(), events_path(), fitted_path_sample_at_adds_offset_to_position(), fitted_path_sample_at_returns_none_for_zero_length_path(), fitted_path_sample_at_wraps_percent_for_open_path(), FittedPath, line_path(), LyonPathEventIterator (+32 more)

### Community 51 - "item_tree.rs"
Cohesion: 0.10
Nodes (31): assert_point_approx_eq(), create_children_nodes(), create_empty_subtree(), create_item_subtree_item(), create_nested_subtrees(), create_one_node_component(), create_subsubtree_items(), create_subsubtree_items_dynamic_elements() (+23 more)

### Community 52 - "flickable.rs"
Cohesion: 0.08
Nodes (21): abs(), CaptureEvents, MouseOrTouchScreen, MouseWheel, DECELERATION, DISTANCE_THRESHOLD, DURATION_THRESHOLD, ensure_in_bound() (+13 more)

### Community 53 - "Image"
Cohesion: 0.07
Nodes (23): ImageCache, ImageWeightInBytes, replace_cached_image(), CachedPath, ImageCacheKey, EmbeddedData, Invalid, Path (+15 more)

### Community 54 - "PhysicalLength"
Cohesion: 0.15
Nodes (9): Glyph, GlyphRenderer, Layout, RectangleBorder, TextParagraph, visible_band(), visible_x_range(), ElisionInfo (+1 more)

### Community 55 - "SelectionSpan"
Cohesion: 0.14
Nodes (11): Hide, PhysicalRect, Layout, run_coverage(), RunCoverage, Full, Partial, Unselected (+3 more)

### Community 56 - "Slot"
Cohesion: 0.11
Nodes (13): Active, ACTIVE, Chromecast, Session, CAST_GEN, Ctx, PENDING, Target (+5 more)

### Community 57 - "i-slint-core/layout.rs"
Cohesion: 0.08
Nodes (38): collect_from_organized_data(), Errors, FlexboxMeasureFnC, GridLayoutData, GridLayoutInputData, GridLayoutOrganizedData, grows_by_stretch_only_under_alignment_stretch(), info() (+30 more)

### Community 58 - "DataTransfer"
Cohesion: 0.06
Nodes (33): cleared_transfer_compares_equal_to_default(), DataTransfer, DataTransferError, TypeNotFound, DataTransferInner, every_payload_counts_as_native_data(), CppUserData, DataTransferOpaque (+25 more)

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
Cohesion: 0.06
Nodes (26): Value, Item, Property, AnimatedBindingKind, Animation, NotAnimated, Transition, BuiltinGlobal (+18 more)

### Community 64 - "drag_n_drop.rs"
Cohesion: 0.09
Nodes (14): ALL, AllowedDragActions, clamp_action_to_allowed(), compute_proposed_action(), compute_proposed_action_modifier_table(), COPY_AND_MOVE, COPY_ONLY, cursor_for_action() (+6 more)

### Community 65 - "input.rs"
Cohesion: 0.08
Nodes (28): Keys, keys_from_parts(), keys_from_parts_inner(), KeysInner, KeysParseError, KeysParseErrorInner, IncompatibleModifiers, MultipleGraphemeClusters (+20 more)

### Community 66 - "VisitChildrenResult"
Cohesion: 0.13
Nodes (25): State, Deceleration, Done, SpringDamper, ItemTreeNode, DynamicTree, ItemTreeRc, ItemVisitorRefMut (+17 more)

### Community 67 - "ModelNotify"
Cohesion: 0.09
Nodes (12): BrokenModel, DependencyListHead, ModelChangeListenerBox<T>, ModelChangeListenerContainer<T>, ModelNotify, ModelNotifyInner, ModelPeer, TrackedRows (+4 more)

### Community 68 - ".first_child"
Cohesion: 0.23
Nodes (11): ItemTreeNodeArray<'a>, test_tree_traversal_children_nodes_backward_focus(), test_tree_traversal_children_nodes_forward_focus(), test_tree_traversal_children_nodes_structure(), test_tree_traversal_item_subtree_item_backward_focus(), test_tree_traversal_item_subtree_item_forward_focus(), test_tree_traversal_item_subtree_item_structure(), test_tree_traversal_nested_subtrees_backward_focus() (+3 more)

### Community 69 - "Model"
Cohesion: 0.06
Nodes (13): ErrorImpl, OutOfBounds, Unsupported, Model, ModelError, ModelIterator, ModelIterator<'a, T>, ModelRc (+5 more)

### Community 70 - "android_media.rs"
Cohesion: 0.13
Nodes (16): as_class(), ASKED_PERMISSION, CB, CB_CLASS, CHANNEL, DEX, Handler, HANDLER (+8 more)

### Community 71 - "items/path.rs"
Cohesion: 0.05
Nodes (21): BoxShadowCache, CacheEntry, MAX_CACHED_SHADOWS, ItemRendererFeatures, .SUPPORTS_TRANSFORMATIONS, ComponentContainer, FittedPathBox, FittedPathInner (+13 more)

### Community 72 - "TestItemTree"
Cohesion: 0.07
Nodes (5): core::ops::Range<usize>, IndexRange, ItemTreeWeak, TestItemTree, TransformTestItemTree

### Community 73 - "ksni.rs"
Cohesion: 0.11
Nodes (16): build_ksni_menu(), dispatch_loop(), entry_to_node(), Event, Activate, Menu, image_to_argb_icon(), KsniTray (+8 more)

### Community 74 - "graphics/image.rs"
Cohesion: 0.11
Nodes (15): load_as_html_image(), load_image_from_data_uri(), LoadImageError, slint_image_load_from_data(), test_image_invalid_svg(), test_image_load_from_data_png(), test_image_load_from_data_svg(), test_image_load_from_data_svgz() (+7 more)

### Community 75 - ".get"
Cohesion: 0.13
Nodes (3): const_sentinel(), Property<T>, PropertyHandle

### Community 76 - "translations.rs"
Cohesion: 0.09
Nodes (30): c_char, DisplayOrInt, Display, Int, format(), FormatArgs, FormatResult, gettext_bindtextdomain() (+22 more)

### Community 78 - "TextCursorAffinity"
Cohesion: 0.18
Nodes (7): CachedTextInputAccessibilityState, compose_text_selection(), position_for_byte_offset(), selection_offsets(), to_actual_offset(), TextInputLayout<'a>, TextInputParagraph

### Community 79 - ".show_detail"
Cohesion: 0.08
Nodes (30): card_for(), CardData, cards_model(), episode_row(), episode_tag(), fav_group(), media_info(), Slot (+22 more)

### Community 80 - "appkit.rs"
Cohesion: 0.06
Nodes (12): ItemWeak, activate_entry(), AppearanceObserver, AppearanceObserverIvars, build_menu(), entry_to_nsmenuitem(), image_to_nsimage(), MAX_DEPTH (+4 more)

### Community 81 - "brush.rs"
Cohesion: 0.11
Nodes (22): GradientStop, line_for_angle(), LinearGradientBrush, ResolvedConicGradient, ResolvedLinearGradient, ResolvedRadialGradient, sanitize_color_stops(), test_conic_gradient_all_stops_above_one() (+14 more)

### Community 82 - "android.rs"
Cohesion: 0.08
Nodes (26): AndroidCtx, bring_to_front(), CHANNEL, CTX, CUSTOM_RESIZE, drag_resize(), drag_window(), export_logs() (+18 more)

### Community 83 - "JNIEnv"
Cohesion: 0.29
Nodes (11): create_session(), enter_pip(), in_system_pip(), load_callback_class(), media_stop(), media_update(), notification_manager(), pip_params() (+3 more)

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
Cohesion: 0.16
Nodes (4): load_from_data(), load_from_path(), ParsedSVG, svg_options()

### Community 89 - "i-slint-core/string.rs"
Cohesion: 0.10
Nodes (26): format(), shared_string_from_number(), shared_string_from_number_fixed(), shared_string_from_number_precision(), shared_string_from_number_unlocalized(), shared_string_replace_all(), slint_shared_string_clone(), slint_shared_string_drop() (+18 more)

### Community 90 - "properties_animations.rs"
Cohesion: 0.16
Nodes (27): AnimationDetail, Component, DELAY, DURATION, get_prop_value(), properties_test_animation_direction_triggered_by_set(), properties_test_animation_negative_delay_triggered_by_set(), properties_test_animation_triggered_by_binding() (+19 more)

### Community 91 - "Brush"
Cohesion: 0.22
Nodes (15): Brush, ConicGradient, LinearGradient, RadialGradient, SolidColor, resolve_brush(), ResolvedBrush, ConicGradient (+7 more)

### Community 92 - "pt"
Cohesion: 0.14
Nodes (29): classify(), Ev, Exit, Moved, PinchCancelled, PinchEnded, PinchMoved, PinchStarted (+21 more)

### Community 93 - "system_tray.rs"
Cohesion: 0.08
Nodes (9): MenuDirtyHandler, MenuState, slint_system_tray_icon_data_free(), slint_system_tray_icon_data_init(), slint_system_tray_icon_set_menu(), SystemTrayIcon, SystemTrayIconData, SystemTrayIconDataBox (+1 more)

### Community 94 - "constant_deceleration.rs"
Cohesion: 0.16
Nodes (14): constant_deceleration_decreasing_limit_not_reached(), constant_deceleration_decreasing_limit_reached(), constant_deceleration_increasing_limit_not_reached(), constant_deceleration_increasing_limit_reached(), constant_deceleration_start_eq_limit(), ConstantDeceleration, ConstantDecelerationParameters, Direction (+6 more)

### Community 96 - "LogicalLength"
Cohesion: 0.17
Nodes (6): LogicalLength, ListViewProperties, RepeaterLayoutState, slint_repeater_ensure_updated_listview(), TypedListViewProps, update_visible_instances()

### Community 97 - "downloads.rs"
Cohesion: 0.09
Nodes (18): Ctx, CANCEL, Ctx, dir(), Entry, expand(), fetch_one(), key() (+10 more)

### Community 98 - "linebreaker.rs"
Cohesion: 0.13
Nodes (26): char_wrap_sentences(), test_basic_line_break(), test_basic_line_break_anywhere_fallback(), test_basic_line_break_anywhere_fallback_max_lines(), test_basic_line_break_anywhere_fallback_multi_line(), test_basic_line_break_anywhere_fallback_multi_line_char_wrap(), test_basic_line_break_anywhere_fallback_multi_line_v2(), test_basic_line_break_char_wrap() (+18 more)

### Community 99 - "Api"
Cohesion: 0.24
Nodes (8): Api, GetProc, gl_trampoline(), GlInitParams, MpvEvent, on_update(), RenderParam, set_java_vm()

### Community 100 - "vec"
Cohesion: 0.10
Nodes (9): FutureRunner, FutureRunner<T>, FutureRunnerInner, FutureState, Finished, Running, JoinHandle, JoinHandle<T> (+1 more)

### Community 101 - "Rgba8Pixel"
Cohesion: 0.16
Nodes (13): dynamic_image_to_shared_image_buffer(), image_to_rgba8_with_target_size(), Rgb8Pixel, Rgba8Pixel, SharedImageBuffer, RGB8, RGBA8, RGBA8Premultiplied (+5 more)

### Community 102 - "GraphicsAPI"
Cohesion: 0.08
Nodes (15): F, GraphicsAPI, NativeOpenGL, WebGL, WGPU29, WGPU30, RenderingNotifier, RenderingState (+7 more)

### Community 103 - "Repeater<C>"
Cohesion: 0.14
Nodes (7): Conditional, ItemTreeRc, Repeater<C>, RepeaterInner, RepeaterInner<C>, RepeaterTracker, RustRepeaterOps

### Community 104 - "callbacks.rs"
Cohesion: 0.23
Nodes (7): Callback<Arg, Ret>, callback_simple_test(), CallbackOpaque, slint_callback_call(), slint_callback_drop(), slint_callback_init(), slint_callback_set_handler()

### Community 105 - "DirtyRegion"
Cohesion: 0.16
Nodes (5): Box2D, DirtyRegion, .MAX_COUNT, PartialRenderer, PartialRenderingState

### Community 106 - "RepeaterInstanceOpsVTable"
Cohesion: 0.09
Nodes (11): ModelChangeListenerContainer, MAX_EAGER_INSTANCE_COUNT, RepeatedInstanceState, Clean, Dirty, Repeater, RepeaterInstanceOps, RepeaterInstanceOpsVTable (+3 more)

### Community 107 - ".compute_dirty_regions"
Cohesion: 0.13
Nodes (9): ItemTransform, CachedItemBoundingBoxAndTransform, ClipItem, ItemWithTransform, RegularItem, clipped_screen_rect(), dirty_region_ignores_empty_rects(), dirty_region_no_intersection() (+1 more)

### Community 108 - "T"
Cohesion: 0.14
Nodes (9): Binding, AnimatedBindingCallable, AnimatedBindingCallable<T, A>, AnimatedValueBinding, AnimatedValueBinding<T>, LogicalLength, Property<T>, PropertyValueAnimationData (+1 more)

### Community 109 - "ChangeTracker"
Cohesion: 0.14
Nodes (5): change_multiple_dependencies(), change_tracker(), ChangeTracker, ChangeTrackerInner, delete_from_eval_fn()

### Community 110 - "LengthType"
Cohesion: 0.13
Nodes (7): euclid::Point2D<T, U>, euclid::Rect<T, U>, euclid::Size2D<T, U>, euclid::Vector2D<T, U>, PointLengths, RectLengths, SizeLengths

### Community 111 - "textlayout.rs"
Cohesion: 0.10
Nodes (6): DEFAULT_FONT_SIZE, test_content_widths(), test_content_widths_max_lines(), test_content_widths_min_never_exceeds_max(), TextLayout, TextParagraphLayout

### Community 112 - "SlintContext"
Cohesion: 0.06
Nodes (9): set_window_event_hook(), set_window_shown_hook(), SlintContext, SlintContextInner, SlintContextWeak, WindowEventHook, with_global_context(), detect_operating_system() (+1 more)

### Community 113 - "ConicGradientBrush"
Cohesion: 0.15
Nodes (9): center_or_bbox(), ConicGradientBrush, .HEADER, nan_eq(), RadialGradientBrush, .HEADER, slint_brush_compare_equal(), slint_conic_gradient_apply_rotation() (+1 more)

### Community 114 - ".fmt"
Cohesion: 0.23
Nodes (3): DisplayOrInt<T>, FormatResult<'_, T>, WithPlural<'_, T>

### Community 115 - "MenuVTable"
Cohesion: 0.32
Nodes (4): MenuVTable, slint_windowrc_setup_menu_bar_shortcuts(), slint_windowrc_setup_native_menu_bar(), slint_windowrc_show_native_popup_menu()

### Community 116 - "Slice"
Cohesion: 0.20
Nodes (26): box_layout_info(), box_layout_info_ortho(), BoxLayoutData, BoxLayoutOrthoData, flexbox_layout_info_cross_axis(), flexbox_layout_info_cross_axis_with_measure(), flexbox_layout_info_main_axis(), flexbox_layout_unwrapped_main() (+18 more)

### Community 117 - "textlayout/shaping.rs"
Cohesion: 0.13
Nodes (11): ShapeBoundaries, ShapeBoundaries<'a>, ShapeBuffer<Length>, test_byte_offsets_are_absolute(), test_letter_spacing(), test_shape_boundaries_empty(), test_shape_boundaries_script_change(), test_shape_boundaries_simple() (+3 more)

### Community 118 - "spring.rs"
Cohesion: 0.10
Nodes (16): regime_decays_to_rest_over_time(), regime_matches_initial_conditions(), SpringDurationBounceParameters, SpringParameters, SpringPhysicalParameters, SpringRegime, Critical, .CRITICAL_ZETA_EPSILON (+8 more)

### Community 119 - ".update_check_inner"
Cohesion: 0.31
Nodes (3): cleanup(), Ctx, exe_path()

### Community 121 - "gdb_pretty_printers.py"
Cohesion: 0.10
Nodes (5): SharedVectorProvider, SharedVectorSubPrinter, SliceProvider, SliceSubPrinter, SlintCorePrettyPrinter

### Community 122 - "timers.rs"
Cohesion: 0.06
Nodes (45): ActiveTimer, _BUG3019, _BUG6141_SET_INTERVAL_FROM_CALLBACK, CallbackVariant, Empty, MultiFire, SingleShot, current_timers() (+37 more)

### Community 123 - "wgpu_29.rs"
Cohesion: 0.18
Nodes (11): any_wgpu29_adapters_with_gpu(), async_init_instance_adapter_device_queue_surface(), default_backends_to_avoid(), default_device_descriptor(), device_descriptor_from_settings(), init_instance_adapter_device_queue_surface(), init_instance_adapter_device_queue_surface_then(), mask_backends() (+3 more)

### Community 124 - "TextCursorDirection"
Cohesion: 0.08
Nodes (23): TextShortcut, DeleteBackward, DeleteForward, DeleteToStartOfLine, DeleteWordBackward, DeleteWordForward, Move, TextCursorDirection (+15 more)

### Community 125 - "AccessibilityAction"
Cohesion: 0.20
Nodes (8): AccessibilityAction, Decrement, Default, Expand, Increment, ReplaceSelectedText, SetSelectionOffsets, SetValue

### Community 126 - "LogicalPosition"
Cohesion: 0.14
Nodes (8): LogicalPosition, PhysicalPosition, WindowPosition, Logical, Physical, physical_position_from_api(), physical_position_to_api(), WithUserData<extern "C" fn(user_data: *mut c_void, pos: &mut LogicalPosition)>

### Community 127 - ".evaluate"
Cohesion: 0.15
Nodes (9): PropertyTracker<NEEDS_SET_DIRTY, ()>, PropertyTracker<NEEDS_SET_DIRTY, DirtyHandler>, PropertyTracker<true, DirtyHandler>, test_nested_property_tracker_dirty(), test_nested_property_tracker_evaluate_if_dirty(), test_nested_property_trackers(), test_property_dirty_handler(), test_property_listener_scope() (+1 more)

### Community 129 - "wgpu_30.rs"
Cohesion: 0.20
Nodes (11): any_wgpu30_adapters_with_gpu(), async_init_instance_adapter_device_queue_surface(), default_backends_to_avoid(), default_device_descriptor(), device_descriptor_from_settings(), init_instance_adapter_device_queue_surface(), init_instance_adapter_device_queue_surface_then(), mask_backends() (+3 more)

### Community 130 - "graphics.rs"
Cohesion: 0.13
Nodes (7): adjust_rect_and_border_for_inner_drawing(), create_graphics_api_wgpu_29(), create_graphics_api_wgpu_30(), physical_size_from_api(), Point, Rect, Transform

### Community 131 - "fragments.rs"
Cohesion: 0.15
Nodes (14): fragment_iterator_break_anywhere(), fragment_iterator_forced_break(), fragment_iterator_forced_break_multi(), fragment_iterator_leading_nbsp(), fragment_iterator_nbsp(), fragment_iterator_simple(), fragment_iterator_simple_v2(), TextFragment (+6 more)

### Community 133 - "SharedPixelBuffer<Pixel>"
Cohesion: 0.20
Nodes (3): clone_from_slice_rejects_a_long_slice(), clone_from_slice_rejects_a_short_slice(), SharedPixelBuffer<Pixel>

### Community 134 - "RendererSealed"
Cohesion: 0.13
Nodes (8): DrawOutcome, Occluded, Skipped, Success, Timeout, Renderer, RendererSealed, T

### Community 135 - ".window_item"
Cohesion: 0.14
Nodes (4): LayoutConstraints, slint_windowrc_is_maximized(), slint_windowrc_is_minimized(), WindowProperties

### Community 136 - "ModelTracker"
Cohesion: 0.07
Nodes (5): BrokenModel<T>, FilterModel<M, F>, MapModel<M, F>, ModelChecker, ModelTracker

### Community 137 - "i-slint-core/accessibility.rs"
Cohesion: 0.29
Nodes (8): accessible_descendents(), find_exposed_text_input(), find_text_input(), find_text_input_with_rc(), nearest_accessible_item(), ParentItemTraversalMode, FindAllParents, StopAtPopups

### Community 138 - "ItemCache"
Cohesion: 0.33
Nodes (3): CachedGraphicsData, CachedGraphicsData<T>, ItemCache

### Community 139 - "Inner"
Cohesion: 0.15
Nodes (8): create_hicon(), ensure_class_registered(), Inner, notify_icon_data(), PlatformTray, show_popup_menu(), taskbar_created_message(), wnd_proc()

### Community 140 - "OnceCell"
Cohesion: 0.15
Nodes (6): FakeThreadStorage, FakeThreadStorage<T>, FakeThreadStorage<T, F>, SLINT__thread_local_inner!, thread_local_!, OnceCell<T>

### Community 141 - "AccessibleStringProperty"
Cohesion: 0.10
Nodes (20): AccessibleStringProperty, Checkable, Checked, DelegateFocus, Description, Enabled, Expandable, Expanded (+12 more)

### Community 142 - "InternalKeyEvent"
Cohesion: 0.03
Nodes (21): Callback, InternalKeyEvent, KeyEventResult, EventAccepted, EventIgnored, FocusReasonArg, FocusScope, KeyBinding (+13 more)

### Community 143 - "TouchState"
Cohesion: 0.12
Nodes (11): ClickState, GestureRecognitionState, Idle, Pinching, TwoFingersDown, TextCursorBlinker, TouchEventBuffer, TouchState (+3 more)

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
Nodes (11): &'a ImageInner, Image, NineSlice, load_image_from_embedded_data(), slint_image_compare_equal(), slint_image_load_from_embedded_data(), slint_image_load_from_path(), slint_image_path() (+3 more)

### Community 148 - "Instant"
Cohesion: 0.14
Nodes (5): animation_tick(), AnimationDriver, current_tick(), Instant, update_animations()

### Community 149 - "MouseInputState"
Cohesion: 0.21
Nodes (9): handle_mouse_grab(), DelayForwarding, MouseInputResult, MouseInputState, offer_native_drag(), process_delayed_event(), process_mouse_input(), send_exit_events() (+1 more)

### Community 150 - "LogicalRect"
Cohesion: 0.05
Nodes (15): BorderRectLayout, clip_content_box(), item_children_bounding_rect(), item_children_bounding_rect_transformed(), item_with_children_bounding_rect_transformed(), ItemRenderer, render_component_items(), render_item_children() (+7 more)

### Community 151 - "DependencyNode"
Cohesion: 0.23
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
Cohesion: 0.09
Nodes (17): cached_paragraphs(), CachedParagraphs, CachedParagraphsGuard, ENTRY_LIMIT, ElisionCut, Layout, layout(), LayoutOptions (+9 more)

### Community 159 - "platform/windows.rs"
Cohesion: 0.13
Nodes (7): RichNote, APP_ID, set_material(), SUPPORTS_MATERIAL, system_notify(), system_notify_rich(), toast_xml()

### Community 160 - "&rustybuzz::Face<'_>"
Cohesion: 0.13
Nodes (4): AbstractFont, &rustybuzz::Face<'_>, T, TextShaper

### Community 161 - "constant_deceleration_spring_damper.rs"
Cohesion: 0.23
Nodes (8): calculate_parameters(), constant_deceleration_decreasing_limit_not_reached(), constant_deceleration_increasing_limit_not_reached(), constant_deceleration_spring_damper_decreasing_limit_reached(), constant_deceleration_spring_damper_increasing_limit_reached(), constant_deceleration_start_eq_limit(), ConstantDecelerationSpringDamper, ConstantDecelerationSpringDamperParameters

### Community 162 - "debug_log.rs"
Cohesion: 0.18
Nodes (10): default_log_message(), log_message(), LogMessage, LogMessage<'a>, LogMessageHandler, LogMessageLocation, LogMessageSource, Runtime (+2 more)

### Community 165 - "platform.rs"
Cohesion: 0.05
Nodes (30): logical_position_to_api(), _ANIM_TICK_UPDATED_ON_PLATFORM_SET, EVENTLOOP_PROXY, EventLoopProxy, INITIAL_INSTANT, InternalEvent, Key, Mouse (+22 more)

### Community 166 - "api.rs"
Cohesion: 0.12
Nodes (6): Global, Weak, WindowHandle, WindowHandleInner, HandleByAdapter, HandleByRcRWH

### Community 168 - "data_ringbuffer.rs"
Cohesion: 0.36
Nodes (7): test_empty_buffer(), test_push_beyond_capacity(), test_push_beyond_capacity_wrap_back(), test_push_single_element(), test_push_two_elements(), test_push_until_full(), VelocityRingBuffer

### Community 170 - "InputMethodProperties"
Cohesion: 0.20
Nodes (6): InputMethodProperties, InputMethodRequest, Disable, Enable, Update, slint_windowrc_set_text_input_focused()

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
Cohesion: 0.14
Nodes (7): PlatformTray, Error, EventLoopError, PlatformError, Rgba8, Unsupported, Params

### Community 178 - "LogicalRect"
Cohesion: 0.13
Nodes (8): LogicalRect, fixed_placement(), place_popup(), Placement, Fixed, r(), test_place_popup_fixed_clipped(), test_place_popup_fixed_unclipped()

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
Cohesion: 0.14
Nodes (7): logical_point_from_api(), logical_size_from_api(), logical_size_to_api(), LogicalEdges, LogicalSize, PhysicalBorderRadius, PhysicalEdges

### Community 186 - "Slint Software License"
Cohesion: 0.13
Nodes (14): 1. Grant of Rights, 2. License Conditions, 3. Limitations, 4. Audit Rights, 5. Termination, 6. Assignment, 7. Severability, 8. Governing Law (+6 more)

### Community 188 - "RenderText"
Cohesion: 0.10
Nodes (4): PlainOrStyledText, Plain, Styled, (SharedString, Brush)

### Community 189 - "InterpolatedPropertyValue"
Cohesion: 0.13
Nodes (5): f32, i32, i64, InterpolatedPropertyValue, u8

### Community 191 - "ItemRef"
Cohesion: 0.21
Nodes (5): ItemTreeRef, ItemTreeRefPin, slint_unregister_item_tree(), unregister_item_tree(), ItemRef

### Community 193 - ".get"
Cohesion: 0.14
Nodes (3): SystemInfo, Task, VirtualFolder

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
Cohesion: 0.14
Nodes (17): f32, fit(), fit9slice(), FitResult, RemEuclid, scalable_render_size(), slint_image_from_embedded_textures(), StaticTexture (+9 more)

### Community 198 - "RequestedGraphicsAPI"
Cohesion: 0.19
Nodes (10): RequestedGraphicsAPI, Direct3D, Metal, OpenGL, Vulkan, WGPU29, WGPU30, RequestedOpenGLVersion (+2 more)

### Community 200 - "SingleLinkedListPinHead<T>"
Cohesion: 0.32
Nodes (3): big_list(), SingleLinkedListPinHead<T>, test_list()

### Community 201 - "AnimatedBindingState"
Cohesion: 0.50
Nodes (4): AnimatedBindingState, Animating, NotAnimating, ShouldStart

### Community 203 - "winit_ops.rs"
Cohesion: 0.15
Nodes (9): bring_to_front(), CUSTOM_RESIZE, drag_resize(), drag_window(), monitor_area(), set_cursor_visible(), set_decorations(), set_window_rect() (+1 more)

### Community 207 - "StandardShortcut"
Cohesion: 0.17
Nodes (11): StandardShortcut, Copy, Cut, Find, Paste, Print, Redo, Refresh (+3 more)

### Community 208 - "LayoutData"
Cohesion: 0.18
Nodes (5): Adjust, adjust_items(), Grow, LayoutData, Shrink

### Community 211 - "src/color.rs"
Cohesion: 0.50
Nodes (3): parse_hex(), rgb_hsv_roundtrip_matches_known_colours(), rgb_to_hsv()

### Community 212 - "CloseRequestResponse"
Cohesion: 0.20
Nodes (6): CloseRequestResponse, HideWindow, KeepWindowShown, slint_windowrc_on_close_requested(), WithUserData<extern "C" fn(user_data: *mut c_void) -> CloseRequestResponse>, WithUserData<extern "C" fn(user_data: *mut c_void, is_open: bool)>

### Community 213 - "FontContext"
Cohesion: 0.18
Nodes (6): char_size(), FontContext, line_height_ratio(), test_font_context(), test_text_line_height_matches_shaped_single_line(), text_line_height()

### Community 214 - "component_factory.rs"
Cohesion: 0.31
Nodes (3): ComponentFactory, ComponentFactoryInner, FactoryContext

### Community 215 - "EventLoopError"
Cohesion: 0.16
Nodes (7): EventLoopError, EventLoopTerminated, NoEventLoopProvider, invoke_from_event_loop(), quit_event_loop(), StrongHandle, Weak<T>

### Community 216 - "User"
Cohesion: 0.20
Nodes (3): AuthResult, Policy, User

### Community 219 - "SharedVector"
Cohesion: 0.10
Nodes (9): FlexboxLayoutCacheGenerator, FlexboxLayoutCacheGenerator<'a>, GridLayoutCacheGenerator, LayoutCacheGenerator, MenuFromItemTree, ShadowTreeNode, slint_menus_create_wrapper(), SharedVector (+1 more)

### Community 221 - "CrossAxisSizing"
Cohesion: 0.50
Nodes (4): CrossAxisSizing, FromMeasure, Minimum, Preferred

### Community 226 - "benches/string.rs"
Cohesion: 0.31
Nodes (5): REFERENCE_NUMBER, RESULT_NUMBER, string_replace(), string_replace_own_character(), string_replacen()

### Community 227 - "android"
Cohesion: 0.09
Nodes (22): secrets, uses, with, notes, jobs, android, publish, name (+14 more)

### Community 229 - "TextLineBreaker"
Cohesion: 0.36
Nodes (4): TextLine, TextLine<Length>, TextLineBreaker, TextLineBreaker<'_, Font>

### Community 231 - "pathbuf"
Cohesion: 0.38
Nodes (5): Asset, find_dll(), install(), Release, RELEASES

### Community 233 - "WindowAdapter"
Cohesion: 0.07
Nodes (17): PropertyDirtyHandler, PopupWindow, PopupWindowLocation, ChildWindow, TopLevel, PopupWindowPropertiesTracker, slint_windowrc_create_child_window_adapter(), slint_windowrc_position() (+9 more)

### Community 235 - "windows"
Cohesion: 0.25
Nodes (8): run, windows, tag, version, shell, defaults, outputs, runs-on

### Community 236 - "bitmapfont.rs"
Cohesion: 0.43
Nodes (4): BitmapFont, BitmapGlyph, BitmapGlyphs, CharacterMapEntry

### Community 239 - "i-slint-core/lib.rs"
Cohesion: 0.06
Nodes (14): ComponentHandle, PlatformError, NoEventLoopProvider, NoPlatform, Other, OtherError, SetPlatformError, Unsupported (+6 more)

### Community 241 - "Cmd"
Cohesion: 0.21
Nodes (10): Cast, Cmd, Pause, Play, Seek, Stop, decode(), frame() (+2 more)

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
- **42 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SharedString` connect `SharedString` to `Property`, `graphics.rs`, `app/src/lib.rs`, `.window_item`, `i-slint-core/accessibility.rs`, `AccessibleStringProperty`, `InternalKeyEvent`, `TextInput`, `Image`, `LogicalRect`, `window.rs`, `KeyEventType`, `WindowInner`, `platform.rs`, `api.rs`, `T`, `InputMethodProperties`, `date_time.rs`, `format!`, `graphics/path.rs`, `item_tree.rs`, `Image`, `DataTransfer`, `RenderText`, `input.rs`, `KeyboardModifiers`, `PhysicalPx`, `items/path.rs`, `TestItemTree`, `translations.rs`, `ParsedSVG`, `i-slint-core/string.rs`, `SharedVector`, `system_tray.rs`, `benches/string.rs`, `i-slint-core/lib.rs`, `SlintContext`, `MenuItem`, `AccessibilityAction`?**
  _High betweenness centrality (0.100) - this node is a cross-community bridge._
- **Are the 46 inferred relationships involving `ItemRc` (e.g. with `.font_request()` and `item_children_bounding_rect()`) actually correct?**
  _`ItemRc` has 46 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Session`, `Chromecast`, `Session` to the rest of the system?**
  _459 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `Property` be split into smaller, more focused modules?**
  _Cohesion score 0.028879720723579815 - nodes in this community are weakly interconnected._
- **Why does `Property` connect `Item` to `Property`, `InternalKeyEvent`, `TouchState`, `MouseEvent`, `TextInput`, `Instant`, `window.rs`, `erased_bindings.rs`, `Coord`, `constant_deceleration_spring_damper.rs`, `WindowInner`, `T`, `properties/ffi.rs`, `item_tree.rs`, `flickable.rs`, `.evaluate`, `items/image.rs`, `Community 62`, `drag_n_drop.rs`, `input.rs`, `ModelNotify`, `items/path.rs`, `.get`, `two_way_binding.rs`, `IntSize`, `properties_animations.rs`, `SharedVector`, `system_tray.rs`, `constant_deceleration.rs`, `LogicalLength`, `Repeater<C>`, `WindowAdapter`, `RepeaterInstanceOpsVTable`, `ChangeTracker`, `SlintContext`, `MenuItem`?**
  _High betweenness centrality (0.069) - this node is a cross-community bridge._
- **Are the 4 inferred relationships involving `SharedString` (e.g. with `.set_xdg_app_id()` and `.xdg_app_id()`) actually correct?**
  _`SharedString` has 4 INFERRED edges - model-reasoned connections that need verification._
- **Should `player/src/lib.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.11904761904761904 - nodes in this community are weakly interconnected._