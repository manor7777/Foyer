# Graph Report - Foyer  (2026-11-17)

## Corpus Check
- 150 files · ~302,494 words
- Verdict: corpus is large enough that graph structure adds value.
- Unclassified: 10 file(s) not represented in the graph (top: (none) 4, .xml 2, .rc 1)

## Summary
- 6738 nodes · 17490 edges · 264 communities (218 shown, 46 thin omitted)
- Extraction: 95% EXTRACTED · 5% INFERRED · 0% AMBIGUOUS · INFERRED: 949 edges (avg confidence: 0.56)
- Token cost: 0 input · 0 output

## Graph Freshness
- Built from commit: `3042f2e0`
- Run `git rev-parse HEAD` and compare to check if the graph is stale.
- Run `graphify update .` after code changes (no API cost).

## Community Hubs (Navigation)
- MouseEvent
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
- steps
- Client
- RemoteEvent
- input_items.rs
- seerr.rs
- InternalKeyEvent
- .plex_tasks
- User
- sharedvector.rs
- Page
- User
- Media
- window.rs
- Ctx
- Foyer 0.22.0
- ItemRc
- Detail
- system_tray/windows.rs
- Error
- BorderRadius<T, U>
- ImageKind
- .send
- WindowInner
- foyer
- PlatformError
- Client
- Color
- adapters.rs
- Community 41
- Android
- super
- SharedString
- FontRequest
- api.rs
- properties/ffi.rs
- Quality
- graphics/path.rs
- item_tree.rs
- flickable.rs
- TextInput
- PhysicalLength
- StyledTextItem
- .show_detail
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
- Renderer
- Model
- android_media.rs
- items.rs
- TestItemTree
- ksni.rs
- graphics/image.rs
- .get
- translations.rs
- Property
- TextCursorAffinity
- i-slint-core/string.rs
- appkit.rs
- brush.rs
- android.rs
- CachedRenderingData
- WindowEvent
- two_way_binding.rs
- steps
- IntSize
- ImageInner
- super
- properties_animations.rs
- Brush
- pt
- RendererSealed
- platform.rs
- .find_sibling
- LogicalLength
- downloads.rs
- linebreaker.rs
- Image
- vec
- Rgba8Pixel
- api.rs
- Repeater<C>
- system_tray.rs
- DirtyRegion
- RepeaterInstanceOpsVTable
- PartialRendererCache
- T
- ChangeTracker
- LengthType
- textlayout.rs
- SlintContext
- ConicGradientBrush
- RenderingResult
- WindowAdapterInternal
- Slice
- textlayout/shaping.rs
- spring.rs
- PopupWindow
- .window_item
- gdb_pretty_printers.py
- timers.rs
- wgpu_29.rs
- TextCursorDirection
- FontMetrics
- Coord
- .evaluate
- item_focus.rs
- wgpu_30.rs
- graphics.rs
- fragments.rs
- PlatformTray
- .load_from_data_with_cache_key
- sharedparley.rs
- FittedPathBox
- SystemTrayIconDataBox
- i-slint-core/accessibility.rs
- PartialRenderer
- Inner
- OnceCell
- AccessibleStringProperty
- ConstantDecelerationSpringDamper
- TouchState
- desktop.rs
- constant_deceleration_spring_damper.rs
- StyledText
- rtti.rs
- Instant
- MouseInputState
- LogicalRect
- ModelTracker
- menus.rs
- erased_bindings.rs
- BackendMouseEvent
- solve_flexbox_layout_with_measure
- src/color.rs
- DrawOutcome
- media.rs
- platform/windows.rs
- ParsedSVG
- constant_deceleration.rs
- debug_log.rs
- RepeatedItemTree
- .plex_folders
- LogicalRect
- context.rs
- Timer
- data_ringbuffer.rs
- PartialRenderer<'_, T>
- ItemTransform
- Changelog
- android_notify.rs
- play_inner
- date_time.rs
- animations.rs
- TouchMap
- Error
- Any
- Length
- CheckedAdd
- Surface
- callbacks.rs
- rendering_metrics_collector.rs
- ItemCache<T>
- T
- Slint Software License
- DependencyNode
- InterpolatedPropertyValue
- TimerList
- ItemRef
- MediaCb
- data_transfer.rs
- KeyboardModifiers
- mod.rs
- .init
- BoxShadowOptions
- RequestedGraphicsAPI
- CallbackVariant
- AnimatedBindingState
- .evaluate
- winit_ops.rs
- WGPUSettings
- WGPUSettings
- StandardShortcut
- Export.java
- preview.rs
- CloseRequestResponse
- component_factory.rs
- TextCursorBlinker
- TwoWayBindingMapping
- T
- SharedVector
- Direction
- LyonPathEventIterator
- Conditional<C>
- Immersive
- benches/string.rs
- android
- i-slint-core/lib.rs
- TextLineBreaker
- pathbuf
- windows
- Scalar
- TexturePixelFormat
- SingleLinkedListPinHead<T>
- FixedTestFont
- Ctx
- bitmapfont.rs
- Slint Royalty-free Desktop, Mobile, and Web Applications License
- WindowEventDispatchResult
- slint_timer_start
- SurfaceTarget
- SurfaceTarget
- Saturating
- BreakOpportunity
- append_menu_entry
- TextureImportError
- TextureImportError
- core/src/lib.rs
- Foyer 0.24.0
- Foyer's patch to i-slint-core 1.18.1
- CrossAxisSizing
- Foyer 0.26.3
- Foyer 0.26.2
- Foyer 0.25.1
- Foyer 0.25.0
- Foyer 0.26.0
- main
- Texture
- Texture
- LineBreakIterator<'a>
- Foyer 0.26.4
- Foyer 0.24.1
- build-mediacb.sh
- RepeaterTracker<C>
- time::Instant
- README.md
- .next
- .next
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
8. `WindowInner` - 98 edges
9. `Client` - 96 edges
10. `Ctx` - 92 edges

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

## Communities (264 total, 46 thin omitted)

### Community 0 - "MouseEvent"
Cohesion: 0.05
Nodes (30): MouseCursorInner, BuiltIn, CustomMouseCursor, InputEventFilterResult, ForwardAndIgnore, ForwardAndInterceptGrab, ForwardAndObserve, ForwardEvent (+22 more)

### Community 1 - "player/src/lib.rs"
Cohesion: 0.10
Nodes (29): Api, api(), available(), DEFAULT_OPTIONS, dll_candidates(), END_EOF, END_ERROR, engine_dir() (+21 more)

### Community 2 - "Ctx"
Cohesion: 0.09
Nodes (3): Ctx, episode_summary(), nice_date()

### Community 3 - "update.rs"
Cohesion: 0.07
Nodes (26): Api, ApiAsset, ASSET, bad_signature(), check(), check_signature(), Checked, Latest (+18 more)

### Community 4 - "Foyer (native Emby client)"
Cohesion: 0.05
Nodes (46): Android and Android TV plan, Android TV (D-pad focus, 10-foot layouts), APK signing key backup, Signed APK self-update, Background notifications (new feature on Android), CI Android job publishing exe and apk, libmpv .so with MediaCodec (ExoPlayer fallback), Material You accent colour (+38 more)

