# Graph Report - Foyer  (2026-11-18)

## Corpus Check
- 152 files · ~311,869 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 10 file(s) not represented in the graph (top: (none) 4, .xml 2, .rc 1)

## Summary
- 6879 nodes · 17739 edges · 265 communities (223 shown, 42 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 956 edges (avg confidence: 0.56)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `a822c52d`
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
- TimerList
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
- ScaleFactor
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
- RenderingResult
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
- PlatformTray
- vec
- Rgba8Pixel
- GraphicsAPI
- Repeater<C>
- callbacks.rs
- DirtyRegion
- RepeaterInstanceOpsVTable
- RendererSealed
- T
- ChangeTracker
- LengthType
- textlayout.rs
- SlintContext
- ConicGradientBrush
- RenderText
- MenuVTable
- Slice
- textlayout/shaping.rs
- spring.rs
- play_one
- text.rs
- gdb_pretty_printers.py
- timers.rs
- wgpu_29.rs
- TextCursorDirection
- i-slint-core/accessibility.rs
- Cast
- .evaluate
- MenuItem
- wgpu_30.rs
- graphics.rs
- fragments.rs
- WindowPinnedFields
- .visual_representation
- ImageInner
- WindowProperties
- ModelTracker
- data_transfer.rs
- TextCursorBlinker
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
- PhysicalEdges
- solve_flexbox_layout_with_measure
- super
- PlainOrStyledText
- SystemTrayIcon
- platform/windows.rs
- &rustybuzz::Face<'_>
- constant_deceleration_spring_damper.rs
- debug_log.rs
- Ctx
- NoVerify
- platform.rs
- WindowHandle
- IndexRange
- data_ringbuffer.rs
- PartialRenderer<'_, T>
- lengths.rs
- Changelog
- android_notify.rs
- play_inner
- date_time.rs
- animations.rs
- .plex_folders
- Error
- TexturePixelFormat
- Length
- CheckedAdd
- Surface
- .evaluate
- rendering_metrics_collector.rs
- .apply_dirty_region
- PartialRendererCache
- Slint Software License
- media.rs
- StyledTextItem
- InterpolatedPropertyValue
- TimerListRc
- ItemRef
- MediaCb
- EasingCurve
- KeyboardModifiers
- mod.rs
- steps
- PhysicalPx
- RequestedGraphicsAPI
- Direction
- SingleLinkedListPinHead<T>
- PropertyValueAnimationData
- Model
- winit_ops.rs
- SurfaceTarget
- WGPUSettings
- WGPUSettings
- StandardShortcut
- LayoutData
- Export.java
- preview.rs
- src/color.rs
- CloseRequestResponse
- popup.rs
- component_factory.rs
- i-slint-core/lib.rs
- VelocityRingBuffer<N>
- T
- menus.rs
- Renderer
- DrawOutcome
- .compute_dirty_regions
- CachedGraphicsData
- Conditional<C>
- Immersive
- benches/string.rs
- android
- RepaintBufferType
- TextLineBreaker
- FixedTestFont
- pathbuf
- DragRequest
- Scalar
- RepeatedItemTree
- windows
- Slice
- WindowEventDispatchResult
- .system_info
- PlatformError
- Slint Royalty-free Desktop, Mobile, and Web Applications License
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

## Communities (265 total, 42 thin omitted)

### Community 0 - "input_items.rs"
Cohesion: 0.05
Nodes (19): FocusReasonArg, FocusScope, KeyBinding, KeyBindingList, MaybeKeyBindingList, ScaleRotateGestureHandler, slint_maybe_key_binding_list_free(), slint_maybe_key_binding_list_init() (+11 more)

### Community 1 - "player/src/lib.rs"
Cohesion: 0.10
Nodes (30): Api, api(), available(), DEFAULT_OPTIONS, dll_candidates(), END_EOF, END_ERROR, engine_dir() (+22 more)

### Community 2 - "Ctx"
Cohesion: 0.10
Nodes (3): cards_model(), Ctx, nice_date()

### Community 3 - "update.rs"
Cohesion: 0.09
Nodes (24): Api, ApiAsset, ASSET, bad_signature(), check(), check_signature(), Checked, Latest (+16 more)

### Community 4 - "Foyer (native Emby client)"
Cohesion: 0.05
Nodes (46): Android and Android TV plan, Android TV (D-pad focus, 10-foot layouts), APK signing key backup, Signed APK self-update, Background notifications (new feature on Android), CI Android job publishing exe and apk, libmpv .so with MediaCodec (ExoPlayer fallback), Material You accent colour (+38 more)

### Community 5 - "app/src/lib.rs"
Cohesion: 0.04
Nodes (43): HideTarget, android_main(), apply_image(), AUDIO_PICK, auto_hw_mode(), cached_image(), ChanData, DEBUG (+35 more)

### Community 6 - "seerr-api/src/lib.rs"
Cohesion: 0.05
Nodes (31): ADMIN, AUTO_APPROVE, CREATE_ISSUES, Download, DownloadEp, downloads_label(), ISSUE_OPEN, ISSUE_RESOLVED (+23 more)

### Community 7 - "Control"
Cohesion: 0.09
Nodes (22): Control, AddSubtitle, AudioDelayBy, CycleAudio, CycleSubtitles, Finish, Quit, ResetAudioDelay (+14 more)

### Community 8 - "emby-api/src/lib.rs"
Cohesion: 0.05
Nodes (31): AuthResult, CLIENT_NAME, CLIENT_VERSION, de_id(), de_opt_id(), detect(), DEVICE_NAME, discover() (+23 more)

### Community 9 - "notes.rs"
Cohesion: 0.07
Nodes (17): Ctx, FOLLOWED, init_tray_events(), INSTANCE_ADDR, load_notes(), MAX_NOTES, NoteRec, notes_path() (+9 more)

### Community 10 - "Seerr"
Cohesion: 0.14
Nodes (7): Person, PublicSettings, Quota, QuotaPart, Result, Seerr, WatchItem

### Community 11 - "Result"
Cohesion: 0.11
Nodes (5): Browse, Chapter, Item, Client, section_key()

### Community 12 - "sharedparley.rs"
Cohesion: 0.07
Nodes (27): HasFont, RenderString, TextCursorAffinity, NextCharacter, PreviousCharacter, ContentWidths, InnerTextLayoutCache, TextLayoutCache (+19 more)

### Community 13 - "Client"
Cohesion: 0.07
Nodes (3): Client, RemoteSubtitle, Result

### Community 14 - "RemoteEvent"
Cohesion: 0.10
Nodes (19): parse(), RemoteEvent, FastForward, Message, Mute, Next, Pause, Play (+11 more)

### Community 15 - "ItemRc"
Cohesion: 0.05
Nodes (13): FocusEvent, FocusIn, FocusOut, FocusEventResult, FocusAccepted, FocusIgnored, InternalKeyEvent, KeyEventResult (+5 more)

### Community 16 - "seerr.rs"
Cohesion: 0.05
Nodes (36): dir(), CardData, SIssue, SReq, DetailCtx, DetailData, EDIT_BITS, ep() (+28 more)

### Community 17 - "gcast.rs"
Cohesion: 0.12
Nodes (13): browse(), decode(), DEFAULT_RECEIVER, Device, discover(), field(), frame(), frames_round_trip() (+5 more)

### Community 18 - "TextInput"
Cohesion: 0.13
Nodes (11): next_paragraph_boundary(), next_word_boundary(), prev_paragraph_boundary(), prev_word_boundary(), safe_byte_offset(), safe_grapheme_boundary_offset(), slint_textinput_select_all(), TextChangeNotify (+3 more)

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
Cohesion: 0.05
Nodes (54): Size, KeyEventType, CommitComposition, KeyPressed, KeyReleased, UpdateComposition, a_host_overrides_the_default_title(), accent_color() (+46 more)

### Community 25 - "Ctx"
Cohesion: 0.06
Nodes (14): apply_accent(), apply_material(), close(), Ctx, fav_group(), goto_episode(), load_posters(), login_error() (+6 more)

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

### Community 32 - "TimerList"
Cohesion: 0.22
Nodes (5): ActiveTimer, SingleShot, current_timers(), single_shot_on(), TimerList

### Community 33 - ".send"
Cohesion: 0.15
Nodes (6): log(), RequestAction, Approve, Decline, Delete, Retry

### Community 34 - "WindowInner"
Cohesion: 0.08
Nodes (9): PopupWindowLocation, ChildWindow, TopLevel, PopupWindowPropertiesTracker, previous_focus_item(), slint_windowrc_set_focus_item(), slint_windowrc_supports_native_menu_bar(), WindowAdapter (+1 more)

### Community 35 - "foyer"
Cohesion: 0.50
Nodes (5): emby-api, foyer, foyer-core, player, seerr-api

### Community 36 - "Quality"
Cohesion: 0.12
Nodes (15): Source, Live, log(), LOG, LogFn, set_log(), auto_steps_down_when_link_is_slow(), decide() (+7 more)

### Community 37 - "Client"
Cohesion: 0.14
Nodes (18): ActivityEntry, log(), arr(), f(), IDENT, lang2(), maps_an_episode(), metas() (+10 more)

### Community 38 - "Color"
Cohesion: 0.07
Nodes (26): Channel, Color, HsvaColor, linear_to_srgb(), OklabColor, OklchColor, quantize(), RgbaColor (+18 more)

### Community 40 - "adapters.rs"
Cohesion: 0.10
Nodes (29): AscendingSortHelper, check_content(), F, FilterModel, FilterModelInner, MapModel, ReverseModel, ReverseModel<M> (+21 more)

### Community 41 - "T"
Cohesion: 0.10
Nodes (19): (), AbsentRowModel, model_all(), model_any(), model_find_index(), ModelIterator<'_, T>, test_any_change_subsumes_row_tracking(), test_any_change_tracking() (+11 more)

### Community 42 - "Android"
Cohesion: 0.40
Nodes (4): Android, Every feature works on both, and lands on `main`, graphify, Rules for keeping it compatible

### Community 44 - "super"
Cohesion: 0.11
Nodes (17): AUTOPLAY_STREAK_LIMIT, BASE, CARD_SCALES, Config, default_accent_mode(), DEFAULT_ACCENT_MODE, default_accent_rgb(), default_card_scale() (+9 more)

### Community 45 - "SharedString"
Cohesion: 0.05
Nodes (14): StandardListViewItem, SharedString, simple_test(), slint_shared_string_append(), slint_shared_string_bytes(), String, test_extend_from_chars(), test_slint_shared_string_append() (+6 more)

### Community 46 - "tests.rs"
Cohesion: 0.18
Nodes (18): paragraph_ranges(), bidi_selection_spans_are_ascending_in_x(), layout_text(), layout_text_with_builder(), layout_text_with_options(), layout_with_max_lines(), paragraphs(), test_cursor_affinity_at_soft_line_break() (+10 more)

### Community 47 - "Window"
Cohesion: 0.07
Nodes (15): logical_physical_pos(), logical_physical_size(), LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize, WindowPosition, Logical (+7 more)

### Community 48 - "properties/ffi.rs"
Cohesion: 0.10
Nodes (42): PropertyAnimation, c_set_animated_binding(), c_set_animated_value(), ChangeTrackerOpaque, drop_fn(), eval_fn(), EvalState, ffi_change_tracker_does_not_leak_dep_nodes() (+34 more)

### Community 49 - "format!"
Cohesion: 0.40
Nodes (3): format!, T, ToSharedString

### Community 50 - "graphics/path.rs"
Cohesion: 0.05
Nodes (44): elements_path(), events_path(), fitted_path_sample_at_adds_offset_to_position(), fitted_path_sample_at_returns_none_for_zero_length_path(), fitted_path_sample_at_wraps_percent_for_open_path(), FittedPath, line_path(), LyonPathEventIterator (+36 more)

### Community 51 - "item_tree.rs"
Cohesion: 0.09
Nodes (39): assert_point_approx_eq(), create_children_nodes(), create_empty_subtree(), create_item_subtree_item(), create_nested_subtrees(), create_one_node_component(), create_subsubtree_items(), create_subtrees_item() (+31 more)

### Community 52 - "flickable.rs"
Cohesion: 0.09
Nodes (18): CaptureEvents, MouseOrTouchScreen, MouseWheel, DECELERATION, DURATION_THRESHOLD, ensure_in_bound(), Flickable, FlickableData (+10 more)

### Community 53 - "items.rs"
Cohesion: 0.03
Nodes (57): BasicBorderRectangle, .cached_rendering_data_offset, BorderRectangle, .cached_rendering_data_offset, .cached_rendering_data_offset, Clip, .cached_rendering_data_offset, .cached_rendering_data_offset (+49 more)

### Community 54 - "PhysicalLength"
Cohesion: 0.10
Nodes (19): Glyph, GlyphRenderer, Layout, RectangleBorder, TextParagraph, visible_band(), visible_x_range(), ElisionInfo (+11 more)

### Community 55 - "ScaleFactor"
Cohesion: 0.12
Nodes (15): FontRequest, ScaleFactor, font_metrics(), FontContext, Brush, content_widths_builder(), create_text_paragraphs(), INLINE_CODE_FONT_SCALE (+7 more)

### Community 56 - "Slot"
Cohesion: 0.07
Nodes (21): Active, ACTIVE, Chromecast, Session, CAST_GEN, Ctx, FALLBACK_QUALITY, IDS (+13 more)

### Community 57 - "i-slint-core/layout.rs"
Cohesion: 0.06
Nodes (41): BoxLayoutOrthoData, collect_from_organized_data(), Constraint, CrossAxisSizing, FromMeasure, Minimum, Preferred, Errors (+33 more)

### Community 58 - "DataTransfer"
Cohesion: 0.08
Nodes (25): DataTransfer, DataTransferError, TypeNotFound, CppUserData, DataTransferOpaque, path_from_units(), path_to_units(), PathValueType (+17 more)

### Community 59 - ".row_data"
Cohesion: 0.11
Nodes (9): BrokenModel, FilterModelInner<M, F>, ModelChecker<Data>, ReverseModelInner<M>, SortModel<M, S>, SortModelInner<M, S>, test_filter_model_broken_model(), test_reversed_broken_model() (+1 more)

### Community 60 - "items/image.rs"
Cohesion: 0.07
Nodes (3): RenderImage, ClippedImage, ImageItem

### Community 61 - "steps"
Cohesion: 0.05
Nodes (44): required, required, runs-on, steps, GH_TOKEN, KEYSTORE_B64, KEYSTORE_PASSWORD, OVERRIDE_URL (+36 more)

### Community 62 - "Community 62"
Cohesion: 0.08
Nodes (27): (), alloc_binding_holder(), BINDING_BORROWED, BINDING_POINTER_MASK, BINDING_POINTER_TO_BINDING, BindingCallable, .IS_TWO_WAY_BINDING, BindingHolder (+19 more)

### Community 63 - "Item"
Cohesion: 0.06
Nodes (27): accessible_descendents(), Value, Item, Property, AnimatedBindingKind, Animation, NotAnimated, Transition (+19 more)

### Community 64 - "drag_n_drop.rs"
Cohesion: 0.10
Nodes (16): Callback, ALL, AllowedDragActions, clamp_action_to_allowed(), compute_proposed_action(), compute_proposed_action_modifier_table(), COPY_AND_MOVE, COPY_ONLY (+8 more)

### Community 65 - "input.rs"
Cohesion: 0.09
Nodes (27): Keys, keys_from_parts(), keys_from_parts_inner(), KeysInner, KeysParseError, KeysParseErrorInner, IncompatibleModifiers, MultipleGraphemeClusters (+19 more)

### Community 66 - "VisitChildrenResult"
Cohesion: 0.14
Nodes (23): State, Deceleration, Done, SpringDamper, ItemTreeRc, ItemVisitorRefMut, ItemVisitorResult, Abort (+15 more)

### Community 67 - "ModelNotify"
Cohesion: 0.08
Nodes (14): TestView, DependencyListHead, ModelChangeListener, ModelChangeListenerBox, ModelChangeListenerBox<T>, ModelChangeListenerContainer, ModelChangeListenerContainer<T>, ModelNotify (+6 more)

### Community 69 - "Model"
Cohesion: 0.08
Nodes (6): ModelRc<T>, SharedVectorModel, SharedVectorModel<T>, T, VecModel, VecModel<T>

### Community 70 - "android_media.rs"
Cohesion: 0.12
Nodes (27): as_class(), ASKED_PERMISSION, CB, CB_CLASS, CHANNEL, create_session(), DEX, enter_pip() (+19 more)

### Community 72 - "TestItemTree"
Cohesion: 0.08
Nodes (6): ItemTreeNode, DynamicTree, ItemTreeNodeArray<'a>, ItemTreeWeak, TestItemTree, TransformTestItemTree

### Community 73 - "ksni.rs"
Cohesion: 0.11
Nodes (16): build_ksni_menu(), dispatch_loop(), entry_to_node(), Event, Activate, Menu, image_to_argb_icon(), KsniTray (+8 more)

### Community 74 - "graphics/image.rs"
Cohesion: 0.11
Nodes (12): clone_from_slice_rejects_a_long_slice(), clone_from_slice_rejects_a_short_slice(), Svg, load_as_html_image(), load_image_from_data_uri(), LoadImageError, SharedPixelBuffer<Pixel>, slint_image_load_from_data() (+4 more)

### Community 75 - ".get"
Cohesion: 0.13
Nodes (3): const_sentinel(), Property<T>, PropertyHandle

### Community 76 - "translations.rs"
Cohesion: 0.07
Nodes (34): c_char, slint_shared_string_from_bytes(), DisplayOrInt, Display, Int, DisplayOrInt<T>, format(), FormatArgs (+26 more)

### Community 77 - "BoxShadowOptions"
Cohesion: 0.16
Nodes (4): BoxShadowCache, BoxShadowOptions, CacheEntry, MAX_CACHED_SHADOWS

### Community 78 - "TextCursorAffinity"
Cohesion: 0.14
Nodes (7): CachedTextInputAccessibilityState, compose_text_selection(), position_for_byte_offset(), selection_offsets(), to_actual_offset(), TextInputLayout<'a>, TextInputParagraph

### Community 79 - ".show_detail"
Cohesion: 0.07
Nodes (36): prepare(), audio_streams(), card_for(), default_audio(), default_sub(), episode_row(), episode_tag(), is_english() (+28 more)

### Community 80 - "appkit.rs"
Cohesion: 0.07
Nodes (6): ItemWeak, activate_entry(), AppearanceObserverIvars, MAX_DEPTH, MenuActionIvars, scheme_for_appearance()

### Community 81 - "brush.rs"
Cohesion: 0.12
Nodes (19): GradientStop, line_for_angle(), LinearGradientBrush, ResolvedBrush, ResolvedConicGradient, ResolvedLinearGradient, ResolvedRadialGradient, sanitize_color_stops() (+11 more)

### Community 82 - "android.rs"
Cohesion: 0.08
Nodes (25): AndroidCtx, bring_to_front(), CHANNEL, CTX, CUSTOM_RESIZE, drag_resize(), drag_window(), export_logs() (+17 more)

### Community 83 - "RenderingResult"
Cohesion: 0.17
Nodes (5): ItemRendererRef, ItemVTable, RenderingResult, ContinueRenderingChildren, ContinueRenderingWithoutChildren

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
Cohesion: 0.11
Nodes (25): format(), shared_string_from_number(), shared_string_from_number_fixed(), shared_string_from_number_precision(), shared_string_from_number_unlocalized(), shared_string_replace_all(), slint_shared_string_clone(), slint_shared_string_drop() (+17 more)

### Community 90 - "properties_animations.rs"
Cohesion: 0.17
Nodes (27): AnimationDetail, Component, DELAY, DURATION, get_prop_value(), properties_test_animation_direction_triggered_by_set(), properties_test_animation_negative_delay_triggered_by_set(), properties_test_animation_triggered_by_binding() (+19 more)

### Community 91 - "Brush"
Cohesion: 0.17
Nodes (19): Brush, ConicGradient, LinearGradient, RadialGradient, SolidColor, resolve_brush(), ConicGradient, LinearGradient (+11 more)

### Community 92 - "pt"
Cohesion: 0.14
Nodes (29): classify(), Ev, Exit, Moved, PinchCancelled, PinchEnded, PinchMoved, PinchStarted (+21 more)

### Community 93 - ".init"
Cohesion: 0.12
Nodes (5): slint_system_tray_icon_data_free(), slint_system_tray_icon_data_init(), SystemTrayIconData, SystemTrayIconDataBox, SystemTrayIconHandle

### Community 94 - "constant_deceleration.rs"
Cohesion: 0.29
Nodes (8): constant_deceleration_decreasing_limit_not_reached(), constant_deceleration_decreasing_limit_reached(), constant_deceleration_increasing_limit_not_reached(), constant_deceleration_increasing_limit_reached(), constant_deceleration_start_eq_limit(), ConstantDeceleration, ConstantDecelerationParameters, test_limit_property()

### Community 95 - "ItemTreeVTable"
Cohesion: 0.14
Nodes (6): create_subsubtree_items_dynamic_elements(), ensure_item_tree_instantiated(), find_sibling_outside_repeater(), ItemTreeVTable, step_into_node(), test_map_to_native_window_popup_dynamic_element()

### Community 96 - "LogicalLength"
Cohesion: 0.11
Nodes (8): abs(), DISTANCE_THRESHOLD, SCROLL_FILTER_DISTANCE_SQUARED, LogicalEdges, LogicalLength, ListViewProperties, TypedListViewProps, update_visible_instances()

### Community 97 - "downloads.rs"
Cohesion: 0.10
Nodes (16): CANCEL, Ctx, Entry, expand(), fetch_one(), key(), LIST, offline_item() (+8 more)

### Community 98 - "linebreaker.rs"
Cohesion: 0.13
Nodes (26): char_wrap_sentences(), test_basic_line_break(), test_basic_line_break_anywhere_fallback(), test_basic_line_break_anywhere_fallback_max_lines(), test_basic_line_break_anywhere_fallback_multi_line(), test_basic_line_break_anywhere_fallback_multi_line_char_wrap(), test_basic_line_break_anywhere_fallback_multi_line_v2(), test_basic_line_break_char_wrap() (+18 more)

### Community 99 - "PlatformTray"
Cohesion: 0.16
Nodes (6): AppearanceObserver, build_menu(), entry_to_nsmenuitem(), image_to_nsimage(), MenuAction, PlatformTray

### Community 100 - "vec"
Cohesion: 0.09
Nodes (9): FutureRunner, FutureRunner<T>, FutureRunnerInner, FutureState, Finished, Running, JoinHandle, JoinHandle<T> (+1 more)

### Community 101 - "Rgba8Pixel"
Cohesion: 0.16
Nodes (13): dynamic_image_to_shared_image_buffer(), image_to_rgba8_with_target_size(), Rgb8Pixel, Rgba8Pixel, SharedImageBuffer, RGB8, RGBA8, RGBA8Premultiplied (+5 more)

### Community 102 - "GraphicsAPI"
Cohesion: 0.05
Nodes (28): EventLoopError, EventLoopTerminated, NoEventLoopProvider, F, Global, GraphicsAPI, NativeOpenGL, WebGL (+20 more)

### Community 103 - "Repeater<C>"
Cohesion: 0.12
Nodes (9): ModelRc, Conditional, ItemTreeRc, Repeater, Repeater<C>, RepeaterInner, RepeaterInner<C>, RepeaterTracker (+1 more)

### Community 104 - "callbacks.rs"
Cohesion: 0.23
Nodes (7): Callback<Arg, Ret>, callback_simple_test(), CallbackOpaque, slint_callback_call(), slint_callback_drop(), slint_callback_init(), slint_callback_set_handler()

### Community 105 - "DirtyRegion"
Cohesion: 0.22
Nodes (6): Box2D, LogicalPx, dirty_region_ignores_empty_rects(), dirty_region_no_intersection(), DirtyRegion, .MAX_COUNT

### Community 106 - "RepeaterInstanceOpsVTable"
Cohesion: 0.11
Nodes (10): MAX_EAGER_INSTANCE_COUNT, RepeatedInstanceState, Clean, Dirty, RepeaterInstanceOps, RepeaterInstanceOpsVTable, RepeaterLayoutState, slint_repeater_ensure_updated() (+2 more)

### Community 107 - "RendererSealed"
Cohesion: 0.14
Nodes (3): Renderer, RendererSealed, T

### Community 108 - "T"
Cohesion: 0.16
Nodes (6): Binding, AnimatedBindingCallable<T, A>, AnimatedValueBinding<T>, LogicalLength, Property<T>, PropertyValueAnimationData<T>

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
Cohesion: 0.07
Nodes (6): set_window_event_hook(), set_window_shown_hook(), SlintContext, SlintContextInner, SlintContextWeak, WindowEventHook

### Community 113 - "ConicGradientBrush"
Cohesion: 0.16
Nodes (8): center_or_bbox(), ConicGradientBrush, .HEADER, nan_eq(), RadialGradientBrush, .HEADER, slint_conic_gradient_apply_rotation(), slint_conic_gradient_normalize_stops()

### Community 114 - "RenderText"
Cohesion: 0.14
Nodes (3): RenderText, (SharedString, Brush), draw_text()

### Community 115 - "MenuVTable"
Cohesion: 0.12
Nodes (10): ItemTreeRefPin, MenuVTable, InputMethodRequest, Disable, Enable, Update, slint_windowrc_setup_menu_bar_shortcuts(), slint_windowrc_setup_native_menu_bar() (+2 more)

### Community 116 - "Slice"
Cohesion: 0.17
Nodes (26): agrees(), agrees_for_a_column(), assert_agrees(), assert_agrees_column(), box_layout_info(), box_layout_info_ortho(), cell(), constraints() (+18 more)

### Community 117 - "textlayout/shaping.rs"
Cohesion: 0.11
Nodes (12): ShapeBoundaries, ShapeBoundaries<'a>, ShapeBuffer<Length>, test_byte_offsets_are_absolute(), test_letter_spacing(), test_shape_boundaries_empty(), test_shape_boundaries_script_change(), test_shape_boundaries_simple() (+4 more)

### Community 118 - "spring.rs"
Cohesion: 0.10
Nodes (16): regime_decays_to_rest_over_time(), regime_matches_initial_conditions(), SpringDurationBounceParameters, SpringParameters, SpringPhysicalParameters, SpringRegime, Critical, .CRITICAL_ZETA_EPSILON (+8 more)

### Community 119 - "play_one"
Cohesion: 0.11
Nodes (16): Ctx, applog(), keep_display_on(), play_one(), PlayResult, Done, Failed, Next (+8 more)

### Community 120 - "text.rs"
Cohesion: 0.09
Nodes (17): PointArg, PASSWORD_CHARACTER, slint_textinput_clear_selection(), slint_textinput_copy(), slint_textinput_cut(), slint_textinput_paste(), slint_textinput_redo(), slint_textinput_set_selection_offsets() (+9 more)

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
Cohesion: 0.13
Nodes (15): AccessibilityAction, Decrement, Default, Expand, Increment, ReplaceSelectedText, SetSelectionOffsets, SetValue (+7 more)

### Community 126 - "Cast"
Cohesion: 0.20
Nodes (9): Cast, Cmd, Load, Pause, Play, Seek, Stop, start() (+1 more)

### Community 127 - ".evaluate"
Cohesion: 0.15
Nodes (9): PropertyTracker<NEEDS_SET_DIRTY, ()>, PropertyTracker<NEEDS_SET_DIRTY, DirtyHandler>, PropertyTracker<true, DirtyHandler>, test_nested_property_tracker_dirty(), test_nested_property_tracker_evaluate_if_dirty(), test_nested_property_trackers(), test_property_dirty_handler(), test_property_listener_scope() (+1 more)

### Community 129 - "wgpu_30.rs"
Cohesion: 0.20
Nodes (11): any_wgpu30_adapters_with_gpu(), async_init_instance_adapter_device_queue_surface(), default_backends_to_avoid(), default_device_descriptor(), device_descriptor_from_settings(), init_instance_adapter_device_queue_surface(), init_instance_adapter_device_queue_surface_then(), mask_backends() (+3 more)

### Community 130 - "graphics.rs"
Cohesion: 0.16
Nodes (5): create_graphics_api_wgpu_29(), create_graphics_api_wgpu_30(), physical_size_from_api(), Point, Transform

### Community 131 - "fragments.rs"
Cohesion: 0.15
Nodes (14): fragment_iterator_break_anywhere(), fragment_iterator_forced_break(), fragment_iterator_forced_break_multi(), fragment_iterator_leading_nbsp(), fragment_iterator_nbsp(), fragment_iterator_simple(), fragment_iterator_simple_v2(), TextFragment (+6 more)

### Community 132 - "WindowPinnedFields"
Cohesion: 0.33
Nodes (4): PropertyDirtyHandler, WindowPinnedFields, WindowPropertiesTracker, WindowRedrawTracker

### Community 133 - ".visual_representation"
Cohesion: 0.20
Nodes (4): cursor_uses_selection_color(), mask_password(), PreEditSelection, TextInputVisualRepresentation

### Community 134 - "ImageInner"
Cohesion: 0.09
Nodes (17): ImageCache, ImageWeightInBytes, replace_cached_image(), CachedPath, ImageCacheKey, EmbeddedData, Invalid, Path (+9 more)

### Community 136 - "ModelTracker"
Cohesion: 0.07
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
Cohesion: 0.05
Nodes (26): MouseCursorInner, BuiltIn, CustomMouseCursor, InputEventFilterResult, ForwardAndIgnore, ForwardAndInterceptGrab, ForwardAndObserve, ForwardEvent (+18 more)

### Community 143 - "TouchState"
Cohesion: 0.19
Nodes (8): GestureRecognitionState, Idle, Pinching, TwoFingersDown, TouchEventBuffer, TouchState, .PINCH_THRESHOLD, .ROTATION_THRESHOLD

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
Cohesion: 0.10
Nodes (13): &'a ImageInner, Image, NineSlice, load_image_from_embedded_data(), NineSliceImage, slint_image_compare_equal(), slint_image_load_from_embedded_data(), slint_image_load_from_path() (+5 more)

### Community 148 - "Instant"
Cohesion: 0.14
Nodes (5): animation_tick(), AnimationDriver, current_tick(), Instant, update_animations()

### Community 149 - "MouseInputState"
Cohesion: 0.19
Nodes (10): handle_mouse_grab(), DelayForwarding, MouseGrabResult, MouseInputResult, MouseInputState, offer_native_drag(), process_delayed_event(), process_mouse_input() (+2 more)

### Community 150 - "LogicalRect"
Cohesion: 0.04
Nodes (21): adjust_rect_and_border_for_inner_drawing(), Rect, BorderRectLayout, clip_content_box(), item_children_bounding_rect(), item_children_bounding_rect_transformed(), item_with_children_bounding_rect_transformed(), ItemCache (+13 more)

### Community 151 - "DependencyNode"
Cohesion: 0.23
Nodes (5): DependencyListHead, DependencyListHead<T>, DependencyNode, DependencyNode<T>, mark_dependencies_dirty()

### Community 152 - "item_focus.rs"
Cohesion: 0.44
Nodes (10): default_next_in_local_focus_chain(), default_previous_in_local_focus_chain(), step_into_node(), step_out_of_node(), test_focus_chain_complex_tree(), test_focus_chain_one_child(), test_focus_chain_root_only(), test_focus_chain_three_children() (+2 more)

### Community 153 - "erased_bindings.rs"
Cohesion: 0.29
Nodes (11): change_tracker_init_erased(), change_tracker_init_impl(), set_animated_property_binding_erased(), set_animated_property_binding_impl(), set_callback_handler_erased(), set_callback_handler_impl(), set_property_binding_erased(), set_property_binding_impl() (+3 more)

### Community 155 - "solve_flexbox_layout_with_measure"
Cohesion: 0.18
Nodes (10): FlexboxLayoutCacheGenerator<'a>, FlexboxMeasureFn, FlexboxTaffyBuilder, percentage_against_unbounded_container_leaves_item_finite(), resolve_measure_defaults(), solve_flexbox_layout_with_measure(), solve_row(), taffy_column_main() (+2 more)

### Community 156 - "super"
Cohesion: 0.09
Nodes (16): Hide, cached_paragraphs(), CachedParagraphs, CachedParagraphsGuard, ENTRY_LIMIT, ElisionCut, Layout, layout() (+8 more)

### Community 157 - "PlainOrStyledText"
Cohesion: 0.29
Nodes (3): PlainOrStyledText, Plain, Styled

### Community 158 - "SystemTrayIcon"
Cohesion: 0.16
Nodes (4): MenuDirtyHandler, MenuState, slint_system_tray_icon_set_menu(), SystemTrayIcon

### Community 159 - "platform/windows.rs"
Cohesion: 0.13
Nodes (7): RichNote, APP_ID, set_material(), SUPPORTS_MATERIAL, system_notify(), system_notify_rich(), toast_xml()

### Community 160 - "&rustybuzz::Face<'_>"
Cohesion: 0.13
Nodes (7): single_line_height(), slint_cpp_text_item_fontmetrics(), slint_text_item_fontmetrics(), AbstractFont, FontMetrics, &rustybuzz::Face<'_>, T

### Community 161 - "constant_deceleration_spring_damper.rs"
Cohesion: 0.23
Nodes (8): calculate_parameters(), constant_deceleration_decreasing_limit_not_reached(), constant_deceleration_increasing_limit_not_reached(), constant_deceleration_spring_damper_decreasing_limit_reached(), constant_deceleration_spring_damper_increasing_limit_reached(), constant_deceleration_start_eq_limit(), ConstantDecelerationSpringDamper, ConstantDecelerationSpringDamperParameters

### Community 162 - "debug_log.rs"
Cohesion: 0.19
Nodes (9): default_log_message(), log_message(), LogMessage, LogMessage<'a>, LogMessageHandler, LogMessageLocation, LogMessageSource, Runtime (+1 more)

### Community 165 - "platform.rs"
Cohesion: 0.04
Nodes (31): _ANIM_TICK_UPDATED_ON_PLATFORM_SET, EVENTLOOP_PROXY, EventLoopProxy, INITIAL_INSTANT, InternalEvent, Key, Mouse, Touch (+23 more)

### Community 166 - "WindowHandle"
Cohesion: 0.14
Nodes (6): WindowHandle, WindowHandleInner, HandleByAdapter, HandleByRcRWH, has_display_handle(), slint_windowrc_wldisplay_wayland()

### Community 168 - "data_ringbuffer.rs"
Cohesion: 0.36
Nodes (7): test_empty_buffer(), test_push_beyond_capacity(), test_push_beyond_capacity_wrap_back(), test_push_single_element(), test_push_two_elements(), test_push_until_full(), VelocityRingBuffer

### Community 170 - "lengths.rs"
Cohesion: 0.38
Nodes (6): logical_point_from_api(), logical_position_to_api(), logical_size_from_api(), logical_size_to_api(), LogicalSize, PhysicalBorderRadius

### Community 171 - "Changelog"
Cohesion: 0.08
Nodes (24): Changelog, Foyer 0.23.0, Foyer 0.23.1, Foyer 0.23.2, Foyer 0.26.10, Foyer 0.26.11, Foyer 0.26.12, Foyer 0.26.5 (+16 more)

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
Cohesion: 0.23
Nodes (7): CubicBezierSegment, ease_out_bounce_curve(), easing_curve(), S, SPRING_SETTLE_POSITION_EPSILON, SPRING_SETTLE_VELOCITY_EPSILON, SPRING_SETTLE_ZETA

### Community 177 - "Error"
Cohesion: 0.14
Nodes (7): PlatformTray, Error, EventLoopError, PlatformError, Rgba8, Unsupported, Params

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
Cohesion: 0.23
Nodes (4): Property<Length<crate::Coord, Unit>>, PropertyPhysicsAnimationData, PropertyPhysicsAnimationData<S>, RefCell<PropertyPhysicsAnimationData<S>>

### Community 183 - "rendering_metrics_collector.rs"
Cohesion: 0.21
Nodes (7): debug_log!, FrameData, RefreshMode, FullSpeed, Lazy, RenderingMetrics, RenderingMetricsCollector

### Community 184 - ".apply_dirty_region"
Cohesion: 0.24
Nodes (3): PartialRenderer, PartialRenderer<'a, T>, PartialRenderingState

### Community 186 - "Slint Software License"
Cohesion: 0.13
Nodes (14): 1. Grant of Rights, 2. License Conditions, 3. Limitations, 4. Audit Rights, 5. Termination, 6. Assignment, 7. Severability, 8. Governing Law (+6 more)

### Community 188 - "StyledTextItem"
Cohesion: 0.06
Nodes (4): StringArg, ComplexText, SimpleText, StyledTextItem

### Community 189 - "InterpolatedPropertyValue"
Cohesion: 0.13
Nodes (5): f32, i32, i64, InterpolatedPropertyValue, u8

### Community 190 - "TimerListRc"
Cohesion: 0.20
Nodes (7): list_from_handle(), list_handle(), set_owning_context(), take_pending_timers(), ThreadTimers, Timer, TimerListRc

### Community 191 - "ItemRef"
Cohesion: 0.25
Nodes (4): ItemTreeRef, slint_unregister_item_tree(), unregister_item_tree(), ItemRef

### Community 193 - "EasingCurve"
Cohesion: 0.20
Nodes (10): EasingCurve, CubicBezier, EaseInBounce, EaseInElastic, EaseInOutBounce, EaseInOutElastic, EaseOutBounce, EaseOutElastic (+2 more)

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
Cohesion: 0.12
Nodes (18): f32, fit(), fit9slice(), FitResult, StaticTextures, RemEuclid, scalable_render_size(), slint_image_from_embedded_textures() (+10 more)

### Community 198 - "RequestedGraphicsAPI"
Cohesion: 0.19
Nodes (10): RequestedGraphicsAPI, Direct3D, Metal, OpenGL, Vulkan, WGPU29, WGPU30, RequestedOpenGLVersion (+2 more)

### Community 199 - "Direction"
Cohesion: 0.20
Nodes (6): Direction, Decreasing, Increasing, assert_approx_eq!, Parameter, Simulation

### Community 200 - "SingleLinkedListPinHead<T>"
Cohesion: 0.32
Nodes (3): big_list(), SingleLinkedListPinHead<T>, test_list()

### Community 201 - "PropertyValueAnimationData"
Cohesion: 0.17
Nodes (11): AnimatedBindingCallable, AnimatedBindingState, Animating, NotAnimating, ShouldStart, AnimatedValueBinding, AnimationState, Animating (+3 more)

### Community 202 - "Model"
Cohesion: 0.08
Nodes (12): InternalToken, bool, ErrorImpl, OutOfBounds, Unsupported, Model, ModelError, ModelIterator (+4 more)

### Community 203 - "winit_ops.rs"
Cohesion: 0.15
Nodes (9): bring_to_front(), CUSTOM_RESIZE, drag_resize(), drag_window(), monitor_area(), set_cursor_visible(), set_decorations(), set_window_rect() (+1 more)

### Community 204 - "SurfaceTarget"
Cohesion: 0.47
Nodes (3): SurfaceTarget, Drm, WindowHandle

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

### Community 213 - "popup.rs"
Cohesion: 0.42
Nodes (7): fixed_placement(), place_popup(), Placement, Fixed, r(), test_place_popup_fixed_clipped(), test_place_popup_fixed_unclipped()

### Community 214 - "component_factory.rs"
Cohesion: 0.31
Nodes (3): ComponentFactory, ComponentFactoryInner, FactoryContext

### Community 215 - "i-slint-core/lib.rs"
Cohesion: 0.25
Nodes (3): with_global_context(), detect_operating_system(), open_url()

### Community 219 - "menus.rs"
Cohesion: 0.12
Nodes (11): FlexboxLayoutCacheGenerator, GridLayoutCacheGenerator, LayoutCacheGenerator, measure_closure_from_c(), slint_solve_flexbox_layout(), solve_flexbox_layout(), MenuFromItemTree, ShadowTreeNode (+3 more)

### Community 220 - "Renderer"
Cohesion: 0.07
Nodes (6): Renderer, WindowAdapter, slint_path_point_at(), slint_item_absolute_position(), ItemTransform, LogicalPoint

### Community 221 - "DrawOutcome"
Cohesion: 0.40
Nodes (5): DrawOutcome, Occluded, Skipped, Success, Timeout

### Community 222 - ".compute_dirty_regions"
Cohesion: 0.15
Nodes (5): CachedItemBoundingBoxAndTransform, ClipItem, ItemWithTransform, RegularItem, clipped_screen_rect()

### Community 226 - "benches/string.rs"
Cohesion: 0.31
Nodes (5): REFERENCE_NUMBER, RESULT_NUMBER, string_replace(), string_replace_own_character(), string_replacen()

### Community 227 - "android"
Cohesion: 0.09
Nodes (22): secrets, uses, with, notes, jobs, android, publish, name (+14 more)

### Community 228 - "RepaintBufferType"
Cohesion: 0.50
Nodes (4): RepaintBufferType, NewBuffer, ReusedBuffer, SwappedBuffers

### Community 229 - "TextLineBreaker"
Cohesion: 0.36
Nodes (4): TextLine, TextLine<Length>, TextLineBreaker, TextLineBreaker<'_, Font>

### Community 231 - "pathbuf"
Cohesion: 0.32
Nodes (5): Asset, find_dll(), install(), Release, RELEASES

### Community 233 - "Scalar"
Cohesion: 0.25
Nodes (7): f32, Scalar, .HALF, .NINE, .ONE, .SIX, .THREE

### Community 235 - "windows"
Cohesion: 0.25
Nodes (8): run, windows, tag, version, shell, defaults, outputs, runs-on

### Community 236 - "Slice"
Cohesion: 0.15
Nodes (15): BitmapFont, BitmapGlyph, BitmapGlyphs, CharacterMapEntry, BoxLayoutData, GridLayoutData, layout_items(), slint_solve_box_layout() (+7 more)

### Community 237 - "WindowEventDispatchResult"
Cohesion: 0.33
Nodes (4): WindowEventDispatchResult, Accepted, Rejected, MouseDispatchResult

### Community 239 - "PlatformError"
Cohesion: 0.06
Nodes (12): ComponentHandle, PlatformError, NoEventLoopProvider, NoPlatform, Other, OtherError, SetPlatformError, Unsupported (+4 more)

### Community 242 - "Slint Royalty-free Desktop, Mobile, and Web Applications License"
Cohesion: 0.29
Nodes (6): 1. Grant of Rights, 2. License Conditions - Attribution, 3. Limitations, 4. Warranty and Liability, Preamble, Slint Royalty-free Desktop, Mobile, and Web Applications License

### Community 245 - "SurfaceTarget"
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
- **460 isolated node(s):** `Session`, `Chromecast`, `Session`, `Home`, `Backdrop` (+455 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1712 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **42 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SharedString` connect `SharedString` to `input_items.rs`, `MenuItem`, `graphics.rs`, `app/src/lib.rs`, `ImageInner`, `.visual_representation`, `WindowProperties`, `data_transfer.rs`, `AccessibleStringProperty`, `ItemRc`, `TextInput`, `Image`, `LogicalRect`, `window.rs`, `PlainOrStyledText`, `SystemTrayIcon`, `WindowInner`, `platform.rs`, `T`, `date_time.rs`, `format!`, `graphics/path.rs`, `item_tree.rs`, `items.rs`, `ScaleFactor`, `DataTransfer`, `StyledTextItem`, `input.rs`, `KeyboardModifiers`, `PhysicalPx`, `TestItemTree`, `translations.rs`, `i-slint-core/lib.rs`, `ParsedSVG`, `i-slint-core/string.rs`, `menus.rs`, `ItemTreeVTable`, `benches/string.rs`, `GraphicsAPI`, `SlintContext`, `text.rs`, `i-slint-core/accessibility.rs`?**
  _High betweenness centrality (0.107) - this node is a cross-community bridge._
- **Are the 46 inferred relationships involving `ItemRc` (e.g. with `.font_request()` and `item_children_bounding_rect()`) actually correct?**
  _`ItemRc` has 46 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Session`, `Chromecast`, `Session` to the rest of the system?**
  _460 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `input_items.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.05336538461538461 - nodes in this community are weakly interconnected._
- **Why does `ItemRc` connect `ItemRc` to `input_items.rs`, `MenuItem`, `sharedparley.rs`, `AccessibleStringProperty`, `MouseEvent`, `TextInput`, `MouseInputState`, `LogicalRect`, `window.rs`, `Coord`, `super`, `SystemTrayIcon`, `&rustybuzz::Face<'_>`, `WindowInner`, `PartialRenderer<'_, T>`, `SharedString`, `item_tree.rs`, `flickable.rs`, `items.rs`, `ScaleFactor`, `PartialRendererCache`, `items/image.rs`, `StyledTextItem`, `Item`, `drag_n_drop.rs`, `input.rs`, `component_container.rs`, `items/path.rs`, `TestItemTree`, `BoxShadowOptions`, `TextCursorAffinity`, `RenderingResult`, `menus.rs`, `Renderer`, `.init`, `.compute_dirty_regions`, `ItemTreeVTable`, `RendererSealed`, `SlintContext`, `RenderText`, `MenuVTable`, `text.rs`, `i-slint-core/accessibility.rs`?**
  _High betweenness centrality (0.081) - this node is a cross-community bridge._
- **Are the 4 inferred relationships involving `SharedString` (e.g. with `.set_xdg_app_id()` and `.xdg_app_id()`) actually correct?**
  _`SharedString` has 4 INFERRED edges - model-reasoned connections that need verification._
- **Should `player/src/lib.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10121457489878542 - nodes in this community are weakly interconnected._