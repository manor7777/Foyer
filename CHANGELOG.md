# Changelog

Each release gets a section headed `## Foyer X.Y.Z`. The Release workflow publishes that section as the GitHub release notes, so write it before you run the workflow.

## Foyer 0.29.15

**Windows: updates restart properly**

- **Fixed: "Restart now" closing Foyer and never reopening it (Windows).** After an update Foyer starts the new copy and then closes itself, and the new copy waits for the old one to let go of the "only one Foyer" port. The old copy could take far longer than the 15 seconds the new one was willing to wait (it was still tidying up background network work), so the new copy gave up and quit, and opening Foyer from the Start menu did nothing until the old one finally ended. Foyer now always ends within about three seconds of its window closing, and the new copy waits up to 30 seconds. Closing Foyer in the normal way is quicker for the same reason.
- **Automatic updates now restart by themselves (Windows).** With *Install updates automatically* on, once an update is installed Foyer restarts into it as soon as nothing is playing or downloading (it checks every 20 seconds). If Foyer was hidden in the tray, the new copy stays hidden in the tray; if its window was open, it comes back open. A copy that has just restarted does not install another update on that start, so a failed swap cannot loop. Updating with the button still shows *Restart now* instead.
- A few lines in the debug log (`shutdown:` and `update:`) show how a close or restart went, if it ever misbehaves again.
- Android is unchanged (its updates still open the download in your browser).

## Foyer 0.29.14

**Android: updates download in your browser**

- **Android updates now open the download in your browser.** Settings > About still checks for a new version (and, if switched on, looks when Foyer opens and sends an "update available" notification). When one is found the button reads *Download update*: it opens the APK's link in your browser, and when the download finishes you open the file and tap Install. Android's own installer does the rest and refuses anything not signed with Foyer's key. This replaces the 0.29.9 to 0.29.13 download inside the app (Android's DownloadManager), which was hard to follow.
- **Why not install inside the app.** An app that declares the install permission or installs packages is blocked as harmful by Google Play Protect (a build with only that permission removed scanned clean; the same build with it did not). Foyer has neither, and now no longer downloads the file itself either.
- Windows is unchanged: it still downloads, checks the signed checksum and replaces `Foyer.exe` for you.
- No new permissions, no new libraries.

## Foyer 0.29.13

**Double-tap to skip, and the Android freeze is fixed**

- **Double-tap to skip (phones and tablets).** Double-tap the left half of the picture to jump back, or the right half to jump forward, by the steps set in Settings > Playback (the same ones as the skip buttons and the media keys). A round marker shows the jump on the side you tapped. A single tap still shows or hides the controls; it now waits a moment (about a quarter of a second) to tell the two apart. Windows and TV are unchanged.
- **Fixed: the app freezing on a black screen after a film ends or you leave the player (Android).** It could then show "Foyer isn't responding". When the player let the screen sleep again it called into Android's native-activity glue from the player thread, which can deadlock with the UI thread. The screen-on flag is now set by a small Java helper on Android's own UI thread. (This is the fix tested in 0.29.12.)
- **mpv's shutdown now has a 3-second limit**, so a stuck player can never keep the screen waiting, and closing the player writes a few lines to the debug log.
- **The mpv log is cleaned of access tokens at start-up**, so a hang or crash during playback cannot leave one in `mpv.log`.

## Foyer 0.29.12

**Android: fixes the freeze when a film ends or you leave the player**

- **Found and fixed the freeze.** After a film ended or you pressed back, the app could stay on a black screen and then show "Foyer isn't responding". When the player let the screen sleep again it called into Android's native-activity glue from the player thread, and that call can deadlock with the UI thread (a lock taken twice, once while waiting for a writer). The screen-on flag is now set by a small Java helper on Android's own UI thread instead, so the two can no longer wait on each other.
- Keeps the 0.29.11 safety net (mpv gets 3 seconds to shut down) and the extra debug-log lines for closing the player.

## Foyer 0.29.11

**Android: diagnostic build for a freeze when leaving the player**