### Community 5 - "app/src/lib.rs"
Cohesion: 0.04
Nodes (46): android_main(), applog(), AUDIO_PICK, audio_streams(), auto_hw_mode(), ChanData, DEBUG, debug_banner() (+38 more)

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
Nodes (19): cached_image(), images_dir(), prune_image_cache(), Ctx, FOLLOWED, init_tray_events(), INSTANCE_ADDR, load_notes() (+11 more)

### Community 10 - "Seerr"
Cohesion: 0.14
Nodes (7): Person, PublicSettings, Quota, QuotaPart, Result, Seerr, WatchItem

### Community 11 - "Result"
Cohesion: 0.11
Nodes (5): next_up_items(), Browse, Item, Client, section_key()

### Community 12 - "steps"
Cohesion: 0.16
Nodes (16): GH_TOKEN, NOTES, RUSTFLAGS, UPDATE_SIGNING_KEY, steps, env, id, name (+8 more)

### Community 14 - "RemoteEvent"
Cohesion: 0.10
Nodes (19): parse(), RemoteEvent, FastForward, Message, Mute, Next, Pause, Play (+11 more)

### Community 15 - "input_items.rs"
Cohesion: 0.05
Nodes (15): FocusReasonArg, FocusScope, KeyBinding, KeyBindingList, MaybeKeyBindingList, ScaleRotateGestureHandler, slint_maybe_key_binding_list_free(), slint_maybe_key_binding_list_init() (+7 more)

### Community 16 - "seerr.rs"
Cohesion: 0.06
Nodes (33): SIssue, SReq, DetailCtx, DetailData, EDIT_BITS, ep(), ISSUE_FILTERS, IssueData (+25 more)

### Community 17 - "InternalKeyEvent"
Cohesion: 0.04
Nodes (18): FocusEvent, FocusIn, FocusOut, FocusEventResult, FocusAccepted, FocusIgnored, InternalKeyEvent, KeyEventResult (+10 more)

### Community 19 - "User"
Cohesion: 0.22
Nodes (3): Comment, Issue, User

### Community 20 - "sharedvector.rs"
Cohesion: 0.07
Nodes (33): alloc_with_capacity(), capacity_for_grow(), compute_inner_layout(), data_ptr(), drop_inner(), insert_out_of_bounds_test(), insert_test(), IntoIter (+25 more)

### Community 21 - "Page"
Cohesion: 0.07
Nodes (27): HideTarget, goto_with(), LibView, Page, Detail, Downloads, Genre, Home (+19 more)

### Community 22 - "User"
Cohesion: 0.20
Nodes (3): AuthResult, Policy, User

### Community 23 - "Media"
Cohesion: 0.25
Nodes (3): Media, Page, PersonCredits

### Community 24 - "window.rs"
Cohesion: 0.08
Nodes (54): Size, a_host_overrides_the_default_title(), accent_color(), context_for_root(), GraphicsAPI, Inaccessible, NativeOpenGL, has_window_handle() (+46 more)

### Community 25 - "Ctx"
Cohesion: 0.06
Nodes (13): apply_accent(), apply_material(), close(), Ctx, fav_group(), fuzzy_match(), goto_episode(), load_posters() (+5 more)

### Community 26 - "Foyer 0.22.0"
Cohesion: 0.17
Nodes (12): 🛠 Admin, Foyer 0.22.0, ✨ Highlights, 🏠 Home, 🔎 Libraries & search, 🖼 Look & feel, 🔔 Notifications, 🎬 Playback (+4 more)

### Community 27 - "ItemRc"
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

### Community 32 - "ImageKind"
Cohesion: 0.22
Nodes (6): ImageKind, Backdrop, Logo, Primary, Thumb, Person

### Community 33 - ".send"
Cohesion: 0.15
Nodes (6): log(), RequestAction, Approve, Decline, Delete, Retry

### Community 34 - "WindowInner"
Cohesion: 0.08
Nodes (5): previous_focus_item(), slint_windowrc_resolved_default_font_size(), slint_windowrc_supports_native_menu_bar(), WindowAdapter, WindowInner

### Community 35 - "foyer"
Cohesion: 0.50
Nodes (5): emby-api, foyer, foyer-core, player, seerr-api

### Community 36 - "PlatformError"
Cohesion: 0.05
Nodes (13): ComponentHandle, PlatformError, NoEventLoopProvider, NoPlatform, Other, OtherError, SetPlatformError, Unsupported (+5 more)

### Community 37 - "Client"
Cohesion: 0.12
Nodes (20): ActivityEntry, log(), RemoteSubtitle, SystemInfo, arr(), f(), IDENT, lang2() (+12 more)

### Community 38 - "Color"
Cohesion: 0.07
Nodes (26): Channel, Color, HsvaColor, linear_to_srgb(), OklabColor, OklchColor, quantize(), RgbaColor (+18 more)

### Community 40 - "adapters.rs"
Cohesion: 0.09
Nodes (31): AscendingSortHelper, check_content(), F, FilterModel, FilterModelInner, MapModel, ReverseModel, ReverseModel<M> (+23 more)

### Community 41 - "Community 41"
Cohesion: 0.07
Nodes (25): InternalToken, (), AbsentRowModel, model_all(), model_any(), model_find_index(), ModelIterator<'_, T>, ModelRc (+17 more)

### Community 42 - "Android"
Cohesion: 0.40
Nodes (4): Android, Every feature works on both, and lands on `main`, graphify, Rules for keeping it compatible

### Community 44 - "super"
Cohesion: 0.10
Nodes (18): home_rows_model(), AUTOPLAY_STREAK_LIMIT, BASE, CARD_SCALES, Config, default_accent_mode(), DEFAULT_ACCENT_MODE, default_accent_rgb() (+10 more)

### Community 45 - "SharedString"
Cohesion: 0.06
Nodes (16): slint_image_path(), StandardListViewItem, c_char, SharedString, slint_shared_string_append(), slint_shared_string_bytes(), slint_shared_string_from_bytes(), test_extend_from_chars() (+8 more)

### Community 46 - "FontRequest"
Cohesion: 0.08
Nodes (32): FontRequest, font_metrics(), FontContext, Brush, content_widths_builder(), create_text_paragraphs(), INLINE_CODE_FONT_SCALE, LayoutWithoutLineBreaksBuilder (+24 more)

### Community 47 - "api.rs"
Cohesion: 0.07
Nodes (14): logical_physical_pos(), logical_physical_size(), LogicalPosition, LogicalSize, PhysicalPosition, PhysicalSize, WindowPosition, Logical (+6 more)

### Community 48 - "properties/ffi.rs"
Cohesion: 0.10
Nodes (42): PropertyAnimation, c_set_animated_binding(), c_set_animated_value(), ChangeTrackerOpaque, drop_fn(), eval_fn(), EvalState, ffi_change_tracker_does_not_leak_dep_nodes() (+34 more)

### Community 49 - "Quality"
Cohesion: 0.16
Nodes (11): quality_rows(), auto_steps_down_when_link_is_slow(), decide(), explicit_choices_and_config_roundtrip(), Preset, PRESETS, Quality, Auto (+3 more)

