//! TV remote: the D-pad moves focus to the nearest control in the pressed direction.
//!
//! Slint only knows Tab order, which on a page of rows means Down walks every card in the row before
//! reaching the next one. This collects the focusable items with their window positions and picks the
//! best one in the pressed direction, the way Android TV apps behave. Focusing an item scrolls it into
//! view (Slint does that on focus), so rows and pages follow the focus.

use i_slint_core::item_tree::ItemRc;
use i_slint_core::items::{FocusReason, FocusScope, TextInput, TouchArea};
use i_slint_core::window::WindowInner;

/// Directions as sent from the UI's `dpad(int)` callback.
pub const UP: i32 = 0;
pub const RIGHT: i32 = 1;
pub const DOWN: i32 = 2;
pub const LEFT: i32 = 3;

/// Anything starting left of this (logical px) is in the navigation rail (collapsed it is 10..66).
const NAV_EDGE: f32 = 70.0;

#[derive(Clone, Copy, Debug)]
struct R {
    x0: f32,
    y0: f32,
    x1: f32,
    y1: f32,
}

impl R {
    fn cx(&self) -> f32 {
        (self.x0 + self.x1) / 2.0
    }
    fn cy(&self) -> f32 {
        (self.y0 + self.y1) / 2.0
    }
    fn in_nav(&self) -> bool {
        self.x0 < NAV_EDGE
    }
}

fn rect(item: &ItemRc) -> R {
    let g = item.geometry();
    let p = item.map_to_native_window(g.origin);
    R { x0: p.x, y0: p.y, x1: p.x + g.size.width, y1: p.y + g.size.height }
}

/// Root of what the remote can reach: the topmost open popup menu, else the window.
fn root(win: &WindowInner) -> Option<ItemRc> {
    if let Some(p) = win.active_popups().last() {
        return Some(ItemRc::new_root(p.component.clone()));
    }
    win.try_component().map(ItemRc::new_root)
}

fn focusable(item: &ItemRc) -> bool {
    if let Some(fs) = item.downcast::<FocusScope>() {
        let fs = fs.as_pin_ref();
        if !fs.enabled() || !fs.focus_on_tab_navigation() {
            return false;
        }
    } else if let Some(ti) = item.downcast::<TextInput>() {
        if !ti.as_pin_ref().enabled() {
            return false;
        }
    } else {
        return false;
    }
    item.is_visible_or_clipped_by_flickable()
}

/// Every focusable item with its window rectangle (skipping full-window catch-all scopes).
fn candidates(win: &WindowInner, size: (f32, f32)) -> Vec<(ItemRc, R)> {
    let mut out = Vec::new();
    let Some(start) = root(win) else { return out };
    let mut cur = start.clone();
    for _ in 0..50_000 {
        // A visible window-sized touch area is a dialog's backdrop (or the video under the player's
        // controls): everything before it is underneath, so the remote stays inside the dialog.
        if cur.downcast::<TouchArea>().is_some() && cur.is_visible() {
            let r = rect(&cur);
            if r.x1 - r.x0 >= size.0 * 0.9 && r.y1 - r.y0 >= size.1 * 0.9 {
                out.clear();
            }
        }
        if focusable(&cur) {
            let r = rect(&cur);
            let big = r.x1 - r.x0 >= size.0 * 0.9 && r.y1 - r.y0 >= size.1 * 0.9;
            if !big && r.x1 > r.x0 && r.y1 > r.y0 {
                out.push((cur.clone(), r));
            }
        }
        cur = cur.next_focus_item();
        if cur == start {
            break;
        }
    }
    out
}

fn window_size(window: &slint::Window) -> (f32, f32) {
    let s = window.size().to_logical(window.scale_factor());
    (s.width, s.height)
}

/// The focused item and its rectangle, unless nothing (or only a full-window catch-all) has focus.
fn current(window: &slint::Window) -> Option<(ItemRc, R)> {
    let win = WindowInner::from_pub(window);
    let item = win.focus_item.borrow().upgrade()?;
    if !item.is_visible() {
        return None;
    }
    let r = rect(&item);
    let (w, h) = window_size(window);
    (!(r.x1 - r.x0 >= w * 0.9 && r.y1 - r.y0 >= h * 0.9)).then_some((item, r))
}