- **Closing the player can no longer wait forever on mpv.** Some Android phones froze on a black screen after skipping near the end of a film and pressing back (then "Foyer isn't responding"). Foyer now gives mpv 3 seconds to shut down and, if it has not finished, lets go of it so the player closes. This is a safety net: the cause of the hang itself is still being tracked down.
- **More detail in the debug log** (Settings > Advanced): the steps of closing the player are now logged (`player:`, `play:` and `ui:` lines), so a freeze shows where it stopped.
- **The mpv log is cleaned of access tokens at start-up**, so a hang or crash during playback cannot leave one in `mpv.log`.
- Windows is unchanged apart from the same 3-second limit on mpv's shutdown.

## Foyer 0.29.10

**Android: updates without the install permission**

- **Google Play Protect no longer blocks Foyer.** It was flagging every build that carried the "install apps" permission as a harmful app, even builds it had accepted the day before. Foyer no longer asks for that permission and no longer installs packages itself.
- **Updating still happens in the app.** Settings > About > Install update downloads the update with Android's own download service (progress shows in a notification), checks it against the release's signed checksum, and then the "Foyer update" notification opens Android's standard install screen. Android still refuses anything not signed with Foyer's key. Downloaded updates are saved in Downloads/Foyer.

## Foyer 0.29.8

**Android: in-app updates are back**

- **Install update** installs inside Foyer again: it downloads the update, checks the signed checksum, and shows Android's standard install screen, with no browser. 0.29.7 had replaced this with a browser download while chasing a Play Protect block; the permission it needs (`REQUEST_INSTALL_PACKAGES`) is back, as in 0.29.0.

## Foyer 0.29.7

**Android: Google Play Protect (again)**

- Play Protect blocked Foyer as a "harmful app" again. Foyer no longer asks for Android's permission to install apps, and no longer installs its own updates: **Install update** now checks the signed checksum, then opens the download in your browser, and you open the downloaded file to install it. Android still refuses anything not signed with Foyer's key. The PackageInstaller helper is removed from the APK.
- Windows is unchanged (it still updates itself).

## Foyer 0.29.6

**Windows and Android**

- The subtitle menu no longer closes when you press the sync - / + buttons or Reset, so you can nudge the timing several times in a row. It still closes when you pick a track, open the subtitle search, or click outside it.
- Clicking an alert during playback no longer leaves the poster of what you were watching as the background of the page it opens (Alerts, Settings, a title).
- The subtitle slide and the controls' fade are smooth again (they were limited to the video's frame rate while the controls were up). Subtitles also move in finer steps where the player supports it.

## Foyer 0.29.5

**Windows and Android**

- Read alerts are readable again: they used a see-through fill that let a bright backdrop wash out the white text. They now use the same solid glass as unread ones, told apart by the accent border, the dot and a bolder title.

## Foyer 0.29.4

**Windows**

- Fixed the mouse pointer staying hidden in fullscreen with "Keep player on top" (the pin) enabled: the pointer is now put back explicitly when the controls return.
- Fixed a subtitle staying stranded in the middle of the screen when you go fullscreen while one is showing and the controls are up. It now moves to the right place straight away instead of waiting for the controls to fade and the next subtitle.
- New **Clock size** setting (Settings > Playback): Small, Medium, Large or Extra large. It applies only when the clock is always shown and the controls are hidden; beside the controls the clock keeps its size.
- Subtitles now slide smoothly when the controls hide or show, instead of stepping about four times a second.

## Foyer 0.29.3

**Manage alerts, and glass buttons everywhere (Windows and Android)**

- New **Manage alerts** button on the Alerts page opens a new **Alerts** tab in Settings. It lists every series you follow, each with a **Stop alerts** button, so a "New episodes of ..." alert can be stopped for good. The alert switches (including the system notification one) now live there too, and no longer depend on Requests being enabled.
- Every alert on the Alerts page now has an X to delete it.
- Settings buttons, tabs, setting groups, dropdowns and text boxes now use the same liquid glass as the bottom bar and sidebar: pill shapes, a lit rim and a soft shadow. Secondary pill buttons elsewhere use the raised glass too.

## Foyer 0.29.2

**Android TV and Chromecast with Google TV**

- The screen no longer falls asleep during playback: the keep-awake setting is re-applied every 15 seconds and kept on while moving to the next episode.
- Moving to the next episode no longer waits up to 8 seconds on a slow server before it starts.

## Foyer 0.29.1

**Download fixes**