### Community 50 - "graphics/path.rs"
Cohesion: 0.08
Nodes (36): elements_path(), events_path(), fitted_path_sample_at_adds_offset_to_position(), fitted_path_sample_at_returns_none_for_zero_length_path(), fitted_path_sample_at_wraps_percent_for_open_path(), FittedPath, line_path(), LyonPathIteratorVariant (+28 more)

### Community 51 - "item_tree.rs"
Cohesion: 0.10
Nodes (33): create_children_nodes(), create_empty_subtree(), create_item_subtree_item(), create_nested_subtrees(), create_one_node_component(), create_subsubtree_items(), create_subtrees_item(), GEOMETRY_HEIGHT (+25 more)

### Community 52 - "flickable.rs"
Cohesion: 0.08
Nodes (21): abs(), CaptureEvents, MouseOrTouchScreen, MouseWheel, DECELERATION, DISTANCE_THRESHOLD, DURATION_THRESHOLD, ensure_in_bound() (+13 more)

### Community 53 - "TextInput"
Cohesion: 0.07
Nodes (32): PointArg, cursor_uses_selection_color(), mask_password(), next_paragraph_boundary(), next_word_boundary(), PASSWORD_CHARACTER, PreEditSelection, prev_paragraph_boundary() (+24 more)

### Community 54 - "PhysicalLength"
Cohesion: 0.07
Nodes (23): Glyph, GlyphRenderer, Layout, RectangleBorder, TextParagraph, visible_band(), visible_x_range(), ElisionCut (+15 more)

### Community 55 - "StyledTextItem"
Cohesion: 0.05
Nodes (9): HasFont, PlainOrStyledText, Plain, Styled, StringArg, ComplexText, SimpleText, StyledTextItem (+1 more)

### Community 56 - ".show_detail"
Cohesion: 0.07
Nodes (32): apply_image(), card_for(), CardData, cards_model(), episode_row(), episode_tag(), media_info(), set_card_image() (+24 more)

### Community 57 - "i-slint-core/layout.rs"
Cohesion: 0.08
Nodes (36): collect_from_organized_data(), Constraint, Errors, FlexboxMeasureFnC, GridLayoutCacheGenerator<'a>, GridLayoutData, GridLayoutInputData, GridLayoutOrganizedData (+28 more)

### Community 58 - "DataTransfer"
Cohesion: 0.07
Nodes (25): DataTransfer, DataTransferError, TypeNotFound, CppUserData, DataTransferOpaque, path_from_units(), path_to_units(), PathValueType (+17 more)

### Community 59 - ".row_data"
Cohesion: 0.11
Nodes (9): BrokenModel, FilterModelInner<M, F>, ModelChecker<Data>, ReverseModelInner<M>, SortModel<M, S>, SortModelInner<M, S>, test_filter_model_broken_model(), test_reversed_broken_model() (+1 more)

### Community 61 - "steps"
Cohesion: 0.05
Nodes (44): required, required, runs-on, steps, GH_TOKEN, KEYSTORE_B64, KEYSTORE_PASSWORD, OVERRIDE_URL (+36 more)

### Community 62 - "Community 62"
Cohesion: 0.08
Nodes (27): (), alloc_binding_holder(), BINDING_BORROWED, BINDING_POINTER_MASK, BINDING_POINTER_TO_BINDING, BindingCallable, .IS_TWO_WAY_BINDING, BindingHolder (+19 more)

### Community 63 - "Item"
Cohesion: 0.11
Nodes (10): Value, Item, CallbackInfo, common_property(), FieldOffset<Item, crate::Callback<(), Ret>>, FieldOffset<Item, crate::Callback<(T,), Ret>>, FieldOffset<Item, Property<T>>, FieldOffset<Item, T> (+2 more)

### Community 64 - "drag_n_drop.rs"
Cohesion: 0.07
Nodes (15): ALL, AllowedDragActions, clamp_action_to_allowed(), compute_proposed_action(), compute_proposed_action_modifier_table(), COPY_AND_MOVE, COPY_ONLY, cursor_for_action() (+7 more)

### Community 65 - "input.rs"
Cohesion: 0.08
Nodes (28): Keys, keys_from_parts(), keys_from_parts_inner(), KeysInner, KeysParseError, KeysParseErrorInner, IncompatibleModifiers, MultipleGraphemeClusters (+20 more)

### Community 66 - "VisitChildrenResult"
Cohesion: 0.14
Nodes (23): State, Deceleration, Done, SpringDamper, ItemTreeRc, ItemVisitorRefMut, ItemVisitorResult, Abort (+15 more)

### Community 67 - "ModelNotify"
Cohesion: 0.11
Nodes (11): DependencyListHead, ModelChangeListener, ModelChangeListenerBox<T>, ModelChangeListenerContainer, ModelChangeListenerContainer<T>, ModelNotify, ModelNotifyInner, ModelPeer (+3 more)

### Community 68 - "Renderer"
Cohesion: 0.10
Nodes (7): assert_point_approx_eq(), create_transform_test_items(), Renderer, test_absolute_clip_rect_and_geometry_under_transform(), test_map_to_window_through_transform_roundtrip(), test_visibility_with_clip_under_transform(), WindowAdapter

### Community 69 - "Model"
Cohesion: 0.07
Nodes (11): ErrorImpl, OutOfBounds, Unsupported, Model, ModelError, ModelIterator, ModelIterator<'a, T>, ModelRc<T> (+3 more)

### Community 70 - "android_media.rs"
Cohesion: 0.12
Nodes (27): as_class(), ASKED_PERMISSION, CB, CB_CLASS, CHANNEL, create_session(), DEX, enter_pip() (+19 more)

### Community 71 - "items.rs"
Cohesion: 0.08
Nodes (15): ContextMenu, builtin_struct_field_default!, builtin_struct_field_default_doc!, declare_builtin_structs!, declare_enums!, declare_item_vtable!, MenuEntryArg, MenuEntryModel (+7 more)

### Community 72 - "TestItemTree"
Cohesion: 0.06
Nodes (8): core::ops::Range<usize>, IndexRange, ItemTreeNode, DynamicTree, register_item_tree(), slint_register_item_tree(), TestItemTree, TransformTestItemTree

### Community 73 - "ksni.rs"
Cohesion: 0.11
Nodes (16): build_ksni_menu(), dispatch_loop(), entry_to_node(), Event, Activate, Menu, image_to_argb_icon(), KsniTray (+8 more)

### Community 74 - "graphics/image.rs"
Cohesion: 0.12
Nodes (19): f32, fit(), fit9slice(), FitResult, StaticTextures, RemEuclid, scalable_render_size(), slint_image_from_embedded_textures() (+11 more)

### Community 75 - ".get"
Cohesion: 0.13
Nodes (3): const_sentinel(), Property<T>, PropertyHandle

### Community 76 - "translations.rs"
Cohesion: 0.07
Nodes (31): DisplayOrInt, Display, Int, DisplayOrInt<T>, format(), FormatArgs, FormatResult, FormatResult<'_, T> (+23 more)

### Community 77 - "Property"
Cohesion: 0.05
Nodes (17): CachedGraphicsData, ItemCache, ItemRendererFeatures, .SUPPORTS_TRANSFORMATIONS, ComponentContainer, Path, slint_path_angle_at(), slint_path_point_at() (+9 more)

