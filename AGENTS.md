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

Foyer ships for Windows (`Foyer.exe`) and Android phones and TV (`Foyer.apk`, one universal APK) from this one codebase, and every release publishes both.

- Keep every change Android-compatible. Anything specific to an operating system (file locations, registry, winit window calls, tray icon, toasts, installers, `.dll` loading, `std::process::Command` launching desktop tools) goes in `crates/app/src/platform/` (Windows, desktop and Android backends), never inline in `lib.rs`. Windows-only crates stay under `[target.'cfg(windows)'.dependencies]`.
- New UI must work with touch and with a TV remote's D-pad (focusable, large enough), not only with a mouse and hover.
- Before pushing, run `cargo check --workspace --all-targets` (Linux). The Android workflow runs on every push and pull request and builds the APK; if it fails, fix that before anything else. `docs/ANDROID-PLAN.md` tracks what is done and what is next.
- Every Android build you hand to someone to install gets a new patch version (and a matching `## Foyer X.Y.Z` section in `CHANGELOG.md`): some phones refuse to install an update with the same version number as the installed app.
- `vendor/i-slint-core` is Slint with a one-function change to drag handling (see `vendor/i-slint-core/FOYER_PATCH.md`). Do not edit it casually; when updating Slint, re-apply the patch or check whether upstream has fixed it.