- Cancel now works straight away, even while a slow server (an rclone or NAS mount) is still sending nothing.
- A download cut off by a dropped connection picks up where it stopped instead of starting over.
- Removing a failed or interrupted offline download now deletes its partial video too, so it no longer takes up space.
- Cancelling just as a download finished could make the next download of that item cancel itself; fixed.
- Saved file names that Windows rejects (trailing dots, names such as CON or NUL, empty titles) are now made safe.

**Casting**

- A malformed message from a Chromecast can no longer upset the connection.

## Foyer 0.29.0

**New look: liquid-glass navigation**

- **Phones and tablets:** the bottom bar is now a floating glass pill, with "More" as a separate round glass button beside it. The selected tab sits in a raised glass lens that stretches like liquid as it slides to the tab you pick.
- **Desktop and TV:** the sidebar is a rounded glass panel with the selected page as an accent-coloured capsule, in light and dark mode.

**Android: Google Play Protect**

- **No more "harmful app" block.** Play Protect was flagging Foyer when it was installed or updated. Two things changed: the media-controls helper (notification shade, lock screen and headset buttons) is now a normal `classes.dex` inside the APK instead of code loaded at runtime, and in-app updates are handed to Android through the standard PackageInstaller session instead of saving a file to Downloads and opening it. Updates are still checked against the signed checksum first, and Android refuses anything not signed with Foyer's key. The previous update file is cleaned up afterwards.

**Android TV / Google TV**

- **Opening a film, series or episode puts the remote on Play (or Resume)**, so Select starts it straight away.
- **The focused button is visible on the accent-coloured Play button** (it used to be ringed in the same colour as the button).
- **Up / Down in a grid goes to the item directly below** (and Left / Right to the one beside it) rather than to something nearby but off to the side. Up / Down no longer land on a control in the same row, such as a card's own corner button.

## Foyer 0.28.14

- **Android TV / Google TV and phones: the screen no longer sleeps during a video.** A Chromecast with Google TV went to its screensaver part-way through playback, and a phone could dim and lock. Foyer now keeps the screen on while a video is playing (as the Windows version already did); when you pause or the video ends, the screensaver and sleep work as normal.

## Foyer 0.28.13

**Android TV / Google TV**

- **Smoother video on TV sticks.** Video played at a few frames a second on a Chromecast with Google TV: every decoded frame was copied back to memory before being drawn, which that chip is very slow at. Android now uses zero-copy hardware decoding when the device supports it, and falls back to the old way when it doesn't. The player also reads ahead less (150 MB instead of 1 GB), as TV sticks have little memory.
- **Settings → Playback → "Decoding method" now shows on Android**: Automatic, Copy (compatible) or Zero-copy (fastest). If the picture glitches, try Copy.
- **Scrolling with the remote keeps headings in view.** Moving down to a row scrolled it flush to the top edge and cut off its heading; the page now leaves room above it, and a row keeps a bit of the next card showing at the side.

## Foyer 0.28.12

**Android TV / Google TV: the remote works**

- **The D-pad now moves to the nearest control in the direction you press**, like other TV apps: up and down between rows, left and right along a row (the row scrolls with you), and Select opens. It used to step through every control one at a time, and stopped working altogether after signing in because nothing kept the focus.
- **The menu opens with its labels while you're in it.** Foyer starts on it; press Right to go into the page, Left from the start of a row to come back. Back on a page goes to the menu, and Back on the menu leaves the app.
- **Player controls by remote.** Left / Right skip back and forward, Select pauses, and Up or Down brings up the controls with the focus on play / pause. From there the arrows move between all the buttons (previous / next episode, quality, audio, subtitles, cast), the menus they open work with the remote, and Back hides the controls. Desktop-only buttons (keep on top, full screen) are hidden on TV and phones.
- **Everything can be reached with the remote**, including the Requests pages (tabs, search results, requests, issues, users), episode lists, genres, the series link on an episode, notifications, search history, Favourites tabs, live TV channels, settings tabs, accent swatches, the cast picker and casting bar, skip intro / keep watching, subtitle search results and the audio sync buttons. Drop-down lists and the Options menu work with the arrows, and Back closes them.
- **Dialogs keep the remote inside them** (cast picker, request and report dialogs, confirmations, subtitle search), and Back closes them.
- Pressing Select on an empty sign-in field brings the keyboard up instead of trying to sign in with a blank password.