fn focus(window: &slint::Window, item: &ItemRc) {
    WindowInner::from_pub(window).set_focus_item(item, true, FocusReason::TabNavigation);
}

/// True when a real control (not the window's catch-all key handler) has focus.
pub fn has_focus(window: &slint::Window) -> bool {
    current(window).is_some()
}

/// True when the focused control sits in the navigation rail.
pub fn focus_in_nav(window: &slint::Window) -> bool {
    current(window).is_some_and(|(_, r)| r.in_nav())
}

/// Moves focus one step in `dir`. Returns false when there is nothing that way.
/// With nothing focused, focuses a starting point instead (see `focus_start`).
pub fn step(window: &slint::Window, dir: i32) -> bool {
    let Some((cur_item, r)) = current(window) else {
        return focus_start(window, false);
    };
    let win = WindowInner::from_pub(window);
    let all = candidates(win, window_size(window));
    // Focus is outside what the remote can reach now (a menu just opened over it): start in the menu.
    if !all.iter().any(|(i, _)| *i == cur_item) {
        return focus_start(window, false);
    }
    let in_nav = r.in_nav() && win.active_popups().is_empty();
    let mut best: Option<(f32, &ItemRc)> = None;
    for (item, c) in &all {
        if *item == cur_item {
            continue;
        }
        // Up / Down inside the navigation rail stays in the rail.
        if in_nav && (dir == UP || dir == DOWN) && !c.in_nav() {
            continue;
        }
        // distance along the pressed direction, and how far off to the side
        let (along, side, centre) = match dir {
            UP if c.cy() < r.cy() - 1.0 && c.y1 <= r.y1 => ((r.y0 - c.y1).max(0.0), gap(c.x0, c.x1, r.x0, r.x1), (c.cx() - r.cx()).abs()),
            DOWN if c.cy() > r.cy() + 1.0 && c.y0 >= r.y0 => ((c.y0 - r.y1).max(0.0), gap(c.x0, c.x1, r.x0, r.x1), (c.cx() - r.cx()).abs()),
            LEFT if c.cx() < r.cx() - 1.0 && c.x1 <= r.x1 => ((r.x0 - c.x1).max(0.0), gap(c.y0, c.y1, r.y0, r.y1), (c.cy() - r.cy()).abs()),
            RIGHT if c.cx() > r.cx() + 1.0 && c.x0 >= r.x0 => ((c.x0 - r.x1).max(0.0), gap(c.y0, c.y1, r.y0, r.y1), (c.cy() - r.cy()).abs()),
            _ => continue,
        };
        // Something in line beats something closer but off to the side; among those in line, the
        // nearest wins, then the most centred.
        let score = along + side * 4.0 + centre * 0.1;
        if best.is_none_or(|(s, _)| score < s) {
            best = Some((score, item));
        }
    }
    match best {
        Some((_, item)) => {
            focus(window, item);
            true
        }
        None => false,
    }
}

/// Gap between two ranges (0 when they overlap).
fn gap(a0: f32, a1: f32, b0: f32, b1: f32) -> f32 {
    (a0 - b1).max(b0 - a1).max(0.0)
}

/// After a dialog opened or closed: if the focus is not somewhere the remote can reach (under the
/// dialog, or gone), move it to the start of what is reachable.
pub fn settle(window: &slint::Window) {
    let reachable = current(window).is_some_and(|(item, _)| {
        let win = WindowInner::from_pub(window);
        candidates(win, window_size(window)).iter().any(|(i, _)| *i == item)
    });
    if !reachable {
        focus_start(window, false);
    }
}

/// Focuses a starting point: the top of the navigation rail when `nav`, else the first control of the
/// page (top-left), falling back to whatever exists.
pub fn focus_start(window: &slint::Window, nav: bool) -> bool {
    let win = WindowInner::from_pub(window);
    let all = candidates(win, window_size(window));
    let key = |r: &R| (r.y0 / 8.0).floor() * 10_000.0 + r.x0;
    let pick = |want_nav: bool| {
        all.iter().filter(|(_, r)| r.in_nav() == want_nav).min_by(|a, b| key(&a.1).total_cmp(&key(&b.1)))
    };
    let first = pick(nav).or_else(|| pick(!nav));
    match first {
        Some((item, _)) => {
            focus(window, item);
            true
        }
        None => false,
    }
}
