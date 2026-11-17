//! Renders a screen of the UI to a PNG with the software renderer, for checking layouts without a
//! device: `cargo run -p foyer --example preview -- <page> <width_px> <height_px> <scale> <touch|tv|desktop> <out.png>`
use slint::platform::software_renderer::{MinimalSoftwareWindow, PremultipliedRgbaColor, RepaintBufferType};
use slint::platform::{Platform, WindowAdapter, WindowEvent};
use slint::{ComponentHandle, PhysicalSize, PlatformError};
use std::rc::Rc;

struct P(Rc<MinimalSoftwareWindow>);
impl Platform for P {
    fn create_window_adapter(&self) -> Result<Rc<dyn WindowAdapter>, PlatformError> {
        Ok(self.0.clone())
    }
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let page = a.get(1).cloned().unwrap_or_else(|| "settings".into());
    let (w, h): (u32, u32) = (a.get(2).and_then(|s| s.parse().ok()).unwrap_or(900), a.get(3).and_then(|s| s.parse().ok()).unwrap_or(2000));
    let scale: f32 = a.get(4).and_then(|s| s.parse().ok()).unwrap_or(2.5);
    let kind = a.get(5).cloned().unwrap_or_else(|| "touch".into());
    let out = a.get(6).cloned().unwrap_or_else(|| "preview.png".into());