**Android**

- **Backing out of the home screen no longer breaks the app.** It now goes to the background like Home does. If Android does close it, the next launch starts fresh; before, it could not be opened again without a force stop.

## Foyer 0.28.11

- **Poster-hover backdrop no longer flashes.** When one backdrop replaced another, both layers faded at once and the page showed through for a moment, like a flash of white. The old backdrop now stays put underneath while the new one fades in over it.

## Foyer 0.28.10

**Casting grows real controls**

- **Tap the "Casting to …" bar** to open a controls sheet: a seek bar with the time, back / forward (your skip times), pause / play, stop, and, for episodes, **previous / next episode** on the same TV without restarting its player. The sheet also has **Audio**, **Subtitles** and **Quality** pickers. (The seek, track and quality controls work on Cast devices; other Foyer / Emby apps signed in to your server keep pause and stop.)
- **Subtitles now show on the TV.** They were never sent before. The one chosen on the title's page (else your remembered language, else the file's default) is burned into the picture by the server, so every subtitle type works on any Cast device. Changing audio, subtitles or quality restarts the picture for a moment at the same spot.
- **Quality starts at "Original (no transcoding)".** The file goes to the TV untouched, and the server transcodes only when it has to: when a subtitle is on (it is burned in) or when you pick an audio track other than the file's default. If the TV can't play the original, Foyer switches to a 1080p transcode by itself and says so. Pick a lower quality by hand for a slow connection.
- The casting bar no longer sits on top of the bottom navigation (it ignored the system gesture area on phones, which is most visible on a folding phone), and it clears by itself when the show ends or is stopped on the TV.
- Changing the audio track or subtitles no longer makes Foyer think the cast ended.
- Android: the phone now presents itself to the server as "Emby Theater" instead of "Android device", because the dashboard offered remote-control buttons for the Windows build but not for the phone.

**Downloads**

- **"Download as file" no longer reports a 0 KB file as finished.** A server that serves files from slow remote storage (rclone, a NAS) can answer straight away with an empty body while it is still fetching the file; Foyer took that as a finished download. It now checks the size against what the server says the file should be, waits and retries up to three times, and only then fails with a clear message. This covers offline copies too.
- **New setting: Settings → Advanced → Downloads → "Save downloaded files to"** chooses the folder "Download as file" saves to (empty keeps Downloads\Foyer). Windows and desktop only; Android keeps saving to its Downloads folder, and offline copies always stay inside Foyer.
- The Downloads page's progress bar was drawn from the middle instead of the left edge; it now fills from the left.
- The Downloads item in the sidebar shows how many downloads are still running (finished ones are not counted), like the Alerts badge.

## Foyer 0.28.8

- Casting to a Cast device now shows the title, a backdrop picture and "Casting from Foyer" on the TV. (A Foyer logo on the receiver itself would need Foyer registered as a Cast app with Google; this is the most the standard receiver allows.)
- Casting to a Cast device or TV now shows up on the server: Foyer reports the playback (start, progress every few seconds, pause, stop) to Emby/Jellyfin/Plex, so the session appears under Now Playing with the right user, and your position is remembered. The TV still streams straight from the server; the phone only controls it.

## Foyer 0.28.9

- Casting to a Cast device now plays the audio track chosen on the title's page (else English, else the file's default) instead of the file's default track.
- The server dashboard's Pause / Stop / seek buttons now control a Cast device while Foyer is casting to it (they did nothing before).
- On Android the server now lists the device as "Android device" instead of "Windows PC".

## Foyer 0.28.7

- Casting: Google Cast devices (Chromecasts, and TVs with Google Cast built in such as some Samsung models) were not being found on some phones even though Emby's own cast menu listed them. Foyer now searches with a proper mDNS client that joins the multicast group on every network adapter, which is how Chromecasts announce themselves. The previous search sent one question and listened for a direct reply, which many Chromecasts never send. What the search finds (or that it found none) is now written to the log, so a phone that still shows no Cast device can say why.

## Foyer 0.28.6

- **The poster-hover backdrop no longer flashes.** Moving the pointer quickly across posters used to flash a different full-screen backdrop for each one, dropping to the plain background in between. A backdrop now appears only once the pointer has rested on a poster for a moment, stays up briefly when you move to the next poster, and cross-fades into the next one instead of cutting. Only affects devices with a mouse; Android touch has no hover.

