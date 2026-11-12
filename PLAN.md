# EmbyRevived: plan

A native Rust, Slint (Fluent style) replacement for the unsupported Emby Theater for Windows.

## What the original actually is

| Piece | What it is | Fate |
|---|---|---|
| `emby-theater-windows` (C#) | WinForms shell: updater, scheduled tasks, DirectShow/madVR, refresh-rate helper, Electron launcher | Replaced by the Rust app; behaviour kept where useful |
| `Emby.Theater.Electron` (JS) | Electron 3 main process, `libmpv.js` plugin, CEC plugin, server discovery, WoL | Replaced by Rust crates |
| **The UI** | Emby's web client, loaded from `https://tv.emby.media` | **Not in either repo.** Rebuilt natively against the Emby REST API |

So "recreate in full" means: a new native client with feature parity, not a port.

## Your FixEmby.ps1, as built-in behaviour

| Patch | Native equivalent |
|---|---|
| mpv cache: `cache=yes`, `demuxer-max-bytes=1GiB`, `demuxer-max-back-bytes=256MiB`, `stream-buffer-size=4MiB`, `cache-pause-wait=10` | Default mpv options in `player` crate |
| HLS: `demuxer-lavf-o=http_persistent=0,http_multiple=0` | Same default, with a settings toggle |
| `detectBitrate` returns 200 Mbit so Auto direct-plays | "Auto" quality = profile max (200 Mbit), with an optional real speed test |
| Back-input guard (1.5 s after stop), cancel start-up on stop (`playGeneration`) | Playback state machine: `Stop` bumps a generation counter and drops in-flight `Play`. Back input is debounced for 1.5 s after `Stopped` |
| Overlapping pages, stale title/backdrop | Not applicable. Native navigation stack has one active page |
| Chromium background throttling off | Not applicable. No Chromium. The mpv render loop doesn't depend on window focus |

## Self-contained and robust by design

- Talks only to the user's own Emby server. No tv.emby.media, no web UI, no hosted pages. Emby Connect is optional; direct server login always works.
- libmpv ships with the app and plays nearly everything natively. Server transcode is a fallback, not the default.
- Playback safety (fixes the Store app's skip-loop problem), enforced in the playback state machine:
  - A playback error never auto-advances to the next item.
  - Progress is reported only after more than 10 s of real playback; a failed start never writes a position.
  - On failure show Retry / Try transcode / Back; Continue Watching stays untouched.
  - Auto-next only fires on a clean end-of-file.

## Architecture (Cargo workspace)

```
EmbyRevived/
  crates/
    emby-api/    reqwest+tokio+serde client: auth, views, items, images, search,
                 PlaybackInfo, sessions/progress reporting, device profile,
                 Emby Connect, UDP server discovery (port 7359)
    player/      libmpv bound at runtime (libloading, reuses mpv-1.dll),
                 render-API -> wgpu/GL texture or child-window embed, track
                 selection, subtitles, chapters, HDR/refresh-rate switching
    input/       keyboard, mouse, XInput gamepad, HID remote, CEC (libcec) ->
                 unified Command enum (Back, Select, Up, ... PlayPause)
    core/        app state, navigation stack, playback state machine,
                 settings, secure credential store (Windows Credential Manager)
    ui/          Slint .slint files + glue (Fluent style, Mica, focus visuals)
    app/         main binary: tray, single-instance, protocol, updater
```

Stack: Rust 2024, `tokio`, `reqwest` (rustls), `serde`, `slint` (Fluent style, `winit` backend), `windows` crate for Mica/DWM/taskbar/SMTC/power requests, `libloading` for mpv, `tracing`.

## Modernisation

- Native Slint UI with Fluent style, Mica backdrop, rounded corners, light/dark/accent-aware.
- Focus-first navigation so mouse, keyboard, gamepad and remote all work (10-foot mode scales layout).
- Credentials in Windows Credential Manager, not plaintext config.
- Windows media integration: SMTC (media keys, lock-screen controls), Discord-style rich presence optional.
- Per-monitor DPI, HDR/auto refresh-rate switching built in (replaces `RefreshRate.exe`).
- Self-updating from GitHub Releases (signed), no old updater.
- Single static exe plus `mpv-1.dll`; no 100 MB Electron runtime. Target under 25 MB.

## Milestones

1. **M0 Scaffold (done):** workspace builds, `emby-api` has discovery + auth + views.
2. **M1 Browse:** login (server list, manual, Emby Connect), home rows, libraries grid, item detail, images, search.
3. **M2 Playback:** mpv embed, direct play/stream selection via PlaybackInfo, progress reporting, audio/subtitle pickers, your patch defaults, stop/back state machine.
4. **M3 Parity:** music player, live TV and guide, recordings, collections/playlists, user switching, settings screens, multi-language strings.
5. **M4 Couch:** gamepad, CEC, remote, 10-foot scaling, screensaver/idle.
6. **M5 Polish:** SMTC, Mica, updater, installer (MSIX or Inno), crash reports.

## Risks and notes

- The Emby web UI is proprietary and not in the repos; screens are re-designed, not copied. The C# shell is GPL-2.0: copy no code from it unless this project is also GPL-2.0-compatible. The API is used as documented.
- Emby's API is server-version dependent. Test against your own server.
- Embedding mpv under Slint is the hardest technical piece (M2): likely mpv render API into a wgpu texture, with a fallback to a borderless child HWND behind a transparent UI layer.
- DirectShow/madVR from the old shell is dropped. mpv with `gpu-next` covers that need.
- Per your standing preference, I build to a folder and you run it. I won't launch the app.