### Community 78 - "TextCursorAffinity"
Cohesion: 0.10
Nodes (12): TextCursorAffinity, NextCharacter, PreviousCharacter, TextInputVisualRepresentation, CachedTextInputAccessibilityState, compose_text_selection(), position_for_byte_offset(), selection_offsets() (+4 more)

### Community 79 - "i-slint-core/string.rs"
Cohesion: 0.10
Nodes (27): format(), shared_string_from_number(), shared_string_from_number_fixed(), shared_string_from_number_precision(), shared_string_from_number_unlocalized(), shared_string_replace_all(), simple_test(), slint_shared_string_clone() (+19 more)

### Community 80 - "appkit.rs"
Cohesion: 0.07
Nodes (6): ItemWeak, activate_entry(), AppearanceObserverIvars, MAX_DEPTH, MenuActionIvars, scheme_for_appearance()

### Community 81 - "brush.rs"
Cohesion: 0.12
Nodes (19): GradientStop, line_for_angle(), LinearGradientBrush, ResolvedBrush, ResolvedConicGradient, ResolvedLinearGradient, ResolvedRadialGradient, sanitize_color_stops() (+11 more)

### Community 82 - "android.rs"
Cohesion: 0.09
Nodes (21): AndroidCtx, bring_to_front(), CHANNEL, CTX, drag_resize(), drag_window(), export_logs(), init() (+13 more)

### Community 83 - "CachedRenderingData"
Cohesion: 0.12
Nodes (33): .cached_rendering_data_offset, .cached_rendering_data_offset, .cached_rendering_data_offset, .cached_rendering_data_offset, .cached_rendering_data_offset, .cached_rendering_data_offset, .cached_rendering_data_offset, .cached_rendering_data_offset (+25 more)

### Community 84 - "WindowEvent"
Cohesion: 0.08
Nodes (20): logical_position_to_api(), InternalEvent, Key, Mouse, Touch, InternalEventBox, WindowEvent, CloseRequested (+12 more)

### Community 85 - "two_way_binding.rs"
Cohesion: 0.11
Nodes (14): Property<T>, property_two_ways_binding_of_two_two_way_bindings(), property_two_ways_binding_of_two_way_binding_first(), property_two_ways_binding_of_two_way_binding_second(), property_two_ways_recurse_from_binding(), property_two_ways_test(), property_two_ways_test_binding(), test_two_way_with_map() (+6 more)

### Community 86 - "steps"
Cohesion: 0.06
Nodes (31): runs-on, steps, timeout-minutes, env, GH_TOKEN, IN_CI, MPV_ANDROID_REF, WGET (+23 more)

### Community 87 - "IntSize"
Cohesion: 0.09
Nodes (9): BorrowedOpenGLTexture, BorrowedOpenGLTextureBuilder, BorrowedOpenGLTextureOrigin, BottomLeft, TopLeft, HTMLImage, svg_intrinsic_size(), slint_image_size() (+1 more)

### Community 88 - "ImageInner"
Cohesion: 0.09
Nodes (17): ImageWeightInBytes, replace_cached_image(), CachedPath, ImageCacheKey, EmbeddedData, Invalid, Path, URL (+9 more)

### Community 89 - "super"
Cohesion: 0.14
Nodes (12): Hide, cached_paragraphs(), CachedParagraphs, CachedParagraphsGuard, ENTRY_LIMIT, layout(), LayoutOptions, line_limit_cut() (+4 more)

### Community 90 - "properties_animations.rs"
Cohesion: 0.17
Nodes (27): AnimationDetail, Component, DELAY, DURATION, get_prop_value(), properties_test_animation_direction_triggered_by_set(), properties_test_animation_negative_delay_triggered_by_set(), properties_test_animation_triggered_by_binding() (+19 more)

### Community 91 - "Brush"
Cohesion: 0.17
Nodes (19): Brush, ConicGradient, LinearGradient, RadialGradient, SolidColor, resolve_brush(), ConicGradient, LinearGradient (+11 more)

### Community 92 - "pt"
Cohesion: 0.14
Nodes (29): classify(), Ev, Exit, Moved, PinchCancelled, PinchEnded, PinchMoved, PinchStarted (+21 more)

### Community 93 - "RendererSealed"
Cohesion: 0.13
Nodes (3): Renderer, RendererSealed, T

### Community 94 - "platform.rs"
Cohesion: 0.12
Nodes (10): _ANIM_TICK_UPDATED_ON_PLATFORM_SET, EVENTLOOP_PROXY, EventLoopProxy, INITIAL_INSTANT, set_platform(), SetPlatformError, AlreadySet, update_timers_and_animations() (+2 more)

### Community 95 - ".find_sibling"
Cohesion: 0.13
Nodes (6): create_subsubtree_items_dynamic_elements(), ensure_item_tree_instantiated(), find_sibling_outside_repeater(), ItemTreeVTable, step_into_node(), test_map_to_native_window_popup_dynamic_element()

### Community 96 - "LogicalLength"
Cohesion: 0.10
Nodes (6): LogicalEdges, LogicalLength, PhysicalEdges, ListViewProperties, TypedListViewProps, update_visible_instances()

### Community 97 - "downloads.rs"
Cohesion: 0.08
Nodes (20): Ctx, CANCEL, Ctx, dir(), Entry, expand(), fetch_one(), key() (+12 more)

### Community 98 - "linebreaker.rs"
Cohesion: 0.13
Nodes (26): char_wrap_sentences(), test_basic_line_break(), test_basic_line_break_anywhere_fallback(), test_basic_line_break_anywhere_fallback_max_lines(), test_basic_line_break_anywhere_fallback_multi_line(), test_basic_line_break_anywhere_fallback_multi_line_char_wrap(), test_basic_line_break_anywhere_fallback_multi_line_v2(), test_basic_line_break_char_wrap() (+18 more)

### Community 99 - "Image"
Cohesion: 0.11
Nodes (12): &'a ImageInner, ImageCache, Image, NineSlice, load_image_from_embedded_data(), NineSliceImage, slint_image_load_from_embedded_data(), slint_image_load_from_path() (+4 more)

### Community 100 - "vec"
Cohesion: 0.09
Nodes (9): FutureRunner, FutureRunner<T>, FutureRunnerInner, FutureState, Finished, Running, JoinHandle, JoinHandle<T> (+1 more)

### Community 101 - "Rgba8Pixel"
Cohesion: 0.16
Nodes (13): dynamic_image_to_shared_image_buffer(), image_to_rgba8_with_target_size(), Rgb8Pixel, Rgba8Pixel, SharedImageBuffer, RGB8, RGBA8, RGBA8Premultiplied (+5 more)

### Community 102 - "api.rs"
Cohesion: 0.04
Nodes (32): EventLoopError, EventLoopTerminated, NoEventLoopProvider, F, Global, GraphicsAPI, NativeOpenGL, WebGL (+24 more)

### Community 103 - "Repeater<C>"
Cohesion: 0.13
Nodes (8): Conditional, ItemTreeRc, Repeater, Repeater<C>, RepeaterInner, RepeaterInner<C>, RepeaterTracker, RustRepeaterOps