## Foyer 0.28.5

- **Casting shows everything you can cast to.** The list now shows apps signed in to your server together with standard Google Cast devices (including older Chromecasts that have no apps of their own), each as soon as it answers. Server apps are listed with the app and the user signed in on it (for example "Emby for Android · Mandy"), so an administrator can see who a cast would go to. The search for Cast devices now asks on every network adapter (a VPN or virtual adapter could send the question to the wrong network and hear nothing), and on Android it holds the Wi-Fi multicast lock, without which the phone discards the answers. I could not test this against a real Chromecast here, so please say if none turn up.
- Casting list: the panel is centred (it sat against the left edge), and the list scrolls when there are more devices than fit, with the Cancel button always on screen. The "Still watching?" and server-message dialogs had the same left-edge problem and are centred too. On phones the player's clock no longer sits underneath the cast button.
- Android: **video no longer looks like static**, and the interface is no longer garbled while a video plays. Text drawn after the video had started (the clock, the cast list) came out with black boxes around the letters, and the pause/play button turned into a white square. Found with a phone connected over `adb`: the video decoder's drawing leaves graphics settings behind that the interface drawing trips over, and the interface leaves settings that corrupt the next video frame. Foyer now resets to the defaults before the video is drawn and puts the interface's settings back afterwards. Windows is unchanged.
- Android: following the system font size (0.28.3) shrank the whole interface to about 80% on phones with a larger font setting, leaving black bands at the right and bottom. It now makes the text bigger and leaves the interface alone.

## Foyer 0.28.3

- Android TV: the D-pad can now leave a text field. Up and Down inside the sign-in boxes (and any other text box) move to the previous or next control instead of moving the text cursor, so you can get from the server address to the user name, password and Sign in button.
- Android: search now works with the on-screen keyboard. The keyboard keeps the word you are typing "in progress" until it is dismissed, so pressing Search or Enter searched the text without that last word, or nothing at all. Searching now hides the keyboard first, which finalises the word, and then searches what the box holds.
- Android: Foyer follows the system font size (Settings > Display > Font size). Because the interface can't scale text on its own, the whole interface scales with it.
- Android: the "Loading…" message in the player is centred on the visible screen.
- The timing log used while tuning picture-in-picture is now only written when debug logging is turned on in Settings > Advanced.

## Foyer 0.28.2

- Android: the APK now also contains a 32-bit ARM build (and the 32-bit video engine). Many Android TVs and set-top boxes run a 32-bit-only system even on a recent Android TV version, and refused to install Foyer with "This app isn't compatible with your TV". The one APK now installs on both 64-bit and 32-bit devices; it is larger because it carries both. If your TV still says it isn't compatible, tell me what `getprop ro.product.cpu.abilist` shows for it.

## Foyer 0.28.1

- Fixed the mouse pointer and player controls hiding while the cast device picker was open on Windows, which made it impossible to pick a device.
- Picture-in-picture (Windows): resizing is smooth now. Foyer follows the mouse and resizes and moves the window in one step instead of letting Windows resize it and correcting the shape afterwards. The window also remembers the size you made it for the rest of the session, so going out and back into picture-in-picture keeps it.
- Picture-in-picture and the in-app mini player: the buttons no longer fade away as the pointer moves onto them, so they are easy to click.
- Mini player: fixed the video freezing unless the mouse was moving over it. Found from a screen recording: the rounded, clipped mini player is cached as a layer, and the video frames are drawn into it without anything telling the cache they changed, so it only refreshed when hovering changed something. Every new video frame now marks it as changed. mpv also no longer waits inside its draw call for each frame's display time in the mini player, which was stalling the interface.
- Picture-in-picture resize: a recording showed the window growing before its picture had been redrawn (a grey band appeared where the new area was). Foyer now waits for the last resize to be drawn before applying the next one.
- Picture-in-picture resize is much smoother: timing showed mpv taking about 30 ms to redraw the video at every new window size, which capped resizing at about 20 steps a second. While you drag, the video now goes through a fixed-size picture that Windows just scales, and switches back to direct drawing when you let go.
- Mini player: going from full screen to the mini player no longer jumps. Building the pages behind it takes about 140 ms, and the shrink animation used to run through that freeze; it now starts once they are ready.
- Android: an attempt at the screen glitch where the player and the cast picker drew garbled icons and text. Video drawing now resets the graphics state it leaves behind. This was found from screenshots only and could not be reproduced here, so please say if it still happens (a wireless-debugging log would then pin it down).

