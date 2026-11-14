## graphify

This project has a knowledge graph at graphify-out/ with god nodes, community structure, and cross-file relationships.

When the user types `/graphify`, use the installed graphify skill or instructions before doing anything else.

Rules:
- When graphify-out/graph.json exists, run `graphify query "<question>"` before your first grep, search or source-file read for a codebase question, and put this rule in every subagent prompt that explores code. Read raw files only for lines the graph or an earlier read already located. Use `graphify path "<A>" "<B>"` for relationships and `graphify explain "<concept>"` for focused concepts. These return a scoped subgraph, usually much smaller than GRAPH_REPORT.md or raw grep output.
- Dirty graphify-out/ files are expected after hooks or incremental updates; dirty graph files are not a reason to skip graphify. Only skip graphify if the task is about stale or incorrect graph output, or the user explicitly says not to use it.
- If graphify-out/wiki/index.md exists, use it for broad navigation instead of raw source browsing.
- Read graphify-out/GRAPH_REPORT.md only for broad architecture review or when query/path/explain do not surface enough context.
- After modifying code, run `graphify update .` to keep the graph current (AST-only, no API cost).
- A committed graphify-out/graph.json is rewritten whole on every build, so a large diff to it in a pull request is expected. Do not flag it as churn or suggest skipping the graph rebuild or its hooks.
- When you commit and push, include the graphify-out/ files (cache, hook session files, graph updates) and AGENTS.md / CLAUDE.md changes too, so they never linger as uncommitted changes.

## Android

Foyer ships for Windows (`Foyer.exe`) and Android phones, tablets and TV (`Foyer.apk`, one universal APK) from this one codebase, and every release publishes both. **Android is not a side project: it is a first-class target of `main`.**

### Every feature works on both, and lands on `main`

- **Every feature and fix must work on Android as well as Windows, in the same change.** Do not add a Windows-only feature and leave Android for later. If a feature genuinely cannot work on a platform (for example a tray icon), give that platform a sensible equivalent or a quiet no-op, hide any setting that would do nothing there (see `Device.desktop` in the UI), and say so in `CHANGELOG.md`.
- **Work goes to `main`.** The `android` branch is only a scratch branch we occasionally use while building Android-specific changes. Never leave work stranded on it: merge it into `main` (fast-forward when possible) so `main` always contains everything, and always release from `main`. Do not keep Android code that exists only on a branch.
- Releases come from `main` and publish both artifacts together; if either build fails, nothing is published.

### Rules for keeping it compatible

- Anything specific to an operating system (file locations, registry, winit window calls, tray icon, toasts, installers, `.dll` loading, `std::process::Command` launching desktop tools) goes in `crates/app/src/platform/` (Windows, desktop and Android backends), never inline in `lib.rs`. Windows-only crates stay under `[target.'cfg(windows)'.dependencies]`. Logic that does not touch the UI or the OS belongs in `crates/core`.
- New UI must work with touch and with a TV remote's D-pad (focusable, large enough), not only with a mouse and hover. Lay pages out so they also fit a phone held upright (about 360 px wide): use `root.narrow`, `Device.touch` / `Device.tv` / `Device.desktop`, and avoid fixed widths that can push a page wider than the screen. Slint cannot size a row from wrapped text inside a layout; see `SettingRow` and the detail-page hero for the pattern that works.
- Check layouts without a device using the headless preview: `cargo run -p foyer --example preview -- <page> <width_px> <height_px> <scale> <touch|tv|desktop> out.png` (for example a phone is `900 2000 2.5 touch`). Look at both a phone-sized and a desktop-sized render of anything you change.
- Before pushing, run `cargo check --workspace --all-targets --locked` and `cargo test --workspace --locked` (Linux). The Android workflow runs on every push and pull request and builds the APK; if it fails, fix that before anything else. `docs/ANDROID-PLAN.md` tracks what is done and what is next.
- Every Android build you hand to someone to install gets a new patch version (and a matching `## Foyer X.Y.Z` section in `CHANGELOG.md`): some phones refuse to install an update with the same version number as the installed app.
- `vendor/i-slint-core` is Slint with a one-function change to drag handling (see `vendor/i-slint-core/FOYER_PATCH.md`). Do not edit it casually; when updating Slint, re-apply the patch or check whether upstream has fixed it.
- Disk in cloud sessions is limited: build with `CARGO_INCREMENTAL=0` and delete `target/debug/incremental` if the disk fills.
