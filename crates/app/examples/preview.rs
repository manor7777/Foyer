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
            overview: "Burnham and Sarek are stranded after a mission goes wrong; Lorca discovers a connection with a stranger who knows far more about the ship than he lets on, and tensions across the crew boil over as the Discovery races to reach them in time.".into(),
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
    if page == "notifications" {
        let mk = |id: &str, title: &str, body: &str, read: bool| foyer::NoteRow { id: id.into(), title: title.into(), body: body.into(), item: "x".into(), when: "Today 17:29".into(), read, ..Default::default() };
        let rows = vec![
            mk("1", "Update available", "Foyer 0.26.3 is ready to install (Settings → About).", false),
            mk("2", "New episode", "Star Trek: Discovery S1:E7 is available.", false),
            mk("3", "Request ready", "Avengers: Doomsday is now in your library.", true),
        ];
        ui.set_notes(slint::ModelRc::new(slint::VecModel::from(rows)));
        ui.set_unread_count(2);
    }
    if page == "sr-discover" {
        ui.set_sr_quota("Movies: unlimited  ·  Series: unlimited".into());
        ui.set_sr_user("Norman".into());
        ui.set_sr_manage_users(true);
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
