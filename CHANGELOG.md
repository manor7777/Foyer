# Changelog

Each release gets a section headed `## Foyer X.Y.Z`. The Release workflow publishes that section as the GitHub release notes, so write it before you run the workflow.

## Foyer 0.26.1

### ✨ Highlights

- **Android: a proper phone layout.** Held upright, the navigation moves to a bar along the bottom (Home, Search, Favourites, Alerts, More; *More* opens the full menu with your libraries). In landscape the side bar stays.
- **Settings fit small screens.** Each setting's control now sits under its text on a phone instead of running off the edge, and the descriptions no longer overlap. Options that only make sense on Windows (tray, F11, keep on top, window material, decoding method) are hidden on Android, and notifications are labelled as system notifications.
- **App icon on Android.**

### 🐞 Fixes

- Android player: the transport controls no longer flash black boxes as they fade in, they appear instantly, and tapping the video shows or hides them instead of pausing (use the play button to pause).

## Foyer 0.26.0

### ✨ Highlights

- **Foyer for Android and Android TV.** Every release now also publishes `Foyer.apk`: one app for phones, tablets, and Android TV / Google TV / Chromecast with Google TV. It has the same library, search, Seerr, notification centre and player as Windows, with hardware decoding through MediaCodec and a bundled LGPL libmpv.
- **Made for touch and for a remote.** On phones and tablets controls are bigger and easier to tap. On a TV the remote's arrows move around the screen with a visible focus ring, **Select** activates, and **Back** goes back. Every control in the Windows app can now be reached and used from the keyboard too (Tab, Enter or Space), with a focus outline.
- **Updates on Android.** *Settings → About* checks for a new version, verifies the signed checksum and opens Android's installer. Android notifications are used for new episodes, issues and replies.
- **One build, two apps.** The release builds Windows and Android together; if either fails, nothing is published.

### 🔧 Under the hood

- The code is split into a shared core (`foyer-core`: settings, playback quality rules, formatting) and a platform layer for everything that differs per system. Windows behaves as before.
- Fixed the project not building on non-Windows systems (a misplaced platform attribute).

## Foyer 0.25.1

### ✨ Highlights

- **Subtitle sync.** The subtitle menu now has a sync bar at the top: −1s, −0.1, +0.1 and +1s nudge the subtitles earlier or later, and clicking the value resets it. For downloaded subtitles that are a second or three out.

### 🐞 Fixes

- Mini player is the right way up, and the player is smoother while the controls fade (the large blurred shadow behind the control bar made every frame slow, which also made the subtitle slide look choppy).
- Picture in picture has no title bar and keeps the video's aspect ratio when resized; the shrink and PiP buttons sit at the far right of the header.

## Foyer 0.25.0

### ✨ Highlights

- **Mini player.** The new shrink button (top right of the player, next to the title) minimises the video into a small window in the corner of Foyer, so you can browse and change settings while it plays. Hover it for pause, close and expand; click it to grow it back to full size with a smooth animation. Starting something else from the mini player replaces what's playing.
- **Picture in picture.** The button beside it turns Foyer into a small, borderless, always-on-top video in the corner of your screen. Drag it anywhere; hover for pause, close and a button to go back to the full window (Esc does the same).

### 🐞 Fixes

- Subtitles now glide up smoothly when the controls appear, instead of stepping at a few frames a second.
- After a right-click hides the controls, they stay hidden for 1.5 seconds even if the mouse twitches, so you have time to move it out of the way.

## Foyer 0.24.1

### 🐞 Fixes

- Fixed the player freezing (stuck on "Loading…", no controls, audio only) when the server was slow to serve a subtitle. Subtitles now load in the background.
- Space and the arrow keys work again: the player takes keyboard focus when playback starts.
- Scroll the mouse wheel over the seek bar to scrub (about 5 seconds per notch).

## Foyer 0.24.0

### ✨ Highlights

- **Plex support.** Pick **Plex** on the sign-in page, sign in with your Plex account, and choose any server you have access to (yours or one shared with you). Foyer finds the best connection itself, on your network first. Libraries, Continue Watching, Next Up, search, genres, cast pages, resume, watched marks, favourites, intro / credits skipping, chapters and quality presets all work. On a Plex server you own, the **Server** page shows sessions, scheduled tasks, libraries and recent activity, with Scan libraries and Stop stream.
- **Jellyfin support.** Jellyfin servers sign in the same way as Emby.
- **The sign-in page works out what you pasted.** Paste an address and Foyer asks the server what it is (Emby, Jellyfin or Plex) and sets a type dropdown for you. If it guesses wrong, change it by hand.
- **Multiple servers.** Turn on **Multiple servers** in Settings, add more with **Add**, and a switcher appears above Home in the sidebar. Switching signs you in with the saved login, so there is no password to retype. Remove a server from the same Settings list.

### 🎬 Player

- Embedded text subtitles are now fetched from the server and shown instead of mpv's own copies, which could freeze playback on remote files. Subtitles the file marks as default or forced are selected automatically, and the subtitle key steps through the available tracks.
- Screenshot button, with a confirmation in the top-right (also kept in the notification centre).
- Transport controls reordered: previous, back, play, forward, next.
- Subtitles slide up above the controls while they are showing, and right-click hides the controls.
- A vertical volume pop-up on the mute button when the player window is narrow.

### 🔧 Other

- Seerr on a Plex server: use "Plex account (current sign-in)" instead of typing a user name and password (Overseerr only).
- A server you can't sign back in to now shows a message on the sign-in page instead of doing nothing.
- On Plex, the controls that can't work there are hidden: Restart / Shut down, and the remote-control buttons on other users' sessions (Plex only lets you stop a stream).
- Not on Plex: Live TV and Emby's dashboard remote control for this device.

## Foyer 0.23.2

- Fixed: remote Rewind / Fast-forward from the server dashboard could freeze or crash Foyer, and the next command (such as a message) with it.

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
