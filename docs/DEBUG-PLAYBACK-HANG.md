# Foyer Android: freeze when leaving the player after skipping to the end

Handoff for a local debugging session. Foyer 0.29.10, Android 17 (API 37), Qualcomm Adreno 830 (a large foldable/tablet screen).

## Symptom (from a friend's bug report)

1. Debug logging on. Play a movie (The Mighty Ducks, direct play, h264 + 5.1 audio, hwdec=mediacodec).
2. Skip to near the end of the film.
3. Press back. Nothing happens: the app stays on a black player screen and back does nothing more.
4. Press the Android Home button. Android shows "Foyer isn't responding" (ANR), with "Close app" / "Wait".

Not yet known: whether it needs the skip to the end, or whether pressing back to leave the player always has a chance of hanging.

## What the logs show

The app's debug log (`logs/debug.log`) is flushed line by line:

- Normal playback, then `report progress pos=59888580000` at 21:18:31.993.
- Then silence. No further mpv status lines (they normally come every 2 s), and **no `report stop` line, ever**.
- The next session starts at 21:19:00 (after "Close app").

mpv's own log (`logs/mpv.log`) is buffered, so the end is partly lost, but the tail that survived (about 17 s into the mpv run) reads:

```
17.015 Run command: quit, flags=64, args=[code="0"]        <- user pressed back (Control::Quit)
17.015 EOF code: 5
17.016 Uninit decoder (ad), (vd)
17.036 Terminating demuxers... / Done terminating demuxers.
17.038 finished playback, success (reason 3)
17.038 Running hook: ytdl_hook/on_after_end_file
17.039 Destroying client handle... (scripts: dnd, positioning, context_menu, ytdl_hook, select, stats, console, commands)
17.239 [vo/libmpv] mpv_render_context_render() not being called or stuck.
17.240 [libmpv_render] flushing shader cache      <- the app freed the render context (render_free)
17.240 [main] Destroying client handle...          <- mpv_terminate_destroy started
17.240 Run command: quit, flags=64, args=[code="0"]  <- LAST LINE
```

Earlier in the same run: two `Audio device underrun detected / restarting audio after underrun` warnings, and `buffer went 100% -> 0%` after a seek to 5980.975 s.

## Working theory

The player thread is stuck inside `(api.destroy)(ctx)` (= `mpv_terminate_destroy`) in `play_inner` in `crates/player/src/lib.rs`, after `surface.unpublish()` succeeded. That is why `play_one` in `crates/app/src/lib.rs` never reaches its `Rep::Stop` report, and why the UI never leaves the player (it waits for the play thread to finish: `s.playing = false` is only set after the loop ends).

Suspects inside core shutdown, **unconfirmed**:
- the Android audio output (`audiotrack` ao; there were underruns), whose stop/release can block in the audio HAL;
- the MediaCodec hardware decoder / SurfaceTexture teardown (`hwdec=mediacodec`, zero-copy).

Not explained: why the whole UI thread froze (the ANR on Home). It may be knock-on from the stuck player (e.g. the activity pause path waiting on something the player holds), but that is not traced.