### Community 104 - "system_tray.rs"
Cohesion: 0.10
Nodes (7): Callback, MenuDirtyHandler, MenuState, slint_system_tray_icon_set_menu(), SystemTrayIcon, VoidArg, MenuItem

### Community 105 - "DirtyRegion"
Cohesion: 0.16
Nodes (6): Box2D, dirty_region_ignores_empty_rects(), dirty_region_no_intersection(), DirtyRegion, .MAX_COUNT, PartialRenderingState

### Community 106 - "RepeaterInstanceOpsVTable"
Cohesion: 0.09
Nodes (11): MAX_EAGER_INSTANCE_COUNT, RepeatedInstanceState, Clean, Dirty, RepeaterInstanceOps, RepeaterInstanceOpsVTable, RepeaterLayoutState, RustRepeaterOps<'_, C> (+3 more)

### Community 108 - "T"
Cohesion: 0.12
Nodes (10): Binding, AnimatedBindingCallable, AnimatedBindingCallable<T, A>, AnimatedValueBinding, AnimatedValueBinding<T>, LogicalLength, Property<Length<crate::Coord, Unit>>, Property<T> (+2 more)

### Community 109 - "ChangeTracker"
Cohesion: 0.14
Nodes (5): change_multiple_dependencies(), change_tracker(), ChangeTracker, ChangeTrackerInner, delete_from_eval_fn()

### Community 110 - "LengthType"
Cohesion: 0.10
Nodes (12): euclid::Point2D<T, U>, euclid::Rect<T, U>, euclid::Size2D<T, U>, euclid::Vector2D<T, U>, logical_point_from_api(), logical_size_from_api(), logical_size_to_api(), LogicalSize (+4 more)

### Community 111 - "textlayout.rs"
Cohesion: 0.10
Nodes (6): DEFAULT_FONT_SIZE, test_content_widths(), test_content_widths_max_lines(), test_content_widths_min_never_exceeds_max(), TextLayout, TextParagraphLayout

### Community 113 - "ConicGradientBrush"
Cohesion: 0.16
Nodes (8): center_or_bbox(), ConicGradientBrush, .HEADER, nan_eq(), RadialGradientBrush, .HEADER, slint_conic_gradient_apply_rotation(), slint_conic_gradient_normalize_stops()

### Community 114 - "RenderingResult"
Cohesion: 0.17
Nodes (5): ItemRendererRef, ItemVTable, RenderingResult, ContinueRenderingChildren, ContinueRenderingWithoutChildren

### Community 115 - "WindowAdapterInternal"
Cohesion: 0.07
Nodes (9): MenuVTable, has_display_handle(), InputMethodProperties, InputMethodRequest, Disable, Enable, Update, slint_windowrc_setup_menu_bar_shortcuts() (+1 more)

### Community 116 - "Slice"
Cohesion: 0.22
Nodes (23): box_layout_info(), box_layout_info_ortho(), BoxLayoutData, BoxLayoutOrthoData, flexbox_layout_info_cross_axis(), flexbox_layout_info_cross_axis_with_measure(), flexbox_layout_info_main_axis(), flexbox_layout_unwrapped_main() (+15 more)

### Community 117 - "textlayout/shaping.rs"
Cohesion: 0.11
Nodes (12): ShapeBoundaries, ShapeBoundaries<'a>, ShapeBuffer<Length>, test_byte_offsets_are_absolute(), test_letter_spacing(), test_shape_boundaries_empty(), test_shape_boundaries_script_change(), test_shape_boundaries_simple() (+4 more)

### Community 118 - "spring.rs"
Cohesion: 0.10
Nodes (16): regime_decays_to_rest_over_time(), regime_matches_initial_conditions(), SpringDurationBounceParameters, SpringParameters, SpringPhysicalParameters, SpringRegime, Critical, .CRITICAL_ZETA_EPSILON (+8 more)

### Community 119 - "PopupWindow"
Cohesion: 0.10
Nodes (12): PropertyDirtyHandler, PopupWindow, PopupWindowLocation, ChildWindow, TopLevel, PopupWindowPropertiesTracker, WindowKind, Menu (+4 more)

### Community 121 - "gdb_pretty_printers.py"
Cohesion: 0.10
Nodes (5): SharedVectorProvider, SharedVectorSubPrinter, SliceProvider, SliceSubPrinter, SlintCorePrettyPrinter

### Community 122 - "timers.rs"
Cohesion: 0.10
Nodes (21): _BUG3019, _BUG6141_SET_INTERVAL_FROM_CALLBACK, _DOUBLY_REGISTER_ACTIVE_TIMER, _DOUBLY_REGISTER_ACTIVE_TIMER_2, LIST_SHIFT, _RESTART_TIMER_BEING_ACTIVATED, _SINGLESHOT_START, slint_timer_destroy() (+13 more)

### Community 123 - "wgpu_29.rs"
Cohesion: 0.18
Nodes (11): any_wgpu29_adapters_with_gpu(), async_init_instance_adapter_device_queue_surface(), default_backends_to_avoid(), default_device_descriptor(), device_descriptor_from_settings(), init_instance_adapter_device_queue_surface(), init_instance_adapter_device_queue_surface_then(), mask_backends() (+3 more)

### Community 124 - "TextCursorDirection"
Cohesion: 0.08
Nodes (23): TextShortcut, DeleteBackward, DeleteForward, DeleteToStartOfLine, DeleteWordBackward, DeleteWordForward, Move, TextCursorDirection (+15 more)

### Community 125 - "FontMetrics"
Cohesion: 0.15
Nodes (6): slint_cpp_text_item_fontmetrics(), slint_text_item_fontmetrics(), AbstractFont, FontMetrics, &rustybuzz::Face<'_>, T

### Community 126 - "Coord"
Cohesion: 0.18
Nodes (5): Adjust, adjust_items(), Grow, LayoutData, Shrink

### Community 127 - ".evaluate"
Cohesion: 0.15
Nodes (9): PropertyTracker<NEEDS_SET_DIRTY, ()>, PropertyTracker<NEEDS_SET_DIRTY, DirtyHandler>, PropertyTracker<true, DirtyHandler>, test_nested_property_tracker_dirty(), test_nested_property_tracker_evaluate_if_dirty(), test_nested_property_trackers(), test_property_dirty_handler(), test_property_listener_scope() (+1 more)

### Community 128 - "item_focus.rs"
Cohesion: 0.44
Nodes (10): default_next_in_local_focus_chain(), default_previous_in_local_focus_chain(), step_into_node(), step_out_of_node(), test_focus_chain_complex_tree(), test_focus_chain_one_child(), test_focus_chain_root_only(), test_focus_chain_three_children() (+2 more)

### Community 129 - "wgpu_30.rs"
Cohesion: 0.20
Nodes (11): any_wgpu30_adapters_with_gpu(), async_init_instance_adapter_device_queue_surface(), default_backends_to_avoid(), default_device_descriptor(), device_descriptor_from_settings(), init_instance_adapter_device_queue_surface(), init_instance_adapter_device_queue_surface_then(), mask_backends() (+3 more)

### Community 130 - "graphics.rs"
Cohesion: 0.11
Nodes (9): adjust_rect_and_border_for_inner_drawing(), create_graphics_api_wgpu_29(), create_graphics_api_wgpu_30(), physical_position_from_api(), physical_position_to_api(), physical_size_from_api(), Point, Rect (+1 more)

