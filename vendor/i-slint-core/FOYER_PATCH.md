# Foyer's patch to i-slint-core 1.18.1

This is `i-slint-core` 1.18.1 from crates.io with one change, used through `[patch.crates-io]` in the
workspace `Cargo.toml`. Slint is used under its Royalty-free 2.0 licence (see `LICENSES/`), which allows
modifying and distributing it as part of an application.

## What changed

`items/flickable.rs`, `Flickable::should_capture_mouse_direction`: a Flickable now takes over a drag only
along the axis the pointer has mostly moved in (horizontal on ties, vertical when clearly vertical).

Upstream checks each axis independently, so a vertically scrolling page stole any drag that moved more
than 8 px vertically, even a clearly sideways swipe started on a horizontally scrolling row. On a phone
that made rows hard to swipe and the page drift down; with a mouse it made dragging a row unreliable.

## Updating Slint

When bumping the `slint` version, take the new `i-slint-core` from crates.io, re-apply the change (look
for `FOYER PATCH`), and check whether upstream has fixed this (then delete `vendor/` and the patch entry).
