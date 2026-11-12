<div align="center">

<img src="crates/app/assets/app.png" width="112" alt="Foyer logo" />

# Foyer

**A fast, native media client for your Emby server. Glassy WinUI-style interface, mpv playback, no web view.**

[![Release](https://img.shields.io/github/v/release/manor7777/Foyer?style=flat-square&color=2bb3ff)](https://github.com/manor7777/Foyer/releases/latest)
[![Platform](https://img.shields.io/badge/platform-Windows%2010%20%7C%2011-0a84ff?style=flat-square)](#install)
[![Built with Rust](https://img.shields.io/badge/built%20with-Rust-b7410e?style=flat-square)](https://www.rust-lang.org/)
[![UI: Slint](https://img.shields.io/badge/UI-Slint-35a7ff?style=flat-square)](https://slint.dev/)
[![License: MIT](https://img.shields.io/badge/license-MIT-green?style=flat-square)](LICENSE)

</div>

Foyer is a from-scratch replacement for the old Windows desktop theatre apps: one small `.exe`, a modern Fluent look with Mica/Acrylic glass, and playback by [mpv](https://mpv.io/) that handles just about any file you throw at it. It talks to your Emby server's REST API directly (Emby only; Jellyfin and Plex aren't supported). There is no embedded browser and no hosted web page involved.

> Foyer is an independent, unofficial client. It is not made by, or affiliated with, Emby or Emby LLC. "Emby" is a trademark of its owner.

## Highlights

| | |
|---|---|
| **Playback that just works** | libmpv with hardware decoding, a big read-ahead cache, direct play by default, and optional transcoding presets (Auto, 20 Mbps … 720p). Audio/subtitle menus, subtitle search through your Emby server, audio-delay nudge, always-on-top. |
| **Skip intro, play next** | A *Skip intro* button appears when your Emby server has marked an intro (press **Space**), with an optional auto-skip. Near the end of an episode: *Keep watching* or *Play next* (**Space** = next). |
| **Looks like it belongs on Windows 11** | Fluent controls, Mica / Acrylic / solid backdrops, a floating glass sidebar that collapses smoothly, and an accent colour that follows Windows or your own pick. |
| **Home that you control** | Continue Watching, Next Up (the episode *after* the one you're on), My Media, Latest, and your requests. Hide any row, reorder them. |
| **Live TV guide** | A proper time-grid guide with now-playing highlights. |
| **Requests built in** | Native [Overseerr / Jellyseerr](https://docs.overseerr.dev/) screens: discover, search, request (with season picking), request status at a glance, issues, an admin view with user limits and permissions. No API key, just sign in. Hidden until you switch it on. |
| **Notifications** | New episodes the moment the *first one or two* are available (Seerr only tells you about whole seasons), new issues, replies, and Windows toasts. |
| **Remote control** | Shows up in your Emby server's dashboard as a controllable player: pause, stop, send messages. |
| **Self-updating, safely** | Updates are checked on start and installed automatically. Every release is signed (ECDSA P-256); an unsigned or tampered download is refused. |
| **Private by design** | Debug logging is off by default and never records passwords, tokens or cookies. |

## Install

1. Download **`Foyer.exe`** from the [latest release](https://github.com/manor7777/Foyer/releases/latest).
2. Put it anywhere you can write to (not Program Files) and run it.
3. First launch downloads the mpv video engine once (about 30 MB, from the LGPL builds at [zhongfly/mpv-winbuild](https://github.com/zhongfly/mpv-winbuild)). If you'd rather bring your own, drop `libmpv-2.dll` (or `mpv-1.dll`) next to `Foyer.exe`.
4. Enter your Emby server address, sign in, and you're in. Use **Find servers** to discover one on your network. Needs an Emby server (4.7 or later for intro markers).

Requires Windows 10 or 11 and a working GPU driver.

## Keys

| Key | Action |
|---|---|
| **Space** | Pause, or press the on-screen *Skip intro* / *Play next* when it shows |
| **←** / **→** | Back 10 s / forward 30 s |
| **↑** / **↓** | Volume |
| **M** | Mute |
| **A** / **J** | Cycle audio / subtitles |
| **F11**, double-click | Full screen |
| **Esc**, **Backspace** | Back / stop |

## Settings worth knowing

- **Playback → Decoding method.** *Automatic* picks a method that draws cleanly on your graphics hardware. If a particular GPU gives a noisy picture, try another entry in the list.
- **Playback → Skip intros automatically.** Uses the intro markers your Emby server already has.
- **Advanced → Extensive debug logging.** Off by default. Writes `%APPDATA%\Foyer\logs`; tokens and passwords are redacted.
- **About → Updates.** Check now, or turn automatic checks and installs off.

## Build from source

```powershell
git clone https://github.com/manor7777/Foyer.git
cd Foyer
cargo build --release -p foyer
```

You need a recent stable Rust toolchain on Windows. The binary is `target/release/foyer.exe`. The workspace has four crates:

| Crate | What it is |
|---|---|
| `crates/app` | The application and the [Slint](https://slint.dev/) UI |
| `crates/emby-api` | Typed client for the Emby server's REST API and remote-control websocket |
| `crates/seerr-api` | Typed client for Overseerr / Jellyseerr |
| `crates/player` | libmpv integration (render API into the window's OpenGL surface) |

## Releasing (maintainers)

Releases are cut by hand: bump `version` in `Cargo.toml`, push, then run **Actions → Release → Run workflow**. The job tests, builds, signs the checksum with the repository secret `UPDATE_SIGNING_KEY` (an ECDSA P-256 private key in PEM form) and publishes `Foyer.exe`, `Foyer.exe.sha256` and `Foyer.exe.sha256.sig`. The matching public key is built into the app.

## Licence

[MIT](LICENSE). Playback is by [mpv](https://mpv.io/) (LGPL build, downloaded at runtime and not redistributed here).
