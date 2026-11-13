# Changelog

Each release gets a section headed `## Foyer X.Y.Z`. The Release workflow publishes that section as the GitHub release notes, so write it before you run the workflow.

## Foyer 0.23.1

- Fixed: other users' sessions on the Server page could no longer be remote controlled. Foyer now identifies to the server as "Emby Theater" again, as it did before 0.21.0.
- Posters: a smaller minimum size, plus an optional auto-size setting for small windows. With auto-size on, the size dropdown acts as a maximum.

## Foyer 0.23.0

- **Favourites** has its own sidebar button (your hearted items, with TV / Movies / Other tabs and search). Playlists now shows only the server's playlists.
- Seek bar: click and drag to scrub, and chapter markers show as ticks.
- Back button lines up with the top of the sidebar; library pages no longer leave a big gap under the title.
- Seerr: you're also notified of replies on issues you've commented on, not only ones you reported.
- Hide spoilers now also hides the summary on an unwatched episode's own page.

## Foyer 0.22.0

> **One combined changelog: everything since Foyer 0.12.0.** 0.12.0 shipped without release notes, so this covers every release from 0.12.0 up to and including 0.22.0. Later releases get their own notes.

### ✨ Highlights

- **Foyer is now a proper Windows media app.** Media keys, the Windows media flyout and remote apps all control playback, and the screen stays awake while a video plays.
- **Browse like you mean it.** Library filters stay put while you scroll, every library has its own search, genres are clickable, and TV shows finally have a Play button.
- **Pictures stop reloading.** Posters are cached on disk, so lists feel instant but the content is always fresh.
- **A real admin "Now playing" view.** See who is watching what, with transport controls, seeking and messages, like the Emby dashboard.

### 🎬 Playback

- Fade transitions and tidier, aligned transport controls with spaced-out time text.
- Previous / next episode buttons that cross seasons.
- Rewind and skip-forward seconds are configurable in Settings.
- "Still watching?" prompt after 5 autoplayed episodes in a row.
- With autoplay off, the player stays on the last frame with **Play next** / **Keep watching** buttons.
- Cursor hides along with the controls; leaving playback leaves full screen.
- **Your place is saved from 5 seconds in** (it used to take 10), and it is saved to the server so it follows you to other devices.
- Windows media controls: the system flyout, keyboard media keys and remote apps.
- The display no longer goes to sleep while you're watching.
- Foyer now identifies itself to your server as **Foyer** in the dashboard.

### 📺 TV shows & movies

- **Play / Resume button on series pages**: continues the episode in progress, otherwise the first unwatched.
- **Play button on season pages**, same logic.
- Seasons now sit **above** cast & crew.
- Seasons without a poster use the series poster.
- A series with a single season skips the season "folder" and lists its episodes directly.
- Season cards open their own season pages.
- **Episode descriptions**: rows read `S01E02 - Name` with the synopsis beneath.
- **Hide spoilers** setting blanks the description of episodes you haven't watched.
- **Clickable genres**: click Comedy or Animation to see everything in that genre.
- The episode logo and "Series S1E2" subtitle link back to the series page.
- Hide any title from every list (undo in Settings), and remove items from Continue Watching.
- Hidden titles are also dropped from *More like this*.

### 🔎 Libraries & search

- **Sticky title and filter bar** in Movies / TV Shows, so you can always sort and filter.
- **Search inside a library**, independent of the global search.
- Favourites search, with no more 300-item cap.
- Playlists and favourites get **Mine / Server** tabs, and favourites split into TV and Movies.

### 🏠 Home

- **Recently added** shows what reached your server lately, whatever its age.
- **Recently released** (new) sorts by release or air date.
- Hide and reorder every home row.

### 🖼 Look & feel

- **Image cache**: posters and artwork are kept on disk, so they don't reload every visit. Lists are always fetched fresh, so a new show's poster loads and nothing else does.
- **Skeleton placeholders** pulse where a poster will be, then the image fades in.
- **Back button** left the sidebar. It slides in at the top of any page with somewhere to go back to. Mouse back and keyboard Back work everywhere.
- Adjustable poster size and remembered window size and position.
- Fixed an unreadable media-info panel in glass mode.
- Optional banner art on hover.

### 🎟 Requests (Seerr)

- Native Seerr screens: discover, search, request, issues, and an admin view.
- **Report a problem** from any movie, series, season or episode page.
- Seerr cast pages everywhere: click any actor, with search and filter.
- "View all" rows, a full **watchlist**, and a watchlist button on titles.
- Hover a title for its blurb and a one-click Request button; your remaining request quota is shown.
- Seerr's real reason is shown when a request is refused (for example quota).
- **New setting**: automatically remove a title from your watchlist once you've watched it.
- Fixed watchlist removal.

### 🔔 Notifications

- A notification centre that logs everything Foyer announces.
- **Follow a series** with the bell to hear about new episodes.
- Tray icon, with an option to close to the tray.
- **Server update alerts**: admins are told when a newer Emby Server is available.

### 🛠 Admin

- **Now playing** replaces the sessions list: only people who are actually watching, with a thumbnail, what they're watching, a progress bar you can click to seek, previous / rewind / play-pause / fast-forward / next / stop, and the send-message box.

### 🧰 Under the hood

- Fixed a deadlock when opening a title in your library from Seerr.
- Assorted small fixes and polish across all of the above.
- Android and Android TV plan added to `docs/`.

### Where it started: Foyer 0.12.0

The first public build: a native Rust and Slint client for Emby with mpv playback, library browsing, a Live TV guide, remote control, Seerr requests, skip intro / play next, notifications and signed self-updates.