### Community 131 - "fragments.rs"
Cohesion: 0.15
Nodes (14): fragment_iterator_break_anywhere(), fragment_iterator_forced_break(), fragment_iterator_forced_break_multi(), fragment_iterator_leading_nbsp(), fragment_iterator_nbsp(), fragment_iterator_simple(), fragment_iterator_simple_v2(), TextFragment (+6 more)

### Community 132 - "PlatformTray"
Cohesion: 0.16
Nodes (6): AppearanceObserver, build_menu(), entry_to_nsmenuitem(), image_to_nsimage(), MenuAction, PlatformTray

### Community 133 - ".load_from_data_with_cache_key"
Cohesion: 0.11
Nodes (12): clone_from_slice_rejects_a_long_slice(), clone_from_slice_rejects_a_short_slice(), Svg, load_as_html_image(), load_image_from_data_uri(), LoadImageError, SharedPixelBuffer<Pixel>, slint_image_load_from_data() (+4 more)

### Community 134 - "sharedparley.rs"
Cohesion: 0.06
Nodes (22): RenderString, RenderText, (SharedString, Brush), ContentWidths, InnerTextLayoutCache, TextLayoutCache, draw_text(), draw_text_input() (+14 more)

### Community 135 - "FittedPathBox"
Cohesion: 0.31
Nodes (4): FittedPathBox, FittedPathInner, slint_path_fitted_cache_free(), slint_path_fitted_cache_init()

### Community 136 - "SystemTrayIconDataBox"
Cohesion: 0.29
Nodes (3): slint_system_tray_icon_data_free(), slint_system_tray_icon_data_init(), SystemTrayIconDataBox

### Community 137 - "i-slint-core/accessibility.rs"
Cohesion: 0.12
Nodes (16): AccessibilityAction, Decrement, Default, Expand, Increment, ReplaceSelectedText, SetSelectionOffsets, SetValue (+8 more)

### Community 139 - "Inner"
Cohesion: 0.16
Nodes (8): create_hicon(), ensure_class_registered(), Inner, notify_icon_data(), PlatformTray, show_popup_menu(), taskbar_created_message(), wnd_proc()

### Community 140 - "OnceCell"
Cohesion: 0.15
Nodes (6): FakeThreadStorage, FakeThreadStorage<T>, FakeThreadStorage<T, F>, SLINT__thread_local_inner!, thread_local_!, OnceCell<T>

### Community 141 - "AccessibleStringProperty"
Cohesion: 0.10
Nodes (20): AccessibleStringProperty, Checkable, Checked, DelegateFocus, Description, Enabled, Expandable, Expanded (+12 more)

### Community 143 - "TouchState"
Cohesion: 0.15
Nodes (11): GestureRecognitionState, Idle, Pinching, TwoFingersDown, TouchEventBuffer, TouchMap, TouchPoint, TouchState (+3 more)

### Community 144 - "desktop.rs"
Cohesion: 0.10
Nodes (11): form_factor(), form_factor(), set_material(), SUPPORTS_MATERIAL, system_notify(), system_notify_rich(), FormFactor, Desktop (+3 more)

### Community 145 - "constant_deceleration_spring_damper.rs"
Cohesion: 0.29
Nodes (8): calculate_parameters(), constant_deceleration_decreasing_limit_not_reached(), constant_deceleration_increasing_limit_not_reached(), constant_deceleration_spring_damper_decreasing_limit_reached(), constant_deceleration_spring_damper_increasing_limit_reached(), constant_deceleration_start_eq_limit(), ConstantDecelerationSpringDamperParameters, test_limit_property()

### Community 146 - "StyledText"
Cohesion: 0.17
Nodes (12): color_to_styled_text(), get_raw_text(), parse_markdown(), slint_styled_text_clone(), slint_styled_text_drop(), slint_styled_text_eq(), slint_styled_text_from_markdown(), slint_styled_text_from_plain_text() (+4 more)

### Community 147 - "rtti.rs"
Cohesion: 0.10
Nodes (10): AnimatedBindingKind, Animation, NotAnimated, Transition, BuiltinGlobal, BuiltinItem, FieldInfo, declare_ValueType! (+2 more)

### Community 148 - "Instant"
Cohesion: 0.14
Nodes (5): animation_tick(), AnimationDriver, current_tick(), Instant, update_animations()

### Community 149 - "MouseInputState"
Cohesion: 0.21
Nodes (9): handle_mouse_grab(), DelayForwarding, MouseInputResult, MouseInputState, offer_native_drag(), process_delayed_event(), process_mouse_input(), send_exit_events() (+1 more)

### Community 150 - "LogicalRect"
Cohesion: 0.04
Nodes (17): BorderRectLayout, clip_content_box(), item_children_bounding_rect(), item_children_bounding_rect_transformed(), item_with_children_bounding_rect_transformed(), ItemRenderer, render_component_items(), render_item_children() (+9 more)

### Community 151 - "ModelTracker"
Cohesion: 0.08
Nodes (5): BrokenModel<T>, FilterModel<M, F>, MapModel<M, F>, ModelChecker, ModelTracker

### Community 152 - "menus.rs"
Cohesion: 0.26
Nodes (3): MenuFromItemTree, ShadowTreeNode, slint_menus_create_wrapper()

### Community 153 - "erased_bindings.rs"
Cohesion: 0.29
Nodes (11): change_tracker_init_erased(), change_tracker_init_impl(), set_animated_property_binding_erased(), set_animated_property_binding_impl(), set_callback_handler_erased(), set_callback_handler_impl(), set_property_binding_erased(), set_property_binding_impl() (+3 more)

### Community 154 - "BackendMouseEvent"
Cohesion: 0.10
Nodes (18): BackendDragEvent, Drop, Leave, Move, BackendMouseEvent, Exit, Moved, PinchGesture (+10 more)

### Community 155 - "solve_flexbox_layout_with_measure"
Cohesion: 0.12
Nodes (18): agrees(), agrees_for_a_column(), assert_agrees(), assert_agrees_column(), cell(), constraints(), FlexboxLayoutItemInfo, FlexboxMeasureFn (+10 more)

### Community 156 - "src/color.rs"
Cohesion: 0.50
Nodes (3): parse_hex(), rgb_hsv_roundtrip_matches_known_colours(), rgb_to_hsv()

### Community 157 - "DrawOutcome"
Cohesion: 0.40
Nodes (5): DrawOutcome, Occluded, Skipped, Success, Timeout

### Community 159 - "platform/windows.rs"
Cohesion: 0.13
Nodes (7): RichNote, APP_ID, set_material(), SUPPORTS_MATERIAL, system_notify(), system_notify_rich(), toast_xml()

### Community 160 - "ParsedSVG"
Cohesion: 0.16
Nodes (4): load_from_data(), load_from_path(), ParsedSVG, svg_options()

### Community 161 - "constant_deceleration.rs"
Cohesion: 0.28
Nodes (7): constant_deceleration_decreasing_limit_not_reached(), constant_deceleration_decreasing_limit_reached(), constant_deceleration_increasing_limit_not_reached(), constant_deceleration_increasing_limit_reached(), constant_deceleration_start_eq_limit(), ConstantDeceleration, ConstantDecelerationParameters