## Foyer 0.28.0

- Fixed being signed out when a file on the server changes while you are watching it. If the title can no longer be found when you leave the player, Foyer now shows a short message at the bottom of the window and takes you to Home, still signed in. Only a genuinely expired sign-in goes back to the login page.
- Settings are split into tabs (Playback, Library, Appearance, Requests, Accounts, Advanced, About) and there is a search box at the top that finds settings by name or description, forgiving small slips.
- Opening the app no longer flashes the login page before Home when you are already signed in.
- The sidebar now highlights the library you are in (for example TV Shows) as well as the other sections.
- New setting, **Only show the time with the controls**: hides the clock while you watch and shows it only when the player controls are visible (paused or hovered).
- **Downloads** (Emby and Jellyfin, for users with "Allow media downloading" on the server). A title's Options menu and the press-and-hold / right-click menu on a poster now offer **Download for offline play** (kept inside Foyer, plays with no server) and **Download as file** (saved to Downloads/Foyer). On a season or series they download every episode. The new **Downloads** page in the sidebar shows progress and lets you cancel, play, remove or (on Windows) open the folder. You get a notification when a download finishes or fails (one for the whole batch for a season or series).
- **Offline mode.** If the server can't be reached and you have offline copies, Foyer stays signed in, shows an "You're offline" banner with **Try again**, greys out the sidebar sections that need the server, and shows your downloads, which play normally. Playback progress isn't sent to the server while offline. Titles with an offline copy play from disk even when you are online.
- **Casting.** A cast button on movie and episode pages and in the player's top bar opens a device list: **Chromecasts** on your network, and **other apps signed in to your server** (Emby / Jellyfin TVs, phones, browsers). Casting from the player continues from where you are and stops playback here. While casting, a bar at the bottom shows the device with pause/play and stop. Chromecasts get a 1080p H.264 stream from your server, so the server must be reachable from the Chromecast. This could not be tried against a real Chromecast here, so please report anything odd. On Android, some phones only answer Chromecast discovery when the Wi-Fi allows multicast; the server-app list always works.
- Windows mini player: smoother. The video is no longer redrawn when only the interface changes (dragging, hovering), and the graphics setup is done once instead of every frame. This could not be profiled with a real video here, so please say if it is still stuttery.

## Foyer 0.27.1

- Requesting a series: no season is ticked for you any more, and if your account has a series request limit the request window shows how many seasons you have left. You can no longer tick more seasons than that, which stops requests failing when you picked more than you were allowed.
- Android: the navigation bar, and the taskbar on Samsung foldables, hide while a video plays so they no longer sit over the controls. Swipe from the edge to bring them back for a moment.
- Debug logging: **Save copy to…** (Windows) lets you pick a folder to copy the log files to, and **Save to Downloads** (Android) copies them to the Downloads/Foyer folder on the phone, so you can find and send them without a computer.

## Foyer 0.27.0

- Rich notifications. New episodes, requests that are ready and updates now come with the poster, and clicking them takes you to the right place.
  - **Windows:** modern toasts with the poster beside the text and working buttons: **Play** and **Open in library** for a film or episode, **Open settings** for an update, **View issue** for an issue. Clicking the toast itself opens the title. Buttons work while Foyer is running (it stays in the tray); a toast clicked after Foyer has quit just opens nothing.
  - **Android:** the notification shows the poster, and tapping it opens Foyer on that title (or Settings for an update, or the issue). Android lets Foyer notice a tap while it is running in the background by watching the notification disappear as the app comes to the front, so a tap is picked up within about a second. No action buttons on Android yet.
  - **In the app:** the banner shows the poster with Play and Open buttons, and the Alerts page shows a poster beside each alert that has one. Episodes use their series' poster.
- Going back to a page now glides to where you were instead of jumping there.

## Foyer 0.26.12

Hotfix.