    let window = MinimalSoftwareWindow::new(RepaintBufferType::NewBuffer);
    slint::platform::set_platform(Box::new(P(window.clone()))).unwrap();
    let ui = foyer::AppWindow::new().unwrap();
    window.dispatch_event(WindowEvent::ScaleFactorChanged { scale_factor: scale });
    window.set_size(PhysicalSize::new(w, h));
    let device = ui.global::<foyer::Device>();
    device.set_touch(kind == "touch");
    device.set_tv(kind == "tv");
    if page == "detail" {
        ui.set_detail(foyer::Detail {
            id: "1".into(),
            kind: "Episode".into(),
            title: "Lethe".into(),
            subtitle: "Star Trek: Discovery S1:E6".into(),
            meta: "7.3 · 21 Oct 2017 · 44m · Ends at 15:29".into(),
            summary: "1080p H264 · English EAC3 5.1 (Default) · 2 subtitle tracks".into(),
            overview: "Burnham and Sarek are stranded after a mission goes wrong; Lorca discovers a connection with a stranger who knows far more about the ship than he lets on, and tensions across the crew boil over as the Discovery races to reach them in time.".repeat(if std::env::var("PREVIEW_LONG").is_ok() { 5 } else { 1 }).into(),
            playable: true,
            can_resume: true,
            progress: 0.3,
            remaining: "31m remaining".into(),
            ..Default::default()
        });
        let genres: Vec<slint::SharedString> = vec!["Drama".into(), "Science Fiction".into(), "Adventure".into()];
        ui.set_detail_genres(slint::ModelRc::new(slint::VecModel::from(genres)));
        ui.set_audio_options(slint::ModelRc::new(slint::VecModel::from(vec![slint::SharedString::from("English - EAC3 5.1 (Default)")])));
        ui.set_sub_options(slint::ModelRc::new(slint::VecModel::from(vec![slint::SharedString::from("Off"), slint::SharedString::from("English - SubRip")])));
        ui.set_sub_index(1);
        ui.set_can_go_back(true);
    }
    if std::env::var("PREVIEW_ACCENT").is_ok() {
        ui.set_accent_mode(1);
    }
    if page == "player" {
        ui.set_playing(true);
        ui.set_player_started(true);
        ui.set_controls_shown(std::env::var("PREVIEW_HIDE").is_err());
        ui.set_player_paused(std::env::var("PREVIEW_PAUSED").is_ok());
        ui.set_player_title("Star Trek: Discovery S1:E7 Magic to Make the Sanest Man Go Mad".into());
        ui.set_clock_text("12:00am".into());
        ui.set_player_pos(1680.0);
        ui.set_player_dur(2849.0);
    }
    if page == "downloads" {
        use foyer::DownloadsUi;
        let row = |t: &str, st: &str, p: f32, state: i32, off: bool| foyer::DownloadRow { id: "x".into(), title: t.into(), status: st.into(), progress: p, state, offline: off };
        let g = ui.global::<DownloadsUi>();
        g.set_allowed(true);
        g.set_any(true);
        g.set_rows(slint::ModelRc::new(slint::VecModel::from(vec![
            row("Star Trek: Discovery S1:E7 Magic to Make the Sanest Man Go Mad", "42% · 610 MB of 1.4 GB", 0.42, 1, true),
            row("Avengers: Doomsday", "Ready to watch offline · 3.2 GB", 1.0, 2, true),
            row("Dune: Part Two", "Saved to Downloads/Foyer/Dune.mkv", 1.0, 2, false),
            row("Severance S2:E3 Who Is Alive?", "Failed: Interrupted", 0.0, 3, true),
        ])));
        if std::env::var("PREVIEW_OFFLINE").is_ok() {
            ui.set_offline(true);
        }
    }
    if page == "notifications" {
        let mk = |id: &str, title: &str, body: &str, read: bool| foyer::NoteRow { id: id.into(), title: title.into(), body: body.into(), item: "x".into(), when: "Today 17:29".into(), read, ..Default::default() };
        // A stand-in poster: a gradient block.
        let poster = || {
            let (w, h) = (92u32, 138u32);
            let mut buf = slint::SharedPixelBuffer::<slint::Rgba8Pixel>::new(w, h);
            for (i, px) in buf.make_mut_slice().iter_mut().enumerate() {
                let (x, y) = ((i as u32 % w) as u8, (i as u32 / w) as u8);
                *px = slint::Rgba8Pixel { r: 40 + x, g: 90 + y / 2, b: 180, a: 255 };
            }
            slint::Image::from_rgba8(buf)
        };
        if std::env::var("PREVIEW_NOTE").is_ok() {
            ui.set_note_title("New episode of Star Trek: Discovery".into());
            ui.set_note_body("S1 E7 is ready to watch.".into());
            ui.set_note_item("x".into());
            ui.set_note_play("x".into());
            ui.set_note_image(poster());
            ui.set_note_open(true);
        }
        let rows = vec![
            mk("1", "Update available", "Foyer 0.26.3 is ready to install (Settings → About).", false),
            { let mut r = mk("2", "New episode", "Star Trek: Discovery S1:E7 is available.", false); r.image = poster(); r },
            mk("3", "Request ready", "Avengers: Doomsday is now in your library.", true),
        ];
        ui.set_notes(slint::ModelRc::new(slint::VecModel::from(rows)));
        ui.set_unread_count(2);
    }
    if page == "sr-discover" {
        ui.set_sr_quota("Movies: unlimited  ·  Series: unlimited".into());
        {
            use foyer::SrQuota;
            let g = ui.global::<SrQuota>();
            g.set_shown(true);
            g.set_movie_text("Movie: 3 of 5 left".into());
            g.set_movie_frac(0.6);
            g.set_movie_limited(true);
            g.set_tv_text("Series: 0 of 3 left".into());
            g.set_tv_frac(0.0);
            g.set_tv_limited(true);
        }
        ui.set_sr_user("Norman".into());
        ui.set_sr_manage_users(true);
    }
    if std::env::var("PREVIEW_DESC_ICON").is_ok() {
        ui.set_desc_icon(true);
        ui.set_desc_open(true);
    }
    if std::env::var("PREVIEW_CAST").is_ok() {
        let card = |n: &str| foyer::Card { id: "x".into(), kind: "Person".into(), title: n.into(), subtitle: "Actor".into(), ..Default::default() };
        let cards: Vec<foyer::Card> = ["Ann", "Bo", "Cy", "Di", "Ed", "Flo", "Gus"].iter().map(|n| card(n)).collect();
        ui.set_detail_cast(slint::ModelRc::new(slint::VecModel::from(cards)));
    }
    if std::env::var("PREVIEW_REPORT").is_ok() {
        use foyer::PickRow;
        let rows = |v: &[(&str, bool)]| slint::ModelRc::new(slint::VecModel::from(v.iter().map(|(l, on)| PickRow { label: (*l).into(), on: *on }).collect::<Vec<_>>()));
        ui.set_sr_report_title("Star Trek: Discovery".into());
        ui.set_sr_report_tv(true);
        ui.set_sr_report_kinds(rows(&[("Video", true), ("Audio", true), ("Subtitles", false), ("Other", false)]));
        ui.set_sr_report_kinds_text("Video, Audio".into());
        ui.set_sr_report_seasons(slint::ModelRc::new(slint::VecModel::from(vec![slint::SharedString::from("All seasons"), "Season 1".into(), "Season 2".into()])));
        ui.set_sr_report_season_index(1);
        ui.set_sr_report_eps(rows(&[("Episode 1", false), ("Episode 2", true), ("Episode 3", true), ("Episode 4", false)]));
        ui.set_sr_report_eps_text("Episodes 2-3".into());
        ui.set_sr_report_open(true);
    }
    if std::env::var("PREVIEW_NAV_OPEN").is_ok() {
        ui.set_nav_open(true);
    }
    ui.set_page(page.into());
    ui.show().unwrap();
    slint::platform::update_timers_and_animations();
    let mut buf = vec![PremultipliedRgbaColor::default(); (w * h) as usize];
    for _ in 0..3 {
        slint::platform::update_timers_and_animations();
        window.draw_if_needed(|r| {
            r.render(&mut buf, w as usize);
        });
    }
    let mut img = image::RgbaImage::new(w, h);
    for (i, p) in buf.iter().enumerate() {
        img.put_pixel((i as u32) % w, (i as u32) / w, image::Rgba([p.red, p.green, p.blue, 255]));
    }
    img.save(&out).unwrap();
    println!("wrote {out}");
}
