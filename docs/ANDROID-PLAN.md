# Android and Android TV plan

Status: in progress (steps 1-2 scaffolded, nothing run on a device yet). Goal: one codebase, one release run that publishes `Foyer.exe` and `Foyer.apk`, with the same features on every platform.

## What carries over

- `emby-api` and `seerr-api` are plain Rust over HTTP and should build for Android with little change.
- Slint supports Android, and the same app can run on Android TV.
- Most of the UI layout is reusable.

## What is Windows-only today

| Area | Today | On Android |
|---|---|---|
| Video | libmpv `.dll` through the render API into the window's OpenGL surface | libmpv `.so` per CPU type, same render API, hardware decoding through MediaCodec. Fallback: ExoPlayer, but then the audio/subtitle menus, subtitle search and quality options must be rebuilt to match |
| Video engine download | 7z of an LGPL libmpv build, first run | Bundled in the APK |
| Accent colour | Windows accent or a picked colour | Material You (wallpaper colour, Android 12+) or a picked colour. TV: picker only |
| Notifications | In-app banner plus Windows toasts | In-app banner (kept, it is part of the Slint UI and the only option on TV) plus system notifications |
| Self-update | Signed `Foyer.exe` swap | Signed APK, install-packages permission, same signing key every release |
| Window material | Mica / Acrylic / solid | Solid |
| GPU detection | Registry | Not needed (MediaCodec) |

## Things to know before starting

- **Background notifications are a new feature, not a port.** The notifier only checks while the app is open. Android pauses background apps, so delivery when the app is closed needs a scheduled background task (and a decision on how often).
- **The code is one large `main.rs` tangled with the UI.** Keeping features in sync means splitting it into a shared core plus small per-platform pieces. That refactor is the real cost, and it makes Windows easier to maintain too.
- **The UI is built for a mouse** (hover controls, click and drag). Phones need touch layouts. Android TV needs full remote (D-pad) focus handling on every screen and large "10-foot" layouts.
- libmpv for Android has to be built or sourced (arm64 at minimum), and packaged per CPU type (roughly 30-50 MB each). Keep it LGPL.
- The APK signing key must be backed up. Losing it means existing installs cannot update.

## Steps

1. **Shared core.** Move app logic out of `crates/app/src/main.rs` into a platform-neutral crate. Define small traits for: player, data/config paths, notifier, accent source, updater. Windows keeps working throughout.
2. **Android phone and tablet.** Build and sign an APK. Get libmpv video playing with MediaCodec. Touch layouts and controls. Accent from Material You or the picker.
3. **Notifications.** In-app banners plus system notifications while the app is open. Background delivery later.
4. **Updates.** Check GitHub releases and verify the same ECDSA P-256 signed checksum used for the exe, then hand the APK to the installer.
5. **CI.** Add an Android job to `.github/workflows/release.yml` (Linux runner; Android tools are preinstalled). Sign with a keystore secret. One run publishes both `Foyer.exe` and `Foyer.apk`.
6. **Android TV.** D-pad navigation, large layouts, TV launcher entry. Sideloaded unless published to Google Play.

## Testing

None of this can be tested on a device from the development machine used so far. Each step needs a real phone or TV (or an emulator) to try it.

## Progress

- Done: OS-specific code sits behind `crates/app/src/platform` (Windows, desktop, Android backends). The app builds and tests pass on Linux.
- Done: app crate is a lib (`run()`, plus `android_main` on Android) and a thin desktop bin. Engine download and the exe updater are skipped on Android; the player loads a bundled `libmpv.so` and asks for MediaCodec.
- Written but untested (no NDK in the dev environment): cargo-apk metadata in `crates/app/Cargo.toml` and the manual `.github/workflows/android.yml` job. Expect fixes on the first real build.
- Next: a libmpv.so build for arm64, then the first APK on a device; touch layouts; Android TV focus handling. `main.rs` was renamed `lib.rs`; the larger split into a core crate is still to do.