- Fixed Report a problem freezing the app (Windows) or showing "not responding" (Android) when opened from Options on a library series, season or episode page. The dialog was waiting on a lock it already held. Opening it from a Seerr title page was unaffected.

## Foyer 0.26.11

- Going back (Back button, gesture or the on-screen arrow) returns you to where you were on the page, instead of the top. This works on every page, including lists and detail pages that load their content a moment after opening. If you start scrolling first, Foyer leaves it alone.
- Requests: the Request button no longer shows on a title that is already requested, in progress or available. A series still shows it while it has a season nobody has requested.

## Foyer 0.26.10

- Report a problem (both platforms): clearly labelled drop-downs. Problem is multiple choice (Video, Audio, Subtitles, Other), Season is a drop-down of the show's real seasons, and Episodes is a multiple-choice list with Select all / Clear, so several bad episodes need one report, not one each. Seerr issues hold a single problem type and episode, so Foyer files one issue per ticked problem and episode and tells you how many it sent. The dialog is also centred on narrow screens.
- Detail pages: Notify me of new episodes, Hide, Remove from Continue Watching and Report a problem now live in one "Options" menu instead of a stack of buttons, which saves a lot of room on phones and foldables.
- Slimmer detail pages: Resume / From beginning (or Play) come first with the Options button, watched, favourite and info icons beside them (on a second row when narrow), then Audio and Subtitles as two slim boxes side by side ("Audio: English", "Subs: English").
- New setting, off by default (Settings > Library): Description behind an info icon. Titles then show no description on the page; the ⓘ button opens it. Media info (codecs and streams) now has its own wrench button, so the two never share a panel.
- Foldables, landscape and wide windows: fixed the description, buttons and Play row overlapping the Seasons and Cast & crew rows. The header's text column had a fixed height, so it could not grow; it now sizes to its content.

## Foyer 0.26.9

- Detail pages (movie, episode, series, season): fixed the description and the buttons sliding under the Cast & crew row on foldables and other mid-width screens. The header now measures its wrapped text (title, details, description) at its real width instead of assuming one line.
- Long descriptions collapse to five lines with a Read more / Show less toggle, so the header stays a sensible height however much text the server has.
- Requests tabs: on phones and tablets the tab row is now a drop-down (Discover, Requests, Watchlist, Issues, Users), since a sideways-scrolling row hid the tabs that were off screen. Desktop keeps the tab row.

## Foyer 0.26.8