### Community 162 - "debug_log.rs"
Cohesion: 0.19
Nodes (9): default_log_message(), log_message(), LogMessage, LogMessage<'a>, LogMessageHandler, LogMessageLocation, LogMessageSource, Runtime (+1 more)

### Community 165 - "LogicalRect"
Cohesion: 0.42
Nodes (7): fixed_placement(), place_popup(), Placement, Fixed, r(), test_place_popup_fixed_clipped(), test_place_popup_fixed_unclipped()

### Community 166 - "context.rs"
Cohesion: 0.10
Nodes (7): set_window_event_hook(), set_window_shown_hook(), SlintContextInner, SlintContextWeak, WindowEventHook, with_global_context(), Platform

### Community 167 - "Timer"
Cohesion: 0.24
Nodes (7): list_from_handle(), list_handle(), set_owning_context(), take_pending_timers(), ThreadTimers, Timer, TimerListRc

### Community 168 - "data_ringbuffer.rs"
Cohesion: 0.22
Nodes (8): test_empty_buffer(), test_push_beyond_capacity(), test_push_beyond_capacity_wrap_back(), test_push_single_element(), test_push_two_elements(), test_push_until_full(), VelocityRingBuffer, VelocityRingBuffer<N>

### Community 170 - "ItemTransform"
Cohesion: 0.16
Nodes (6): ItemTransform, CachedItemBoundingBoxAndTransform, ClipItem, ItemWithTransform, RegularItem, clipped_screen_rect()

### Community 171 - "Changelog"
Cohesion: 0.12
Nodes (15): Changelog, Foyer 0.23.0, Foyer 0.23.1, Foyer 0.23.2, Foyer 0.26.10, Foyer 0.26.11, Foyer 0.26.12, Foyer 0.26.5 (+7 more)

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
Cohesion: 0.12
Nodes (17): CubicBezierSegment, ease_out_bounce_curve(), easing_curve(), EasingCurve, CubicBezier, EaseInBounce, EaseInElastic, EaseInOutBounce (+9 more)

### Community 176 - "TouchMap"
Cohesion: 0.33
Nodes (8): touch_map_capacity(), touch_map_first(), touch_map_first_two_ids(), touch_map_get_mut(), touch_map_insert_and_get(), touch_map_remove(), touch_map_remove_nonexistent(), touch_map_update_existing()

### Community 177 - "Error"
Cohesion: 0.14
Nodes (7): PlatformTray, Error, EventLoopError, PlatformError, Rgba8, Unsupported, Params

### Community 179 - "Length"
Cohesion: 0.26
Nodes (4): PositionedGlyph, test_text_size_with_max_lines(), TextLayout<'_, Font>, TextParagraphLayout<'_, Font>

### Community 180 - "CheckedAdd"
Cohesion: 0.12
Nodes (6): CheckedAdd, DivCount, euclid::Length<i16, U>, euclid::Length<T, U>, f32, i16

### Community 181 - "Surface"
Cohesion: 0.19
Nodes (6): GlFns, GlLoader, MiniTex, Surface, SurfaceInner, SurfState

### Community 182 - "callbacks.rs"
Cohesion: 0.23
Nodes (7): Callback<Arg, Ret>, callback_simple_test(), CallbackOpaque, slint_callback_call(), slint_callback_drop(), slint_callback_init(), slint_callback_set_handler()

### Community 183 - "rendering_metrics_collector.rs"
Cohesion: 0.21
Nodes (7): debug_log!, FrameData, RefreshMode, FullSpeed, Lazy, RenderingMetrics, RenderingMetricsCollector

### Community 184 - "ItemCache<T>"
Cohesion: 0.12
Nodes (3): ItemCache<T>, LayerRenderer, render_layer()

### Community 186 - "Slint Software License"
Cohesion: 0.13
Nodes (14): 1. Grant of Rights, 2. License Conditions, 3. Limitations, 4. Audit Rights, 5. Termination, 6. Assignment, 7. Severability, 8. Governing Law (+6 more)

### Community 188 - "DependencyNode"
Cohesion: 0.23
Nodes (5): DependencyListHead, DependencyListHead<T>, DependencyNode, DependencyNode<T>, mark_dependencies_dirty()

### Community 189 - "InterpolatedPropertyValue"
Cohesion: 0.13
Nodes (5): f32, i32, i64, InterpolatedPropertyValue, u8

### Community 190 - "TimerList"
Cohesion: 0.26
Nodes (3): ActiveTimer, current_timers(), TimerList

### Community 191 - "ItemRef"
Cohesion: 0.21
Nodes (5): ItemTreeRef, ItemTreeRefPin, slint_unregister_item_tree(), unregister_item_tree(), ItemRef

### Community 193 - "data_transfer.rs"
Cohesion: 0.24
Nodes (8): cleared_transfer_compares_equal_to_default(), DataTransferInner, every_payload_counts_as_native_data(), set_file_paths_with_empty_list_clears(), set_image_with_default_image_clears(), set_image_with_default_image_on_default_stays_empty(), set_plain_text_with_empty_string_clears(), set_plain_text_with_empty_string_on_default_stays_empty()

### Community 194 - "KeyboardModifiers"
Cohesion: 0.26
Nodes (6): InternalKeyboardModifierState, KeyboardModifiers, modifiers(), AnchorMode, KeepAnchor, MoveAnchor

### Community 195 - "mod.rs"
Cohesion: 0.10
Nodes (13): MediaAction, Forward, Next, Pause, Play, Prev, Rewind, Seek (+5 more)

### Community 197 - "BoxShadowOptions"
Cohesion: 0.16
Nodes (4): BoxShadowCache, BoxShadowOptions, CacheEntry, MAX_CACHED_SHADOWS

### Community 198 - "RequestedGraphicsAPI"
Cohesion: 0.19
Nodes (10): RequestedGraphicsAPI, Direct3D, Metal, OpenGL, Vulkan, WGPU29, WGPU30, RequestedOpenGLVersion (+2 more)

### Community 200 - "CallbackVariant"
Cohesion: 0.19
Nodes (11): CallbackVariant, Empty, MultiFire, SingleShot, single_shot_on(), SingleShotTimerCallback, TimerCallback, TimerData (+3 more)

### Community 201 - "AnimatedBindingState"
Cohesion: 0.50
Nodes (4): AnimatedBindingState, Animating, NotAnimating, ShouldStart

### Community 202 - ".evaluate"
Cohesion: 0.21
Nodes (7): AnimationState, Animating, Delaying, Done, PropertyPhysicsAnimationData, PropertyPhysicsAnimationData<S>, RefCell<PropertyPhysicsAnimationData<S>>

### Community 203 - "winit_ops.rs"
Cohesion: 0.26
Nodes (7): bring_to_front(), drag_resize(), drag_window(), monitor_area(), set_cursor_visible(), set_decorations(), watch_window_events()

### Community 207 - "StandardShortcut"
Cohesion: 0.17
Nodes (11): StandardShortcut, Copy, Cut, Find, Paste, Print, Redo, Refresh (+3 more)