Ruled out: the render-context free ordering. `Surface::unpublish` waits for `render == 0` (the GL thread's `render_free`) before `destroy`, and the log shows `flushing shader cache` (render free) before `main Destroying client handle`.

## Proposed mitigation (not yet implemented; it needs testing on a device)

Run `(api.destroy)(ctx)` on a helper thread and wait at most about 3 s for it (`recv_timeout`). If it hasn't finished, abandon it (leak that one mpv instance), then carry on: send `Rep::Stop`, return from `play_one`, and let the UI leave the player. Pass the pointer as `usize`. Also log a line (via `applog`) when the timeout fires, so it shows up in debug.log. Needs a patch version (0.29.11) and a `CHANGELOG.md` entry, as per AGENTS.md.

That would stop the stuck-on-black-screen symptom but not fix the root cause of the hang.

## Already done on branch `ccr-c25d320d-y61umn` (pushed, not merged to main)

- `player::redact_log` is now `pub`, reads bytes (so invalid UTF-8 can't skip redaction) and returns early if there's no `api_key=`.
- The app calls it on `logs/mpv.log` at start-up, so a hang or crash during playback can't leave the access token in the file. Previously redaction only ran when playback ended cleanly.
- No version bump, no changelog entry.

## How to get decisive evidence on a device (wireless adb)

On the phone: Settings > Developer options > Wireless debugging. Phone and computer on the same Wi-Fi.

1. Tap "Pair device with pairing code". It shows an IP:port and a 6-digit code (valid only briefly).
2. On the computer: `adb pair <ip>:<pairing-port>` and enter the code.
3. The main Wireless debugging screen shows a different IP:port. Run `adb connect <ip>:<port>`.
4. Check: `adb devices` shows the phone as `device`.
5. Clear old logs: `adb logcat -c`.
6. Start a capture: `adb logcat -v threadtime > foyer-logcat.txt`.
7. Reproduce: Foyer > Settings > turn on extensive debug logging. Play a movie, drag the seek bar to near the end, press back. If the player doesn't close within a few seconds, press Home and wait for the "isn't responding" dialog.
8. Stop logcat (Ctrl+C). Then:
   - `adb bugreport foyer-bugreport.zip` (contains the ANR trace)
   - `adb shell ls /data/anr` (may need root; the ANR stacks are also in the bugreport under `FS/data/anr/`)
   - or, while it is hung and before closing it: `adb shell "kill -3 $(pidof com.<foyer package>)"` and read the thread dump from logcat (find the package with `adb shell pm list packages | grep -i foyer`).
9. Look for the thread dump of the process: what is the `main` thread blocked on, and what is the player thread blocked on (names like `Thread-N` calling `mpv_terminate_destroy`)? Frames in `libmpv.so`, `libaaudio`/`AudioTrack`, `MediaCodec` or `SurfaceTexture` point to the culprit.

Useful logcat filters: `adb logcat -d | grep -i -E "foyer|ANR|mpv|AudioTrack|AAudio|MediaCodec|SurfaceTexture"`.

Also try: does it reproduce with hardware decoding off (Settings > hardware decoding) or with `ao=opensles` only? If hwdec off fixes it, it points at MediaCodec; if not, at the audio path. Foyer's mpv options come from `play_inner` (`set("ao", "audiotrack,opensles")` on Android) and an optional `mpv.conf`.

## Repo pointers

- Player thread / mpv teardown: `crates/player/src/lib.rs` (`play_inner`, `Surface::unpublish`, `Surface::render`).
- Play loop and reporting: `crates/app/src/lib.rs` (`play_one`, `start_playback`, `spawn_reporter`).
- Debug logging: `dlog`, `applog`, `logs_dir`, `on_export_logs` in `crates/app/src/lib.rs`; mpv logging options are set in `play_inner` (`log-file`, `msg-level`).
- Android rules (both platforms, Play Protect, versioning): `AGENTS.md`. `docs/ANDROID-PLAN.md` tracks Android work.
- `graphify` was not installed in the cloud session, so no graph queries were run.
- Note: line endings in `crates/player/src/lib.rs` and `crates/app/src/lib.rs` are CRLF. Preserve them when editing.

## Resolution (23 Nov 2026, 0.29.11 thread dump with symbols)

mpv was not the cause. The 0.29.11 debug log shows mpv's core destroyed in about 30 ms and then `play: mpv finished`, after which nothing. The symbolized ANR dump shows the player thread blocked in `AndroidApp::set_window_flags` (`RwLock::write_contended`, reached from `keep_display_on` at the end of `play_one`) and the UI thread blocked in `AndroidApp::native_window` (`RwLock::read_contended`). The UI thread already holds android-activity's read lock for the whole of `poll_events`, so when a writer is waiting, its own second read (a Slint timer calling `Window::size`) waits behind that writer: a deadlock. The Java main thread then blocks in `on_pause` waiting for the native thread, which is the ANR on Home.

Fix (0.29.12): `keep_screen_on` no longer calls `set_window_flags`; the Java helper `KeepOn` sets the flag on Android's UI thread. Never call `AndroidApp` methods that take the window lock from another thread.