- Requests: the number of requests you have left (movies and series) now shows as two slim pills under the tabs on every Requests page, not just Discover. Each has a bar that drains as you use requests, and turns amber when low and red at zero.
- Android: phones and tablets use the bottom navigation bar in landscape too (including a foldable's inner screen), instead of the sidebar. It still sits above the system bar / taskbar.
- Android: the "Hide", "Remove from Continue Watching" and "Report a problem" buttons stack at their natural width instead of stretching across the whole page.

## Foyer 0.26.7

- Episode and movie pages get a Subtitles drop-down beside Audio. It always shows (Off at minimum), and the language you pick is remembered: it is chosen first on every other title that has it, ahead of English and the file's own default. Picking Off is remembered too. Picture-based subtitles (e.g. PGS) aren't listed here; use the player's subtitle menu for those.
- The Audio drop-down now shows even when a file has only one audio track.
- Detail pages on foldables and other mid-width screens: the header grows to fit its buttons, so the watched, favourite and Play buttons no longer slide under the next section.
- Posters that failed to load (a dropped connection) are retried automatically, and a season or episode with no picture of its own shows its series' poster instead of a blank box.

## Foyer 0.26.6

- Press and hold a poster (Android) or right-click it (Windows) for a menu: Play / Resume, Open details, Mark watched / unwatched, Add to / Remove from favourites, Follow for new episodes, Remove from Continue Watching, and Request on Seerr titles. It only offers what Foyer can do for that title.
- Series and seasons now have watched and favourite buttons on their detail page (only episodes and movies did before).
- Android: real picture-in-picture. The PiP button in the player shrinks Foyer into Android's floating video window, and on Android 12+ pressing Home or swiping up while a video plays does it automatically.
- Android: playback now registers as a media player, so the notification shade, lock screen and headset buttons show the title and control play, pause, previous, next and seek.
- "Notifications" is now "Alerts" everywhere (navigation, page heading, settings).
- Build: the Android build now uses a patched cargo-apk (adds the picture-in-picture manifest flag) and compiles a tiny Java helper for the media controls; see `.github/scripts`.

## Foyer 0.26.5

- Android: the Back button/gesture now goes to the previous page instead of closing the app (and closes the side menu first).
- Android: the bottom tab bar clears the system navigation bar and gesture area (foldable inner screens, three-button navigation).
- Android: the centre pause/play button is drawn as shapes, so it no longer changes appearance.
- Android: detail pages on foldable/medium-width screens stack the picture above the text so no buttons are cut off.
- Player: a live clock (12 or 24 hour) in the top right that slides into the corner when the controls fade; toggle in Settings. Works on both platforms.
- Windows: resizing the picture-in-picture window keeps the opposite corner fixed.
- Requests moved up to sit under Favourites in the navigation.
- Android: mini player/PiP buttons stay hidden (real Android PiP needs a packaging change).

## Foyer 0.26.4

### 🐞 Fixes

- **Windows notifications say "Foyer".** Toasts showed up as coming from "Windows PowerShell"; Foyer now registers its own name and icon with Windows at start-up so they are labelled correctly.
- **Notifications page on a phone.** The Mark all read / Clear all buttons sit under the title instead of being cut off, the title lines up with them, and each notification grows to fit its text instead of spilling out of its card. On a phone the hint says "tap to open" instead of "click to open".

## Foyer 0.26.3

### ✨ Highlights

- **Android: the keyboard no longer hides what you type.** When the on-screen keyboard opens, the sign-in card and the page above it move up so you can see the field (and the bottom bar steps out of the way). This covers the Foyer and Requests sign-ins.
- **Android: videos play in landscape.** A phone or tablet turns to landscape when playback starts and goes back to normal when it stops, so the whole control bar is always visible.
- **Android: a big play / pause button** in the middle of the video while the controls are showing. A tap on the video shows or hides the controls.
- **A proper Android app icon** that fills your launcher's shape (round, square or squircle) instead of sitting on a white circle.

### 🐞 Fixes

- **After watching, you land on the episode you finished on** (Windows and Android). If you started an episode from its page and autoplay carried on to later ones, backing out of the player now shows the last episode you watched, not the one you started on.
- **Windows: the picture-in-picture window can be resized again.** It has no frame, so there was nothing to grab; dragging any edge or corner now resizes it (it keeps the video's shape).
- Android: the in-app mini player and the shrink-the-window picture-in-picture are desktop features; their buttons no longer appear on Android, where they left the screen broken. Real system picture-in-picture for Android needs changes to how the app is packaged and is planned.

## Foyer 0.26.2

### ✨ Highlights

- **Android: a proper phone layout.** Held upright, the navigation moves to a bar along the bottom (Home, Search, Favourites, Alerts, More; *More* opens the full menu with your libraries). In landscape the side bar stays.
- **Settings fit small screens.** Each setting's control now sits under its text on a phone instead of running off the edge, and the descriptions no longer overlap. Options that only make sense on Windows (tray, F11, keep on top, window material, decoding method) are hidden on Android, and notifications are labelled as system notifications.
- **Title pages fit a phone.** On narrow screens (movie, episode and series pages, and request pages) the picture sits on top with the text and buttons underneath, and the buttons stack instead of running off the edge. The library's sort and filter menus and pop-up messages also shrink to fit.
- **Readable menus.** The navigation bar and menu are solid on Android instead of see-through, so text over busy artwork stays legible.
- **Material You colours on Android.** Settings → Appearance → Accent colour can follow your phone's wallpaper colour (Android 12 and newer), and does by default on a new install. The colour picker fits a phone screen.
- **App icon on Android.**

### 🐞 Fixes

- **Scrolling rows no longer fight the page.** Dragging a row of posters sideways used to scroll the whole page down if your finger drifted even slightly, which made rows hard to swipe on a phone and unreliable to drag with a mouse on Windows. A drag now goes to whichever way you are mostly moving, and rows follow your finger instead of lagging behind it. The Requests page no longer pans sideways as a whole.
- **Requests on a phone.** The Seerr sign-in box is centred, and the Requests header, tabs and search box fit the screen (the tabs scroll sideways on their own).
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