### Community 212 - "CloseRequestResponse"
Cohesion: 0.29
Nodes (4): CloseRequestResponse, HideWindow, KeepWindowShown, WithUserData<extern "C" fn(user_data: *mut c_void) -> CloseRequestResponse>

### Community 214 - "component_factory.rs"
Cohesion: 0.29
Nodes (4): ComponentFactory, ComponentFactoryInner, FactoryContext, ItemTreeWeak

### Community 217 - "TwoWayBindingMapping"
Cohesion: 0.38
Nodes (3): (F1, F2), link_common_properties(), TwoWayBindingMapping

### Community 219 - "SharedVector"
Cohesion: 0.10
Nodes (15): FlexboxLayoutCacheGenerator, FlexboxLayoutCacheGenerator<'a>, GridLayoutCacheGenerator, LayoutCacheGenerator, LayoutCacheGenerator<'a>, resolve_cross_axis_alignment(), slint_solve_box_layout(), slint_solve_box_layout_ortho() (+7 more)

### Community 222 - "Direction"
Cohesion: 0.25
Nodes (5): Direction, Decreasing, Increasing, assert_approx_eq!, Parameter

### Community 223 - "LyonPathEventIterator"
Cohesion: 0.20
Nodes (4): LyonPathEventIterator, FromEvents, FromPath, TransformedLyonPathIterator<EventIt>

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

### Community 232 - "windows"
Cohesion: 0.25
Nodes (8): run, windows, tag, version, shell, defaults, outputs, runs-on

### Community 233 - "Scalar"
Cohesion: 0.25
Nodes (7): f32, Scalar, .HALF, .NINE, .ONE, .SIX, .THREE

### Community 234 - "TexturePixelFormat"
Cohesion: 0.25
Nodes (7): test_image_load_from_data_png(), TexturePixelFormat, AlphaMap, Rgb, Rgba, RgbaPremultiplied, SignedDistanceField

### Community 236 - "SingleLinkedListPinHead<T>"
Cohesion: 0.32
Nodes (3): big_list(), SingleLinkedListPinHead<T>, test_list()

### Community 241 - "bitmapfont.rs"
Cohesion: 0.43
Nodes (4): BitmapFont, BitmapGlyph, BitmapGlyphs, CharacterMapEntry

### Community 242 - "Slint Royalty-free Desktop, Mobile, and Web Applications License"
Cohesion: 0.29
Nodes (6): 1. Grant of Rights, 2. License Conditions - Attribution, 3. Limitations, 4. Warranty and Liability, Preamble, Slint Royalty-free Desktop, Mobile, and Web Applications License

### Community 243 - "WindowEventDispatchResult"
Cohesion: 0.33
Nodes (4): WindowEventDispatchResult, Accepted, Rejected, MouseDispatchResult

### Community 244 - "slint_timer_start"
Cohesion: 0.38
Nodes (3): slint_timer_singleshot(), slint_timer_start(), WrapFn

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

### Community 252 - "core/src/lib.rs"
Cohesion: 0.24
Nodes (7): log(), LOG, LogFn, set_log(), format!, T, ToSharedString

### Community 253 - "Foyer 0.24.0"
Cohesion: 0.50
Nodes (4): Foyer 0.24.0, ✨ Highlights, 🔧 Other, 🎬 Player

### Community 256 - "Foyer's patch to i-slint-core 1.18.1"
Cohesion: 0.50
Nodes (3): Foyer's patch to i-slint-core 1.18.1, Updating Slint, What changed

### Community 257 - "CrossAxisSizing"
Cohesion: 0.50
Nodes (4): CrossAxisSizing, FromMeasure, Minimum, Preferred

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
- **452 isolated node(s):** `Home`, `Backdrop`, `Poster`, `DetailLogo`, `SRow` (+447 more)
  These have ≤1 connection - possible missing edges or undocumented components. (Counts symbols only; 1661 node(s) total have ≤1 connection when file, concept and rationale nodes are included.)
- **46 thin communities (<3 nodes) omitted from report** — run `graphify query` to explore isolated nodes.

## Suggested Questions
_Questions this graph is uniquely positioned to answer:_

- **Why does `SharedString` connect `SharedString` to `graphics.rs`, `app/src/lib.rs`, `i-slint-core/accessibility.rs`, `AccessibleStringProperty`, `InternalKeyEvent`, `LogicalRect`, `menus.rs`, `window.rs`, `ParsedSVG`, `WindowInner`, `context.rs`, `Community 41`, `date_time.rs`, `FontRequest`, `graphics/path.rs`, `item_tree.rs`, `TextInput`, `StyledTextItem`, `DataTransfer`, `data_transfer.rs`, `input.rs`, `KeyboardModifiers`, `items.rs`, `TestItemTree`, `graphics/image.rs`, `translations.rs`, `Property`, `TextCursorAffinity`, `i-slint-core/string.rs`, `WindowEvent`, `ImageInner`, `SharedVector`, `platform.rs`, `.find_sibling`, `benches/string.rs`, `Image`, `i-slint-core/lib.rs`, `api.rs`, `system_tray.rs`, `SlintContext`, `WindowAdapterInternal`, `.window_item`, `core/src/lib.rs`?**
  _High betweenness centrality (0.132) - this node is a cross-community bridge._
- **Are the 46 inferred relationships involving `ItemRc` (e.g. with `.font_request()` and `item_children_bounding_rect()`) actually correct?**
  _`ItemRc` has 46 INFERRED edges - model-reasoned connections that need verification._
- **What connects `Home`, `Backdrop`, `Poster` to the rest of the system?**
  _452 weakly-connected nodes found - possible documentation gaps or missing edges._
- **Should `MouseEvent` be split into smaller, more focused modules?**
  _Cohesion score 0.045421511627906974 - nodes in this community are weakly interconnected._
- **Why does `ItemRc` connect `MouseEvent` to `sharedparley.rs`, `i-slint-core/accessibility.rs`, `PartialRenderer`, `AccessibleStringProperty`, `input_items.rs`, `InternalKeyEvent`, `MouseInputState`, `LogicalRect`, `menus.rs`, `window.rs`, `ItemRc`, `WindowInner`, `context.rs`, `PartialRenderer<'_, T>`, `ItemTransform`, `SharedString`, `FontRequest`, `graphics/path.rs`, `item_tree.rs`, `flickable.rs`, `TextInput`, `StyledTextItem`, `ItemCache<T>`, `items/image.rs`, `drag_n_drop.rs`, `input.rs`, `Renderer`, `BoxShadowOptions`, `.init`, `items.rs`, `TestItemTree`, `Property`, `TextCursorAffinity`, `super`, `RendererSealed`, `.find_sibling`, `system_tray.rs`, `PartialRendererCache`, `RenderingResult`, `FontMetrics`?**
  _High betweenness centrality (0.067) - this node is a cross-community bridge._
- **Are the 4 inferred relationships involving `SharedString` (e.g. with `.set_xdg_app_id()` and `.xdg_app_id()`) actually correct?**
  _`SharedString` has 4 INFERRED edges - model-reasoned connections that need verification._
- **Should `player/src/lib.rs` be split into smaller, more focused modules?**
  _Cohesion score 0.10121457489878542 - nodes in this community are weakly interconnected._