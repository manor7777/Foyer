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
